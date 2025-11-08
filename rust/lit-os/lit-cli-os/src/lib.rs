extern crate core;

pub mod cmd;
pub mod config;
pub mod error;
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
pub mod guest;
#[cfg(feature = "host-core")]
pub mod host;

pub use cmd::os::{OsArgs, handle_cmd_os};
