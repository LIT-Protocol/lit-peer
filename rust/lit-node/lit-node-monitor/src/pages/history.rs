use crate::components::bottom_modal::BottomModal;
use crate::components::network_status_at_block::NetWorkStatusAtBlock;
use crate::components::right_drawer::RightDrawer;
use crate::models::GlobalState;
use crate::utils::get_address;
use crate::utils::get_lit_config;
use crate::utils::rpc_calls;
use chrono::Days;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::{
    contract_resolver, lit_token, pkp_helper, pkpnft, pubkey_router, staking, ledger, price_feed
};
use serde::{Deserialize, Serialize};
use thaw::DatePicker;
use thaw::TimePicker;
use thaw::{Checkbox, Label, Pagination, Select};

#[derive(TableRow, Clone, Serialize, Deserialize, Debug)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct ChainHistoryRow {
    #[table(skip)]
    block_hash: String,
    #[table(skip)]
    time_stamp: String,
    #[table(renderer = "TransactionRenderer", title = "Transaction")]
    transaction: String,
    #[table(renderer = "DescriptionRenderer")]
    description: String,
    #[table(title = "Block")]
    block_number: String,
    #[table(renderer = "ToFromRenderer", title = "From/To")]
    to_from: String,
    #[table(skip)]
    from: String,
    #[table(skip)]
    to: String,
    #[table(skip)]
    decoded_input: String,
}

// Easy cell renderer that just displays an image from an URL.
#[component]
fn DescriptionRenderer(
    class: String,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    value: Signal<String>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    row: RwSignal<ChainHistoryRow>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    index: usize,
) -> impl IntoView {
    let description = row.get_untracked().decoded_input;
    let description = description
        .split("|")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let name = description[1].clone();
    let details = match description.len() {
        1 => "".to_string(),
        _ => description[2].clone(),
    };

    let details = details
        .split("~")
        .map(|d| d.to_string())
        .collect::<Vec<String>>();
    let details = details.join(", ");

    view! {
        <td class=class>
            {name}<Label>"("</Label> <i class="text-muted">{details}</i> <Label>")"</Label>
        </td>
    }
}

#[component]
fn TransactionRenderer(
    class: String,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    #[prop(into)]
    value: Signal<String>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    row: RwSignal<ChainHistoryRow>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    index: usize,
) -> impl IntoView {
    view! {
        <td class=class>
            {simple_hex(row.get_untracked().transaction)}
            <br/>
            {row.get_untracked().time_stamp}
        </td>
    }
}

#[component]
fn ToFromRenderer(
    class: String,
    #[allow(unused_variables)] //value needs to be part of the signature
    #[prop(into)]
    value: Signal<String>,
    row: RwSignal<ChainHistoryRow>,
    #[allow(unused_variables)] //index needs to be part of the signature
    index: usize,
) -> impl IntoView {
    let to = row.get_untracked().to;
    let from = row.get_untracked().from;

    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let common_addresses = gs.common_addresses.get_untracked();
    let from = common_addresses
        .get(&from.clone())
        .unwrap_or(&from.clone())
        .clone();
    let to = common_addresses
        .get(&to.clone())
        .unwrap_or(&to.clone())
        .clone();

    view! {
        <td class=class>
            {from}
            <br/>
            {to}
        </td>
    }
}

