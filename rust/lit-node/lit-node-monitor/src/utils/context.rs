use crate::models::{FixedGlobalState, GlobalState, NetworkConfig};
use ethers_web::leptos::EthereumContext;
use leptos::prelude::use_context;
use thaw::{ToastIntent, ToasterInjection};

#[derive(Clone)]
pub struct WebCallBackContext {
    pub global_state: FixedGlobalState,
    pub ethereum_context: EthereumContext,
    pub active_network: NetworkConfig,
    pub toast_context: ToasterInjection,
}

impl Default for WebCallBackContext {
    fn default() -> Self {
        let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
        let ec = use_context::<EthereumContext>().expect("Ethereum Context Failed to Load");
        let network_config = gs.active_network().clone();
        let toast_context = ToasterInjection::expect_context();
        Self {
            global_state: gs.to_fixed(),
            ethereum_context: ec,
            active_network: network_config,
            toast_context,
        }
    }
}

impl WebCallBackContext {
    pub fn new_with_global_state(gs: GlobalState) -> Self {
        let ec = use_context::<EthereumContext>().expect("Ethereum Context Failed to Load");
        let network_config = gs.active_network().clone();
        let toast_context = ToasterInjection::expect_context();
        Self {
            global_state: gs.to_fixed(),
            ethereum_context: ec,
            active_network: network_config,
            toast_context,
        }
    }

    pub fn show_info(&self, title: &str, body: &str) {
        crate::components::toast::do_toast(self, title, body, ToastIntent::Info);
    }

    pub fn show_success(&self, title: &str, body: &str) {
        crate::components::toast::do_toast(self, title, body, ToastIntent::Success);
    }

    pub fn show_warning(&self, title: &str, body: &str) {
        crate::components::toast::do_toast(self, title, body, ToastIntent::Warning);
    }

    pub fn show_error(&self, title: &str, body: &str) {
        crate::components::toast::do_toast(self, title, body, ToastIntent::Error);
    }
}

pub fn get_web_callback_context() -> WebCallBackContext {
    WebCallBackContext::default()
}
