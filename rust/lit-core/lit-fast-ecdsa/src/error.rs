use thiserror::Error;

/// Error type for this crate
#[derive(Debug, Error)]
pub enum EcdsaError {
    /// VSSS error
    #[error("VSSS error")]
    Vsss(vsss_rs::Error),
    /// Invalid ID error
    #[error("Invalid ID")]
    InvalidId,
    /// Invalid round error
    #[error("Invalid round {0}")]
    InvalidRound(u8),
    /// Invalid round parse error
    #[error("Invalid round parse: {0}")]
    InvalidRoundParse(String),
    /// Missing participant error
    #[error("Missing participant id in participant list")]
    MissingParticipant,
    /// Duplicate commitment error
    #[error("Duplicate commitment")]
    DuplicateCommitment,
    /// Duplicate share payload error
    #[error("Duplicate share payload")]
    DuplicateSecretPayload,
    /// Threshold minimum error
    #[error("Invalid threshold: {0}")]
    MinThreshold(String),
    /// Minimum number of participants error
    #[error("Invalid number of participants")]
    MinParticipants,
    /// Maximum number of participants error
    #[error("Invalid maximum participants")]
    MaxParticipants,
    /// Threshold greater than participants error
    #[error("Threshold greater than participants")]
    ThresholdGreaterThanParticipants,
    /// Threshold to number of participants ratio error
    #[error("Threshold to number of participants ratio must be n >= 2t + 1")]
    ThresholdToNumberParticipantsRatio,
    /// Insufficient commitments error
    #[error("Insufficient commitments received from participants")]
    InsufficientCommitments,
    /// Invalid round 1 payload error
    #[error("Invalid round 1 payload")]
    InvalidRound1Payload,
    /// Invalid round result error
    #[error("Invalid round result: {0}")]
    InvalidRoundResult(&'static str),
    /// Invalid round 2 payload error
    #[error("Invalid round 2 payload")]
    InvalidRound2Payload,
    /// Invalid round 3 payload error
    #[error("Invalid round 3 payload")]
    InvalidRound3Payload,
    /// Invalid round 4 payload error
    /// Insufficient shares error
    #[error("Insufficient round 1 payloads received")]
    InsufficientRound1Payloads,
    /// Invalid ID in commitment or share error
    #[error("Received commitment or share from an invalid participant id")]
    InvalidIdInCommitmentOrShare,
    /// Insufficient round 2 payloads error
    #[error("Insufficient round 2 payloads received")]
    InsufficientRound2Payloads,
    /// Insufficient round 3 payloads error
    #[error("Insufficient round 3 payloads received")]
    InsufficientRound3Payloads,
    /// Invalid round 4 payload error
    /// Invalid computed pre-signature big R error
    #[error("Invalid computed pre-signature big R")]
    InvalidBigR,
    /// Invalid computed pre-signature k error
    #[error("Invalid computed k, cannot invert")]
    InvalidScalarK,
    /// Invalid computed signature z error
    #[error("Invalid computed z for signature message")]
    InvalidScalarZ,
    /// Invalid computed pre-signature big W error
    #[error("Invalid computed pre-signature big W")]
    InvalidBigW,
    /// Invalid computed pre-signature w error
    #[error("Invalid computed pre-signature w")]
    InvalidW,
    /// Invalid pre-signature
    #[error("Invalid pre-signature")]
    InvalidPreSignature,
    /// Insufficient shares error
    #[error("Insufficient shares during combining")]
    InsufficientShares,
    /// Invalid signature share error
    #[error("Invalid signature share")]
    InvalidSignatureShare,
    /// Invalid signature error
    #[error("Invalid signature after combining shares")]
    InvalidSignatureResult,
    /// Invalid round error
    #[error("Incorrect round function called: {0}")]
    IncorrectRound(&'static str),
    /// Invalid secret key error
    #[error("Invalid secret key")]
    InvalidSecretKey,
    /// Missing OT sender round 1 output error
    #[error("Missing OT sender round 1 output")]
    MissingOtSenderRound1Output,
    /// Received OT sender round 1 output error
    #[error("Received OT sender round 1 output when the participant is the sender")]
    UnexpectedOtSenderRound1Output,
    /// Missing OT receiver round 2 output error
    #[error("Missing OT receiver round 2 output")]
    MissingOtReceiverRound2Output,
    /// Received OT sender round 2 output error
    #[error("Received OT receiver round 2 output when the participant is the receiver")]
    UnexpectedOtReceiverRound2Output,
    /// Received OT sender round 3 output error
    #[error("Received Random OT sender round 3 output when the participant is the receiver")]
    UnexpectedRotSenderOutput,
    /// Missing OT receiver round 3 output error
    #[error("Missing Random OT sender round 3 output")]
    MissingRotSenderOutput,
    /// Received OT sender round 3 output error
    #[error("Received Multiply OT sender round 1 output when the participant is the sender")]
    UnexpectedMulOtSenderOutput,
    /// Missing OT receiver round 3 output error
    #[error("Missing Multiply OT sender round 1 output")]
    MissingMulOtSenderOutput,
}

impl From<vsss_rs::Error> for EcdsaError {
    fn from(value: vsss_rs::Error) -> Self {
        EcdsaError::Vsss(value)
    }
}

/// Result type for this crate
pub type EcdsaResult<T> = Result<T, EcdsaError>;
