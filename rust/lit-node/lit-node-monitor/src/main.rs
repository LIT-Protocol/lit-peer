// TODOS:
// When we upgraded to lepto 0.7, we turned all "match arm" viewes into any.  Sub-optimal, but works.
pub mod components;
pub mod models;
pub mod pages;
pub mod utils;

use lit_blockchain_lite::contracts;

use components::connectweb3::ConnectWeb3;
use ethers_web::leptos::Ethereum;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use log::Level;
use models::GlobalState;
use thaw::*;
use utils::context::get_web_callback_context;

use crate::components::loading_main::LoadingMain;
use crate::components::nav_menu::NavMenu;
use crate::utils::base_path;

pub mod listener;

fn main() {
    mount_to_body(|| view! { <App />})
}

#[component]
pub fn App() -> impl IntoView {
    // set_interval( move || { utils::poll_network() }, Duration::from_secs(10));
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    console_error_panic_hook::set_once();

    log::info!("Starting LNE");
    let theme = RwSignal::new(Theme::light());

    let gs = GlobalState::new();
    provide_context(gs);

    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let new_networks = gs.network_write_signal().clone();
    let new_staker_names = gs.staker_names_write_signal();
    let new_common_addresses = gs.common_addresses_write_signal();
    let active_network = gs.current_network.clone();
    let network_status_signal = RwSignal::new("Loading network data ...".to_string());
    let (_, write_status_signal) = network_status_signal.split();
    let page_name_signal = RwSignal::new("Home".to_string());

    let network_loading = LocalResource::new(move || {
        GlobalState::get_refreshed_networks(
            new_networks,
            active_network,
            new_staker_names,
            new_common_addresses,
            write_status_signal,
        )
    });

    // load the networks for this instance
    move || {
        match network_loading.get() {
            None => view! {
            <LoadingMain network_status_signal />
            }
            .into_any(),
            Some(_data) => {
                view! {

    <Ethereum>
    <Router base=base_path()>
        <ConfigProvider theme>
            <ToasterProvider>
                {
                    let _listener = LocalResource::new(move || async move {
                        let ctx = get_web_callback_context();
                        let _r = listener::listen_for_events(&ctx).await;
                    });
                }
                <div class="container-fluid min-vh-100 d-flex flex-column p-2">
                        <div class="row align-items-center pb-2">
                            <div class="col-2">
                                <image class="d-inline" src="images/lit-logo-black.svg" height="36" />
                                <h5 class="text-center align-center p-2 d-inline"> Network Explorer</h5>
                            </div>
                            <div class="col-5">
                                { move || page_name_signal.get() }
                            </div>
                            <div class="col text-end align-text-top">
                                <ConnectWeb3 />
                            </div>
                        </div>


                    <div class="row">
                        <div class="col-2" style="min-width: 260px;">
                            <NavMenu page_name_signal />
                        </div>
                        <div class="col">
                            <main>
                                <Routes fallback=|| "Not found.">
                                    <Route path=path!("/") view=pages::home::Home />
                                    <Route path=path!("/home") view=pages::home::Home />
                                    <Route path=path!("/history") view=pages::history::History />
                                    <Route path=path!("/contracts") view=pages::network_settings::contracts::Contracts />
                                    <Route path=path!("events") view=pages::events::Events />
                                    <Route path=path!("/action_playground") view=pages::action_playground::ActionPlayground />
                                    <Route path=path!("/epoch") view=pages::network_settings::epoch::Epoch />
                                    <Route path=path!("/network_configuration") view=pages::network_settings::network_configuration::NetworkConfiguration />
                                    <Route path=path!("pkps") view=pages::network_settings::pkps::PKPs />
                                    <Route path=path!("/root_keys") view=pages::network_settings::root_keys::RootKeys />
                                    <Route path=path!("/validators") view=pages::validators::Validators />
                                    <Route path=path!("/staking") view=pages::staking::staking_details::StakingDetails />
                                    <Route path=path!("/wallets") view=pages::staking::wallets::Wallets />
                                    <Route path=path!("/rewards") view=pages::staking::rewards::Rewards />
                                    <Route path=path!("/admin") view=pages::admin::realms::Realms />
                                    <Route path=path!("/validator_admin") view=pages::admin::validator_admin::ValidatorAdmin />
                                    <Route path=path!("/chain_config") view=pages::app_settings::ChainConfig />
                                    <Route path=path!("/pricing") view=pages::network_settings::pricing::Pricing />
                                </Routes>
                            </main>
                        </div>
                    </div>
                    </div>
                </ToasterProvider>
            </ConfigProvider>
        </Router>
    </Ethereum>
    }.into_any()
                // }
            }
        }
    }
}
