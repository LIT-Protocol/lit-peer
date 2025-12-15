// use crate::testnet::{WhichTestnet, actions::Actions};
// use lit_blockchain_lite::contracts::pubkey_router::{RootKey, PubkeyRouter};
// use anyhow::Result;

// use lit_blockchain::contracts::staking::{ComplaintConfig, UncompressedK256Key, staking};
// use lit_blockchain::contracts::{
//     lit_token::lit_token::LITToken,
//     staking::{Staking, StakingErrors, Validator},
// };

// impl Actions {
//     pub async fn set_datil_testnet_root_keys(&self, root_keys: Vec<RootKey>) -> Result<()> {

//         if self.which_testnet != WhichTestnet::Anvil {
//             panic!("Datil testnet root keys can only be set on Anvil testnet.");
//         }

//         let pubkey_router = PubkeyRouter::new(self.contracts.pubkey_router.address(), self.deployer_provider().clone());
//         let tx = pubkey_router.admin_reset_root_keys(
//             self.contracts.staking.address(),
//             "datil".to_string(),
//             root_keys,
//         );
//         tx.send().await.map_err(|e| anyhow::anyhow!("Error sending tx to set datil testnet root keys: {:?}", e))?;
//         Ok(())
//     }

// }
