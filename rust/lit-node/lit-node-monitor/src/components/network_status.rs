use crate::models::GlobalState;
use leptos::prelude::*;

#[component]
pub fn NetworkStatus(realm_id: u64) -> impl IntoView {
    let mut gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let index = gs.index_for_realm_id(realm_id);
    crate::utils::polling::poll_network(realm_id);

    view! {
        <div class="card" >
            <div class="card-header">

            { move || format!("Status: {} at epoch# {}",  gs.network_state.get()[index].network_state.get(), gs.network_state.get()[index].epoch.get() ) }

            </div>
        </div>
        <br />
    }
}
