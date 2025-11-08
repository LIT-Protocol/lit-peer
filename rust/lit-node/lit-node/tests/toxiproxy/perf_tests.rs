use crate::common::ecdsa::sign_with_hd_key;
use crate::common::faults::generate_and_save_proxy_mappings_for_local_testing;
use crate::common::faults::{inject_latency_fault, setup_proxies};
use crate::common::networking::get_local_url_from_port;
use crate::common::setup_logging;
use lit_node_common::proxy_mapping::ClientProxyMapping;
use lit_node_core::SigningScheme;
use lit_node_testnet::TestSetupBuilder;
use once_cell::sync::Lazy;
use std::io::BufRead;
use std::time::Duration;
use tracing::info;

const PERF_TEST_NUM_NODES: usize = 10;
const STARTING_PORT: usize = 7470;

static PROXY_MAPPINGS: Lazy<ClientProxyMapping> = Lazy::new(|| {
    generate_and_save_proxy_mappings_for_local_testing(PERF_TEST_NUM_NODES, STARTING_PORT).unwrap()
});

fn setup() {
    setup_logging();
    // Set up proxies
    setup_proxies(&PROXY_MAPPINGS);
}

// This is the baseline load test with no additional latency between nodes
#[tokio::test]
pub async fn load_with_no_latency() {
    setup_logging();
    info!("Starting test: load_with_no_latency");
    let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;

    let num_nodes = validator_collection.size();

    let pubkey = end_user.first_pkp().pubkey.clone();
    // open the log files
    let mut log_readers = validator_collection.log_readers();

    let _start = std::time::Instant::now();
    let _ = sign_with_hd_key(
        &validator_collection,
        &end_user,
        pubkey.clone(),
        false,
        true,
        1,
        Some("First Test message".to_string()),
        SigningScheme::EcdsaK256Sha256,
        &vec![],
    )
    .await;

    // give the nodes a few seconds to populate a pre-signature or two.
    let warmup_time = Duration::from_millis(30000);
    validator_collection
        .actions()
        .sleep_millis(warmup_time.as_millis() as u64)
        .await;

    // clear the log buffer
    for reader in log_readers.iter_mut() {
        let _lines = reader
            .lines()
            .map(|line| line.unwrap_or("".to_string()))
            .collect::<Vec<String>>();
    }
    let messages_to_sign = 10;

    let mut bt_cache_hit = 0;
    let mut bt_cache_hit_duration: Duration = Duration::from_millis(0);
    let mut bt_cache_miss = 0;
    let mut sign_success = 0;
    let mut bt_cache_miss_duration: Duration = Duration::from_millis(0);

    let start = std::time::Instant::now();
    for i in 0..messages_to_sign {
        info!("Starting sig #{}", i);
        let message_to_sign = Some(format!("Test message #{}", i));
        let start_1 = std::time::Instant::now();
        let validation = sign_with_hd_key(
            &validator_collection,
            &end_user,
            pubkey.clone(),
            false,
            false,
            1,
            message_to_sign,
            SigningScheme::EcdsaK256Sha256,
            &vec![],
        )
        .await;

        if validation {
            sign_success += 1;
        };

        'outer: for reader in &mut log_readers {
            let lines = reader
                .lines()
                .map(|line| line.unwrap_or("".to_string()))
                .collect::<Vec<String>>();

            for line in lines {
                if line.contains("BT Cache Hit") {
                    bt_cache_hit += 1;
                    bt_cache_hit_duration += start_1.elapsed();
                    break 'outer;
                }
                if line.contains("BT Cache Miss") {
                    bt_cache_miss += 1;
                    bt_cache_miss_duration += start_1.elapsed();
                    break 'outer;
                }
            }
        }
    }
    let total_elapsed = start.elapsed();
    info!(
        "
        Signing {} messages randomly in a {} node network 
        Pre-gen Pre-Signature  Warmup: {:?}
        Pre-Signature Cache Hit (qty/time): {} / {:?}
        Pre-Signature Cache Miss (qty/time): {} / {:?}
        Cache success: {:?} 
        Total Time elapsed: {:?} 
        Sign success: {:?} ",
        messages_to_sign,
        num_nodes,
        warmup_time,
        bt_cache_hit,
        bt_cache_hit_duration,
        bt_cache_miss,
        bt_cache_miss_duration,
        bt_cache_hit as f64 / messages_to_sign as f64,
        total_elapsed,
        sign_success
    );

    let validation = true;
    assert!(validation);
}

