use crate::{
    components::network_status_at_block::NetWorkStatusAtBlock, models::GlobalState,
    utils::rpc_calls,
};
use chrono::{FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use leptos::prelude::*;
use leptos_meta::*;
use thaw::{DatePicker, Slider, SliderLabel, TimePicker, Button};

#[component]
pub fn StatusAtTime() -> impl IntoView {
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let rpc_api_type: u32 = gs.active_network().rpc_api_type.into();
    let chain_api_url = format!("{}{}", &gs.proxy_url, &gs.active_network().chain_api_url);

    let selected_date = RwSignal::new(Some(Local::now().date_naive()));
    let selected_time = RwSignal::new(NaiveTime::from_num_seconds_from_midnight_opt(0, 0));
    let selected_date_time = RwSignal::new(Some(Local::now().naive_utc()));
    let (selected_date_time_read, selected_date_time_write) = selected_date_time.split();

    let block_number_input: u64 = 6668050;
    let (block_number_read, block_number_write) = RwSignal::new(Some(block_number_input)).split();

    let slider_value = RwSignal::new(block_number_input as f64);
    let slider_max = RwSignal::new(block_number_input as f64 + 5.0);
    let slider_min = RwSignal::new(block_number_input as f64 - 5.0);
    let (_slider_max_read, slider_max_write) = slider_max.split();
    let (_slider_min_read, slider_min_write) = slider_min.split();

    let time_delta = TimeDelta::seconds(3);

    crate::utils::set_header("Status At Time");

    let chain_api_url2 = chain_api_url.clone();
    LocalResource::new(move || {
        let chain_api_url = chain_api_url2.clone();
        async move {
            get_block_time(
                rpc_api_type,
                &chain_api_url,
                selected_date_time_read,
                block_number_write,
                slider_max_write,
                slider_min_write,
            )
            .await;
        }
    });

    LocalResource::new(move || {
        let chain_api_url = chain_api_url.clone();
        async move {
            get_datetime_from_block_number(
                rpc_api_type,
                &chain_api_url,
                block_number_read,
                selected_date_time_write,
            )
            .await;
        }
    });


    Effect::new(move || {
        if selected_date.get().is_some() && selected_time.get().is_some() {
            selected_date_time_write.set(Some(NaiveDateTime::new(selected_date.get().unwrap(), selected_time.get().unwrap())));
        }       
    });
    view! {
        <Title text="Status At Time"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Status At Time</b>
            </div>
            <div class="card-body">

            <div class="row">
                <div class="col">
                    <p>"Date/Time (UTC): "</p>
                </div>
                <div class="col">
                    <DatePicker value=selected_date  />
                </div>
                <div class="col">
                    <Button on:click={move |_| selected_time.set(Some(selected_time.get().unwrap().overflowing_add_signed(time_delta).0 ))}> "<<" </Button>
                </div>
                <div class="col">
                    <TimePicker value=selected_time  />
                </div>
                <div class="col">
                    <Button on:click={move |_| selected_time.set(Some(selected_time.get().unwrap().overflowing_sub_signed(time_delta).0 ))}> ">>" </Button>
                </div>
            </div>

            {move ||  match block_number_read.get().as_ref() {
                Some(block_number) => view! {
                    <div class="row">
                        <div class="col">
                            <p>"Block Number: " {*block_number}</p>
                        </div>
                    </div>
                    <div class="row" style="margin-top: 10px;">
                        <div class="col-1">
                            <Slider value=slider_value  vertical=true step=1.0 min=slider_min max=slider_max />
                        </div>
                        <div class="col-11">
                            <NetWorkStatusAtBlock realm_id=1 block_number=*block_number block_time="20251120 12:00:00".to_string() />
                        </div>
                    </div>
                }.into_any(),
                None => view! { <p>"Loading..."</p> }.into_any(),
            }}
            </div>
        </div>

    }
}

pub async fn get_block_time(
    rpc_api_type: u32,
    chain_api_url: &str,
    selected_date_time_read: ReadSignal<Option<NaiveDateTime>>,
    block_number_write: WriteSignal<Option<u64>>,
    slider_max_write: WriteSignal<f64>,
    slider_min_write: WriteSignal<f64>,
) {
    

    block_number_write.set(None);

    let selected_date_time = selected_date_time_read.get().unwrap();
    let block_timestamp = selected_date_time.and_utc().timestamp().to_string();
    let block_number =
        rpc_calls::get_block_number(rpc_api_type, chain_api_url, block_timestamp).await;
    let block_number = block_number.unwrap();
    block_number_write.set(Some(block_number.clone()));
    slider_max_write.set(block_number as f64 + 5.0);
    slider_min_write.set(block_number as f64 - 5.0);
}

pub async fn get_datetime_from_block_number(
    rpc_api_type: u32,
    chain_api_url: &str,
    block_number_read: ReadSignal<Option<u64>>,
    selected_date_time_write: WriteSignal<Option<NaiveDateTime>>,
) {
    if block_number_read.get().is_none() {
        return;
    }
    let block_number = block_number_read.get().unwrap();
    log::info!("block_number: {:?}", block_number);
    let datetime =
        rpc_calls::get_datetime_from_block_number(rpc_api_type, chain_api_url, block_number).await;
    let datetime = datetime.unwrap();
    // selected_date_write.set(Some(datetime.naive_utc().into()));
    // selected_time_write.set(Some(datetime.naive_utc().time()));
}
