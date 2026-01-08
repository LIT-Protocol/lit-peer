use crate::components::right_drawer::RightDrawer;
use crate::utils::{get_address, get_lit_config};
use chrono::Days;
use chrono::Local;
use chrono::NaiveTime;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::ledger::Ledger;
use lit_blockchain_lite::contracts::ledger::WithdrawRequest;
use thaw::DatePicker;
use thaw::TimePicker;
use thaw::{Button, Checkbox, Input, Label, Pagination, Select, Card, CardHeader, CardPreview};

use super::history::ChainHistoryRow;
use super::history::fetch_chain_tx_rows;

#[component]
pub fn AccountInspector() -> impl IntoView {
    let page = RwSignal::new(1);
    let open_buttom = RwSignal::new(false);
    let selected_index = RwSignal::new(None);
    let page_size = RwSignal::new("20".to_string());
    let include_internal_transactions = RwSignal::new(false);
    let filter_title = RwSignal::new("Filters".to_string());
    let filter_open = RwSignal::new(false);
    let filter_text = RwSignal::new("No filters".to_string());
    let pagination_pages = RwSignal::new(100);
    let alt_address = RwSignal::new(None);

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

    let sel_row_write = RwSignal::new(None::<ChainHistoryRow>).write_only();
    let address_input_value = RwSignal::new("".to_string());

    let data = LocalResource::new(move || async move {
        fetch_chain_tx_rows(
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
            alt_address.clone().read_only(),
        )
        .await
    });

    let overview = LocalResource::new(move || async move {
        get_overview(alt_address.clone().read_only()).await
    });

    crate::utils::set_header("History");

    view! {
           <Title text="Account Inspector"/>
           <Card class="min-w-full">
               <CardHeader>
                   <div class="row">
                       <div class="col">
                           <b class="mb-0"> Wallet Account Inspector </b>
                       </div>
                       <div class="col text-end">
                           <Label on:click={move |_| filter_open.set(true)}> {move || filter_text.get()} </Label>
                       </div>
                   </div>
               </CardHeader>


               // thaw text input for address
               <CardPreview class="p-3">
                   <Input value=address_input_value input_size=45 />
                   <Button on:click={move |_| alt_address.set(Some(address_input_value.get().to_string()))}> "Search" </Button>
                   <br/><br/>
                   {move || match overview.get().as_deref() {
                       None => view! { <p>"Loading..."</p> }.into_any(),
                       Some(balance) => view! { <p>"Balance for address: "    {balance.to_string()} </p> }.into_any()
                   }}
               </CardPreview>
           </Card>

               <Card class="m-3 min-w-full">
               <CardHeader>
                   <div class="row">
                       <div class="col">
                           <b class="mb-0"> Ledger Transactions for address: {address_input_value.get()}</b>
                       </div>                     
                   </div>
               </CardHeader>

                <CardPreview class="p-3">
                    {move || match data.get().as_deref() {
                       None => view! { <p>"Loading..."</p> }.into_any(),
                       Some(rows) => view! {
                           <table class="table w-full">
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
                </CardPreview>
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
               </Card>
           <br />


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

async fn get_overview(user_address: ReadSignal<Option<String>>) -> String {
    
    if user_address.get().is_none() {
        return "".to_string();
    }
    let user_address = user_address.get().unwrap();

    let address = get_address(crate::contracts::LEDGER_CONTRACT)
        .await
        .unwrap();

    let cfg = &get_lit_config();
    let ledger = Ledger::node_monitor_load(cfg, address).unwrap();


    let user_address = hex::decode(user_address.replace("0x", "")).unwrap();
    let user_address = ethers::types::H160::from_slice(&user_address);

    let balance = ledger.balance(user_address).call().await.unwrap();
    
    let latest_withdraw_request : WithdrawRequest = ledger.latest_withdraw_request(user_address).call().await.unwrap();
    let latest_withdraw_request_amount = latest_withdraw_request.amount;


    let stable_balance = ledger.stable_balance(user_address).call().await.unwrap();

    format!("{}, Stable Balance: {}, Latest Withdraw Request Amount: {}", balance.to_string(), stable_balance.to_string(), latest_withdraw_request_amount.to_string())
    
}
