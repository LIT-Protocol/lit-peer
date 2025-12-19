use crate::pages::history::simple_hex;
use leptos::prelude::*;

use crate::pages::validators::Validator;

#[component]
pub fn ValidatorDetails(validator: Validator) -> impl IntoView {

    // lit-chain-explorer.litprotocol.com
    // yellowstone-explorer.litprotocol.com
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
                        <td>Operator Address</td>
                        <td><a href={format!("https://lit-chain-explorer.litprotocol.com/address/{}", validator.operator_address.clone())} target="_blank">{simple_hex(validator.operator_address.clone())}</a></td>
                    </tr>
                    <tr>
                        <td>Wallet Address</td>
                        <td><a href={format!("https://lit-chain-explorer.litprotocol.com/address/{}", validator.wallet_address.clone())} target="_blank">{simple_hex(validator.wallet_address.clone())}</a></td>
                        <td></td>
                        <td>Staker Address</td>
                        <td><a href={format!("https://lit-chain-explorer.litprotocol.com/address/{}", validator.staker_address.clone())} target="_blank">{simple_hex(validator.staker_address.clone())}</a></td>
                    </tr>
                    <tr>
                        <td>Node Identity Key</td>
                        <td colspan="3">{validator.node_identity_key.clone()}</td>
                    </tr>
                    <tr>
                        <td>Commit Hash</td>
                        <td><a href={format!("https://github.com/lit-protocol/lit-peer/commit/{}", validator.commit_hash.clone())} target="_blank">{validator.commit_hash.clone()}</a></td>
                        <td></td>
                        <td>Version</td>
                        <td>{validator.ver.clone()}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
