use crate::endpoints::admin::utils::{
    check_admin_auth_sig, encrypt_and_tar_backup_keys, purge_precomputes, untar_keys_stream,
};
use crate::error::{EC, config_err, parser_err, unexpected_err, validation_err_code};
use crate::models;
use crate::tss::common::backup::get_recovery_party;
use crate::tss::common::restore::{NodeRecoveryStatus, RestoreState, report_progress};

use crate::auth::auth_material::JsonAuthSigExtended;
use crate::tss::common::tss_state::TssState;
use crate::version::DataVersionReader;
use chrono::{DateTime, Utc};
use lit_api_core::error::ApiError;
use lit_blockchain::resolver::rpc::config::{RPC_CONFIG_PROTECTED_CHAINS, RpcConfig};
use lit_blockchain::resolver::rpc::{RPC_RESOLVER, RpcResolver};
use lit_core::config::{CFG_ADMIN_OVERRIDE_NAME, ReloadableLitConfig};
use lit_node_common::config::LitNodeConfig;
use lit_node_core::Blinders;
use rocket::data::ByteUnit;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket::{Data, State};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[instrument(level = "debug", name = "POST /web/admin/set", skip_all, ret)]
pub async fn admin_set(
    remote_addr: SocketAddr,
    reloadable_cfg: &State<ReloadableLitConfig>,
    request: Json<models::JsonAdminSetRequest>,
) -> status::Custom<Value> {
    let cfg = reloadable_cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &request.auth_sig) {
        return e.handle();
    }

    // validate the config
    if let Err(e) = cfg.verify_user_editable(&request.new_config) {
        return parser_err(
            e,
            Some(
                "Invalid config.  You may have a non-user editable config key in there somewhere."
                    .into(),
            ),
        )
        .add_msg_to_details()
        .handle();
    }

    // write the config to the config file
    if let Err(e) = cfg.save_local_config(CFG_ADMIN_OVERRIDE_NAME, &request.new_config) {
        return e.handle();
    }

    if let Err(e) = reloadable_cfg.reload() {
        return e.handle();
    }

    // ok now do the same for the rpc config
    if let Err(e) = request.rpc_config.verify() {
        return e.handle();
    }

    // Prevent any changes to the RPC entries that are not allowed
    let existing_rpc_resolver = RPC_RESOLVER.load();
    let new_rpc_config = request.rpc_config.chains();
    for chain in RPC_CONFIG_PROTECTED_CHAINS {
        // Get RPC entries from request
        let new_rpc_entries = match new_rpc_config.get(chain) {
            Some(new_rpc_entries) => new_rpc_entries,
            None => {
                return validation_err_code(
                    "Missing RPC config entry for mandatory chain",
                    EC::NodeRpcConfigForbidden,
                    None,
                )
                .add_msg_to_details()
                .handle();
            }
        };

        // Get RPC entries from existing config
        let existing_rpc_entries = match existing_rpc_resolver.resolve(chain) {
            Ok(existing_rpc_entries) => existing_rpc_entries,
            Err(e) => {
                return validation_err_code(
                    e,
                    EC::NodeRpcConfigForbidden,
                    Some("Cannot resolve chain in existing RPC config".into()),
                )
                .add_msg_to_details()
                .handle();
            }
        };

        // Compare - if they are different, then the user is trying to change a protected value. Reject if so.
        if *existing_rpc_entries != *new_rpc_entries {
            return validation_err_code(
                "Unauthorized to edit protected chain values",
                EC::NodeRpcConfigForbidden,
                None,
            )
            .handle();
        }
    }

    if let Err(e) = request.rpc_config.write_file_local() {
        return e.handle();
    }

    if let Err(e) = RpcResolver::reload() {
        return e.handle();
    }

    status::Custom(
        Status::Ok,
        json!({
            "success": "true",
        }),
    )
}

#[instrument(level = "debug", name = "POST /web/admin/get", skip_all, ret)]
pub async fn admin_get(
    cfg: &State<ReloadableLitConfig>,
    auth: JsonAuthSigExtended,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &auth.auth_sig) {
        return e.handle();
    }

    let exported = match cfg.export_user_editable() {
        Ok(exported) => exported,
        Err(e) => {
            return parser_err(e, Some("Error exporting config".into()))
                .add_msg_to_details()
                .handle();
        }
    };

    // get rpc config, too
    let rpc_config = match RpcConfig::load() {
        Ok(rpc_config) => rpc_config,
        Err(e) => {
            return parser_err(e, Some("Error loading rpc config".into()))
                .add_msg_to_details()
                .handle();
        }
    };
    let chains = rpc_config.chains();

    status::Custom(
        Status::Ok,
        json!({
            "success": "true",
            "config": exported,
            "chains": chains,
        }),
    )
}

