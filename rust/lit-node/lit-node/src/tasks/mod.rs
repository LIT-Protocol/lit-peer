pub mod chatter_sender;
pub mod endpoint_channels;
pub mod fsm;
pub mod fsm_worker;
mod payment;
pub mod presign_manager;
pub mod utils;

use crate::error::Result;
use crate::functions::{ActionStore, ActionWorker};
use crate::models::{AuthContextCache, DenoExecutionEnv};
use crate::payment::payment_tracker::PaymentTracker;
use crate::peers::peer_reviewer::{PeerComplaint, PeerReviewer};
use crate::siwe_db::db;
use crate::siwe_db::rpc::EthBlockhashCache;
use crate::tasks::fsm::node_fsm_worker;
use crate::tasks::payment::{batch_payment_processor, usage_processor};
use crate::tss::common::dkg_type::DkgType;
use crate::tss::common::models::RoundData;
use crate::tss::common::peer_checker::peer_checker_worker;
use crate::tss::common::restore::RestoreState;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::common::tss_state::TssState;
use lit_node_common::client_state::ClientState;
use lit_node_common::config::LitNodeConfig;

use lit_observability::channels::{TracedReceiver, TracedSender};
use moka::future::Cache;
use std::cmp::max;
use std::future::Future;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;

use crate::tss::dkg::manager::DkgManager;
use crate::version::DataVersionReader;
use endpoint_channels::rounds_worker;
use lit_blockchain::resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller};

pub struct Handle {
    join: Vec<thread::JoinHandle<()>>,
    quit: Vec<mpsc::Sender<bool>>,
}

impl Handle {
    fn new(join: Vec<thread::JoinHandle<()>>, quit: Vec<mpsc::Sender<bool>>) -> Self {
        Self { join, quit }
    }

