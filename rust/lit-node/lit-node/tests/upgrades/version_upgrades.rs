use crate::common::assertions::NetworkIntegrityChecker; // version::get_crate_version};
use async_std::stream::StreamExt;
use ethers::types::U256;
use futures::future::BoxFuture;
use lit_node_testnet::{
    DEFAULT_KEY_SET_NAME, TestSetupBuilder,
    node_collection::get_node_versions,
    testnet::{BeforeStartValidatorsFn, actions::Actions},
    validator::ValidatorCollection,
};
use std::{fs, io::Write};
use test_case::test_case;
use tracing::info;

struct UpgradeStepData {
    pub upgrade_round: usize,
    pub initial_node_count: usize,
    pub initial_node_versions: Vec<String>,
    pub realm_id: U256,
    pub epoch_length: usize,
}

#[test_case("2.1.5", false; "Upgrade against the latest NAGA-Prod release branch, assuming chain state was updated manually.")]
#[tokio::test]
async fn test_version_upgrade_against_old_version(
    release_version: &str,
    use_old_chain_state: bool,
) {
    crate::common::setup_logging();
    info!("TEST: Upgrade against release: {}", release_version);

    // First check if we have the build.
    let release_build_path = format!("./target/{}/debug/lit_node", release_version);

    if fs::metadata(&release_build_path).is_ok() {
        info!(
            "Build exists at {}, skipping download...",
            release_build_path
        );
    } else {
        info!(
            "Build does not exist at {}, downloading...",
            release_build_path
        );
        download_release_build(release_version).await;
    }

    assert!(
        fs::metadata(&release_build_path).is_ok(),
        "Build does not exist at {}",
        release_build_path
    );

    if use_old_chain_state {
        // TODO if required: Implement old chain state setup, by passing a parameter to the chain state data.
    }

    let setup_function = before_start_validators_fn().await;

    let initial_node_count = 5;
    // Set up a network of nodes running the old build.
    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(initial_node_count)
        .custom_binary_path(Some(release_build_path))
        .max_presign_count(0)
        .min_presign_count(0)
        .force_deploy(true)
        .before_start_validators_fn(Some(setup_function))
        .build()
        .await;

    let actions = testnet.actions();
    let vc = validator_collection.clone();

    let complete_node_set = validator_collection
        .actions()
        .active_node_set()
        .await
        .unwrap();
    let initial_node_versions = get_node_versions(&complete_node_set).await;
    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;

    let mut upgrade_step_data = UpgradeStepData {
        upgrade_round: 0,
        initial_node_count,
        initial_node_versions,
        realm_id: U256::from(1),
        epoch_length: actions
            .get_epoch_length(U256::from(1))
            .await
            .unwrap()
            .as_u64() as usize,
    };

    info!(
        "Initial node versions: {:?}",
        upgrade_step_data.initial_node_versions
    );

    // Assert all node versions are the same.
    assert!(
        upgrade_step_data
            .initial_node_versions
            .iter()
            .all(|v| v == &upgrade_step_data.initial_node_versions[0])
    );

    info!("Validating initial network state");
    network_checker.check(&vc, &vec![]).await;

    // Keep dealing in new node versions and dealing out old node versions until the entire network is upgraded.
    for upgrade_round in 0..initial_node_count {
        info!("Upgrading node {} to the new build", upgrade_round + 1);
        upgrade_step_data.upgrade_round = upgrade_round;

        // select a random validator to upgrade
        let validator = validator_collection.get_validator_by_index_as_mut(upgrade_round);

        // request to leave
        validator
            .request_to_leave(&actions)
            .await
            .expect("Failed to request to leave");

        // advance and validate
        advance_and_validate_step(&actions, &network_checker, &upgrade_step_data, 1).await;

        validator.stop_node().expect("Failed to stop node");

        // we're going to use the same validator / staker, to match what we do in production.
        // force_search binary clears any custom binary paths - causing the test to use the binary from this branch ( rebuilding if required )
        validator.force_search_binary();

        validator
            .start_node(false, true)
            .await
            .expect("Failed to start node");

        // let new_validator = validator_collection.add_one(false, None, Some(U256::from(1))).await.unwrap();
        // request to join
        validator
            .request_to_join(&actions, U256::from(1))
            .await
            .expect("Failed to request to join");

        // test that we can advance and validate the step
        advance_and_validate_step(&actions, &network_checker, &upgrade_step_data, 0).await;
    }

    network_checker.check(&vc, &vec![]).await;

    uncomment_anvil_datil_chain_in_rpc_config().await;
}