#[instrument(level = "debug", name = "POST /web/admin/get_blinders", skip_all, ret)]
pub async fn admin_get_blinders(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    auth: JsonAuthSigExtended,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &auth.auth_sig) {
        return e.handle();
    }

    let blinders = restore_state.get_blinders();
    let json_blinders = serde_json::to_value(*blinders).expect("Failed to serialize blinders");

    status::Custom(Status::Ok, json_blinders)
}

#[instrument(level = "debug", name = "POST /web/admin/set_blinders", skip_all, ret)]
pub async fn admin_set_blinders(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    admin_auth_sig: &lit_node_core::AdminAuthSig,
    blinders: &Blinders,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &admin_auth_sig.auth_sig) {
        error!("Admin auth sig is not valid");
        return e.handle();
    }

    // Only allow this to be done during an active restore
    match restore_state.assert_actively_restoring() {
        Ok(_) => {}
        Err(e) => {
            error!("Not actively restoring yet, can't set blinders");
            return e.handle();
        }
    }
    if blinders.any_blinders_invalid() {
        error!("Blinders are invalid. One or more are zero");
        return unexpected_err("Blinders are invalid. One or more are zero", None).handle();
    }

    restore_state.set_blinders(*blinders);

    status::Custom(Status::Ok, json!({ "success": true }))
}

#[instrument(level = "debug", name = "GET /web/admin/get_key_backup", skip_all, ret)]
pub async fn admin_get_key_backup(
    cfg: &State<ReloadableLitConfig>,
    tss_state: &State<Arc<TssState>>,
    restore_state: &State<Arc<RestoreState>>,
    auth: JsonAuthSigExtended,
    epoch: Option<u64>,
) -> Result<Vec<u8>, status::Custom<Value>> {
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &auth.auth_sig) {
        return Err(e.handle());
    }
    trace!("Auth sig check passed");

    let now: DateTime<Utc> = Utc::now();

    let blinders = restore_state.get_blinders();

    if !blinders.are_blinders_set() {
        return Err(config_err("Blinders are not set", None).handle());
    }

    trace!("Got blinders");
    let recovery_party = match get_recovery_party(&cfg).await {
        Ok(recovery_party) => recovery_party,
        Err(e) => return Err(e.handle()),
    };
    trace!("Got recovery party");

    let epoch = match epoch {
        Some(epoch) => epoch,
        None => tss_state.peer_state.epoch(),
    };

    let peers = if epoch == tss_state.peer_state.epoch() {
        tss_state.peer_state.peers()
    } else {
        tss_state.peer_state.peers_in_prior_epoch()
    };
    let self_peer = match peers.peer_at_address(&tss_state.addr) {
        Ok(peer) => peer,
        Err(e) => return Err(e.handle()),
    };
    let default_key_set = DataVersionReader::read_field_unchecked(
        &tss_state.chain_data_config_manager.generic_config,
        |generic_config| generic_config.default_key_set.clone(),
    );
    let key_set_root_keys = DataVersionReader::read_field_unchecked(
        &tss_state.chain_data_config_manager.key_sets,
        |key_sets| match &default_key_set {
            Some(id) => match key_sets.get(id) {
                Some(key_set) => Ok(key_set.root_keys_by_curve.clone()),
                None => Err(
                    unexpected_err(format!("No key set root keys exist for {}", id), None).handle(),
                ),
            },
            None => match key_sets.first_key_value() {
                Some((_id, key_set)) => Ok(key_set.root_keys_by_curve.clone()),
                None => {
                    Err(unexpected_err("No key sets exist for backup".to_string(), None).handle())
                }
            },
        },
    )?;

    // Zip up and encrypt.
    match encrypt_and_tar_backup_keys(
        cfg,
        self_peer.peer_id,
        &key_set_root_keys,
        &blinders,
        &recovery_party,
        &peers,
        epoch,
    )
    .await
    {
        Ok(data) => Ok(data),
        Err(e) => Err(e.handle()),
    }
}

#[instrument(
    level = "trace",
    name = "POST /web/admin/set_key_backup",
    skip_all,
    ret
)]
pub async fn admin_set_key_backup(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    admin_auth_sig: JsonAuthSigExtended,
    data: Data<'_>,
) -> status::Custom<Value> {
    trace!("admin_set_key_backup() called");
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &admin_auth_sig.auth_sig) {
        return e.handle();
    }

    trace!("admin_set_key_backup() - decrypting and untaring file");

    // Unzip the file, which should replace the BLS and ECDSA key material.
    let stream = data.open(ByteUnit::Gigabyte(u64::MAX));
    if let Err(e) = untar_keys_stream(&cfg, restore_state, stream).await {
        return e.handle();
    }

    report_progress(cfg.as_ref(), NodeRecoveryStatus::BackupsAreLoaded).await;

    trace!("admin_set_key_backup() - removing precompute files");
    if let Err(e) = purge_precomputes(&cfg).await {
        return e.handle();
    }

    status::Custom(
        Status::Ok,
        json!({
            "success": "true",
        }),
    )
}
