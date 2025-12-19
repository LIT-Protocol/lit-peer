use crate::utils::{get_address, get_lit_config};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::{
    backup_recovery::{BackupRecovery, RecoveredPeerId},
    staking::{KeySetConfig, Staking},
};
use serde::{Deserialize, Serialize};

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct BackupRecoverParty {
    pub origional_node_address: String,
    pub old_peer_id: String,
    pub new_peer_id: String,
}

#[component]
pub fn BackupRecovery() -> impl IntoView {

    let data = LocalResource::new(|| async move { get_backup_recover_parties().await });

    crate::utils::set_header("Backup Recovery");
    view! {
        <Title text="Backup Recovery"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Backup Recovery Parties</b>
            </div>
            <div class="card-body">
                {move || match data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {
                        <table class="table">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </div>
        </div>
    }
}
pub async fn get_backup_recover_parties() -> Vec<BackupRecoverParty> {
    let backup_recovery_contract_address = get_address(crate::contracts::BACKUP_RECOVERY_CONTRACT)
        .await
        .unwrap();

    let cfg = &get_lit_config();

    let backup_recovery =
        BackupRecovery::node_monitor_load(cfg, backup_recovery_contract_address).unwrap();

    let parties: Vec<RecoveredPeerId> = backup_recovery
        .get_recovered_peer_ids()
        .call()
        .await
        .unwrap();

    let mut backup_recover_parties: Vec<BackupRecoverParty> = vec![];
    for party in parties {
        backup_recover_parties.push(BackupRecoverParty {
            origional_node_address: party.node_address.to_string(),
            old_peer_id: party.old_peer_id.to_string(),
            new_peer_id: party.new_peer_id.to_string(),
        });
    }

    backup_recover_parties
}
