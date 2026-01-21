#![allow(unused_variables)]
#[macro_use]
pub extern crate rocket;

pub mod common;
pub mod config;
pub mod endpoints;
pub mod models;
pub mod peers;
pub mod siwe_db;
pub mod version;

pub mod access_control;
#[allow(dead_code)]
pub mod auth;
#[cfg(feature = "lit-actions")]
pub mod functions;
pub mod jwt;
pub mod metrics;
pub mod networking;
pub mod node_state;
pub mod p2p_comms;
pub mod payment;
pub mod pkp;
pub mod tss;
pub mod utils {
    pub mod attestation;
    pub mod consensus;
    pub mod contract;
    pub mod cose_keys;
    pub mod encoding;
    pub mod eth;
    pub mod future;
    pub mod networking;
    pub mod rocket;
    pub mod serde_encrypt;
    pub mod siwe;
    pub mod traits;

    #[allow(dead_code)]
    pub mod web;

    pub mod tracing;

    pub mod key_share_proof;
}
pub mod error;
pub mod services;
pub mod tasks;

pub mod client_session;
pub mod git_info;
#[cfg(test)]
mod tests;
