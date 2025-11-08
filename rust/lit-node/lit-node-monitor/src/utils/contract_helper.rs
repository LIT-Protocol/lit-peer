use crate::utils::context::WebCallBackContext;
use ethers::providers::Provider;
use ethers::types::H160;
use ethers_providers::Http;
use ethers_web::Ethereum;
use lit_blockchain_lite::contracts::lit_token::LITToken;
use lit_blockchain_lite::contracts::pubkey_router::PubkeyRouter;
use lit_blockchain_lite::contracts::staking::Staking;
use std::sync::Arc;

use super::get_lit_config_with_network;

pub async fn get_staking_with_signer(
    ctx: &WebCallBackContext,
) -> (Staking<Provider<Ethereum>>, H160) {
    let ec = ctx.ethereum_context.clone();
    let network = &ctx.active_network;
    let client = Arc::new(ec.provider());
    let staking_contract_address =
        super::get_address_with_network(network, crate::contracts::STAKING_CONTRACT)
            .await
            .unwrap();
    let staking = Staking::new(staking_contract_address, client);
    let from = ec.accounts();

    let from = match from {
        Some(from) => from[0],
        None => {
            ctx.show_error(
                "Error getting signer!",
                "No account found.  Are you connected to MetaMask?",
            );
            H160::zero()
        }
    };

    (staking, from)
}

pub async fn get_staking(ctx: &WebCallBackContext) -> Staking<Provider<Http>> {
    let network = &ctx.active_network;
    let cfg = &get_lit_config_with_network(&network);
    let staking_contract_address =
        super::get_address_with_network(&network, crate::contracts::STAKING_CONTRACT)
            .await
            .unwrap();

    Staking::node_monitor_load(cfg, staking_contract_address).unwrap()
}

pub async fn get_pubkey(ctx: &WebCallBackContext) -> PubkeyRouter<Provider<Http>> {
    let network = &ctx.active_network;
    let cfg = &get_lit_config_with_network(&network);
    let pubkey_contract_address =
        super::get_address_with_network(&network, crate::contracts::PUB_KEY_ROUTER_CONTRACT)
            .await
            .unwrap();

    PubkeyRouter::node_monitor_load(cfg, pubkey_contract_address).unwrap()
}

pub async fn get_lit_token(ctx: &WebCallBackContext) -> LITToken<Provider<Http>> {
    let network = &ctx.active_network;
    let cfg = &get_lit_config_with_network(&network);
    let token_contract_address =
        super::get_address_with_network(&network, crate::contracts::LIT_TOKEN_CONTRACT)
            .await
            .unwrap();

    LITToken::node_monitor_load(cfg, token_contract_address).unwrap()
}
