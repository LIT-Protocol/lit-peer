#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeStakingStatus {
    PreviouslyStaked,
    StakedAndJoined,
    FailedToStake,
    Unstaked,
}
