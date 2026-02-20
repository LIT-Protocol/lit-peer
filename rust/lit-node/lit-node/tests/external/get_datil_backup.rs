use ethers::types::{Address, TransactionRequest};
use lit_core::utils::binary::hex_to_bytes;
use lit_node_testnet::testnet::chain::anvil::first_anvil_account;
// Notes, this test is used to download the datil shares and run the lit-recovery-mac binary to recover the shares.
// It can be used as part of a script to migrate shares from networks, where the node-op/staker address wallets are known ( ie, internal )
// It can also be used to generate the appropriate date for the restore test

use lit_node_testnet::testnet::datil::contracts::DatilContracts;
use ethers::providers::Middleware;
use flume::{Receiver, unbounded};
use lit_blockchain::resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};


#[derive(Debug)]
struct LitRecoveryProcess {
    process: Child,
    input: ChildStdin,
    rx: Receiver<String>,
    verification_key: String,
    address: String,
}

#[ignore]
#[tokio::test]
async fn download_datil_shares() {

    let (resolver_address, _rpc_url, chain_id, _env) = get_local_anvil_config();
    let contracts = get_datil_contracts("anvil", &resolver_address, chain_id).await;

    let recovery_party_size = 3;
    let (binary_name, test_path) = setup_lit_recovery_test();


    let mut lrts = Vec::new();

    // start up the tool and get the verification key and address for each party member.
    for i in 0..recovery_party_size {
        // start the lit-recovery-mac process and get the output and input streams.
        let (lrt_io, lrt_output, lrt_input) =
            start_lit_recovery_tool(&binary_name, &test_path, i);

        // setup the output listener to get the output from the lit-recovery-mac process.
        let rx = setup_output_listener(lrt_output, i + 1);

        let verification_key = wait_for_kv_output(&rx, "Verification key");
        let address = wait_for_kv_output(&rx, "Address");
        // this is an artificial wait point, before we start setting things up.
        let _waitpoint = wait_for_output(&rx, "quit");

        lrts.push(LitRecoveryProcess {
            process: lrt_io,
            input: lrt_input,
            rx: rx,
            verification_key: verification_key,
            address: address,
        });
    }

    // register the party members with the contracts.
    println!("Registering party members with the datil backup / recovery contract.");
    let mut party_members: Vec<Address> = Vec::new();
    for lrt in &lrts {
        party_members.push(str_to_address(&lrt.address));
        set_wallet_balance_internal(str_to_address(&lrt.address), "100000000000000000000", &contracts).await;
    }
    let f = contracts.backup_recovery.register_new_backup_party(party_members);
    let tx = f.send().await.unwrap();
    let receipt = tx.await.unwrap();
    println!("Transaction is complete: {:?}", receipt.is_some());

    // now register the party members using the lit-recovery-mac tool.
    for lrt in lrts.iter_mut() {
        send_command(&mut lrt.input, "register");
        // println!("Verification key: {}", lrt.verification_key);
        // println!("Address: {}", lrt.address);
        let _waitpoint = wait_for_output(&lrt.rx, "Please email");

    }

    println!("Next backup party members: {:?}", contracts.backup_recovery.get_next_backup_party_members().call().await.unwrap());
    println!("Next backup state: {:?}", contracts.backup_recovery.get_next_backup_state().call().await.unwrap());

    println!("Waiting for recovery dkg to complete.");

    loop{
        let is_recovery_dkg_completed = contracts.backup_recovery.is_recovery_dkg_completed().call().await.unwrap();
        println!("Recovery DKG completed: {:?}", is_recovery_dkg_completed);
        if is_recovery_dkg_completed {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    println!("Recovery DKG completed, downloading decryption shares.");

    tokio::time::sleep(std::time::Duration::from_secs(5000)).await;


    for lrt in lrts.iter_mut() {
        send_command(&mut lrt.input, "download");
        let _waitpoint = wait_for_output(&lrt.rx, "Recovery share deleted from node");

        send_command(&mut lrt.input, "upload-pub-keys");
        let _waitpoint = wait_for_output(&lrt.rx, "Upload pub keys txn hash");

        // and we're done ! 
        lrt.process.kill().unwrap();
    }


}

fn setup_lit_recovery_test() -> (String, PathBuf) {
    let binary_name = "lit-recovery-mac".to_string();
    let lit_recovery_binary_src =
        format!("./tests/test_data/datil_recovery_into_naga/{}", binary_name);
    let lit_recovery_binary_dest = format!("./lit-recovery-test/{}", binary_name);

    let test_dir = "./lit-recovery-test";
    let test_path = Path::new(test_dir);
    if fs::exists(test_dir).unwrap_or(false) {
        fs::remove_dir_all(test_dir).unwrap();
    }
    fs::create_dir_all(test_dir).unwrap();
    fs::copy(lit_recovery_binary_src, lit_recovery_binary_dest).unwrap();

    (binary_name, test_path.to_path_buf())
}

fn start_lit_recovery_tool(
    binary_name: &str,
    test_path: &PathBuf,
    i: usize,
) -> (std::process::Child, ChildStdout, ChildStdin) {
    let share_db_path = format!("./sdb{}.db3", i + 1);
    let recovery_command = format!(
        "SHARE_DB=\"{}\" ./{} --password=a --file=./{}",
        share_db_path,
        binary_name,
        i + 1
    );
    println!("recovery_command: {}", recovery_command);
    let recovery_command = format!("./{}", binary_name);
    let mut lrt_io: Child = Command::new(recovery_command)
        .args(&[
            "--password=a",
            &format!("--file=./lit-recovery-test/{}", i + 1),
        ])
        .env("SHARE_DB", &share_db_path)
        .env("SHARE_DB_PATH", &share_db_path)
        .current_dir(test_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to launch lit-recovery-mac");

    let lrt_output = lrt_io.stdout.take().unwrap();
    let lrt_input = lrt_io.stdin.take().unwrap();

    (lrt_io, lrt_output, lrt_input)
}

fn setup_output_listener(lrt_output: ChildStdout, index: usize) -> Receiver<String> {
    let (tx, rx) = unbounded::<String>();

    let _lrt_task = std::thread::spawn(move || {
        let reader = BufReader::new(lrt_output);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    println!("LRT-{}: > {}", index, line);
                    tx.send(line).unwrap();
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    });
    rx
}

fn wait_for_kv_output(rx: &Receiver<String>, key: &str) -> String {
    let value = wait_for_output(rx, key);
    value.split(":").nth(1).unwrap().trim().to_string()
}

fn wait_for_output(rx: &Receiver<String>, contains_value: &str) -> String {
    loop {
        let line = rx.recv().unwrap();
        if line.contains(contains_value) {
            return line;
        }
    }
}

fn send_command(lrt_input: &mut ChildStdin, command: &str) {
    println!("INPUT: {}", command);
    let command = format!("{}\n", command);
    lrt_input.write_all(command.as_bytes()).unwrap();
}

fn get_local_anvil_config() -> (String, String, usize, usize) {
    let resolver_address = "0x5fbdb2315678afecb367f032d93f642f64180aa3".to_string();
    let rpc_url = "http://127.0.0.1:8545".to_string();
    let chain_id = 31337;
    let env = 0;
    (resolver_address, rpc_url, chain_id, env)
}

// ignore unused code
#[allow(dead_code)]
fn get_datil_test_config() -> (String, String, usize, usize) {
    // set the resolver address, rpc url, chain id, and env.
    // let resolver_address = "0xCf908e1E4Ee79fb540e144C3EDB2796E8D413548";
    // let rpc_url = "https://yellowstone-rpc.litprotocol.com/";
    // let chain_id = 175188;

    let resolver_address = "0xCf908e1E4Ee79fb540e144C3EDB2796E8D413548".to_string();
    let rpc_url = "https://yellowstone-rpc.litprotocol.com/".to_string();
    let chain_id = 175188;
    let env = 0;
    (resolver_address, rpc_url, chain_id, env)
}

async fn get_datil_contracts(chain_name: &str, contract_resolver_address: &str, chain_id: usize) -> DatilContracts {
    let provider = ENDPOINT_MANAGER
        .get_provider(chain_name)
        .expect(&format!(
            "Error retrieving provider for chain {} - check name and/or rpc_config yaml.",
            chain_name
        ));

    let deployer_account = first_anvil_account(chain_id as u64, chain_name);
    let deployer_signing_provider = deployer_account.signing_provider.clone();

    let contract_resolver_address = str_to_address(contract_resolver_address);
    let contracts =
        DatilContracts::new(deployer_signing_provider.clone(), contract_resolver_address).await;
    contracts
}

fn str_to_address(address: &str) -> Address {
    let address = address.replace("0x", "");
    let address = hex_to_bytes(address).unwrap();
    Address::from_slice(&address)
}

 async fn set_wallet_balance_internal(
        to_address: Address,
        amount: &str,
        datil_contracts: &DatilContracts,
    ) {

        let provider = datil_contracts.deployer_provider.clone();
        info!(
            "Deployer provider {:?} balance: {:?}",
            provider.address(),
            provider.get_balance(provider.address(), None).await
        );

        let tx = TransactionRequest::new()
            .to(to_address)
            .value(ethers::types::U256::from_dec_str(amount).expect("Failed to convert amount to U256"))
            .from(provider.address());

        let pending_tx = provider.send_transaction(tx, None).await;
        if let Err(e) = pending_tx {
            panic!("Couldn't set balance: {:?}", e);
        }
        let pending_tx = pending_tx.unwrap().interval(std::time::Duration::from_millis(100));
        let receipt = pending_tx.await.unwrap().expect("No receipt from txn");

        info!("Transaction receipt: {:?}", receipt);
        info!(
            "Wallet balance: {:?}",
            provider.get_balance(to_address, None).await
        );
        info!(
            "Deployer provider balance: {:?}",
            provider.get_balance(provider.address(), None).await
        );
    }