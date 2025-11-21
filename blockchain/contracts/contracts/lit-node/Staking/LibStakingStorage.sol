//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSetViewFriendly } from "@lit-protocol/openzeppelin-contracts/utils/structs/EnumerableSetViewFriendly.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { console } from "hardhat/console.sol";
interface IPubkeyRouter {
    struct RootKey {
        bytes pubkey;
        uint256 keyType; // 1 = BLS, 2 = ECDSA.  Not doing this in an enum so we can add more key-types in the future without redeploying.
    }

    struct Signature {
        bytes32 r;
        bytes32 s;
        uint8 v;
    }
}

library LibStakingStorage {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSetViewFriendly for EnumerableSetViewFriendly.AddressSet;

    bytes32 constant GLOBAL_STAKING_POSITION =
        keccak256("global.staking.storage");

    enum States {
        Active,
        NextValidatorSetLocked,
        ReadyForNextEpoch,
        Unlocked,
        Paused,
        Restore
    }

    /**
     * @dev Implementation in LibStakingStorage, where Validator struct is defined and handled.
     */
    struct StakeRecord {
        uint256 id;
        uint256 amount;
        uint256 unfreezeStart;
        /// @notice the time lock for the stake, measured in seconds with precision to the nearest day
        uint256 timeLock;
        uint256 lastUpdateTimestamp;
        /// @notice The last reward epoch that the staker has claimed rewards for
        uint256 lastRewardEpochClaimed;
        /// @dev This is fine to always be set to PRECISION. The validator share price only ever decreases
        /// when a validator gets slashed, however, when a validator gets slashed, they are perma-banned
        /// from joining any realm / validator set, and pretty much the only way for that validator to
        /// re-join any realm is via admin intervention by reducing their demerit counters. When this
        /// validator proceeds to re-join a realm, their slate is "wiped clean" and they should have a
        /// validator share price reset to PRECISION initially. Hence, there is no need to set this
        /// initialSharePrice to anything else other than PRECISION always.
        uint256 initialSharePrice;
        bool loaded;
        bool frozen;
        /// @notice This is used to track the address of the validator that the stake record has been
        /// attributed to at the time of creation via Validator.delegatedStakeAmount and Validator.delegatedStakeWeight
        /// fields.
        address attributionAddress;
    }

    struct StakerVault {
        uint256 lastStakeRecordId;
        StakeRecord[30] stakes;
    }

    struct Validator {
        uint32 ip;
        uint128 ipv6;
        uint32 port;
        address nodeAddress;
        uint256 reward;
        uint256 senderPubKey;
        uint256 receiverPubKey;
        uint256 lastActiveEpoch;
        uint256 commissionRate;
        /// @notice the last reward epoch that the validator has earned rewards for
        /// participating in the validator set of a particular realm. If the validator
        /// has never been in an active set, then this will be 0.
        /// @dev This should always at most be 1 less than the current realm's epoch number.
        uint256 lastRewardEpoch;
        /// @notice the last realmId that the validator was in
        uint256 lastRealmId;
        /// @notice the sum of all stake amounts from delegated stakers to this validator
        uint256 delegatedStakeAmount;
        /// @notice the sum of all stake weights from delegated stakers to this validator
        /// This does not get updated as the delegated stakes are in the unfreezing process.
        uint256 delegatedStakeWeight;
        /// @notice The last reward epoch that the staker has claimed fixed cost rewards for.
        /// @dev This only works because reward epoch numbers are monotonically increasing.
        uint256 lastRewardEpochClaimedFixedCostRewards;
        /// @notice The last reward epoch that the staker has claimed commission for.
        /// @dev This only works because reward epoch numbers are monotonically increasing.
        uint256 lastRewardEpochClaimedCommission;
        /// @notice The address of the node operator's wallet. This is only used for
        /// registering the node attested wallet currently.
        address operatorAddress;
        /// @notice The number of unique delegating stakers that have staked to this validator.
        uint256 uniqueDelegatingStakerCount;
        /// @notice Whether the validator is allowed to register an attested wallet. When true (disabled),
        /// the validator cannot register an attested wallet.
        bool registerAttestedWalletDisabled;
    }

    struct StakingAggregateDetails {
        Epoch epoch;
        uint256 currentValidatorCountForConsensus;
        Validator[] activeUnkickedValidators;
    }

    struct KeyedStakingAggregateDetails {
        address stakingContractAddress;
        StakingAggregateDetails details;
    }

    struct AddressMapping {
        address nodeAddress;
        address stakerAddress;
    }

    struct PubKeyMapping {
        address nodeAddress;
        UncompressedK256Key pubKey;
    }

    struct VoteToKickValidatorInNextEpoch {
        uint256 votes;
        mapping(address => bool) voted;
    }

    struct PendingRejoin {
        address addr;
        uint256 timestamp;
    }

    struct Epoch {
        uint256 epochLength; // in seconds
        uint256 number; // the current epoch number
        uint256 rewardEpochNumber; // the current reward epoch number
        /// @notice While there is a global reward epoch number that is incremented, that counter is shared across realms,
        /// so we need this parameter to know for sure which number will be the next reward epoch number for a realm.
        uint256 nextRewardEpochNumber;
        uint256 endTime; // the end timestamp where the next epoch can be kicked off
        uint256 retries; // incremented upon failure to advance and subsequent unlock
        uint256 timeout; // timeout in seconds, where the nodes can be unlocked.
        uint256 startTime; // the start timestamp of the epoch
        uint256 lastAdvanceVoteTime; // the last time a node voted to advance the epoch .... defaults to 0.
    }

    struct RewardEpoch {
        uint256 epochEnd;
        uint256 totalStakeWeight;
        /// @notice The total rewards for the validator and their delegated stakers for staking.
        uint256 totalStakeRewards;
        /// @notice The fixed cost rewards for the validator.
        uint256 validatorFixedCostRewards;
        /// @notice The commission for the validator.
        uint256 validatorCommission;
        /// @notice The slope is used to calculate the stake weight of the validator accounting for unfrozen stakes. If all stakes are frozen,
        /// or fully unfrozen, the slope should be zero.
        /// This is the amount of stake weight that will be deducted per second.
        uint256 slope;
        /// @notice The share price of the validator tracks the number of units of `stakeAmount`. For example,
        /// if the validator gets slashed, then the validatorSharePrice will decrease and the amount available
        /// for withdrawal will decrease. The share price is initialized to PRECISION and only ever decreases, if ever.
        uint256 validatorSharePrice;
        uint256 stakeAmount;
        uint256 validatorSharePriceAtLastUpdate;
        bool initial;
    }

    // this is the global stats for the reward epoch - which is actually an aggregate of epochs in various states in different realms
    struct RewardEpochGlobalStats {
        uint256 stakeAmount;
        uint256 stakeWeight;
        /// @notice This is the set of validators that are in the current validator set
        address[] validatorsInCurrentEpoch;
        /// @notice The actual length that the epoch has elapsed for in seconds.
        uint256 actualEpochLength;
    }

    struct ComplaintConfig {
        uint256 tolerance;
        uint256 intervalSecs;
        uint256 kickPenaltyPercent;
        uint256 kickPenaltyDemerits;
    }

    struct Version {
        uint256 major;
        uint256 minor;
        uint256 patch;
    }

    struct KeySetConfig {
        uint32 minimumThreshold;
        uint32 monetaryValue;
        bool completeIsolation;
        string identifier;
        string description;
        uint256[] realms;
        uint256[] curves;
        uint256[] counts;
        /// Set when the recovery DKG completes for the key set
        bytes recoverySessionId;
    }

    struct RealmConfig {
        uint256 maxConcurrentRequests;
        uint256 maxPresignCount;
        uint256 minPresignCount;
        uint256 peerCheckingIntervalSecs;
        uint256 maxPresignConcurrency;
        bool rpcHealthcheckEnabled;
        /// @notice The minimum epoch number for which rewards are distributed.
        /// @dev This should be set to 3, since the first epoch does not usually have any validators,
        /// which in turn causes epoch 2 to have no rewards.
        uint256 minEpochForRewards;
        /// @notice Whether the validator set allows for an allowlist of operators to join the validator set.
        bool permittedValidatorsOn;
        /// The default key set identifier to use if the realm has more than one
        /// This allows the realm to operate without asking this value from clients
        /// for some operations like session keys and sign as action
        string defaultKeySet;
    }

    struct GlobalConfig {
        uint256 tokenRewardPerTokenPerEpoch;
        uint256 minimumValidatorCount;
        /// @notice Keep this the same as the epoch length for now.
        uint256 rewardEpochDuration;
        uint256 maxTimeLock;
        uint256 minTimeLock;
        uint256 bmin; // Minimum reward budget (in basis points, i.e., 0.1%)
        uint256 bmax; // Maximum reward budget (in basis points, i.e., 0.5%)
        uint256 k; // Kink parameter for rewards (e.g., 0.5)
        uint256 p; // Power parameter (e.g., 0.5)
        bool enableStakeAutolock; // if true, stake will be autolocked
        /// @notice The number of LIT tokens per USD.
        uint256 tokenPrice;
        /// @notice The profit multiplier. Recommended = 6 (500% profit margin), Max = 6, Min = 1.1 (10% profit margin).
        uint256 profitMultiplier;
        /// @notice The cost of running a single validator node per month in USD.
        uint256 usdCostPerMonth;
        uint256 maxEmissionRate;
        uint256 minStakeAmount;
        uint256 maxStakeAmount;
        uint256 minSelfStake;
        uint256 minSelfStakeTimelock;
        /// @notice The minimum validator count by which the minimum threshold will start clamping and taking effect.
        /// For example, if minValidatorCountToClampMinimumThreshold is 4 and minThresholdToClampAt is 3, then the
        /// threshold is 3 for validator counts up to 4.
        uint256 minValidatorCountToClampMinimumThreshold;
        /// @notice The threshold at which to clamp at if the conditions are met per minValidatorCountToClampMinimumThreshold.
        /// For example, if minValidatorCountToClampMinimumThreshold is 4 and minThresholdToClampAt is 3, then the
        /// threshold is 3 for validator counts up to 4.
        uint256 minThresholdToClampAt;
        /// @notice The time out for voting to advance the epoch.  If all nodes haven't voted within this time ( in seconds )
        /// the next time an "advanceEpoch" function is called the contract will unlock the current epoch, and reset the state.
        /// Once the nodes identify the change, they will attempt to perform DKG again.   This change catches nodes that go offline
        /// unintentionally, or miss emitted values from the chain, leaving the network in a state where it can perform operations,
        /// but ceases to do further proactive secret sharing without manual intervention.
        uint256 voteToAdvanceTimeOut;
    }

    struct GlobalStakingStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        address devopsAdmin;
        /// @notice This is a temporary hardcoded value for the total supply of the token.
        /// @notice Since the token supply will be distributed across various L2s and L3s, and that
        /// simply using totalSupply() will NOT give us the total supply of the token, we will hardcode
        /// this for now. Doing so is not known to cause any short term issues, though it will
        /// need to be adjusted over time as more tokens get minted.
        uint256 tokenTotalSupplyStandIn;
        uint256 totalStaked;
        /// @notice This is a monotonically increasing counter that is used to derive realm IDs.
        uint256 nextRealmId;
        /// @notice This is the next available reward epoch number that is used as an identifier for reward epochs
        /// across various realms.
        uint256 nextAvailableRewardEpochNumber;
        EnumerableSet.UintSet realmIds;
        /// @notice This is the set of all validators in all realms. A validator is a node-op staker address.
        EnumerableSet.AddressSet allValidators;
        /// @notice This is the set of all active validators in all realms. An active validator is a validator who is assigned to either the
        /// a current or upcoming epoch.
        EnumerableSet.AddressSet allActiveValidators;
        /// @notice This mapping tracks the realmId of any currently active validator sets that the validator is a part of.
        mapping(address => uint256) validatorToCurrentRealmId;
        /// @notice This mapping tracks the upcoming realmId of any validator sets that the validator will actively become a part of.
        mapping(address => uint256) validatorToNextRealmId;
        // this mapping lets you go from the nodeAddressto the stakingAddress.
        mapping(address => address) nodeAddressToStakerAddress;
        mapping(address => address) stakerAddressToNodeAddress;
        mapping(address => address) operatorAddressToStakerAddress;
        // this mapping lets you go from the userStakerAddress to the stakerAddress.
        mapping(address => address) userStakerAddressToStakerAddress;
        // Mapping of the complaint reason code to the config for that reason
        mapping(uint256 => ComplaintConfig) complaintReasonToConfig;
        // Thunderhead - Staking Vaults & rewards
        mapping(address => mapping(address => StakerVault)) vaults; // stakerAddress => Vaults => StakerVault.  The second address is the userStakerAddress
        // there is an issue here, because the rewardEpochNumber is not unique to the stakerAddress.  It is unique to the realmId.
        // so we need to add the realmId to the mapping, for find something else that works.
        mapping(address => mapping(uint256 => RewardEpoch)) rewardEpochs; // stakerAddress => rewardEpochNumber => RewardEpoch
        mapping(uint256 => RewardEpochGlobalStats) rewardEpochsGlobalStats; // rewardEpochNumber => RewardEpochGlobalStats
        // maps attested address to pub key
        mapping(address => UncompressedK256Key) attestedAddressToPubKey;
        // Mapping of nodes and how many demerits they have
        mapping(address => uint256) demeritsCounter;
        uint256 demeritRejoinThreshold;
        /// @notice This mapping tracks how long a node has left to rejoin before it's slashed
        PendingRejoin[] pendingRejoins;
        /// @notice This mapping tracks whether a validator is in the pending rejoin set
        mapping(address => bool) isValidatorInPendingRejoin;
        // The amount of time a node has to rejoin before it's slashed
        uint256 pendingRejoinTimeout;
        // must be a mapping so that we can add fields to GlobalConfig without breaking diamond storage.  always use globalConfig[0].
        mapping(uint256 => GlobalConfig) globalConfig;
        // list of all validators, even ones that are not in the current or next epoch of any particular realm
        // maps STAKER address to Validator struct
        mapping(address => Validator) validators;
        // mapping of src realm -> target realm for Shadow Splicing of keys
        mapping(uint256 => uint256) realmToShadowRealm;
        /// @notice This mapping tracks the validators that a staker has staked to.
        mapping(address => EnumerableSet.AddressSet) stakerToValidatorsTheyStakedTo;
        /// @notice This mapping tracks the stakers with currently unfreezing stakes that have been delegated to a validator.
        mapping(address => EnumerableSetViewFriendly.AddressSet) validatorToDelegatedStakersWithUnfreezingStakes;
        /// @notice This mapping tracks the upcoming unfreeze schedules for all stake records staked to a validator.
        /// The first key is the validator address, the second key is the unfreeze end, and the value is the slope increase.
        /// The slope increase is the amount of slope that will be added to the validator's slope when the stake record fully unfreezes.
        mapping(address => mapping(uint256 => uint256)) validatorToUnfreezeEndToSlopeIncrease;
        /// @notice This mapping tracks the unfreeze ends for all stake records staked to a validator. These are keys
        /// that are used to track the slope increase for the validator's slope in the `validatorToUnfreezeEndToSlopeIncrease` mapping.
        mapping(address => EnumerableSet.UintSet) validatorToUnfreezeEnds;
        /// @notice This mapping tracks the upcoming unfreeze schedule starts for all stake records staked to a validator.
        /// The first key is the validator address, the second key is the unfreeze start, and the value is the slope.
        /// The slope is used for calculating the stake weight decremented from the validator's total stake weight over each epoch
        /// advancement.
        mapping(address => mapping(uint256 => uint256)) validatorToUnfreezeStartToSlope;
        /// @notice This mapping tracks the unfreeze starts for all stake records staked to a validator. These are keys
        /// that are used to track the slope in the `validatorToUnfreezeStartToSlope` mapping.
        mapping(address => EnumerableSet.UintSet) validatorToUnfreezeStarts;
        // keccak256(keySetConfig.identifier) mapping so it can be changed later if needed
        bytes32[] keySetIds;
        mapping(bytes32 => KeySetConfig) keySetsConfigs;
        mapping(bytes32 => mapping(uint256 => uint256)) keySetKeyCounts;
    }

    struct RealmStorage {
        uint256 realmId;
        States state;
        EnumerableSet.AddressSet validatorsInCurrentEpoch;
        EnumerableSet.AddressSet validatorsInNextEpoch;
        EnumerableSet.AddressSet validatorsKickedFromNextEpoch;
        EnumerableSet.AddressSet validatorsWaitingToReplaceShadowNodes; // these are nodes that are free to join into the next epoch
        EnumerableSet.AddressSet shadowValidators; // these are nodes that are part of an active epoch rewarded in a different realm
        EnumerableSet.AddressSet shadowValidatorsInNextEpoch; // shadowValidators for the next epoch
        uint256 targetValidatorCount;
        // This is the set of validators that are in the current validator set that are also kicked
        // from the next validator set.
        EnumerableSet.AddressSet currentValidatorsKickedFromNextEpoch;
        // versionRequirements[0] is the min version
        // versionRequirements[1] is the max version
        mapping(uint256 => Version) versionRequirements;
        // storing this in a mapping so that it can be changed in the future
        // always use epochs[0]
        mapping(uint256 => Epoch) epochs;
        // storing this in a mapping so that it can be changed in the future.
        // always use configs[0]
        mapping(uint256 => RealmConfig) realm_configs;
        mapping(uint256 => LitActionConfig) litActionConfigs;
        // stakers join by staking, but nodes need to be able to vote to kick.
        // to avoid node operators having to run a hotwallet with their staking private key,
        // the node gets it's own private key that it can use to vote to kick,
        // or signal that the next epoch is ready.
        // after the validator set is locked, nodes vote that they have successfully completed the PSS
        // operation.  Once a threshold of nodes have voted that they are ready, then the epoch can advance
        mapping(address => bool) readyForNextEpoch;
        // nodes can vote to kick another node.  If a threshold of nodes vote to kick someone, they
        // are removed from the next validator set
        mapping(uint256 => mapping(address => VoteToKickValidatorInNextEpoch)) votesToKickValidatorsInNextEpoch;
        /// Maps kick reason to amount to slash
        // maps hash(comms_sender_pubkey,comms_receiver_pubkey) to a boolean to show if
        // the set of comms keys has been used or not
        mapping(bytes32 => bool) usedCommsKeys;
        address devopsAdmin;
        // the reward pool is the total amount of rewards that have been distributed to validators in this realm
        uint256 rewardPool;
        /// @notice The set of validators that are permitted to join the validator set.
        EnumerableSet.AddressSet permittedValidators;
    }

    struct UncompressedK256Key {
        uint256 x;
        uint256 y;
    }

    struct LitActionConfig {
        uint256 timeoutMs;
        uint256 memoryLimitMb;
        uint256 maxCodeLength;
        uint256 maxResponseLength;
        uint256 maxConsoleLogLength;
        uint256 maxFetchCount;
        uint256 maxSignCount;
        uint256 maxContractCallCount;
        uint256 maxBroadcastAndCollectCount;
        uint256 maxCallDepth;
        uint256 maxRetries;
        bool asyncActionsEnabled;
    }

    function getStakingStorage()
        internal
        pure
        returns (GlobalStakingStorage storage storageStruct)
    {
        bytes32 position = GLOBAL_STAKING_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }

    function getRealmStorage(
        uint256 realmId
    ) internal view returns (RealmStorage storage storageStruct) {
        if (realmId == 0) {
            revert("Realm Id can not be 0.");
        }

        if (!EnumerableSet.contains(getStakingStorage().realmIds, realmId)) {
            revert("RealmId not found in storage.");
        }

        bytes32 position = keccak256(abi.encode(realmId));
        assembly {
            storageStruct.slot := position
        }
    }
}