    pub fn shutdown(self) {
        self.quit.iter().for_each(|f| {
            let _ = f.try_send(true);
        });
        for f in self.join {
            let _ = f.join();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn launch(
    eth_blockhash_cache: Arc<EthBlockhashCache>,
    pr_tx: TracedSender<PeerComplaint>,
    pr_rx: TracedReceiver<PeerComplaint>,
    auth_context_cache: Arc<AuthContextCache>,
    tss_state: Arc<TssState>,
    restore_state: Arc<RestoreState>,
    tx_round_manager: Arc<flume::Sender<RoundData>>,
    rx_round_manager: flume::Receiver<RoundData>,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
    payment_tracker: Arc<PaymentTracker>,
    ipfs_cache: Cache<String, Arc<String>>,
    action_store: ActionStore,
    client_state: Arc<ClientState>,
    http_client: reqwest::Client,
) -> Result<Handle> {
    // Dedicated runtime just for tasks.  Give at least 4 threads, but
    // up to physical cpu count.
    let worker_threads = max(4, num_cpus::get_physical());
    let cfg = tss_state.lit_config.load_full();

    info!("Starting node tasks (worker_threads: {worker_threads})");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("node-tasks")
        .worker_threads(worker_threads)
        .max_blocking_threads(512)
        .enable_all()
        .build()
        .expect("create tokio runtime");

    let (quit_tx, mut quit_rx) = mpsc::channel(1);

    let j = thread::spawn(move || {
        runtime.block_on(async move {
            let mut tasks = vec![];

            let chain_data_manager_clone = tss_state.chain_data_config_manager.clone();
            let chain_data_manager_clone2 = tss_state.chain_data_config_manager.clone();
            tasks.push(spawn(|q| async move {
                chain_data_manager_clone
                    .as_ref()
                    .watch_chain_data_config(q)
                    .await;
            }));

            tasks.push(spawn(|q| async move {
                eth_blockhash_cache
                    .as_ref()
                    .fetch_and_store_latest_blockhash(q)
                    .await;
            }));

            let cfg_clone = cfg.clone();
            tasks.push(spawn(|mut q| async move {
                let poll_interval =
                    cfg_clone.rpc_health_poll_interval().unwrap_or(60000) as u64;
                let mut interval = tokio::time::interval(Duration::from_millis(poll_interval));
                loop {
                    tokio::select! {
                        _ = q.recv() => {
                            break;
                        }
                        _ = interval.tick() => {
                            // Continue below.
                        }
                    }
                    let rpc_healthcheck_enabled = DataVersionReader::read_field_unchecked(
                        &chain_data_manager_clone2.generic_config,
                        |generic_config| generic_config.rpc_healthcheck_enabled,
                    );
                    if !rpc_healthcheck_enabled
                    {
                        continue;
                    }
                    ENDPOINT_MANAGER.poll_rpcs_for_latency().await;
                }
            }));

            match cfg.external_port() {
                Ok(port) => {
                    let http_cache_clone = http_client.clone();
                    tasks.push(spawn(move |q| async move {
                        if let Err(e) = db::init_fill_db(port, q, http_cache_clone).await {
                            error!("Error while fetching init blocks: {}", e);
                        }
                    }));
                }
                Err(e) => error!("Error getting external port from config: {}", e),
            }

            // the threads below read data from the chain data manager and are subject to throwing errors
            // if the CDM isn't loaded.   We'll pause here for a while
            while tss_state.chain_data_config_manager.get_staker_address()
                .is_zero()
            {
                debug!("Waiting for staker address to be set in CDM prior to launching background threads ... ");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            debug!(
                "Staker address is {:?}",
                tss_state.chain_data_config_manager.get_staker_address()
                    .to_string()
            );


            // start up FSM worker
            if matches!(cfg.enable_epoch_transitions(), Ok(true)) {
                // peer_reviewer
                let mut peer_reviewer = PeerReviewer::new(
                    pr_rx,
                    tss_state.peer_state.clone(),
                );

                tasks.push(spawn(|q| async move {
                    peer_reviewer.receive_complaints(q).await;
                }));

                // regular DKG
                let tss_state2 = tss_state.clone();
                let fsm_worker_metadata_shadow = fsm_worker_metadata.clone();
                let restore_state_shadow = restore_state.clone();
                let client_state_shadow = client_state.clone();
                tasks.push(spawn(|quit_rx| async move {
                    node_fsm_worker(
                        quit_rx,
                        false,
                        restore_state,
                        client_state.clone(),
                        DkgManager::new(tss_state2.clone(), DkgType::RecoveryParty),
                         DkgManager::new(tss_state2.clone(), DkgType::Standard),
                        fsm_worker_metadata,
                    )
                    .await;
                }));

                // Shadow DKG
                let tss_state2 = tss_state.clone();
                tasks.push(spawn(|quit_rx| async move {
                    node_fsm_worker(
                        quit_rx,
                        true,
                        restore_state_shadow,
                        client_state_shadow,
                        DkgManager::new(tss_state2.clone(), DkgType::RecoveryParty),
                         DkgManager::new(tss_state2.clone(), DkgType::Standard),
                        fsm_worker_metadata_shadow,
                    )
                    .await;
                }));


                let peer_state = tss_state.peer_state.clone();
                tasks.push(spawn(|quit_channel_rx| async move {
                    let res = peer_state.listen_for_events(quit_channel_rx).await;
                    if let Err(e) = res {
                        error!("Error listening for events: {}", e);
                    }
                }));
            } else {
                info!("Epoch transitions disabled, not starting FSM");
            }

            let payment_config = tss_state.lit_config.clone();
            let peer_state_for_payment = tss_state.peer_state.clone();
            let payment_tracker_for_payment = payment_tracker.clone();
            tasks.push(spawn(|quit_rx| async move {
                batch_payment_processor(quit_rx, payment_config, payment_tracker_for_payment, peer_state_for_payment).await;
            }));


            let usage_config = tss_state.lit_config.clone();
            let peer_state_for_usage = tss_state.peer_state.clone();
            tasks.push(spawn(|quit_rx| async move {
                usage_processor(quit_rx, usage_config, payment_tracker, peer_state_for_usage).await;
            }));

            // TSS workers
            {
                let peer_state = tss_state.peer_state.clone();
                tasks.push(spawn(move |q| async move {
                    peer_checker_worker(q, peer_state).await;
                }));

                let lit_config_for_rounds_queue = cfg.clone();
                tasks.push(spawn(|q| async move {
                    rounds_worker(
                        q,
                        lit_config_for_rounds_queue,
                        rx_round_manager,
                        tx_round_manager,
                    )
                    .await;
                }));
            }

            #[cfg(feature = "lit-actions")]
            {
                let lit_config = cfg.clone();
                let store = action_store.clone();
                let http_cache_clone = http_client.clone();

                tasks.push(spawn(|mut quit_channel_rx| async move {
                    info!("Starting: action job workers");

                    let signal = async move {
                        quit_channel_rx.recv().await;
                        info!("Stopped: action job workers");
                        Ok(())
                    };

                    let worker = ActionWorker::new(
                        store,
                        DenoExecutionEnv {
                            tss_state: Some(tss_state.as_ref().clone()),
                            cfg: lit_config,
                            ipfs_cache: Some(ipfs_cache),
                            http_client: Some(http_cache_clone),
                        },
                    );

                    if let Err(e) = worker.start_with_shutdown(signal).await {
                        error!("Error starting action job workers: {e:#}");
                    }
                }));

                tasks.push(spawn(|mut quit_channel_rx| async move {
                    info!("Starting: action job janitor");

                    // Run the janitor once at startup and then every 10 minutes
                    let mut interval = tokio::time::interval_at(tokio::time::Instant::now(), Duration::from_secs(10 * 60));
                    let max_job_age = Duration::from_secs(24 * 60 * 60); // 1 day

                    loop {
                        tokio::select! {
                            _ = quit_channel_rx.recv() => {
                                info!("Stopped: action job janitor");
                                break;
                            }
                            _ = interval.tick() => {
                                match action_store.delete_completed_jobs(max_job_age).await {
                                    Ok(count) if count > 0 => {
                                        info!("Deleted {count} completed action job(s) older than {max_job_age:?}");
                                    },
                                    Err(e) => {
                                        error!("Error deleting completed action jobs: {e:#}");
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }));
            }

            #[cfg(feature = "lit-actions-server")]
            tasks.push(spawn(|mut quit_channel_rx| async move {
                let socket = cfg
                    .actions_socket()
                    .expect("invalid socket path in config");

                info!("Starting: lit_actions server listening on {socket:?}");

                let signal = async move {
                    quit_channel_rx.recv().await;
                    info!("Stopped: lit_actions server");
                };

                if let Err(e) = lit_actions_server::start_server(socket, Some(signal)).await
                {
                    error!("Error starting lit_actions server: {e:#}");
                }
            }));

            let _ = quit_rx.recv().await;
            shutdown(tasks).await;
        });
    });

    Ok(Handle::new(vec![j], vec![quit_tx]))
}

fn spawn<F, Fut, T>(f: F) -> (mpsc::Sender<bool>, task::JoinHandle<T>)
where
    F: FnOnce(mpsc::Receiver<bool>) -> Fut,
    F: Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (quit_tx, quit_rx) = mpsc::channel(1);
    let j = tokio::spawn(f(quit_rx));

    (quit_tx, j)
}

async fn shutdown<T>(tasks: Vec<(mpsc::Sender<bool>, task::JoinHandle<T>)>)
where
    T: Send + 'static,
{
    // Send shutdown signal
    for (q, _) in tasks.iter() {
        let _ = q.try_send(true);
    }

    let shutdown_timeout = tokio::time::sleep(Duration::from_secs(60));
    tokio::pin!(shutdown_timeout);

    loop {
        let still_alive = tasks.iter().any(|(_, j)| !j.is_finished());
        if !still_alive {
            break;
        }

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(1)) => {}
            _ = &mut shutdown_timeout => {
                break;
            }
            _ = tokio::signal::ctrl_c() => {
                warn!("Ctrl-C received, forcing shutdown");
                break;
            }
        }
    }

    // Kill remaining
    for (_, j) in tasks {
        if !j.is_finished() {
            warn!("Task still alive after 60s, aborting");
            j.abort();
        }
    }
}
