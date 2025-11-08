use crate::common::storage::{read_from_disk, write_to_disk};
use crate::error::Result;
use async_std::path::PathBuf;
use blsful::inner_types::{G1Projective, InnerBls12381G1};
use bulletproofs::BulletproofCurveArithmetic as BCA;
use lit_node_core::CompressedHex;

#[allow(async_fn_in_trait)]
pub trait PointReader: BCA {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point>;
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()>;
    // DATIL_BACKUP: Remove this function once old Datil backup is obsolete.
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point>;
}

impl PointReader for InnerBls12381G1 {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        read_from_disk::<G1Projective>(path, file_name).await
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, point).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        hex::decode(public_key_hex)
            .ok()
            .and_then(|b| b.try_into().ok())
            .and_then(|b| G1Projective::from_uncompressed(&b).into_option())
    }
}

impl PointReader for k256::Secp256k1 {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        let k256_affine_point: k256::AffinePoint = read_from_disk(path, file_name).await?;
        Ok(k256::ProjectivePoint::from(&k256_affine_point))
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, &point.to_affine()).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        k256::AffinePoint::from_uncompressed_hex(public_key_hex).map(k256::ProjectivePoint::from)
    }
}

impl PointReader for p256::NistP256 {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        let p256_affine_point: p256::AffinePoint = read_from_disk(path, file_name).await?;
        Ok(p256::ProjectivePoint::from(&p256_affine_point))
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, &point.to_affine()).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        None
    }
}

impl PointReader for p384::NistP384 {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        let p384_affine_point: p384::AffinePoint = read_from_disk(path, file_name).await?;
        Ok(p384::ProjectivePoint::from(&p384_affine_point))
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, &point.to_affine()).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        None
    }
}

impl PointReader for bulletproofs::Ed25519 {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        read_from_disk::<vsss_rs::curve25519::WrappedEdwards>(path, file_name).await
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, point).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        None
    }
}

impl PointReader for bulletproofs::Ristretto25519 {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        read_from_disk::<vsss_rs::curve25519::WrappedRistretto>(path, file_name).await
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, point).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        None
    }
}

impl PointReader for ed448_goldilocks::Ed448 {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        read_from_disk::<ed448_goldilocks::EdwardsPoint>(path, file_name).await
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, point).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        None
    }
}

impl PointReader for bulletproofs::JubJub {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        read_from_disk::<jubjub::SubgroupPoint>(path, file_name).await
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, point).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        None
    }
}

impl PointReader for bulletproofs::Decaf377 {
    async fn read_point(path: PathBuf, file_name: &str) -> Result<Self::Point> {
        read_from_disk::<decaf377::Element>(path, file_name).await
    }
    async fn write_point(path: PathBuf, file_name: &str, point: &Self::Point) -> Result<()> {
        write_to_disk(path, file_name, point).await
    }
    fn parse_old_backup_public_key(public_key_hex: &str) -> Option<Self::Point> {
        None
    }
}
