use crate::ParticipantList;
use ecdsa::elliptic_curve::CurveArithmetic;

/// The parameters for a pre-signature
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PreSignatureParams<C: CurveArithmetic> {
    /// The threshold used to sign
    pub threshold: usize,
    /// The list of participants
    pub participant_list: ParticipantList<C>,
}
