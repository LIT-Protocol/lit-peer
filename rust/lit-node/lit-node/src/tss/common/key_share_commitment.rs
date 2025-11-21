use lit_rust_crypto::elliptic_curve::{Field, Group, group::GroupEncoding};
use serde::{Deserialize, Serialize};

/// KeyShareCommitment is a struct that holds the commitment of a key share.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyShareCommitments<G: Group + GroupEncoding + Default> {
    /// The DKG ID that the commitment was generated
    pub dkg_id: String,
    /// The commitment to the polynomial for all key shares
    #[serde(with = "group")]
    pub commitments: Vec<G>,
}

unsafe impl<G: Group + GroupEncoding + Default> Send for KeyShareCommitments<G> {}
unsafe impl<G: Group + GroupEncoding + Default> Sync for KeyShareCommitments<G> {}

impl<G: Group + GroupEncoding + Default> KeyShareCommitments<G> {
    pub fn compute_key_share_commitment(&self, x: &G::Scalar) -> G {
        // Horner's method
        let mut i = <G::Scalar as Field>::ONE;

        // Horner's method
        // c_0 * c_1^i * c_2^{i^2} ... c_t^{i^t}
        let mut res = self.commitments[0];
        for &v in &self.commitments[1..] {
            i *= x;

            // c_0 * c_1^i * c_2^{i^2} ... c_t^{i^t}
            res += v * i;
        }
        res
    }
}

mod group {
    use super::*;
    use serde::{Deserializer, Serializer, ser::SerializeSeq};

    pub fn serialize<G, S>(g: &[G], s: S) -> Result<S::Ok, S::Error>
    where
        G: Group + GroupEncoding + Default,
        S: Serializer,
    {
        if s.is_human_readable() {
            let mut arr = s.serialize_seq(Some(g.len()))?;
            for elem in g {
                arr.serialize_element(&hex::encode(elem.to_bytes().as_ref()))?;
            }
            arr.end()
        } else {
            s.serialize_bytes(
                &g.iter()
                    .flat_map(|elem| elem.to_bytes().as_ref().to_vec())
                    .collect::<Vec<u8>>(),
            )
        }
    }

    pub fn deserialize<'de, G, D>(d: D) -> Result<Vec<G>, D::Error>
    where
        G: Group + GroupEncoding + Default,
        D: Deserializer<'de>,
    {
        fn bytes_to_group<'de, G: Group + GroupEncoding + Default, D: Deserializer<'de>>(
            bytes: &[u8],
        ) -> Result<G, D::Error> {
            let mut repr = G::Repr::default();
            let expected_len = repr.as_ref().len();
            if bytes.len() != expected_len {
                return Err(serde::de::Error::custom(format!(
                    "Invalid group element length: expected {}, found {}",
                    expected_len,
                    bytes.len()
                )));
            }
            repr.as_mut().copy_from_slice(bytes);
            Option::from(G::from_bytes(&repr))
                .ok_or(serde::de::Error::custom("Invalid group element"))
        }
        if d.is_human_readable() {
            let points: Vec<String> = Vec::deserialize(d)?;
            let mut elems = vec![G::default(); points.len()];
            for (i, point) in points.iter().enumerate() {
                elems[i] = bytes_to_group::<G, D>(&hex::decode(point).map_err(|e| {
                    serde::de::Error::custom(format!("Unable to decode hex: {:?}", e))
                })?)?;
            }
            Ok(elems)
        } else {
            let bytes: Vec<u8> = Vec::deserialize(d)?;
            let repr = G::Repr::default();
            let len = repr.as_ref().len();
            if !bytes.len().is_multiple_of(len) {
                return Err(serde::de::Error::custom(format!(
                    "Invalid group element length: expected multiple of {}, found {}",
                    len,
                    bytes.len()
                )));
            }
            let mut elems = vec![G::default(); bytes.len() / len];
            for (i, chunk) in bytes.chunks(len).enumerate() {
                elems[i] = bytes_to_group::<G, D>(chunk)?;
            }
            Ok(elems)
        }
    }
}
