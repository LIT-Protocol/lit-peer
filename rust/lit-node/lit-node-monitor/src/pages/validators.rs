use crate::components::bottom_modal::BottomModal;
use crate::components::validator_details::ValidatorDetails;
use crate::components::validator_handshake::ValidatorHandshake;
use crate::utils::table_classes::TailwindClassesPreset;
use crate::{
    components::network_status::NetworkStatus,
    models::GlobalState,
    pages::history::simple_hex,
    utils::{
        context::{WebCallBackContext, get_web_callback_context},
        contract_helper::get_staking,
    },
};
use ethers::types::H160;
use ethers_providers::{Http, Provider};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::staking::Staking;
use thaw::{Card, CardHeader, CardPreview};
// use lit_sdk::models::response::JsonSDKHandshakeResponse;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Clone, Copy, Debug)]
pub enum ValidatorType {
    Current = 1,
    Next = 2,
    Available = 3,
}

#[derive(TableRow, Clone, Serialize, Deserialize, Debug)]
#[table(impl_vec_data_provider)]
#[table(classes_provider = "TailwindClassesPreset")]
pub struct Validator {
    #[table(skip)]
    pub validator_type: u8,
    #[table(title = "#")]
    pub id: u32,
    #[table(title = "Host Name")]
    pub host_name: String,
    pub status: String,
    #[table(title = "Guest IP")]
    pub socket_address: String,
    #[table(renderer = "WalletAddressRenderer")]
    pub wallet_address: String,
    #[table(renderer = "WalletAddressRenderer")]
    pub staker_address: String,
    #[table(renderer = "ValidatorStatusRenderer")]
    pub ver: String,
    #[table(skip)]
    pub operator_address: String,
    #[table(skip)]
    pub commit_hash: String,
    #[table(skip)]
    pub network_public_key: String,
    #[table(skip)]
    pub node_identity_key: String,
    #[table(skip)]
    pub epoch: u64,

    #[table(skip)]
    pub last_realm_id: u64,
    #[table(skip)]
    pub last_epoch: u64,
}

#[derive(Clone)]
pub struct Realm {
    id: u8,
    current_validators: Vec<Validator>,
    next_validators: Vec<Validator>,
}

#[component]
fn WalletAddressRenderer(
    class: String,
    #[prop(into)] value: Signal<String>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    row: RwSignal<Validator>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    index: usize,
) -> impl IntoView {
    view! {
        <td class=class>
            {simple_hex(value.get_untracked())}
        </td>
    }
}

#[component]
fn ValidatorStatusRenderer(
    class: String,
    #[allow(unused_variables)]
    #[prop(into)]
    value: Signal<String>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    row: RwSignal<Validator>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    index: usize,
) -> impl IntoView {
    view! {
        <td class=class>
            <ValidatorHandshake row />
        </td>
    }
}

