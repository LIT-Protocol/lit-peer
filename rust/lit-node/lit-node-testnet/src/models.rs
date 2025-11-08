use ethers::types::U256;

#[derive(Debug, Clone)]
pub struct VotingStatusToKickValidator {
    pub votes: U256,
    pub did_voter_vote_to_kick_validator: bool,
}
