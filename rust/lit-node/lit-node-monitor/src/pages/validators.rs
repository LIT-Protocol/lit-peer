use crate::utils::table_classes::BootstrapClassesPreset;
use crate::{
    components::network_status::NetworkStatus,
    models::GlobalState,
    pages::history::simple_hex,
    utils::{
        context::{WebCallBackContext, get_web_callback_context},
        contract_helper::get_staking,
        sdk_models::{JsonSDKHandshakeResponse, ResponseWrapper},
    },
};
use ethers::types::{H160, U256};
use ethers_providers::{Http, Provider};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::staking::Staking;
// use lit_sdk::models::response::JsonSDKHandshakeResponse;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(impl_vec_data_provider)]
#[table(classes_provider = "BootstrapClassesPreset")]
pub struct Validator {
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
    pub ver: String,
    #[table(skip)]
    pub hs_status: RwSignal<Option<JsonSDKHandshakeResponse>>,
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

impl Validator {
    pub fn status2(&self) -> String {
        let status = self.hs_status.get_untracked();
        match status {
            Some(status) => status.node_version,
            None => "?".to_string(),
        }
    }
}

#[component]
pub fn Validators() -> impl IntoView {
    crate::utils::set_header("Validators (by realm)");
    let handshake_state = RwSignal::new("Loading... Please wait...".to_string());
    let realms = LocalResource::new(move || {
        let ctx = get_web_callback_context();
        async move { get_validators_for_all_realms(&ctx, handshake_state.clone()).await }
    });
    let floaters = LocalResource::new(move || {
        let ctx = get_web_callback_context();
        async move { get_floaters(&ctx, handshake_state.clone()).await }
    });

    view! {
        <Title text="Validators (by realm)"/>
        {
        move || match realms.get().as_deref() {
            None => view! { <p>{ move || handshake_state.get() }</p> }.into_any(),
            Some(realms) =>
                realms.iter().map(|realm|
                    view! {
                        <div class="col-12"><h4>Realm: {realm.id}</h4></div>
                        <div class="col-12"><NetworkStatus realm_id=realm.id as u64 /></div>
                        <div class="row">
                            <div class="col-md-6">
                                <div class="card" >
                                    <div class="card-header">
                                        <b class="card-title">Current Nodes</b>
                                    </div>
                                    <div class="card-body">
                                        <table class="table">
                                            <TableContent rows = realm.current_validators.clone() scroll_container="html"  />
                                        </table>
                                    </div>
                                </div>
                            </div>
                            <div class="col-md-6">
                                <div class="card" >
                                    <div class="card-header">
                                        <b class="card-title">Next Nodes</b>
                                    </div>
                                    <div class="card-body">
                                        <table class="table">
                                            <TableContent rows = realm.next_validators.clone() scroll_container="html"  />
                                        </table>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <br />
                    }).collect_view().into_any()
                }
            }



        <h4>Staked Inactive</h4>
        <div class="card" >
            <div class="card-body">
            <h5 class="card-title">Nodes awaiting deployment</h5>

            {move || match floaters.get().as_deref() {
                None => view! { <p>"Loading..."</p> }.into_any(),
                Some(rows) => view! {
                    <table class="table">
                        <TableContent rows = rows.clone() scroll_container="html"  />
                    </table>
                }.into_any()
            }}

            </div>
        </div>
        <br />


    }
}

pub async fn get_validators_for_all_realms(
    ctx: &WebCallBackContext,
    handshake_state: RwSignal<String>,
) -> Vec<Realm> {
    let staking = get_staking(ctx).await;
    let num_realms = U256::from(1); //  staking.num_realms().call().await.unwrap();
    let num_realms = num_realms.as_u64() as u8;

    let mut realms = Vec::new();
    for i in 1..=num_realms {
        let current_validators = get_validators(&staking, true, i, handshake_state, true).await;
        let next_validators = get_validators(&staking, false, i, handshake_state, true).await;
        realms.push(Realm {
            id: i,
            current_validators,
            next_validators,
        });
    }

    realms
}

pub async fn get_floaters(
    ctx: &WebCallBackContext,
    handshake_state: RwSignal<String>,
) -> Vec<Validator> {
    let staking = get_staking(ctx).await;
    get_validators(&staking, true, 0, handshake_state, true).await
}

pub async fn get_validators(
    staking: &Staking<Provider<Http>>,
    is_current: bool,
    realm_id: u8,
    handshake_state: RwSignal<String>,
    do_handshake_node: bool,
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

    let validators = match is_current {
        true => match realm_id.as_u64() {
            0 => {
                let reserve_addresses = staking.get_all_reserve_validators().call().await;
                let reserve_addresses = reserve_addresses.unwrap();
                staking
                    .get_validators_structs(reserve_addresses)
                    .call()
                    .await
            }
            _ => {
                staking
                    .get_validators_structs_in_current_epoch(realm_id)
                    .call()
                    .await
            }
        },
        false => {
            staking
                .get_validators_structs_in_next_epoch(realm_id)
                .call()
                .await
        }
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
        let guest_ip = socket_address.split(":").nth(0).unwrap().to_string();
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
        let hs_status = RwSignal::new(None);

        rows.push(Validator {
            id: count,
            status: match kicked.contains(&v.node_address) {
                true => "K".to_string(),
                false => "".to_string(),
            },
            socket_address: socket_address.clone(),
            wallet_address: format!("0x{}", hex::encode(v.node_address.as_bytes())),
            staker_address: format!("0x{}", hex::encode(staker_address.as_bytes())),
            ver: "?".to_string(),
            host_name: info,
            hs_status: hs_status.clone(),
        });

        // if do_handshake_node {

        //         leptos::task::spawn_local(async move {

        //         log::info!("inside task"    );
        //         let handshake_result = handshake_node(socket_address).await;

        //         hs_status.set(Some(handshake_result));
        //         // log::info!("Handshake result: {:?}", handshake_result);
        //     });
        // }

        // hs_signals.push(hs_status);
    }
    rows.sort_by(|a, b| a.staker_address.cmp(&b.staker_address));
    // let do_handshake_node = false;

    if do_handshake_node {
        for row in &mut rows {
            row.id = count;
            count += 1;

            log::info!("Handshaking node: {:?}", row.socket_address);
            handshake_state.set(format!("Handshaking node {}...", row.host_name));
            let socket_address = row.socket_address.clone();
            let handshake_result = handshake_node(socket_address).await;

            log::info!("Handshake result: {:?}", handshake_result);
            row.ver = handshake_result.node_version;
            if row.status.is_empty() && !handshake_result.network_public_key.is_empty() {
                row.status = "Up".to_string();
            }
            // if shadow_ids.contains(&row.staker_address) {
            //     row.status += " (S";
            //     if non_shadow_ids.contains(&row.staker_address) {
            //         row.status += " /NS";
            //     }
            //     row.status += ")";
            // }
        }
    }

    rows
}

async fn handshake_node(socket_address: String) -> JsonSDKHandshakeResponse {
    let socket_address = format!("https://{}", socket_address);
    log::info!("Handshaking node: {:?}", socket_address);
    if socket_address.contains("0.0.0.0") {
        return JsonSDKHandshakeResponse::default();
    }

    let json_body = r#"{"clientPublicKey":"blah","challenge":"0x1234123412341234123412341234123412341234123412341234123412341234"}"#.to_string();
    let cmd = "/web/handshake";
    let request_id = &uuid::Uuid::new_v4().to_string();
    let client = reqwest::Client::new();
    let resp_string = client
        .post(format!("{}/{}", socket_address, cmd))
        .header("Content-Type", "application/json")
        .header("X-Request-Id", request_id)
        .body(json_body.clone())
        .send()
        .await;

    if resp_string.is_err() {
        log::error!("Error getting node info: {:?}", resp_string.err());
        return JsonSDKHandshakeResponse::default();
    }

    let resp = resp_string.unwrap();

    let resp_string = match resp.text().await {
        Ok(text) => text,
        Err(e) => {
            log::error!("Error getting handshake body: {:?}", e);
            return JsonSDKHandshakeResponse::default();
        }
    };

    let response_wrapper: ResponseWrapper = match serde_json::from_str(&resp_string) {
        Ok(response_wrapper) => response_wrapper,
        Err(e) => {
            log::error!("Error parsing response wrapper: {:?}", e);
            return JsonSDKHandshakeResponse::default();
        }
    };

    let handshake_result: JsonSDKHandshakeResponse = match response_wrapper.ok {
        true => match serde_json::from_value(response_wrapper.data) {
            Ok(handshake_result) => handshake_result,
            Err(e) => {
                log::error!("Error parsing handshake response: {:?}", e);
                return JsonSDKHandshakeResponse::default();
            }
        },
        false => {
            if let Some(error_object) = response_wrapper.error_object {
                let error_handshake_result: JsonSDKHandshakeResponse =
                    match serde_json::from_str(&error_object) {
                        Ok(error_handshake_result) => error_handshake_result,
                        Err(e) => {
                            log::error!("Error parsing error handshake response: {:?}", e);
                            JsonSDKHandshakeResponse::default()
                        }
                    };
                error_handshake_result
            } else {
                JsonSDKHandshakeResponse::default()
            }
        }
    };

    handshake_result
}