#[component]
pub fn Validators() -> impl IntoView {
    crate::utils::set_header("Validators (by realm)");
    let handshake_state = RwSignal::new("Loading... Please wait...".to_string());
    let realms = LocalResource::new(move || {
        let ctx = get_web_callback_context();
        async move { get_validators_for_all_realms(&ctx).await }
    });
    let floaters = LocalResource::new(move || {
        let ctx = get_web_callback_context();
        async move { get_floaters(&ctx).await }
    });
    let selected_index = RwSignal::new(None);
    let (get_selected_row, set_selected_row) = signal(None::<Validator>);
    let open_buttom = RwSignal::new(false);
    let pop_up_title = RwSignal::new("".to_string());

    view! {
        <Title text="Validators (by realm)"/>
        {
        move || match realms.get().as_deref() {
            None => view! { <p>{ move || handshake_state.get() }</p> }.into_any(),
            Some(realms) => {
                let realms2 = realms.clone();
                realms2.iter().map(|realm|
                    view! {
                        <div class="col-12"><h4>Realm: {realm.id}</h4></div>
                        <div class="col-12"><NetworkStatus realm_id=realm.id as u64 /></div>
                        <div class="row">
                            <div class="col-12 col-sm-6">
                                // <Card>
                                //     <CardHeader>
                                //         <b class="card-title">Current Nodes</b>
                                //     </CardHeader>
                                //     <CardPreview>
                                        <table class="table w-full">
                                            <TableContent
                                                selection=Selection::Single(selected_index)
                                                    on_selection_change={move |evt: SelectionChangeEvent<Validator>| {
                                                        log::info!("evt: {:?}", evt);
                                                        set_selected_row.write().replace(evt.row.get_untracked());
                                                        open_buttom.set(true);
                                                    }}
                                                rows = realm.current_validators.clone() scroll_container="html" />
                                        </table>
                                //     </CardPreview>
                                // </Card>
                            </div>
                            <div class="col-md-6">
                                // <Card>
                                //     <CardHeader>
                                //         <b class="card-title">Next Nodes</b>
                                //     </CardHeader>
                                //     <CardPreview>
                                        <table class="table w-full">
                                            <TableContent
                                                selection=Selection::Single(selected_index)
                                                    on_selection_change={move |evt: SelectionChangeEvent<Validator>| {
                                                        log::info!("evt: {:?}", evt);
                                                        set_selected_row.write().replace(evt.row.get_untracked());
                                                        open_buttom.set(true);
                                                    }}
                                                rows = realm.next_validators.clone() scroll_container="html" />
                                        </table>
                                    // </CardPreview>
                                // </Card>
                            </div>
                        </div>
                        <br />
                    }).collect_view().into_any()
                }}
            }



        <h4>Available</h4>
        <Card class="min-w-full">
            <CardHeader>
                <b class="card-title">Not assigned to any realm</b>
            </CardHeader>
            <CardPreview class="p-3">
            {move || match floaters.get().as_deref() {
                None => view! { <p>"Loading..."</p> }.into_any(),
                Some(rows) => view! {
                    <table class="table w-full">
                        <TableContent
                            selection=Selection::Single(selected_index)
                                on_selection_change={move |evt: SelectionChangeEvent<Validator>| {
                                    log::info!("evt: {:?}", evt);
                                    set_selected_row.write().replace(evt.row.get_untracked());
                                    open_buttom.set(true);
                                }}
                            rows = rows.clone() scroll_container="html" />
                    </table>
                }.into_any()
            }}

            </CardPreview>
        </Card>
        <br />


          { move || get_selected_row.get().map(|selected_row| {
                let title = format!("Validator Details {}", selected_row.host_name);
                pop_up_title.set(title);
                view! {
                    <BottomModal open=open_buttom title=pop_up_title.clone() >
                            <ValidatorDetails validator=selected_row />
                    </BottomModal>
                }
            }) }

    }
}

pub async fn get_validators_for_all_realms(ctx: &WebCallBackContext) -> Vec<Realm> {
    let staking = get_staking(ctx).await;
    let num_realms = staking.num_realms().call().await.unwrap();
    let num_realms = num_realms.as_u64() as u8;

    let mut realms = Vec::new();
    for i in 1..=num_realms {
        let current_validators = get_validators(&staking, true, i).await;
        let next_validators = get_validators(&staking, false, i).await;
        realms.push(Realm {
            id: i,
            current_validators,
            next_validators,
        });
    }

    realms
}

pub async fn get_floaters(ctx: &WebCallBackContext) -> Vec<Validator> {
    let staking = get_staking(ctx).await;
    get_validators(&staking, true, 0).await
}

