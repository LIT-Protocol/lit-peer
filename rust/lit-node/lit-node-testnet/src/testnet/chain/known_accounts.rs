use crate::testnet::NodeAccount;
use ethers::core::k256::SecretKey;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use lit_blockchain::resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller};
use lit_core::utils::binary::hex_to_bytes;
use lit_node_common::coms_keys::ComsKeys;
use std::sync::Arc;

pub fn first_anvil_account_private_key() -> Vec<u8> {
    hex_to_bytes("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").unwrap()
}

pub fn first_anvil_account(chain_id: u64, chain_name: &str) -> NodeAccount {
    let secret = first_anvil_account_private_key();

    let sk =
        SigningKey::from(SecretKey::from_bytes(k256::FieldBytes::from_slice(&secret)).unwrap());
    let private_key = H256::from_slice(&sk.to_bytes());

    let wallet = LocalWallet::from(sk).with_chain_id(chain_id);
    let address = wallet.address();
    let provider = ENDPOINT_MANAGER.get_provider(chain_name).unwrap();

    let signing_provider = Arc::new(SignerMiddleware::new(provider, wallet));

    let coms_keys = ComsKeys::new();

    let staker_address = address;

    NodeAccount {
        node_address: Address::zero(),
        signing_provider,
        node_address_private_key: H256::zero(),
        staker_address_private_key: private_key,
        staker_address,
        coms_keys,
    }
}
