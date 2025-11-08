use crate::{contracts, utils::get_hex_encoded_address};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct ContractAddress {
    name: String,
    address: String,
}

#[component]
pub fn Contracts() -> impl IntoView {
    crate::utils::set_header("Contracts");
    let data = LocalResource::new(|| async move {
        let rows = vec![
            // ContractAddress { name: contracts::CONTRACT_RESOLVER_CONTRACT.to_string(), address: get_address(contracts::CONTRACT_RESOLVER_CONTRACT).await },
            ContractAddress {
                name: contracts::ALLOWLIST_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::ALLOWLIST_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::BACKUP_RECOVERY_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::BACKUP_RECOVERY_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::CONTRACT_RESOLVER_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::CONTRACT_RESOLVER_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::DOMAIN_WALLET_REGISTRY_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::DOMAIN_WALLET_REGISTRY_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::HOST_COMMANDS_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::HOST_COMMANDS_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::KEY_DERIVER_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::KEY_DERIVER_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::LIT_TOKEN_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::LIT_TOKEN_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::MULTI_SENDER_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::MULTI_SENDER_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::PAYMENT_DELEGATION_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::PAYMENT_DELEGATION_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::PKP_HELPER_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::PKP_HELPER_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::PKP_PERMISSIONS_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::PKP_PERMISSIONS_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::PKP_NFT_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::PKP_NFT_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::PKP_NFT_METADATA_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::PKP_NFT_METADATA_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::PUB_KEY_ROUTER_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::PUB_KEY_ROUTER_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::RELEASE_REGISTER_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::RELEASE_REGISTER_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::STAKING_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::STAKING_CONTRACT)
                    .await
                    .unwrap(),
            },
            ContractAddress {
                name: contracts::WLIT_CONTRACT.to_string(),
                address: get_hex_encoded_address(contracts::WLIT_CONTRACT)
                    .await
                    .unwrap(),
            },
        ];
        rows
    });

    view! {
        <Title text="Contracts"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Network Contract Overview</b>
            </div>
            <div class="card-body">

                {move || match data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {
                        <table class="table">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </div>
        </div>

    }
}
