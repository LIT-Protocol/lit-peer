//! LIT Fast ECDSA
//!
//! This algorithm is based on
//! [Fast Threshold ECDSA with Honest Majority](https://eprint.iacr.org/2020/501.pdf)
//! by Ivan Damgard,
//! Thomas Pelle Jakobsen,
//! Jesper Buus Nielsen,
//! Jakob Illeborg Pagter,
//! and Michael Bæksvang Østergaard.
#![deny(unsafe_code)]
#![warn(
    missing_docs, trivial_casts, trivial_numeric_casts, unused_import_braces,
    unused_qualifications, rust_2018_idioms, clippy::unwrap_used, clippy::mod_module_files
)]
mod error;
#[cfg(feature = "presign")]
mod presign;
mod sign;
#[cfg(test)]
mod tests;
mod utils;

pub use error::*;
#[cfg(feature = "presign")]
pub use presign::*;
pub use sign::*;
pub use utils::ParticipantList;
