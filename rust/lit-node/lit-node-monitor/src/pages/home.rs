use crate::models::GlobalState;
use leptos::prelude::*;
use leptos_meta::*;
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {
    crate::utils::set_header("Home - Network Selection");
    let gs = expect_context::<GlobalState>();
    let network = gs.active_network();
    log::info!("Home: active network: {:?}", network.network_name.clone());
    let network_name_value = RwSignal::new(network.network_name.clone());
    let environment_value = RwSignal::new(network.environment.to_string());
    let resolver_contract_value = RwSignal::new(network.resolver_contract.clone());
    let rpc_api_type_value = RwSignal::new(network.rpc_api_type.to_string());
    let chain_url_value = RwSignal::new(network.chain_url.clone());
    let chain_name_value = RwSignal::new(network.chain_name.clone());
    let subnet_id_value = RwSignal::new(network.subnet_id.clone());
    let branch_os_value = RwSignal::new(network.branch_os.clone());
    let branch_assets_value = RwSignal::new(network.branch_assets.clone());

    let networks = gs.networks.clone();
    // log::info!(" Home 1 : networks: {:?}", networks);
    let tab_value = RwSignal::new(network.network_name.clone());
    let (tab_value_get, _tab_value_set) = tab_value.split();

    Effect::new(move |_| {
        let network_name = tab_value_get.get();
        let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
        gs.current_network.set(network_name);

        gs.network_state.get()[0].epoch.set(0);
        gs.block.set(0);
        gs.network_state.get()[0]
            .network_state
            .set("[connecting...]".to_string());
        let network = gs.active_network();

        network_name_value.set(network.network_name.clone());
        environment_value.set(network.environment.to_string());
        resolver_contract_value.set(network.resolver_contract.clone());
        rpc_api_type_value.set(network.rpc_api_type.to_string());
        chain_url_value.set(network.chain_url.clone());
        chain_name_value.set(network.chain_name.clone());
        subnet_id_value.set(network.subnet_id.clone());
        branch_os_value.set(network.branch_os.clone());
        branch_assets_value.set(network.branch_assets.clone());
    });

    view! {
        <Title text="Lit Node Explorer"/>
         <div class="card" >
            <div class="card-header">
                <b class="card-title">Lit Network Selection</b>
            </div>
            <div class="card-body">

                 <TabList selected_value = tab_value >
                //     // { tabs() }
                    { networks.get_untracked().into_iter().map(|network| {
                        view! {
                            <Tab value={network.network_name.clone()}> {network.network_name.clone()} </Tab>
                        }
                    }).collect_view() }

                 </TabList>

                <div class="m-3" />
                <Space>
                    <label for="network_name" class="pt-2">Network Name:</label>
                    <Input input_size=45 attr:id="network_name" value=network_name_value />
                </Space>
                <div class="m-3" />
                <Space>
                    <label for="subnet_id" class="pt-2">Subnet ID:</label>
                    <Input input_size=45 attr:id="subnet_id" value=subnet_id_value />
                </Space>
                <div class="m-3" />
                <Space>
                    <label for="branch_os" class="pt-2">Branch OS:</label>
                    <Input input_size=45 attr:id="branch_os" value=branch_os_value />
                </Space>
                <div class="m-3" />
                <Space>
                    <label for="branch_assets" class="pt-2">Branch Lit-Assets:</label>
                    <Input input_size=45 attr:id="branch_assets" value=branch_assets_value />
                </Space>
                <div class="m-3" />
                <Space>
                    <label for="environment" class="pt-2">Environment (Dev = 0, Staging = 1, Production = 2):</label>
                    <Input value=environment_value />
                </Space>
                <div class="m-3" />
                <Space>
                    <label for="resolver_contract" class="pt-2">Resolver Contract Address</label>
                    <Input input_size=45 value=resolver_contract_value />
                </Space>
                <div class="m-3" />
                <div class="m-3" />
                <Space>
                    <label for="rpc_api_type" class="pt-2">RPC API Type (1 = BlockScout, 2 = OtterScan)</label>
                    <Input value=rpc_api_type_value/>
                </Space>
                <div class="m-3" />
                <Space>
                    <label for="chain_url" class="pt-2">RPC API URL</label>
                    <Input input_size=45 value=chain_url_value/>
                </Space>
                <div class="m-3" />
                <Space>
                    <label for="chain_name" class="pt-2">Explorer URL</label>
                    <Input input_size=45 value=chain_name_value />
                </Space>

            </div>
        </div>
    }
}
