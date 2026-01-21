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
pub mod error;
#[cfg(feature = "lit-actions")]
pub mod functions;
pub mod jwt;
pub mod metrics;
pub mod networking;
pub mod node_state;
pub mod p2p_comms;
pub mod payment;
pub mod pkp;
pub mod services;
pub mod tasks;
pub mod tss;
pub mod utils;

pub mod client_session;
pub mod git_info;
#[cfg(test)]
mod tests;
