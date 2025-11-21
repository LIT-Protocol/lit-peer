extern crate core;

pub use crate::contracts::release_register::ReleaseRegister;

pub mod config;
pub mod contracts;
pub mod error;
pub mod resolver;
pub mod util;

use ethers::prelude::*;
use k256::ecdsa::SigningKey;
use std::sync::Arc;

pub type SignerProvider = SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>;
