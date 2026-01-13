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
        <div>
            { move || format!("{} : block # {} ",  gs.active_network().chain_name, gs.block.get()) }
        
            <Button on:click=move |_| { ec2.connect(WalletType::Injected); }>
                { move || if ec.is_connected() {
                    format!("{:?}", &ec.accounts().unwrap()[0])[..6].to_string() }
                    else { "Connect".to_string() }      }
            </Button>
        </div>
    }
}
