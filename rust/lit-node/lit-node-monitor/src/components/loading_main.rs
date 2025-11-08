use crate::utils::make_nav_url;
use leptos::prelude::*;

#[component]
pub fn LoadingMain(network_status_signal: RwSignal<String>) -> impl IntoView {
    view! {

            <div class="row p-5 mt-20">
                <div class="col-4 offset-4">
                    <div class="card text-center">
                        <div class="card-header">
                            <image class="d-inline" src=make_nav_url("/images/lit-logo-black.svg") height="36" />
                            <h5 class="text-center align-center p-2 d-inline"> Lit Protocol Network Explorer</h5>
                        </div>
                        <div class="card-body">
                            <p>{ move || network_status_signal.get() }</p>

                        </div>
                    </div>
                </div>
            </div>

    }
}