#[component]
pub fn History() -> impl IntoView {
    let page = RwSignal::new(1);
    let open_buttom = RwSignal::new(false);
    let selected_index = RwSignal::new(None);
    let pop_up_title = RwSignal::new("Network Status at Block".to_string());
    let page_size = RwSignal::new("20".to_string());
    let include_internal_transactions = RwSignal::new(false);
    let filter_title = RwSignal::new("Filters".to_string());
    let filter_open = RwSignal::new(false);
    let filter_text = RwSignal::new("No filters".to_string());
    let pagination_pages = RwSignal::new(100);

    let time_zones = vec![
        chrono_tz::UTC,
        chrono_tz::US::Pacific,
        chrono_tz::US::Mountain,
        chrono_tz::US::Central,
        chrono_tz::US::Eastern,
        chrono_tz::Europe::London,
    ];

    let time_zone = RwSignal::new(time_zones[0].to_string());
    let start_date = RwSignal::new(Some(
        Local::now()
            .checked_sub_days(Days::new(1))
            .unwrap()
            .date_naive(),
    ));
    let start_time = RwSignal::new(NaiveTime::from_num_seconds_from_midnight_opt(0, 0));
    let end_date = RwSignal::new(Some(
        Local::now()
            .checked_add_days(Days::new(1))
            .unwrap()
            .date_naive(),
    ));
    let end_time = RwSignal::new(NaiveTime::from_num_seconds_from_midnight_opt(0, 0));
    let start_block = RwSignal::new("1000".to_string());
    let end_block = RwSignal::new("115494129".to_string());

    let (sel_row_read, sel_row_write) = signal(None::<ChainHistoryRow>);

    let data = LocalResource::new(move || async move {
        fetch_chain_rows(
            page.read_only(),
            page_size.read_only(),
            include_internal_transactions.read_only(),
            start_block.read_only(),
            end_block.read_only(),
            time_zone.read_only(),
            start_date.read_only(),
            start_time.read_only(),
            end_date.read_only(),
            end_time.read_only(),
            filter_text,
            pagination_pages.write_only(),
        )
        .await
    });

    crate::utils::set_header("History");

    view! {
           <Title text="History"/>
           <div class="card" >
               <div class="card-header">
                   <div class="row">
                       <div class="col">
                           <b class="mb-0"> Network History </b>
                       </div>
                       <div class="col text-end">
                           <Label on:click={move |_| filter_open.set(true)}> {move || filter_text.get()} </Label>
                       </div>
                   </div>
               </div>
               <div class="card-body">
                    {move || match data.get().as_deref() {
                       None => view! { <p>"Loading..."</p> }.into_any(),
                       Some(rows) => view! {
                           <table class="table">
                               <TableContent
                                selection=Selection::Single(selected_index)
                                   on_selection_change={move |evt: SelectionChangeEvent<ChainHistoryRow>| {
                                       log::info!("evt: {:?}", evt);
                                       sel_row_write.write().replace(evt.row.get_untracked());
                                       open_buttom.set(true);
                                   }}

                                rows = rows.clone() scroll_container="html" />
                           </table>
                           }.into_any()
                   }}
               </div>
    <div class="card-footer">
                   <div class="row">
                       <div class="col-6">
                       <Pagination page page_count=pagination_pages />
                       </div>
                       <div class="col-5 text-end">
                           <Checkbox checked=include_internal_transactions />
                           "Include Internal Transactions  |  Page Size: "

                       </div>
                       <div class="col-1">
                       <Select value=page_size  >
                           <option value=10>10</option>
                           <option value=20>20</option>
                           <option value=30>30</option>
                           <option value=50>50</option>
                           <option value=100>100</option>
                       </Select> </div>
                   </div>
               </div>
           </div>
           <br />

              { move || sel_row_read.get().map(|selected_row| {
                   let block_number = selected_row.block_number.parse::<u64>().unwrap();
                   let block_time = selected_row.time_stamp.clone();
                   let title = format!("Network at Block {}", block_number);
                   pop_up_title.set(title);
                   view! {
                       <BottomModal open=open_buttom title=pop_up_title.clone() >
                               <NetWorkStatusAtBlock realm_id=1 block_number block_time />
                       </BottomModal>
                   }
               }) }
       <RightDrawer open=filter_open title=filter_title >
                   <div class="row">
                       <div class="col-12">
                           "Time Zone"
                       </div>
                       <div class="col-12">
                           <Select value=time_zone  >
                               {time_zones.iter().map(|tz| view! { <option value=tz.to_string()>{tz.to_string()}</option> }).collect::<Vec<_>>()}
                           </Select>
                       </div>
                       <div class="col-12">
                           <br />"From"
                       </div>
                       <div class="col-12">
                           <DatePicker value=start_date  />
                       </div>
                       <div class="col-12">
                           <TimePicker value=start_time  />
                       </div>
                       <div class="col-12">
                           <br />"To"
                       </div>
                       <div class="col-12">
                           <DatePicker value=end_date  />
                       </div>
                       <div class="col-12">
                           <TimePicker value=end_time  />
                       </div>
                   </div>

               </RightDrawer>
       }
}

#[component]
pub fn NetWorkStatus(realm_id: u64, block_number: String) -> impl IntoView {
    let block_number = block_number.parse::<u64>().unwrap();
    let data =
        LocalResource::new(move || async move { get_network_status(realm_id, block_number).await });

    view! {
        <div>
            {move || data.get().map(|d| format!("Network Status: {:?}", d))}
        </div>
    }
}

