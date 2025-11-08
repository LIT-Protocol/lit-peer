// use crate::utils::context::{get_web_callback_context, WebCallBackContext};
// use crate::utils::contract_helper::get_staking_with_signer;
// use ethers::types::U64;
// use ethers_providers::{Http, Provider, ProviderError};
use leptos::prelude::*;
use leptos_meta::*;
// use gloo_timers::future::TimeoutFuture;
use codee::string::FromToStringCodec;
use leptos_use::storage::use_local_storage;

#[component]
pub fn Events() -> impl IntoView {
    // let ctx = get_web_callback_context();
    let (count, _set_count, _) = use_local_storage::<String, FromToStringCodec>("my-count");
    let time_value = LocalResource::new(move || async move { "".to_string() });
    crate::utils::set_header("Events");

    view! {
        <Title text="Events"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">App Settings</b>
            </div>
            <div class="card-body">

                count: {count}
                <label class="pt-2">Time: {move || match  time_value.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(time) => view! {
                        {format!("{:?}", time)}
                        }.into_any()
                }}  </label>

                // <br/>
                // <br/>


                <br/>
                <br/>

            </div>
        </div>
    }
}
