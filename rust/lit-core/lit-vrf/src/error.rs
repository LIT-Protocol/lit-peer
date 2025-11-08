use thiserror::Error;

/// Error type for VRF operations
#[derive(Debug, Error)]
pub enum VrfError {
    /// Error when the VRF input secret key is not valid
    #[error("Invalid secret key")]
    SecretKeyError,
    /// Error when the VRF input public key is not valid
    #[error("Invalid public key")]
    PublicKeyError,
    /// Error when the VRF input message hashed is not valid
    #[error("Hash to curve returned a zero result")]
    HashToCurveError,
    /// Error when the VRF nonce is not valid
    #[error("Nonce generation returned a zero result")]
    NonceGenerationError,
    /// Error when the VRF challenge is not valid
    #[error("Challenge generation returned a zero result")]
    ChallengeGenerationError,
    /// Error when the VRF proof input is not valid
    #[error("Invalid input proof")]
    InvalidProofInput,
    /// Error when the VRF proof is not valid
    #[error("Invalid proof")]
    InvalidProof,
    /// Error when combining VRF proofs with a duplicate id
    #[error("Duplicate id when combining proofs")]
    DuplicateId,
    /// Error when the VRF proof id is not valid
    #[error("Invalid proof id - cannot be zero")]
    InvalidProofId,
    /// Error when the VRF proof round is not valid
    #[error("Invalid proof round")]
    InvalidProofRound,
    /// Error when the VRF proof params are not valid
    #[error("Invalid proof params")]
    InvalidProofParams,
}

/// Result type for VRF operations
pub type VrfResult<T> = Result<T, VrfError>;
