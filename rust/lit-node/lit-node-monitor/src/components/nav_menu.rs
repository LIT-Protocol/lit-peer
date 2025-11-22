use crate::{models::GlobalState, utils::base_path};
use leptos::prelude::*;
use thaw::{NavCategory, NavCategoryItem, NavDrawer, NavItem, NavSubItem};
use web_sys::window;

#[component]
pub fn NavMenu(page_name_signal: RwSignal<String>) -> impl IntoView {
    
    let url = window()
            .and_then(|win| win.location().pathname().ok())
            .unwrap_or_else(|| "home".to_string());
    let url = url.replace(base_path(), "");
    
    let nav_value = RwSignal::new(url);
    let (nav_value_get, _nav_value_set) = nav_value.split();

    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let network_name = gs.active_network().network_name.clone();
    let updtime_url = format!("https://uptime.getlit.dev/{}", network_name);

    view! {

        { move || {
            let nav_to = nav_value_get.get();
            let page_name = format!("{} > {}", network_name, nav_to.replace("/", ""));
            page_name_signal.set(page_name);

            let navigate = leptos_router::hooks::use_navigate();
            navigate(nav_to.as_str(), Default::default()); }
        }
        <nav>
            <NavDrawer selected_value=nav_value>
                <NavItem icon=icondata::AiGlobalOutlined value="/home">
                    "Home"
                </NavItem>
                <NavItem icon=icondata::AiAppstoreOutlined value="/validators">
                    "Validators"
                </NavItem>
                <NavItem icon=icondata::AiLinkOutlined value="/history">
                    "History"
                </NavItem>
                <NavItem value="/pkps" icon=icondata::AiKeyOutlined>
                    "PKPs"
                </NavItem>
                // <NavItem icon=icondata::AiCalendarOutlined value="events">
                //     <a class={disabled_class} href="events">Events</a>
                // </NavItem>
                // <NavItem icon=icondata::AiRocketOutlined value="actions">
                //     <a class={disabled_class} href="action_playground">Actions</a>
                // </NavItem>
                <NavCategory value="/network_category">
                    <NavCategoryItem slot icon=icondata::AiAreaChartOutlined>
                        "Network Settings"
                    </NavCategoryItem>
                    <NavSubItem value="/contracts" icon=icondata::AiFileTextOutlined>
                        "Contracts"
                    </NavSubItem>
                    <NavSubItem value="/network_configuration" icon=icondata::AiSettingOutlined>
                        "Network Configuration"
                    </NavSubItem>
                    <NavSubItem value="/pricing" icon=icondata::AiDollarOutlined>
                        "Pricing"
                    </NavSubItem>
                    <NavSubItem value="/epoch" icon=icondata::AiClockCircleOutlined>
                        "Epoch Details"
                    </NavSubItem>
                    <NavSubItem value="/root_keys" icon=icondata::AiCompassOutlined>
                        "Root Keys"
                    </NavSubItem>
                </NavCategory>
                <NavCategory value="/staking_category">
                    <NavCategoryItem slot icon=icondata::AiRiseOutlined>
                        "Staking / Rewards"
                    </NavCategoryItem>
                    <NavSubItem value="/staking" icon=icondata::AiDollarOutlined>
                        "Staking"
                    </NavSubItem>
                    <NavSubItem value="/wallets" icon=icondata::AiWalletOutlined>
                        "Wallets"
                    </NavSubItem>
                    <NavSubItem value="/rewards" icon=icondata::AiGiftOutlined>
                        "Rewards"
                    </NavSubItem>
                </NavCategory>
                // <NavCategory value="/admin_category">
                //     <NavCategoryItem slot icon=icondata::AiToolOutlined>
                //         "Admin Tools"
                //     </NavCategoryItem>
                //     <NavSubItem value="/admin_tools" icon=icondata::AiToolOutlined>
                //         "Realms"
                //     </NavSubItem>
                //     <NavSubItem value="/validator_admin" icon=icondata::AiToolOutlined>
                //         "Validators"
                //     </NavSubItem>
                // </NavCategory>
                <NavItem icon=icondata::AiSettingOutlined value="/chain_config">
                    "Settings"
                </NavItem>
            </NavDrawer>
        </nav>

        <br />
        <br />

        <nav>
            <NavDrawer>
                <NavItem icon=icondata::AiCompassOutlined value="#" href="https://yellowstone-explorer.litprotocol.com/txs">
                    "Yellowstone Explorer"
                </NavItem>
                <NavItem icon=icondata::AiBarChartOutlined value="#" href=updtime_url>
                    "Uptime Monitor"
                </NavItem>
                <NavItem icon=icondata::AiFullscreenExitOutlined value="#" href="/monitor">
                    "Exit Network"
                </NavItem>
            </NavDrawer>
        </nav>
    }
}
