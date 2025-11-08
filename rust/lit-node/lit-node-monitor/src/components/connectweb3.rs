use ethers_web::WalletType;
use ethers_web::leptos::EthereumContext;
use leptos::prelude::*;
use thaw::Button;

use crate::models::GlobalState;

#[component]
pub fn ConnectWeb3() -> impl IntoView {
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");

    let ec: EthereumContext = use_context::<EthereumContext>()
        .expect("Ethereum Context Failed to Load in NetworkStatus intoview");

    let ec2 = ec.clone();
    view! {
        <div class="row">
            <div class="col-9 mt-1">
                { move || format!("Connected to {} at block # {}",  gs.active_network().chain_name, gs.block.get()) }
            </div>
            <div class="col-3 text-end">
                <Button on:click=move |_| { ec2.connect(WalletType::Injected); }>
                    { move || if ec.is_connected() {
                        format!("{:?}", &ec.accounts().unwrap()[0]) }
                        else { "Connect".to_string() }      }
                </Button>
            </div>
         </div>
    }
}