pub async fn get_network_status(realm_id: u64, block_number: u64) -> String {
    let realm_id = ethers::types::U256::from(realm_id);

    let address = match get_address(crate::contracts::STAKING_CONTRACT).await {
        Ok(address) => address,
        Err(e) => {
            log::warn!("Error getting staking contract address: {:?}", e);
            return "Error getting staking contract address".to_string();
        }
    };
    let cfg = &get_lit_config();
    let staking = crate::contracts::staking::Staking::node_monitor_load(cfg, address).unwrap();

    let mut epoch_details = staking.epoch(realm_id);
    epoch_details.block = Some(block_number.into());
    let epoch_details = epoch_details.call().await.unwrap();

    let number = epoch_details.number;
    let mut state_call = staking.state(realm_id);
    state_call.block = Some(block_number.into());
    let state = state_call.call().await.unwrap();
    let network_state = match state {
        0 => "Active".to_string(),
        1 => "NextValidatorSetLocked".to_string(),
        2 => "ReadyForNextEpoch".to_string(),
        3 => "Unlocked".to_string(),
        4 => "Paused".to_string(),
        5 => "Restore".to_string(),
        _ => "Unknown".to_string(),
    };

    let mut kicked_validators = staking.get_kicked_validators(realm_id);
    kicked_validators.block = Some(block_number.into());
    let kicked_validators = kicked_validators.call().await.unwrap();

    let mut validators_call = staking.get_validators_in_current_epoch(realm_id);
    validators_call.block = Some(block_number.into());
    let validators = validators_call.call().await.unwrap();

    format!(
        "Network Status: {} at epoch# {} | Validators: {} | Kicked Validators: {}",
        network_state,
        number,
        validators.len(),
        kicked_validators.len()
    )
}

pub async fn fetch_chain_rows(
    page_signal: ReadSignal<usize>,
    page_size: ReadSignal<String>,
    include_internal_transactions: ReadSignal<bool>,
    start_block: ReadSignal<String>,
    end_block: ReadSignal<String>,
    time_zone: ReadSignal<String>,
    start_date: ReadSignal<Option<NaiveDate>>,
    start_time: ReadSignal<Option<NaiveTime>>,
    end_date: ReadSignal<Option<NaiveDate>>,
    end_time: ReadSignal<Option<NaiveTime>>,
    filter_text: RwSignal<String>,
    pagination_pages: WriteSignal<usize>,
) -> Vec<ChainHistoryRow> {
    let page = move || page_signal.get() as u64;
    let include_internal_transactions = move || include_internal_transactions.get();
    let page_size = move || page_size.get().parse::<u64>().unwrap();
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let rpc_api_type = gs.active_network().rpc_api_type.into();
    let chain_api_url =  match   &gs.active_network().chain_api_url.contains("127.0.0.1") {
        true =>  gs.active_network().chain_api_url.clone(),
        false =>   format!("{}{}", &gs.proxy_url, &gs.active_network().chain_api_url)
    };
    let address = &get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .expect("Error getting staking contract address");
    let address = &hex::encode(address.0);
    log::info!("address: {:?}", address);
    let block_start = start_block.get().parse::<u64>().unwrap();
    let block_end = end_block.get().parse::<u64>().unwrap();
    let start_date = match start_date.get() {
        Some(date) => Some(NaiveDateTime::new(
            date,
            start_time.get().unwrap_or_default(),
        )),
        None => None,
    };

    let end_date = match end_date.get() {
        Some(date) => Some(NaiveDateTime::new(date, end_time.get().unwrap_or_default())),
        None => None,
    };

    filter_text.set(format!(
        "Date Range: {:?} to {:?} {}",
        start_date.unwrap().format("%Y-%m-%d %H:%M").to_string(),
        end_date.unwrap().format("%Y-%m-%d %H:%M").to_string(),
        time_zone.get()
    ));
    // let block_end = 161000001000;
    let txs = rpc_calls::get_tx_list_async(
        rpc_api_type,
        &chain_api_url,
        address,
        block_start,
        block_end,
        start_date,
        end_date,
        include_internal_transactions(),
        page(),
        page_size(),
    )
    .await
    .unwrap();

    let time_zone = time_zone.get();

    log::info!("txs.len(): {:?}", txs.len());
    log::info!("page_size(): {:?}", page_size());
    log::info!(
        "txs.len() / page_size() as usize: {:?}",
        txs.len() / page_size() as usize
    );
    pagination_pages.set(txs.len() / page_size() as usize);

    let rows = txs
        .iter()
        .map(|tx| ChainHistoryRow {
            block_number: tx.block_number.clone(),
            transaction: tx.hash.clone(),
            block_hash: tx.block_hash.clone(),
            time_stamp: tx.chain_time_stamp(&time_zone),
            description: "".to_string(),
            from: tx.from.clone(),
            to: tx.to.clone(),
            to_from: "".to_string(),
            decoded_input: get_description(tx.input.clone()),
        })
        .collect();
    rows
}

pub fn simple_hex(input: String) -> String {
    if input.len() < 8 {
        return input;
    }
    format!("{}..{}", &input[0..6], &input[input.len() - 4..])
}

