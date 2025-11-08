use lit_api_core::error::ApiError;
#[allow(unused_imports)]
use lit_core::config::ReloadableLitConfig;
use rocket::State;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value, serde_json::json};
use std::sync::Arc;
use tracing::instrument;

use crate::endpoints::recovery::utils::{
    check_auth_sig_for_dec_share_upload, check_auth_sig_for_share_download,
};
use crate::endpoints::recovery::{do_delete_share_from_disk, do_share_download_from_rec_dkg};
use crate::error::unexpected_err;
use crate::models::{self};
use crate::tss::common::restore::RestoreState;
use crate::tss::common::tss_state::TssState;
use crate::utils::contract::get_backup_recovery_contract_with_signer;

#[instrument(
    level = "trace",
    name = "POST /web/recovery/set_dec_shares",
    skip_all,
    ret
)]
pub async fn recovery_set_dec_shares(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    request: Json<models::JsonRecoverySetDecShares>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    if let Err(e) =
        check_auth_sig_for_dec_share_upload(&cfg, restore_state, &request.auth_sig).await
    {
        return e.handle();
    }

    if let Err(e) = restore_state
        .add_decryption_shares(&request.auth_sig.address, &request.share_data)
        .await
    {
        return e.handle();
    }

    info!(
        "Recovery: Decryption shares corresponding to member {:?} uploaded to node",
        request.auth_sig.address
    );
    status::Custom(
        Status::Ok,
        json!({
            "success": "true",
        }),
    )
}

// DATIL_BACKUP: Remove this function once old Datil backup is obsolete.
#[instrument(
    level = "trace",
    name = "POST /web/recovery/set_dec_share",
    skip_all,
    ret
)]
pub async fn recovery_set_dec_share(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    request: Json<models::JsonRecoverySetDecShare>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    if let Err(e) =
        check_auth_sig_for_dec_share_upload(&cfg, restore_state, &request.auth_sig).await
    {
        return e.handle();
    }

    if let Err(e) = restore_state
        .add_decryption_shares(&request.auth_sig.address, &[request.share_data.clone()])
        .await
    {
        return e.handle();
    }

    info!(
        "Recovery: Decryption share corresponding to member {:?} uploaded to node",
        request.auth_sig.address
    );
    status::Custom(
        Status::Ok,
        json!({
            "success": "true",
        }),
    )
}

#[instrument(
    level = "trace",
    name = "POST /web/recovery/get_dec_share",
    skip_all,
    ret
)]
pub async fn recovery_get_dec_key_share(
    tss_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::DownloadShareRequest>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    let recovery_contract = match get_backup_recovery_contract_with_signer(&cfg).await {
        Ok(recovery_contract) => recovery_contract,
        Err(e) => {
            return e.handle();
        }
    };
    let party_member = match recovery_contract.get_member_for_node_dkg().await {
        Ok(address) => address,
        Err(e) => {
            return unexpected_err(
                e,
                Some("Could not query backup party member for this peer, aborting".into()),
            )
            .handle();
        }
    };
    info!("found party member: {:?}", party_member);
    if let Err(e) = check_auth_sig_for_share_download(&cfg, &request.auth_sig, &party_member) {
        return e.handle();
    }
    info!("Party member authenticated, moving to share download");
    let shares =
        match do_share_download_from_rec_dkg(tss_state, &cfg, &party_member, &recovery_contract)
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return unexpected_err(
                    e,
                    Some("Error while trying to resolve key share from disk".into()),
                )
                .handle();
            }
        };

    info!(
        "Recovery: Decryption key shares corresponding to member {:?} downloaded",
        party_member
    );
    status::Custom(Status::Ok, json!(shares))
}

#[instrument(
    level = "trace",
    name = "POST /web/recovery/delete_dec_share",
    skip_all,
    ret
)]
pub async fn recovery_delete_dec_key_share(
    tss_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::DownloadShareRequest>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    let recovery_contract = match get_backup_recovery_contract_with_signer(&cfg).await {
        Ok(recovery_contract) => recovery_contract,
        Err(e) => {
            return e.handle();
        }
    };
    let party_member = match recovery_contract.get_member_for_node_dkg().await {
        Ok(address) => address,
        Err(e) => {
            return unexpected_err(
                e,
                Some("Could not query backup party member for this peer, aborting".into()),
            )
            .handle();
        }
    };
    info!("found party member: {:?}", party_member);
    if let Err(e) = check_auth_sig_for_share_download(&cfg, &request.auth_sig, &party_member) {
        return e.handle();
    }

    match do_delete_share_from_disk(tss_state, &cfg, &party_member, &recovery_contract).await {
        Ok(true) => {
            info!(
                "Recovery: Decryption key shares corresponding to member {:?} deleted",
                party_member
            );
            status::Custom(Status::Ok, json!({ "shareDeleted": true }))
        }
        _ => unexpected_err("Error while deleting share from disk", None).handle(),
    }
}
