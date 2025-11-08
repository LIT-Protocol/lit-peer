use ethers::providers::{Http, Provider};
use ethers::types::{Address, I256, U256};
use std::sync::Arc;

use lit_blockchain::contracts::ledger::Ledger;

use crate::error::{EC, Result, generic_err_code, unexpected_err};
use crate::models::auth::SessionKeySignedMessageV2;
use crate::payment::delegated_usage::DelegatedUsageDB;
use crate::payment::payment_delegation::check_for_payment_db;
use lit_core::config::LitConfig;

use crate::payment::{
    batches::PendingPayment, payed_endpoint::PayedEndpoint,
    payment_delegation::check_for_payment_delegation, payment_tracker::PaymentTracker,
};
use crate::utils::contract::{
    get_ledger_contract, get_pkp_nft_contract, get_price_feed_contract, get_pub_key_router_contract,
};

#[allow(clippy::too_many_arguments)]
pub async fn get_payment_method(
    user_address: &Address,
    endpoint: PayedEndpoint,
    threshold: usize,
    max_price: U256,
    session_key_signed_message: Option<SessionKeySignedMessageV2>,
    payment_tracker: &Arc<PaymentTracker>,
    delegation_usage_db: &DelegatedUsageDB, // TODO!: Use a new model instead of DelegatedUsageDB
    bls_root_pubkey: &str,
    cfg: &LitConfig,
) -> Result<PendingPayment> {
    let usage = payment_tracker.get_usage_percentage();
    if usage >= 100 {
        return Err(unexpected_err("Node usage is above 100%", None));
    }

    let endpoint_price = fetch_current_price(cfg, usage, &endpoint).await?;
    trace!(
        "endpoint & price: {} - {:?}",
        endpoint.as_str(),
        endpoint_price
    );
    trace!("threshold : {} .  Max price: {}", threshold, max_price);

    // The max_price is always used for the comparison as it allows for finer price tuning
    if endpoint_price > max_price {
        let err_msg = format!(
            "max_price: {} is less than the endpoint price:  {}",
            max_price, endpoint_price
        );
        warn!("{}", err_msg);
        return Err(generic_err_code(err_msg, EC::PaymentFailed, None).add_source_to_details());
    }

    let (payer, spending_limit) = select_payment_method(
        user_address,
        endpoint_price,
        &endpoint,
        threshold,
        session_key_signed_message,
        payment_tracker,
        delegation_usage_db,
        bls_root_pubkey,
        cfg,
    )
    .await?;

    let spending_limit = if max_price == U256::MAX {
        spending_limit
    } else {
        std::cmp::min(spending_limit, convert_price_to_i256(max_price)?)
    };

    let endpoint_price_i256 = convert_price_to_i256(endpoint_price)?;
    Ok(PendingPayment {
        payer,
        price: endpoint_price_i256,
        spending_limit,
    })
}