#[tokio::test]
pub async fn load_with_50ms_latency_single_link() {
    setup();
    info!("Starting test: load_with_50ms_latency");

    let latency_ms = 50;
    let jitter_ms = 0;
    let toxicity = 1.0;

    // Inject fault between node 1 and node 0
    inject_latency_fault(
        get_local_url_from_port(STARTING_PORT + 1),
        get_local_url_from_port(STARTING_PORT),
        latency_ms,
        jitter_ms,
        toxicity,
    );

    // Inject fault between node 0 and node 1
    inject_latency_fault(
        get_local_url_from_port(STARTING_PORT),
        get_local_url_from_port(STARTING_PORT + 1),
        latency_ms,
        jitter_ms,
        toxicity,
    );

    let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;
    let pubkey = end_user.first_pkp().pubkey.clone();
    let num_nodes = validator_collection.size();

    // open the log files
    let mut log_readers = validator_collection.log_readers();

    let _start = std::time::Instant::now();
    let _ = sign_with_hd_key(
        &validator_collection,
        &end_user,
        pubkey.clone(),
        false,
        true,
        1,
        Some("First Test message".to_string()),
        SigningScheme::EcdsaK256Sha256,
        &vec![],
    )
    .await;

    // give the nodes a few seconds to populate a triple or two.
    let warmup_time = Duration::from_millis(30000);
    validator_collection
        .actions()
        .sleep_millis(warmup_time.as_millis() as u64)
        .await;

    // clear the log buffer
    for reader in log_readers.iter_mut() {
        let _lines = reader
            .lines()
            .map(|line| line.unwrap_or("".to_string()))
            .collect::<Vec<String>>();
    }

    let messages_to_sign = 10;

    let mut bt_cache_hit = 0;
    let mut bt_cache_hit_duration: Duration = Duration::from_millis(0);
    let mut bt_cache_miss = 0;
    let mut sign_success = 0;
    let mut bt_cache_miss_duration: Duration = Duration::from_millis(0);

    let start = std::time::Instant::now();
    for i in 0..messages_to_sign {
        info!("Starting sig #{}", i);
        let message_to_sign = Some(format!("Test message #{}", i));
        let start_1 = std::time::Instant::now();
        let validation = sign_with_hd_key(
            &validator_collection,
            &end_user,
            pubkey.clone(),
            false,
            false,
            1,
            message_to_sign,
            SigningScheme::EcdsaK256Sha256,
            &vec![],
        )
        .await;

        if validation {
            sign_success += 1;
        };

        'outer: for reader in &mut log_readers {
            let lines = reader
                .lines()
                .map(|line| line.unwrap_or("".to_string()))
                .collect::<Vec<String>>();

            for line in lines {
                if line.contains("BT Cache Hit") {
                    bt_cache_hit += 1;
                    bt_cache_hit_duration += start_1.elapsed();
                    break 'outer;
                }
                if line.contains("BT Cache Miss") {
                    bt_cache_miss += 1;
                    bt_cache_miss_duration += start_1.elapsed();
                    break 'outer;
                }
            }
        }
    }
    let total_elapsed = start.elapsed();
    info!(
        "
        Signing {} messages randomly in a {} node network 
        Pre-gen Pre-Signature  Warmup: {:?}
        Pre-Signature Cache Hit (qty/time): {} / {:?}
        Pre-Signature Cache Miss (qty/time): {} / {:?}
        Cache success: {:?} 
        Total Time elapsed: {:?} 
        Sign success: {:?} ",
        messages_to_sign,
        num_nodes,
        warmup_time,
        bt_cache_hit,
        bt_cache_hit_duration,
        bt_cache_miss,
        bt_cache_miss_duration,
        bt_cache_hit as f64 / messages_to_sign as f64,
        total_elapsed,
        sign_success
    );

    let validation = true;
    assert!(validation);
}