pub fn get_description(input: String) -> String {
    // log::info!("Input: {:?}", input);
    if input.is_empty() {
        return "".to_string();
    }
    // / to handle the forwarder request, we remove the function
    // / struct ForwardRequest {
    // /     address from;  40 chars
    // /     address to;    40 chars
    // /     uint256 value; 64 chars
    // /     uint256 gas;   64 chars
    // /     uint256 nonce; 64 chars
    // /     bytes data; << this is our data
    // / }

    let short_signature = &input[2..10];
    let data = &input[10..];

    if short_signature == "60806040" {
        return "Contract Deployment".to_string();
    }

    let (short_signature, data) = if short_signature == "47153f82" {
        let fwd_pad = 10 + 9 * 64;
        (&input[fwd_pad..fwd_pad + 8], &input[fwd_pad + 8..])
    } else {
        (short_signature, data)
    };

    // log::info!("Short Signature: {:?}", short_signature);

    let (contract_name, abi_function) = get_abi_function(short_signature);

    let data_bytes = hex::decode(data).unwrap();
    let tokens = abi_function.decode_input(&data_bytes);
    let inputs = abi_function
        .inputs
        .iter()
        .map(|i| i.name.clone())
        .collect::<Vec<String>>();
    let input_values = tokens.unwrap();

    // map inputs and output values by index
    let input_values = input_values
        .iter()
        .enumerate()
        .map(|(i, v)| format!("{}: {}", inputs[i], format_token(v)))
        .collect::<Vec<String>>();

    format!(
        "{}|{}|{}",
        contract_name,
        abi_function.name,
        input_values.join("~ ")
    )
}

fn get_abi_function(short_signature: &str) -> (&str, &ethers::abi::Function) {
    let abi_function = staking::STAKING_ABI
        .functions()
        .find(|f| hex::encode(f.short_signature()) == short_signature);
    if let Some(abi_function) = abi_function {
        return ("Staking", abi_function);
    }
    let abi_function = pubkey_router::PUBKEYROUTER_ABI
        .functions()
        .find(|f| hex::encode(f.short_signature()) == short_signature);
    if let Some(abi_function) = abi_function {
        return ("PubkeyRouter", abi_function);
    }

    let abi_function = pkpnft::PKPNFT_ABI
        .functions()
        .find(|f| hex::encode(f.short_signature()) == short_signature);
    if let Some(abi_function) = abi_function {
        return ("PKPNFT", abi_function);
    }

    let abi_function = pkp_helper::PKPHELPER_ABI
        .functions()
        .find(|f| hex::encode(f.short_signature()) == short_signature);
    if let Some(abi_function) = abi_function {
        return ("PKPHelper", abi_function);
    }

    let abi_function = lit_token::LITTOKEN_ABI
        .functions()
        .find(|f| hex::encode(f.short_signature()) == short_signature);
    if let Some(abi_function) = abi_function {
        return ("LITToken", abi_function);
    }

    let abi_function = contract_resolver::CONTRACTRESOLVER_ABI
        .functions()
        .find(|f| hex::encode(f.short_signature()) == short_signature);
    if let Some(abi_function) = abi_function {
        return ("ContractResolver", abi_function);
    }

    let abi_function = ledger::LEDGER_ABI
        .functions()
        .find(|f| hex::encode(f.short_signature()) == short_signature);
    if let Some(abi_function) = abi_function {
        return ("Ledger", abi_function);
    }

    let abi_function = price_feed::PRICEFEED_ABI
        .functions()
        .find(|f| hex::encode(f.short_signature()) == short_signature);
    if let Some(abi_function) = abi_function {
        return ("PriceFeed", abi_function);
    }
    log::error!("Unknown function: {:?}", short_signature);
    let abi_function = staking::STAKING_ABI.functions().last().unwrap();
    ("Unknown", abi_function)
}

fn format_token(token: &ethers::abi::Token) -> String {
    // log::info!("Token: {:?}", token);
    match token {
        ethers::abi::Token::Uint(u) => format!("{}", u),
        ethers::abi::Token::Int(i) => format!("{}", i),
        ethers::abi::Token::Bool(b) => format!("{}", b),
        ethers::abi::Token::Address(a) => simple_hex(format!("0x{}", a)),
        ethers::abi::Token::Array(a) => a
            .iter()
            .map(format_token)
            .collect::<Vec<String>>()
            .join(", ")
            .to_string(),
        ethers::abi::Token::Tuple(t) => t
            .iter()
            .map(format_token)
            .collect::<Vec<String>>()
            .join(", ")
            .to_string(),
        ethers::abi::Token::Bytes(b) => simple_hex(format!("0x{}", hex::encode(b))),
        _ => format!("{:?}", token),
    }
}
