use std::fmt::Display;

use crate::utils::{get_address, get_lit_config, table_classes::TailwindClassesPreset};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::{ColumnSort, *};
use lit_blockchain_lite::contracts::staking::Staking;
use serde::{Deserialize, Serialize};
use thaw::{Card, CardHeader, CardPreview};

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "TailwindClassesPreset",
    // thead_cell_renderer = "CustomTableHeaderCellRenderer",
    impl_vec_data_provider
)]
pub struct ComplaintConfig {
    pub reason_name: String,
    pub reason: u128,
    pub tolerance: u128,
    pub interval_secs: u128,
    pub kick_penalty_percent: u128,
    pub kick_penalty_demerits: u128,
}

#[component]
pub fn Complaints() -> impl IntoView {
    let data = LocalResource::new(|| async move { get_complaint_configs().await });

    crate::utils::set_header("Complaints");
    view! {
        <Title text="Complaints"/>
        <Card class="min-w-full">
            <CardHeader>
                <b class="card-title">Complaint Configurations</b>
            </CardHeader>
            <CardPreview class="p-3">
          {move || match data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {
                        <table class="table w-full">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </CardPreview>
        </Card>
    }
}

pub async fn get_complaint_configs() -> Vec<ComplaintConfig> {
    let staking_contract_address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();

    let cfg = &get_lit_config();
    let staking = Staking::node_monitor_load(cfg, staking_contract_address).unwrap();

    let mut complaint_configs: Vec<ComplaintConfig> = vec![];

    for reason in ComplaintReason::iter() {
        let reason_u256 = ethers::types::U256::from(reason as u128);
        let complaint_config = staking.complaint_config(reason_u256).call().await.unwrap();
        complaint_configs.push(ComplaintConfig {
            reason_name: reason.to_string(),
            reason: reason as u128,
            tolerance: complaint_config.tolerance.as_u128(),
            interval_secs: complaint_config.interval_secs.as_u128(),
            kick_penalty_percent: complaint_config.kick_penalty_percent.as_u128(),
            kick_penalty_demerits: complaint_config.kick_penalty_demerits.as_u128(),
        });
    }
    complaint_configs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComplaintReason {
    Unresponsive = 1,
    NonParticipation,
    IncorrectInfo,
    KeyShareValidationFailure,
}

impl Display for ComplaintReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ComplaintReason {
    pub fn iter() -> impl Iterator<Item = ComplaintReason> {
        vec![
            ComplaintReason::Unresponsive,
            ComplaintReason::NonParticipation,
            ComplaintReason::IncorrectInfo,
            ComplaintReason::KeyShareValidationFailure,
        ]
        .into_iter()
    }
}