async fn advance_and_validate_step(
    actions: &Actions,
    _network_checker: &NetworkIntegrityChecker,
    data: &UpgradeStepData,
    nodes_removed: usize,
) {
    let current_epoch = actions.get_current_epoch(data.realm_id).await;
    let next_epoch = current_epoch + 1;
    actions
        .increase_blockchain_timestamp(data.epoch_length)
        .await;

    // After next epoch arrives, run interpolation and decryption tests.
    actions.wait_for_epoch(data.realm_id, next_epoch).await;

    let _ = actions.clear_presigns().await;
    actions.sleep_millis(1000).await;

    // network_checker.check(validator_collection, &vec![]).await;

    let active_node_set = actions.active_node_set().await.unwrap();
    if nodes_removed == 0 {
        let mut node_versions = get_node_versions(&active_node_set).await;
        // Assert node versions.
        // assert_eq!(node_versions.len() - nodes_removed, data.initial_node_count);

        // Sort the node versions to make it easier to compare.
        node_versions.sort();
        info!(
            "node versions ({:?}) {:?} and initial node versions {:?}",
            node_versions.len(),
            node_versions,
            data.initial_node_versions
        );

        // Get current crate version.
        // let current_crate_version = get_crate_version();
        // for (i, version) in node_versions.iter().enumerate() {
        //     if i < (data.initial_node_count - data.upgrade_round) {
        //         assert_eq!(version, &data.initial_node_versions[0]);
        //     } else {
        //         assert_eq!(version.to_owned(), current_crate_version);
        //     }
        // }
    }
}

async fn download_release_build(release_version: &str) {
    let download_path = format!("./target/{}", release_version);
    let release_build_path = format!("./target/{}/debug/", release_version);
    let release_build_url = format!(
        "https://github.com/LIT-Protocol/lit-node-binary-releases/releases/download/{}/lit_node.tar.gz",
        release_version
    );
    let zip_name = format!("{}/lit_node.tar.gz", download_path);

    info!("Downloading {}...", release_build_url);

    let mut stream = reqwest::get(&release_build_url)
        .await
        .unwrap()
        .bytes_stream();

    fs::create_dir_all(&release_build_path).expect("Failed to create directory"); // includes the download path
    let mut file = std::fs::File::create(&zip_name).expect("Failed to create file");

    let mut total_downloaded: u64 = 0;
    let mut print_threshold: u64 = 5 * 1024 * 1024; // 5MB
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.expect("Failed to get stream from GitHub");
        total_downloaded += chunk.len() as u64;
        if total_downloaded >= print_threshold {
            info!("Downloaded {} kb.", total_downloaded / 1024);
            print_threshold += 5 * 1024 * 1024; // 5MB
        }
        file.write_all(&chunk)
            .expect("Failed to write to stream to local file");
    }

    file.flush().expect("Failed to flush file");
    info!("Downloaded {} to {}", release_build_url, download_path);

    info!("Unzipping {} to {}", zip_name, release_build_path);

    lit_core::utils::tar::read_tar_gz_file(&zip_name, &release_build_path)
        .expect("Failed to read tar.gz file");

    info!("Unzipped {} to {}", zip_name, download_path);
}

use lit_blockchain::contracts::staking::KeySetConfig;
async fn before_start_validators_fn()
-> Box<dyn BeforeStartValidatorsFn<Future = BoxFuture<'static, Result<(), anyhow::Error>>>> {
    let fut = Box::new(move |actions: Actions| {
        Box::pin(async move {
            // remove the last curve from the keyset, which isn't in the default node version 2.1.5 release, and will prevent the node from completing it's DKG.
            let mut keyset_config: KeySetConfig = actions
                .contracts()
                .staking
                .get_key_set(DEFAULT_KEY_SET_NAME.to_string())
                .await
                .unwrap();
            let curve_count = keyset_config.counts.len();
            keyset_config.counts = keyset_config
                .counts
                .iter()
                .take(curve_count - 8)
                .cloned()
                .collect();
            keyset_config.curves = keyset_config
                .curves
                .iter()
                .take(curve_count - 8)
                .cloned()
                .collect();

            actions
                .contracts()
                .staking
                .delete_key_set(keyset_config.identifier.clone())
                .await
                .unwrap();
            actions.add_keyset_config(keyset_config).await.unwrap();

            // read the rpc_config.yaml file and comment out the anvilDatil chain
            // this also causes the old nodes to fail to start ( won't affect the new nodes for THIS test )
            comment_out_anvil_datil_chain_in_rpc_config().await; // function here to increase blockchain timestamp by 1000 seconds
            Ok(())
        }) as BoxFuture<'static, Result<(), anyhow::Error>>
    });

    fut
}

async fn comment_out_anvil_datil_chain_in_rpc_config() {
    let rpc_config = fs::read_to_string("rpc-config.yaml").unwrap();
    let rpc_config = rpc_config
        .lines()
        .map(|line| {
            if line.contains("anvilDatil") && !line.starts_with("#") {
                format!("# {}", line)
            } else if line.contains(" http://127.0.0.1:8549") && !line.starts_with("#") {
                format!("# {}", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");
    fs::write("rpc-config.yaml", rpc_config).unwrap();
}

async fn uncomment_anvil_datil_chain_in_rpc_config() {
    let rpc_config = fs::read_to_string("rpc-config.yaml").unwrap();
    let rpc_config = rpc_config
        .lines()
        .map(|line| {
            if line.contains("anvilDatil") && line.starts_with("#") {
                line.to_string().replace("# ", "")
            } else if line.contains(" http://127.0.0.1:8549") && line.starts_with("#") {
                line.to_string().replace("# ", "")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");
    fs::write("rpc-config.yaml", rpc_config).unwrap();
}
