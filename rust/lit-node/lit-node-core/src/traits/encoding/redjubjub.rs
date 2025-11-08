use super::{BeBytes, CompressedBytes, LeBytes};
use hd_keys_curves_wasm::jubjub;
use vsss_rs::elliptic_curve::{PrimeField, group::GroupEncoding};

impl CompressedBytes for jubjub::SubgroupPoint {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let mut repr = <jubjub::SubgroupPoint as GroupEncoding>::Repr::default();
        if bytes.len() != repr.len() {
            return None;
        }
        repr.copy_from_slice(bytes);
        Option::from(Self::from_bytes(&repr))
    }
}

impl BeBytes for jubjub::Scalar {
    fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = self.to_bytes();
        bytes.reverse();
        bytes.to_vec()
    }

    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        let mut bytes = bytes.to_vec();
        bytes.reverse();
        let mut repr = <jubjub::Scalar as PrimeField>::Repr::default();
        repr.copy_from_slice(bytes.as_slice());
        Option::from(Self::from_repr(repr))
    }
}

impl LeBytes for jubjub::Scalar {
    fn to_le_bytes(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = <jubjub::Scalar as PrimeField>::Repr::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}

impl CompressedBytes for jubjub::Scalar {
    fn to_compressed(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn from_compressed(bytes: &[u8]) -> Option<Self> {
        let mut repr = <jubjub::Scalar as PrimeField>::Repr::default();
        repr.copy_from_slice(bytes);
        Option::from(Self::from_repr(repr))
    }
}
