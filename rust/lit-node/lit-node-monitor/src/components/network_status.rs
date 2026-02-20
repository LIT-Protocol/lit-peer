use crate::{models::GlobalState, utils::responsive::is_mobile};
use leptos::prelude::*;

#[component]
pub fn NetworkStatus(realm_id: u64) -> impl IntoView {
    let mut gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let index = gs.index_for_realm_id(realm_id);
    crate::utils::polling::poll_network(realm_id);

    let status_text = if is_mobile() { "" } else { "Status: " };

    view! {
        <div>
            { move || format!("{} {} at epoch# {}", status_text,  gs.network_state.get()[index].network_state.get(), gs.network_state.get()[index].epoch.get() ) }
        </div>
    }
}
