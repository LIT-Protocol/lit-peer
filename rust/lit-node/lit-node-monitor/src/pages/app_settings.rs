use crate::utils::context::{WebCallBackContext, get_web_callback_context};
use ethers::types::U64;
use ethers_providers::{Http, Provider, ProviderError};
use leptos::prelude::*;
use leptos_meta::*;
use thaw::*;
#[component]
pub fn ChainConfig() -> impl IntoView {
    let ctx = get_web_callback_context();
    let value = RwSignal::new(0.0);
    let fast_forward_blocks_value = RwSignal::new("".to_string());
    crate::utils::set_header("App Settings");

    view! {
        <Title text="App Settings"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">App Settings</b>
            </div>
            <div class="card-body">

                <label class="pt-2">Poll speed: {value.get()} seconds.  </label>
                <Slider value min=1.0 max=30.0 step=5.0 style="width: 200px;">
                    <SliderLabel value=0.0>
                        "0"
                    </SliderLabel>
                    <SliderLabel value=5.0>
                        "5"
                    </SliderLabel>
                    <SliderLabel value=10.0>
                        "10"
                    </SliderLabel>
                </Slider>
                <br />
                <br />
                <label class="pt-2">Note that these commands are mostly for testing purposes and are specific to the Anvil network.</label>
                <br/><br/>

                <label for="fast_forward_blocks" class="pt-2">Fast Forward Blocks:</label>
                <Input input_size=5 attr:id="fast_forward_blocks" value=fast_forward_blocks_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| { fast_forward_blocks_function( &ctx, &fast_forward_blocks_value.get()); } }> "Fast Forward" </Button>


            </div>
        </div>
    }
}

fn fast_forward_blocks_function(ctx: &WebCallBackContext, fast_forward_blocks: &str) {
    log::info!("Fast Forward Blocks !");
    let ctx = ctx.clone();
    let fast_forward_blocks = fast_forward_blocks.to_string();
    leptos::task::spawn_local(async move {
        let network = ctx.active_network.clone();
        let provider = Provider::<Http>::try_from(network.chain_url)
            .expect("could not instantiate HTTP Provider");

        let old_block_number: Result<U64, ProviderError> =
            provider.request("eth_blockNumber", ()).await;

        ctx.show_info(
            "Attempting to fast forward blocks",
            format!("Current block number: {:?}", old_block_number).as_str(),
        );

        let command = "anvil_mine";
        let mine_blocks_res: Result<(), ProviderError> = provider
            .request(
                command,
                [
                    serialize(&format!("0x{}", fast_forward_blocks)),
                    serialize(&0),
                ],
            )
            .await;

        let new_block_number: Result<U64, ProviderError> =
            provider.request("eth_blockNumber", ()).await;

        if mine_blocks_res.is_ok() {
            ctx.show_success(
                "Blocks mined successfully",
                format!("{:?} -> {:?}", old_block_number, new_block_number).as_str(),
            );
        } else {
            ctx.show_error(
                "Failed to mine blocks",
                format!("{:?} ", mine_blocks_res).as_str(),
            );
        }
    });
}

pub fn serialize<T: serde::Serialize>(t: &T) -> serde_json::Value {
    serde_json::to_value(t).expect("Failed to serialize value")
}