pub async fn get_validators(
    staking: &Staking<Provider<Http>>,
    is_current: bool,
    realm_id: u8,
) -> Vec<Validator> {
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");

    let realm_id = ethers::types::U256::from(realm_id);
    let kicked = match staking.get_kicked_validators(realm_id).call().await {
        Ok(kicked) => kicked,
        Err(e) => {
            log::error!("Error getting kicked validators: {:?}", e);
            vec![]
        }
    };

    let (validator_type, validators) = match is_current {
        true => match realm_id.as_u64() {
            0 => {
                let reserve_addresses = staking.get_all_reserve_validators().call().await;
                let reserve_addresses = match reserve_addresses {
                    Ok(addresses) => addresses,
                    Err(e) => {
                        log::error!("Error getting reserve validators: {:?}", e);
                        vec![]
                    }
                };
                (
                    ValidatorType::Available,
                    staking
                        .get_validators_structs(reserve_addresses)
                        .call()
                        .await,
                )
            }
            _ => (
                ValidatorType::Current,
                staking
                    .get_validators_structs_in_current_epoch(realm_id)
                    .call()
                    .await,
            ),
        },
        false => (
            ValidatorType::Next,
            staking
                .get_validators_structs_in_next_epoch(realm_id)
                .call()
                .await,
        ),
    };

    if validators.is_err() {
        let err = validators.err();

        // ctx.show_warning("Error getting validators", &format!("{:?}", err));
        log::error!("Error getting validators: {:?}", err);
        return vec![];
    }

    let validators = validators.unwrap();

    let node_addresses = validators
        .iter()
        .map(|v| v.node_address)
        .collect::<Vec<H160>>();

    let mappings: Vec<crate::contracts::staking::AddressMapping> = staking
        .get_node_staker_address_mappings(node_addresses)
        .call()
        .await
        .unwrap();

    let shadow_ids = match staking.get_shadow_validators(realm_id).call().await {
        Ok(shadow_ids) => shadow_ids
            .iter()
            .map(|v| format!("0x{}", hex::encode(v.as_bytes())))
            .collect::<Vec<String>>(),
        Err(e) => {
            log::error!("Error getting shadow validators: {:?}", e);
            vec![]
        }
    };
    log::info!("Shadow IDs: {:?}", shadow_ids);

    let _non_shadow_ids = match staking.get_non_shadow_validators(realm_id).call().await {
        Ok(non_shadow_ids) => non_shadow_ids
            .iter()
            .map(|v| format!("0x{}", hex::encode(v.as_bytes())))
            .collect::<Vec<String>>(),
        Err(e) => {
            log::error!("Error getting non-shadow validators: {:?}", e);
            vec![]
        }
    };

    let mut rows: Vec<Validator> = vec![];
    // let mut hs_signals: Vec<RwSignal<String>> = vec![];
    let mut count = 1;
    for v in &validators {
        let ip_address = match Ipv4Addr::try_from(v.ip) {
            Ok(ip) => ip.to_string(),
            Err(e) => {
                log::error!("Error getting IP address: {:?}", e);
                "".to_string()
            }
        };

        let socket_address = format!("{}:{}", ip_address, v.port);
        let guest_ip = match socket_address.contains("127.0.0.1") {
            true => socket_address.clone(),
            false => socket_address.split(":").nth(0).unwrap().to_string(),
        };
        // log::info!("Guest IP: {:?} / {:?}", guest_ip, gs.staker_names.get());
        let info = gs
            .staker_names
            .get()
            .get(&guest_ip)
            .unwrap_or(&"".to_string())
            .clone();

        let staker_address = mappings
            .iter()
            .find(|m| m.node_address == v.node_address)
            .unwrap()
            .staker_address;

        rows.push(Validator {
            validator_type: validator_type.clone() as u8,
            id: count,
            status: match kicked.contains(&v.node_address) {
                true => "K".to_string(),
                false => "".to_string(),
            },
            socket_address: socket_address.clone(),
            wallet_address: format!("0x{}", hex::encode(v.node_address.as_bytes())),
            staker_address: format!("0x{}", hex::encode(staker_address.as_bytes())),
            operator_address: format!("0x{}", hex::encode(v.operator_address.as_bytes())),
            ver: "?".to_string(),
            host_name: info,
            commit_hash: "".to_string(),
            network_public_key: "".to_string(),
            node_identity_key: "".to_string(),
            epoch: 0,
            last_realm_id: v.last_realm_id.as_u64(),
            last_epoch: v.last_active_epoch.as_u64(),
        });
    }

    rows.sort_by(|a, b| a.staker_address.cmp(&b.staker_address));

    for row in &mut rows {
        row.id = count;
        count += 1;
    }

    rows
}
