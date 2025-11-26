use crate::utils::{get_address, get_lit_config};
use ethers::types::H160;
use leptos::prelude::*;

use crate::pages::validators::Validator;

#[component]
pub fn ValidatorDetails(validator: Validator) -> impl IntoView {
    view! {
        <div class="row">
            <table class="table">
                <tbody>
                    <tr>
                        <td>Host Name</td>
                        <td>{validator.host_name.clone()}</td>
                        <td></td>
                        <td>Node Status</td>
                        <td>{validator.status.clone()} </td>
                    </tr>
                    <tr>
                        <td>Guest IP</td>
                        <td>{validator.socket_address.clone()}</td>
                        <td></td>
                        <td>Node Identity Key</td>
                        <td>{validator.node_identity_key.clone()}</td>
                    </tr>
                    <tr>
                        <td>Wallet Address</td>
                        <td><a href={format!("https://yellowstone-explorer.litprotocol.com/address/{}", validator.wallet_address.clone())} target="_blank">{validator.wallet_address.clone()}</a></td>
                        <td></td>
                        <td>Staker Address</td>
                        <td><a href={format!("https://yellowstone-explorer.litprotocol.com/address/{}", validator.staker_address.clone())} target="_blank">{validator.staker_address.clone()}</a></td>
                    </tr>
                    <tr>
                        <td>Commit Hash</td>
                        <td><a href={format!("https://github.com/lit-protocol/lit-assets/commit/{}", validator.commit_hash.clone())} target="_blank">{validator.commit_hash.clone()}</a></td>
                        <td></td>
                        <td>Version</td>
                        <td>{validator.ver.clone()}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