#[tokio::test]
pub async fn load_with_50ms_latency_all_links() {
    setup();
    info!("Starting test: load_with_50ms_latency");

    let latency_ms = 50;
    let jitter_ms = 0;
    let toxicity = 1.0;

    // inject faults between all nodes
    let ports = [STARTING_PORT, STARTING_PORT + 1, STARTING_PORT + 2];
    for i in 0..ports.len() {
        for j in i + 1..ports.len() {
            inject_latency_fault(
                get_local_url_from_port(ports[i]),
                get_local_url_from_port(ports[j]),
                latency_ms,
                jitter_ms,
                toxicity,
            );
            inject_latency_fault(
                get_local_url_from_port(ports[j]),
                get_local_url_from_port(ports[i]),
                latency_ms,
                jitter_ms,
                toxicity,
            );
        }
    }

    let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;
    let pubkey = end_user.first_pkp().pubkey.clone();
    let num_nodes = validator_collection.size();

    // open the log files
    let mut log_readers = validator_collection.log_readers();

    let _start = std::time::Instant::now();
    let _ = sign_with_hd_key(
        &validator_collection,
        &end_user,
        pubkey.clone(),
        false,
        true,
        1,
        Some("First Test message".to_string()),
        SigningScheme::EcdsaK256Sha256,
        &vec![],
    )
    .await;

    // give the nodes a few seconds to populate a triple or two.
    let warmup_time = Duration::from_millis(30000);
    validator_collection
        .actions()
        .sleep_millis(warmup_time.as_millis() as u64)
        .await;

    // clear the log buffer
    for reader in log_readers.iter_mut() {
        let _lines = reader
            .lines()
            .map(|line| line.unwrap_or("".to_string()))
            .collect::<Vec<String>>();
    }

    let messages_to_sign = 10;

    let mut bt_cache_hit = 0;
    let mut bt_cache_hit_duration: Duration = Duration::from_millis(0);
    let mut bt_cache_miss = 0;
    let mut sign_success = 0;
    let mut bt_cache_miss_duration: Duration = Duration::from_millis(0);

    let start = std::time::Instant::now();
    for i in 0..messages_to_sign {
        info!("Starting sig #{}", i);
        let message_to_sign = Some(format!("Test message #{}", i));
        let start_1 = std::time::Instant::now();
        let validation = sign_with_hd_key(
            &validator_collection,
            &end_user,
            pubkey.clone(),
            false,
            false,
            1,
            message_to_sign,
            SigningScheme::EcdsaK256Sha256,
            &vec![],
        )
        .await;

        if validation {
            sign_success += 1;
        };

        'outer: for reader in &mut log_readers {
            let lines = reader
                .lines()
                .map(|line| line.unwrap_or("".to_string()))
                .collect::<Vec<String>>();

            for line in lines {
                if line.contains("BT Cache Hit") {
                    bt_cache_hit += 1;
                    bt_cache_hit_duration += start_1.elapsed();
                    break 'outer;
                }
                if line.contains("BT Cache Miss") {
                    bt_cache_miss += 1;
                    bt_cache_miss_duration += start_1.elapsed();
                    break 'outer;
                }
            }
        }
    }
    let total_elapsed = start.elapsed();
    info!(
        "
        Signing {} messages randomly in a {} node network 
        Pre-gen Pre-Signature  Warmup: {:?}
        Pre-Signature Cache Hit (qty/time): {} / {:?}
        Pre-Signature Cache Miss (qty/time): {} / {:?}
        Cache success: {:?} 
        Total Time elapsed: {:?} 
        Sign success: {:?} ",
        messages_to_sign,
        num_nodes,
        warmup_time,
        bt_cache_hit,
        bt_cache_hit_duration,
        bt_cache_miss,
        bt_cache_miss_duration,
        bt_cache_hit as f64 / messages_to_sign as f64,
        total_elapsed,
        sign_success
    );

    let validation = true;
    assert!(validation);
}