pub async fn check_payer_has_funds(
    ledger: &Ledger<Provider<Http>>,
    user_address: &Address,
    required_balance: I256,
    payment_tracker: &Arc<PaymentTracker>,
) -> Result<I256> {
    let balance = ledger.stable_balance(*user_address).await.map_err(|e| {
        let err_msg = format!("Cannot get the funds for user {}: {:?}", user_address, e);
        error!("{}", err_msg);
        unexpected_err(e, Some(err_msg))
    })?;
    let pending_spending = payment_tracker
        .batches()
        .get_unregistered_spending(user_address)
        .await;
    let spending_limit = balance - pending_spending;
    match required_balance <= spending_limit {
        true => {
            info!(
                "User {}'s balance {} is enough to cover the minimum estimated price {}",
                user_address, balance, required_balance
            );
            Ok(spending_limit)
        }
        false => {
            let err_msg = format!(
                "User {}'s balance {} minus their pending spending of {} is not enough \
                to cover the minimum estimated price {}",
                user_address, balance, pending_spending, required_balance
            );
            warn!("{}", err_msg);
            Err(unexpected_err(err_msg, None))
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn select_payment_method(
    user_address: &Address,
    endpoint_price: U256,
    endpoint: &PayedEndpoint,
    threshold: usize,
    session_key_signed_message: Option<SessionKeySignedMessageV2>,
    payment_tracker: &Arc<PaymentTracker>,
    delegation_usage_db: &DelegatedUsageDB,
    bls_root_pubkey: &str,
    cfg: &LitConfig,
) -> Result<(Address, I256)> {
    let mut payment_err_msg = String::new();

    let ledger = get_ledger_contract(cfg).await?;

    let required_balance =
        fetch_required_balance(cfg, endpoint, &endpoint_price, threshold).await?;
    let required_balance_i256 = convert_price_to_i256(required_balance)?;

    trace!("1. Checking Payment DB");
    match check_for_payment_db(
        user_address,
        session_key_signed_message
            .clone()
            .map(|sksm| sksm.max_price),
        required_balance_i256,
        threshold,
        payment_tracker,
        delegation_usage_db,
        &ledger,
        cfg,
    )
    .await
    {
        Ok(Some((delegator_address, spending_limit))) => {
            return Ok((delegator_address, spending_limit));
        }
        Ok(None) => {
            info!(
                "No payer in the Payment DB for {}, checking Capacity Delegation next",
                user_address
            );
            payment_err_msg.push_str(" No payer in the Payment DB; ");
        }
        Err(e) => {
            payment_err_msg.push_str(" Payment DB failed to pay: ");
            payment_err_msg.push_str(&e.to_string());
        }
    }

    if let Some(session_key_signed_message) = session_key_signed_message {
        trace!("2. Checking Delegation Authsig");
        match check_for_payment_delegation(
            user_address,
            session_key_signed_message,
            required_balance_i256,
            payment_tracker,
            endpoint,
            bls_root_pubkey,
            &ledger,
        )
        .await
        {
            Ok(Some((delegator_address, spending_limit))) => {
                return Ok((delegator_address, spending_limit));
            }
            Ok(None) => {
                trace!("No Capacity delegation, checking Self pay next");
                payment_err_msg.push_str(" No Capacity Delegation; ");
            }
            Err(e) => {
                payment_err_msg.push_str(" Delegator failed to pay: ");
                payment_err_msg.push_str(&e.to_string());
            }
        }
    }

    trace!("3. Checking Self pay");
    match check_payer_has_funds(
        &ledger,
        user_address,
        required_balance_i256,
        payment_tracker,
    )
    .await
    {
        Ok(spending_limit) => return Ok((*user_address, spending_limit)),
        Err(e) => {
            payment_err_msg.push_str(" Self failed to pay: ");
            payment_err_msg.push_str(&e.to_string());
        }
    }

    trace!("4. Checking an alternative payer: PKP owner");
    match check_pkp_owner_has_funds(
        cfg,
        user_address,
        &ledger,
        required_balance_i256,
        payment_tracker,
    )
    .await
    {
        Ok((owner_address, spending_limit)) => return Ok((owner_address, spending_limit)),
        Err(e) => {
            payment_err_msg.push_str(" Attempt to charge the PKP owner failed: ");
            payment_err_msg.push_str(&e.to_string());
        }
    }

    Err(generic_err_code(payment_err_msg, EC::PaymentFailed, None).add_source_to_details())
}

async fn check_pkp_owner_has_funds(
    cfg: &LitConfig,
    address: &Address,
    ledger: &Ledger<Provider<Http>>,
    required_balance: I256,
    payment_tracker: &Arc<PaymentTracker>,
) -> Result<(Address, I256)> {
    let pub_key_router_contract = get_pub_key_router_contract(cfg).await?;
    let token_id = match pub_key_router_contract
        .eth_address_to_pkp_id(*address)
        .call()
        .await
    {
        Ok(id) => id,
        Err(e) => {
            return Err(unexpected_err(
                e,
                Some(format!(
                    "Unable to get the token id of the PKP : {}",
                    address
                )),
            ));
        }
    };

    let pkp_nft_contract = get_pkp_nft_contract(cfg).await?;
    let owner_address = match pkp_nft_contract.owner_of(token_id).call().await {
        Ok(address) => address,
        Err(e) => {
            return Err(unexpected_err(
                e,
                Some(format!(
                    "Unable to get the address of the PKP owner.  PKP: {}",
                    address
                )),
            ));
        }
    };

    let spending_limit =
        check_payer_has_funds(ledger, &owner_address, required_balance, payment_tracker).await?;

    Ok((owner_address, spending_limit))
}

async fn fetch_current_price(
    cfg: &LitConfig,
    usage: u64,
    endpoint: &PayedEndpoint,
) -> Result<U256> {
    let price_feed = get_price_feed_contract(cfg).await?;
    price_feed
        .usage_percent_to_price(U256::from(usage), U256::from(u8::from(endpoint)))
        .await
        .map_err(|e| {
            let err_msg = format!("Cannot get the price: {:?}", e);
            error!("{}", err_msg);
            unexpected_err(e, Some(err_msg))
        })
}

async fn fetch_required_balance(
    cfg: &LitConfig,
    endpoint: &PayedEndpoint,
    current_price: &U256,
    threshold: usize,
) -> Result<U256> {
    let price_feed = get_price_feed_contract(cfg).await?;

    let min_price = price_feed
        .usage_percent_to_price(U256::from(0), U256::from(u8::from(endpoint)))
        .await
        .map_err(|e| {
            let err_msg = format!("Cannot get the price: {:?}", e);
            error!("{}", err_msg);
            unexpected_err(e, Some(err_msg))
        })?;

    // We know the price for this node, assume the minimum for the others as a lower limit
    Ok(current_price + U256::from(threshold - 1) * min_price)
}

fn convert_price_to_i256(price_u256: U256) -> Result<I256> {
    I256::try_from(price_u256).map_err(|e| {
        // This should never happen in practice due to the upper bound
        // in the Price Feed contract.
        let err_msg = format!("Cannot convert the price {} from U256 to I256", price_u256);
        error!("{}", err_msg);
        unexpected_err(e, Some(err_msg))
    })
}
