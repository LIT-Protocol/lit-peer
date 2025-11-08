pub const PKG_NAME: &str = "lit_os_metrics";

use derive_more::Display;

pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};

#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code, clippy::enum_variant_names)]
pub(crate) enum EC {
    /// An unexpected fault in the lit-os-metrics-internal has occured.
    #[code(kind = Unexpected)]
    UnexpectedFault,
    /// An invalid input was provided.
    #[code(kind = Validation)]
    InvalidInput,
}

generate_pkg_constructors!(PKG_NAME, pub(crate), EC);
