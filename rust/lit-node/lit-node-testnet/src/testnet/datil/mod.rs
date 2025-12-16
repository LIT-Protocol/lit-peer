use crate::testnet::NodeAccount;
use crate::testnet::chain::ChainTrait;
use crate::testnet::datil::contracts::DatilContracts;
use command_group::GroupChild;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use std::sync::Arc;

pub mod actions;
pub mod contracts;
pub mod datil_testnet;
pub struct DatilTestnet {
    process: GroupChild,
    pub datil_chain: Box<dyn ChainTrait>,
    pub provider: Arc<Provider<Http>>,
    pub node_accounts: Arc<Vec<NodeAccount>>,
    pub deployer_signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub contracts: DatilContracts,
}
