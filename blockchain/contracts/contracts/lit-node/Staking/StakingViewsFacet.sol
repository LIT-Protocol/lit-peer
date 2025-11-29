//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { EnumerableSetViewFriendly } from "@lit-protocol/openzeppelin-contracts/utils/structs/EnumerableSetViewFriendly.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingAcrossRealmsFacet } from "./StakingAcrossRealmsFacet.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { SafeCast } from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import { console } from "hardhat/console.sol";
import { ABDKMathQuad } from "abdk-libraries-solidity/ABDKMathQuad.sol";
import { FixedPointMathLib } from "solady/src/utils/FixedPointMathLib.sol";

contract StakingViewsFacet {
    using EnumerableSet for EnumerableSet.UintSet;
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSetViewFriendly for EnumerableSetViewFriendly.AddressSet;

    error NodeAddressNotFoundForStaker();

    /* ========== VIEWS ========== */

    function s()
        internal
        pure
        returns (LibStakingStorage.GlobalStakingStorage storage)
    {
        return LibStakingStorage.getStakingStorage();
    }

    function realm(
        uint256 realmId
    ) internal view returns (LibStakingStorage.RealmStorage storage) {
        return LibStakingStorage.getRealmStorage(realmId);
    }

    function realms() internal view returns (StakingAcrossRealmsFacet) {
        return StakingAcrossRealmsFacet(getStakingContractAddress());
    }

    function staking() internal view returns (StakingFacet) {
        return StakingFacet(getStakingContractAddress());
    }

    function epoch(
        uint256 realmId
    ) public view returns (LibStakingStorage.Epoch memory) {
        return realm(realmId).epochs[0];
    }

    function realmConfig(
        uint256 realmId
    ) public view returns (LibStakingStorage.RealmConfig memory) {
        return realm(realmId).realm_configs[0];
    }

    function globalConfig()
        public
        view
        returns (LibStakingStorage.GlobalConfig memory)
    {
        return LibStakingStorage.getStakingStorage().globalConfig[0];
    }

    function litActionsConfig(
        uint256 realmId
    ) public view returns (LibStakingStorage.LitActionConfig memory) {
        return realm(realmId).litActionConfigs[0];
    }

    function complaintConfig(
        uint256 reason
    ) public view returns (LibStakingStorage.ComplaintConfig memory) {
        return s().complaintReasonToConfig[reason];
    }

    function getKeyTypes() external view returns (uint256[] memory) {
        return s().globalConfig[0].keyTypes;
    }

    function contractResolver() external view returns (address) {
        return address(s().contractResolver);
    }

    function kickPenaltyPercentByReason(
        uint256 reason
    ) external view returns (uint256) {
        return s().complaintReasonToConfig[reason].kickPenaltyPercent;
    }

    function getNodeDemerits(
        address stakerAddress
    ) external view returns (uint256) {
        return s().demeritsCounter[stakerAddress];
    }

    function nodeAddressToStakerAddress(
        address nodeAddress
    ) external view returns (address) {
        return s().nodeAddressToStakerAddress[nodeAddress];
    }

    function operatorAddressToStakerAddress(
        address operatorAddress
    ) external view returns (address) {
        return s().operatorAddressToStakerAddress[operatorAddress];
    }

    function readyForNextEpoch(
        uint256 realmId,
        address stakerAddress
    ) external view returns (bool) {
        return realm(realmId).readyForNextEpoch[stakerAddress];
    }

    function state(
        uint256 realmId
    ) external view returns (LibStakingStorage.States) {
        return realm(realmId).state;
    }

    /// get the token address from the resolver
    function getTokenContractAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.LIT_TOKEN_CONTRACT(),
                s().env
            );
    }

    function getStakingContractAddress() internal view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.STAKING_CONTRACT(),
                s().env
            );
    }

    function validators(
        address stakerAddress
    ) public view returns (LibStakingStorage.Validator memory) {
        return s().validators[stakerAddress];
    }

    function isActiveValidator(
        uint256 realmId,
        address stakerAddress
    ) external view returns (bool) {
        return realm(realmId).validatorsInCurrentEpoch.contains(stakerAddress);
    }

    function isActiveValidatorForNextEpoch(
        uint256 realmId,
        address stakerAddress
    ) external view returns (bool) {
        return realm(realmId).validatorsInNextEpoch.contains(stakerAddress);
    }

    function isActiveValidatorByNodeAddress(
        uint256 realmId,
        address account
    ) external view returns (bool) {
        return
            realm(realmId).validatorsInCurrentEpoch.contains(
                s().nodeAddressToStakerAddress[account]
            );
    }

    function isActiveValidatorByNodeAddressForNextEpoch(
        uint256 realmId,
        address nodeAddress
    ) external view returns (bool) {
        return
            realm(realmId).validatorsInNextEpoch.contains(
                s().nodeAddressToStakerAddress[nodeAddress]
            );
    }

    function isActiveShadowValidator(
        uint256 realmId,
        address stakerAddress
    ) external view returns (bool) {
        return realm(realmId).shadowValidators.contains(stakerAddress);
    }

    function getVotingStatusToKickValidator(
        uint256 realmId,
        uint256 epochNumber,
        address validatorToBeKickedStakerAddress,
        address voterStakerAddress
    ) external view returns (uint256, bool) {
        LibStakingStorage.VoteToKickValidatorInNextEpoch
            storage votingStatus = realm(realmId)
                .votesToKickValidatorsInNextEpoch[epochNumber][
                    validatorToBeKickedStakerAddress
                ];
        return (votingStatus.votes, votingStatus.voted[voterStakerAddress]);
    }

    function getValidatorsInCurrentEpoch(
        uint256 realmId
    ) public view returns (address[] memory) {
        address[] memory values = new address[](
            realm(realmId).validatorsInCurrentEpoch.length() +
                realm(realmId).shadowValidators.length()
        );
        uint256 validatorLength = realm(realmId)
            .validatorsInCurrentEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            values[i] = realm(realmId).validatorsInCurrentEpoch.at(i);
        }
        for (uint256 i = 0; i < realm(realmId).shadowValidators.length(); i++) {
            values[validatorLength + i] = realm(realmId).shadowValidators.at(i);
        }

        return values;
    }

    function getNonShadowValidatorsInCurrentEpochLength(
        uint256 realmId
    ) external view returns (uint256) {
        return realm(realmId).validatorsInCurrentEpoch.length();
    }

    function getValidatorsInNextEpoch(
        uint256 realmId
    ) public view returns (address[] memory) {
        uint256 target_length = realm(realmId).validatorsInNextEpoch.length();
        if (realm(realmId).targetValidatorCount > 0) {
            target_length = realm(realmId).targetValidatorCount;
        }

        address[] memory values = new address[](target_length);

        uint256 validatorLength = realm(realmId).validatorsInNextEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            values[i] = realm(realmId).validatorsInNextEpoch.at(i);
        }

        if (realm(realmId).shadowValidatorsInNextEpoch.length() > 0) {
            uint256 index = 0;
            for (uint256 i = validatorLength; i < target_length; i++) {
                values[i] = realm(realmId).shadowValidatorsInNextEpoch.at(
                    index
                );
                index++;
            }
        }
        return values;
    }

    function getValidatorsStructs(
        address[] memory stakerAddresses
    ) public view returns (LibStakingStorage.Validator[] memory) {
        LibStakingStorage.Validator[]
            memory values = new LibStakingStorage.Validator[](
                stakerAddresses.length
            );
        for (uint256 i = 0; i < stakerAddresses.length; i++) {
            values[i] = s().validators[stakerAddresses[i]];
        }
        return values;
    }

    function getValidatorsStructsInCurrentEpoch(
        uint256 realmId
    ) external view returns (LibStakingStorage.Validator[] memory) {
        address[] memory addresses = getValidatorsInCurrentEpoch(realmId);
        return getValidatorsStructs(addresses);
    }

    function getValidatorsStructsInNextEpoch(
        uint256 realmId
    ) external view returns (LibStakingStorage.Validator[] memory) {
        address[] memory addresses = getValidatorsInNextEpoch(realmId);
        return getValidatorsStructs(addresses);
    }

    function getTotalStake(
        address stakerAddress
    ) public view returns (uint256, uint256) {
        uint256 realmId = realms().getRealmIdForStakerAddress(stakerAddress);

        LibStakingStorage.RewardEpoch memory rewardEpoch = s().rewardEpochs[
            stakerAddress
        ][epoch(realmId).rewardEpochNumber];

        return (rewardEpoch.stakeAmount, rewardEpoch.totalStakeWeight);
    }

    function getTotalStakeByUser(
        address stakerAddress,
        address user
    ) public view returns (uint256, uint256) {
        uint256 realmId = realms().getRealmIdForStakerAddress(stakerAddress);
        LibStakingStorage.StakerVault memory vault = s().vaults[stakerAddress][
            user
        ];
        uint256 totalStake;
        uint256 totalStakeWeight;
        for (uint256 i = 0; i < vault.stakes.length; i++) {
            if (vault.stakes[i].loaded) {
                totalStake += vault.stakes[i].amount;
                totalStakeWeight += getStakeWeightInEpoch(
                    stakerAddress,
                    vault.stakes[i].id,
                    user,
                    epoch(realmId).rewardEpochNumber
                );
            }
        }
        return (totalStake, totalStakeWeight);
    }

    function getNodeStakerAddressMappings(
        address[] memory addresses
    ) public view returns (LibStakingStorage.AddressMapping[] memory) {
        LibStakingStorage.AddressMapping[]
            memory values = new LibStakingStorage.AddressMapping[](
                addresses.length
            );
        for (uint256 i = 0; i < addresses.length; i++) {
            values[i].nodeAddress = addresses[i];
            values[i].stakerAddress = s().nodeAddressToStakerAddress[
                addresses[i]
            ];
        }
        return values;
    }

    function getNodeAttestedPubKeyMappings(
        address[] memory addresses
    ) public view returns (LibStakingStorage.PubKeyMapping[] memory) {
        LibStakingStorage.PubKeyMapping[]
            memory values = new LibStakingStorage.PubKeyMapping[](
                addresses.length
            );
        for (uint256 i = 0; i < addresses.length; i++) {
            values[i].nodeAddress = addresses[i];
            values[i].pubKey = s().attestedAddressToPubKey[addresses[i]];
        }
        return values;
    }

    function countOfCurrentValidatorsReadyForNextEpoch(
        uint256 realmId
    ) public view returns (uint256) {
        uint256 total = 0;
        uint256 validatorLength = realm(realmId)
            .validatorsInCurrentEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            if (
                realm(realmId).readyForNextEpoch[
                    realm(realmId).validatorsInCurrentEpoch.at(i)
                ]
            ) {
                total++;
            }
        }
        return total;
    }

    function countOfNextValidatorsReadyForNextEpoch(
        uint256 realmId
    ) public view returns (uint256) {
        uint256 total = 0;
        uint256 validatorLength = realm(realmId).validatorsInNextEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            if (
                realm(realmId).readyForNextEpoch[
                    realm(realmId).validatorsInNextEpoch.at(i)
                ]
            ) {
                total++;
            }
        }
        return total;
    }

    function isReadyForNextEpoch(uint256 realmId) public view returns (bool) {
        // confirm that current validator set is ready iff we are NOT in the genesis epoch
        // and the current validator set is empty

        uint256 currentReady = countOfCurrentValidatorsReadyForNextEpoch(
            realmId
        );
        uint256 currentRequired = currentValidatorCountForConsensus(realmId);
        uint256 nextReady = countOfNextValidatorsReadyForNextEpoch(realmId);
        uint256 nextRequired = nextValidatorCountForConsensus(realmId);
        bool isGenesis = realm(realmId).validatorsInCurrentEpoch.length() == 0;

        if (isGenesis && (nextReady >= nextRequired)) {
            return true;
        }

        // handle shadow validators for shadow splicing, where shadow validators can't vote, and the may be is less than the threshold.
        if (realm(realmId).shadowValidators.length() > 0) {
            if (
                realm(realmId).validatorsInCurrentEpoch.length() <
                currentRequired
            ) {
                currentRequired = realm(realmId)
                    .validatorsInCurrentEpoch
                    .length();
            }
        }

        // confirm that current validator set is ready
        if (currentReady < currentRequired) {
            return false;
        }

        // confirm that next validator set is ready
        if (nextReady < nextRequired) {
            return false;
        }

        return true;
    }

    function shouldKickValidator(
        uint256 realmId,
        address stakerAddress
    ) public view returns (bool) {
        // 2/3 of validators must vote
        // and we don't want to kick below the threshold
        if (
            realm(realmId)
            .votesToKickValidatorsInNextEpoch[epoch(realmId).number][
                stakerAddress
            ].votes >= currentValidatorCountForConsensus(realmId)
        ) {
            return true;
        }
        return false;
    }

    // currently set to 2/3.  this could be changed to be configurable.
    function currentValidatorCountForConsensus(
        uint256 realmId
    ) public view returns (uint256) {
        uint256 currentValidatorCount = realm(realmId)
            .validatorsInCurrentEpoch
            .length() + realm(realmId).shadowValidators.length();
        return getThreshold(currentValidatorCount);
    }

    function isRecentValidator(
        uint256 realmId,
        address stakerAddresses
    ) external view returns (bool) {
        // this will be true if they are in the current or previous epoch

        return
            s().validators[stakerAddresses].lastActiveEpoch >=
            epoch(realmId).number - 1;
    }

    /// require all nodes in the next validator set to vote that they're ready
    /// any offline nodes will be kicked from the next validator set so that's why this is safe
    function nextValidatorCountForConsensus(
        uint256 realmId
    ) public view returns (uint256) {
        return realm(realmId).validatorsInNextEpoch.length();
    }

    function getThreshold(uint256 nodeCount) public view returns (uint256) {
        if (
            nodeCount <=
            s().globalConfig[0].minValidatorCountToClampMinimumThreshold
        ) {
            return s().globalConfig[0].minThresholdToClampAt;
        }

        return (nodeCount * 2) / 3;
    }

    function getKickedValidators(
        uint256 realmId
    ) public view returns (address[] memory) {
        return realm(realmId).validatorsKickedFromNextEpoch.values();
    }

    function getShadowValidators(
        uint256 realmId
    ) public view returns (address[] memory) {
        return realm(realmId).shadowValidators.values();
    }

    function getNonShadowValidators(
        uint256 realmId
    ) public view returns (address[] memory) {
        return realm(realmId).validatorsInCurrentEpoch.values();
    }

    function getActiveUnkickedValidators(
        uint256 realmId
    ) public view returns (address[] memory) {
        uint256 currentCount = realm(realmId).validatorsInCurrentEpoch.length();
        uint256 activeCount = getActiveUnkickedValidatorCount(realmId);
        address[] memory values = new address[](activeCount);
        uint256 index = 0;
        for (uint256 i = 0; i < currentCount; i++) {
            address validator = realm(realmId).validatorsInCurrentEpoch.at(i);
            if (
                realm(realmId).validatorsKickedFromNextEpoch.contains(validator)
            ) {
                continue;
            }
            values[index] = validator;
            index++;
        }
        return values;
    }

    function getStakeRecordCount(
        address userStakerAddress,
        address operatorStakerAddress
    ) public view returns (uint256) {
        LibStakingStorage.StakerVault memory vault = s().vaults[
            operatorStakerAddress
        ][userStakerAddress];

        uint256 count;
        for (uint i = 0; i < vault.stakes.length; i++) {
            if (vault.stakes[i].loaded == true) {
                count += 1;
            }
        }
        return count;
    }

    function getEmptyStakeRecordSlots(
        address userStakerAddress,
        address operatorStakerAddress
    ) public view returns (uint256) {
        LibStakingStorage.StakerVault memory vault = s().vaults[
            operatorStakerAddress
        ][userStakerAddress];

        uint256 count;
        for (uint i = 0; i < vault.stakes.length; i++) {
            if (!vault.stakes[i].loaded) {
                count += 1;
            }
        }
        return count;
    }

    function getUnfrozenStakeCountForUser(
        address userStakerAddress,
        address operatorStakerAddress
    ) public view returns (uint256) {
        LibStakingStorage.StakerVault memory vault = s().vaults[
            operatorStakerAddress
        ][userStakerAddress];

        uint256 count;
        for (uint i = 0; i < vault.stakes.length; i++) {
            if (
                vault.stakes[i].loaded == true &&
                vault.stakes[i].unfreezeStart > 0 &&
                vault.stakes[i].frozen
            ) {
                count += 1;
            }
        }
        return count;
    }

    function getLastStakeRecord(
        address user,
        address stakerAddress
    ) public view returns (LibStakingStorage.StakeRecord memory) {
        LibStakingStorage.StakerVault memory vault = s().vaults[stakerAddress][
            user
        ];

        uint256 lastStakeRecordId = vault.lastStakeRecordId;
        for (uint i = 0; i < vault.stakes.length; i++) {
            if (vault.stakes[i].id == lastStakeRecordId) {
                return vault.stakes[i];
            }
        }

        revert StakingUtilsLib.StakeRecordNotFound(lastStakeRecordId);
    }

    function getValidatorsDelegated(
        address user
    ) public view returns (uint256[] memory) {
        // address[] memory stakerAddresses = getActiveUnkickedValidators(realmId);
        address[] memory stakerAddresses = s().allActiveValidators.values();

        uint256 length;
        for (uint i = 0; i < stakerAddresses.length; i++) {
            if (getStakeRecordCount(user, stakerAddresses[i]) > 0) {
                length += 1;
            }
        }

        uint256[] memory values = new uint256[](length);

        uint256 index = 0;
        for (uint i = 0; i < stakerAddresses.length; i++) {
            if (getStakeRecordCount(user, stakerAddresses[i]) > 0) {
                values[index] = i;
                index += 1;
            }
        }

        return values;
    }

    function getStakeRecordsForUser(
        address userStakerAddress,
        address operatorStakerAddress
    ) public view returns (LibStakingStorage.StakeRecord[] memory) {
        uint256 count = getStakeRecordCount(
            userStakerAddress,
            operatorStakerAddress
        );
        LibStakingStorage.StakeRecord[]
            memory values = new LibStakingStorage.StakeRecord[](count);

        uint256 index;

        LibStakingStorage.StakerVault memory vault = s().vaults[
            operatorStakerAddress
        ][userStakerAddress];
        for (uint j = 0; j < vault.stakes.length; j++) {
            if (vault.stakes[j].loaded == true) {
                values[index] = vault.stakes[j];
                index += 1;
            }
        }

        return values;
    }

    function getActiveUnkickedValidatorCount(
        uint256 realmId
    ) public view returns (uint256) {
        uint256 currentCount = realm(realmId).validatorsInCurrentEpoch.length();
        uint256 kickedCountInNextEpoch = realm(realmId)
            .validatorsKickedFromNextEpoch
            .length();
        return currentCount - kickedCountInNextEpoch;
    }

    function getActiveUnkickedValidatorStructs(
        uint256 realmId
    ) public view returns (LibStakingStorage.Validator[] memory) {
        address[] memory activeValidators = getActiveUnkickedValidators(
            realmId
        );
        return getValidatorsStructs(activeValidators);
    }

    function getActiveUnkickedValidatorStructsAndCounts(
        uint256 realmId
    )
        public
        view
        returns (
            LibStakingStorage.Epoch memory,
            uint256,
            LibStakingStorage.Validator[] memory
        )
    {
        return (
            epoch(realmId),
            currentValidatorCountForConsensus(realmId),
            getActiveUnkickedValidatorStructs(realmId)
        );
    }

    /**
     * @notice get the timelock of a StakeRecord, since its possible its currently unfrozen
     * but we want to know what it was prior to being unfrozen
     * @param stakerAddress address of the staker who has this record
     * @param stakeRecord the record to check
     * @param rewardEpochNumber the number of the epoch to check this
     * @dev For a stake that is frozen, this simply returns the timelock registered in the stake record.
     * @dev For a stake that is unfrozen, if the reward epoch is before the time of unfreezing, this returns
     *      the timelock registered in the stake record.
     * @dev For a stake that is unfrozen, if the reward epoch is after the time of unfreezing, this returns
     *      the remaining time of the timelock considering the elapsed time in the reward epoch.
     */
    function getTimelockInEpoch(
        address stakerAddress,
        LibStakingStorage.StakeRecord memory stakeRecord,
        uint256 rewardEpochNumber
    ) public view returns (uint256) {
        LibStakingStorage.RewardEpoch memory rewardEpoch = s().rewardEpochs[
            stakerAddress
        ][rewardEpochNumber];

        uint256 timeLock;
        if (stakeRecord.unfreezeStart == 0) {
            timeLock = stakeRecord.timeLock;
        } else {
            if (rewardEpoch.epochEnd < stakeRecord.unfreezeStart) {
                timeLock = stakeRecord.timeLock; // because we are querying at an epoch prior to us unfreezing
            } else {
                uint256 timeSinceUnfreeze = rewardEpoch.epochEnd -
                    stakeRecord.unfreezeStart;
                if (timeSinceUnfreeze > stakeRecord.timeLock) {
                    return 0;
                } else {
                    timeLock = stakeRecord.timeLock - timeSinceUnfreeze;
                }
            }
        }

        return timeLock;
    }

    function getStakeWeightInEpoch(
        address stakerAddress,
        uint256 recordId,
        address userStakerAddress,
        uint256 rewardEpochNumber
    ) public view returns (uint256) {
        LibStakingStorage.StakeRecord memory stakeRecord = getStakeRecord(
            stakerAddress,
            recordId,
            userStakerAddress
        );

        uint256 timeLock = getTimelockInEpoch(
            stakerAddress,
            stakeRecord,
            rewardEpochNumber
        );
        uint256 amount = getTokensStaked(
            stakerAddress,
            stakeRecord,
            rewardEpochNumber
        );

        return calculateStakeWeight(timeLock, amount);
    }

    /**
     * @notice calculate stake weight. If the timeLock is greater than the maxTimeLock, the stake weight is equal to the amount
     * @param timeLock length of lock time
     * @param amount amount of tokens staked
     * @dev See Equation 1 in the paper. This formula sets w_min = 0, which means that it is possible for nodes to have 0 stake weight,
     * and they need to commit their pledged stake for at least 2 weeks to have a non-zero stake weight and hence be eligible for rewards.
     */
    function calculateStakeWeight(
        uint256 timeLock,
        uint256 amount
    ) public view returns (uint256) {
        uint256 maxLock = s().globalConfig[0].maxTimeLock;

        return timeLock > maxLock ? amount : (amount * timeLock) / maxLock;
    }

    /**
     * @notice returns the amount of tokens in a StakeRecord, acknowleding there could have been a slashing
     * @param stakerAddress address of the staker who has this record
     * @param rewardEpochNumber number of epoch to calculate this in
     * @param stakeRecord StakeRecord to calculate the amount of tokens staked
     */
    function getTokensStaked(
        address stakerAddress,
        LibStakingStorage.StakeRecord memory stakeRecord,
        uint256 rewardEpochNumber
    ) public view returns (uint256) {
        uint256 initialSharePrice = stakeRecord.initialSharePrice;
        uint256 currentSharePrice = s()
        .rewardEpochs[stakerAddress][rewardEpochNumber].validatorSharePrice;

        // FIXME: why is this being called - doesn't seem to be used anywhere
        // getRewardEpoch(stakerAddress, rewardEpochNumber);

        if (initialSharePrice == 0) {
            return 0;
        } else {
            return (stakeRecord.amount * currentSharePrice) / initialSharePrice;
        }
    }

    function getRewardEpochNumber(
        uint256 realmId
    ) public view returns (uint256) {
        return realm(realmId).epochs[0].rewardEpochNumber;
    }

    function pow(uint256 base, uint256 exponent) public pure returns (uint256) {
        return
            SafeCast.toUint256(
                FixedPointMathLib.powWad(
                    SafeCast.toInt256(base),
                    SafeCast.toInt256(exponent)
                )
            );
    }
    /**
     * @notice Function to calculate the rewards per day.
     * @dev The usage of .mulWad and .divWad is for the parameters that are using ether for fixed point math.
     */
    function calculateRewardsPerDay(
        LibStakingStorage.RewardEpochGlobalStats memory globalStats
    ) external view returns (uint256) {
        if (globalStats.stakeAmount == 0) {
            return 0;
        }

        uint256 ONE = 10 ** 18;

        uint256 firstTerm = getLitCirc() / 30;
        uint256 secondTermNumerator = pow(
            globalStats.stakeWeight,
            s().globalConfig[0].p
        );
        uint256 secondTermDenominator = pow(
            globalStats.stakeAmount,
            s().globalConfig[0].p
        );
        uint256 stakeAmount = globalStats.stakeAmount;
        uint256 thirdTerm = pow(
            FixedPointMathLib.mulWad(
                (pow(
                    s().globalConfig[0].bmax,
                    FixedPointMathLib.divWad(ONE, s().globalConfig[0].p)
                ) -
                    pow(
                        s().globalConfig[0].bmin,
                        FixedPointMathLib.divWad(ONE, s().globalConfig[0].p)
                    )),
                FixedPointMathLib.divWad(
                    StakingUtilsLib.min(
                        s().globalConfig[0].k,
                        FixedPointMathLib.divWad(stakeAmount, getLitCirc())
                    ),
                    s().globalConfig[0].k
                )
            ) +
                pow(
                    s().globalConfig[0].bmin,
                    FixedPointMathLib.divWad(ONE, s().globalConfig[0].p)
                ),
            s().globalConfig[0].p
        );

        return
            (FixedPointMathLib.mulWad(firstTerm, secondTermNumerator) *
                thirdTerm) / secondTermDenominator;
    }

    /// @notice For simplicity, we will use total supply here instead of total circulating supply.
    /// @notice Since the token supply will be distributed across various L2s and L3s, and that
    /// simply using totalSupply() will NOT give us the total supply of the token, we will hardcode
    /// this to 1 billion for now. Doing so is not known to cause any short term issues, though it will
    /// need to be adjusted over time as more tokens get minted.
    function getLitCirc() public view returns (uint256) {
        return s().tokenTotalSupplyStandIn;
    }

    function minPower(
        bytes16 base,
        uint256 p,
        uint256 q
    ) internal pure returns (bytes16) {
        if (p < q) {
            return power(base, ABDKMathQuad.fromUInt(p));
        } else {
            return power(base, ABDKMathQuad.fromUInt(q));
        }
    }

    function power(
        bytes16 base,
        bytes16 exponent
    ) internal pure returns (bytes16) {
        return
            ABDKMathQuad.exp(ABDKMathQuad.mul(ABDKMathQuad.ln(base), exponent));
    }

    /**
     * @param recordId id of the record to pull
     * @param stakerAddress address of the staker who has this record
     */
    function getStakeRecord(
        address stakerAddress,
        uint256 recordId,
        address userStakerAddress
    ) public view returns (LibStakingStorage.StakeRecord memory) {
        return
            StakingUtilsLib.getStakeRecord(
                stakerAddress,
                recordId,
                userStakerAddress
            );
    }

    /**
     * @notice Check if the validator has an eligible self stake to remain
     * in the validator set. There's a minimum amount of stake and a minimum
     * length of lock. If it goes under either of these thresholds the validator
     * cannot join the set. Users can move their StakeRecord to a new validator
     * to keep earning rewards.
     * @param realmId The realmId of the realm
     * @param stakerAddress The staker address of the validator to check
     * @param stakerInCurrentValidatorSet Whether the staker is in the current validator set
     */

    function validatorSelfStakeWillExpire(
        uint256 realmId,
        address stakerAddress,
        bool stakerInCurrentValidatorSet
    ) public view returns (bool) {
        address nodeAddress = s().stakerAddressToNodeAddress[stakerAddress];
        if (nodeAddress == address(0)) {
            revert NodeAddressNotFoundForStaker();
        }

        LibStakingStorage.StakerVault memory vault = s().vaults[stakerAddress][
            stakerAddress
        ];

        // If the staker is not part of the current validator set, they are about to join the next epoch
        // and we should check against the next reward epoch number.
        uint256 rewardEpochNumberToCheck;
        if (stakerInCurrentValidatorSet) {
            rewardEpochNumberToCheck = epoch(realmId).rewardEpochNumber;
        } else {
            rewardEpochNumberToCheck = epoch(realmId).nextRewardEpochNumber;
        }

        bool invalid = true;
        for (uint256 i = 0; i < vault.stakes.length; i++) {
            LibStakingStorage.StakeRecord memory stakeRecord = vault.stakes[i];
            if (stakeRecord.loaded) {
                uint256 tokensStaked = getTokensStaked(
                    stakerAddress,
                    stakeRecord,
                    rewardEpochNumberToCheck
                );
                uint256 timelock = getTimelockInEpoch(
                    stakerAddress,
                    stakeRecord,
                    rewardEpochNumberToCheck
                );

                if (
                    tokensStaked >= s().globalConfig[0].minSelfStake &&
                    timelock >= s().globalConfig[0].minSelfStakeTimelock
                ) {
                    invalid = false;
                }
            }
        }
        return invalid;
    }

    function getRewardEpochGlobalStats(
        uint256 epochNumber
    ) public view returns (LibStakingStorage.RewardEpochGlobalStats memory) {
        return StakingUtilsLib.getRewardEpochGlobalStats(epochNumber);
    }
    // function parseTimeLock(LibStakingStorage.TimeLock timeLock) public pure returns (uint256) {
    //     if (timeLock == LibStakingStorage.TimeLock.OneMonth) {
    //         return 30 days;
    //     } else if (timeLock == LibStakingStorage.TimeLock.ThreeMonths) {
    //         return 90 days;
    //     } else if (timeLock == LibStakingStorage.TimeLock.SixMonths) {
    //         return 180 days;
    //     } else if (timeLock == LibStakingStorage.TimeLock.OneYear) {
    //         return 365 days;
    //     } else if (timeLock == LibStakingStorage.TimeLock.TwoYears) {
    //         return 730 days;
    //     } else {
    //         revert InvalidTimeLock();
    //     }
    // }

    function getTokenPrice() public view returns (uint256) {
        return s().globalConfig[0].tokenPrice;
    }

    function minSelfStake() external view returns (uint256) {
        return s().globalConfig[0].minSelfStake;
    }

    function minStake() external view returns (uint256) {
        return s().globalConfig[0].minStakeAmount;
    }

    function maxStake() external view returns (uint256) {
        return s().globalConfig[0].maxStakeAmount;
    }

    function minTimeLock() external view returns (uint256) {
        return s().globalConfig[0].minTimeLock;
    }

    function maxTimeLock() external view returns (uint256) {
        return s().globalConfig[0].maxTimeLock;
    }

    /// @return A tuple with the realmId and the reward epoch number
    function getLowestRewardEpochNumber()
        public
        view
        returns (uint256, uint256)
    {
        // Loop through all the realm epochs and find the lowest current reward epoch number
        uint256 lowestRewardEpochNumber = type(uint256).max; // set to highest number
        uint256 realmIdWithLowestRewardEpochNumber = 0;
        for (uint256 i = 0; i < s().realmIds.length(); i++) {
            uint256 realmId = s().realmIds.at(i);
            if (epoch(realmId).rewardEpochNumber < lowestRewardEpochNumber) {
                lowestRewardEpochNumber = epoch(realmId).rewardEpochNumber;
                realmIdWithLowestRewardEpochNumber = realmId;
            }
        }
        return (realmIdWithLowestRewardEpochNumber, lowestRewardEpochNumber);
    }

    function getAllValidators() external view returns (address[] memory) {
        return s().allValidators.values();
    }

    /// @notice This will return all the validators that are in the reserve set.
    /// @dev A reserve set is the set of validators that are not in the current or next validator set,
    /// but are self-stakers.
    /// @dev When this becomes large, we may need to eliminate usage of the EnumerableSet and use a combination
    /// of mappings, arrays and cursors.
    function getAllReserveValidators()
        external
        view
        returns (address[] memory)
    {
        // Initialize the array to store all the reserve validators.
        uint256 numReserveValidators = s().allValidators.length() -
            s().allActiveValidators.length();
        if (numReserveValidators == 0) {
            return new address[](0);
        }

        address[] memory allReserveValidators = new address[](
            numReserveValidators
        );
        uint256 arrIndex = 0;
        // For each validator in allValidators, check if it is an active validator
        // that is not perma-banned.
        for (uint256 i = 0; i < s().allValidators.length(); i++) {
            address validator = s().allValidators.at(i);
            if (
                !s().allActiveValidators.contains(validator) &&
                !isValidatorBanned(validator)
            ) {
                allReserveValidators[arrIndex] = validator;
                arrIndex++;
            }
        }

        // Now return the array without empty elements.
        uint256 numEmptyElements = allReserveValidators.length - arrIndex;
        address[]
            memory allReserveValidatorsWithoutEmptyElements = new address[](
                allReserveValidators.length - numEmptyElements
            );
        for (uint256 i = 0; i < allReserveValidators.length; i++) {
            // We can only do this because all the empty elements are at the end of the array.
            if (allReserveValidators[i] == address(0)) {
                break;
            }

            allReserveValidatorsWithoutEmptyElements[i] = allReserveValidators[
                i
            ];
        }

        return allReserveValidatorsWithoutEmptyElements;
    }

    function isValidatorBanned(address validator) public view returns (bool) {
        return s().demeritsCounter[validator] >= s().demeritRejoinThreshold;
    }

    /// @notice This function returns the number of self-stake records for a given staker.
    function getSelfStakeRecordCount(
        address stakerAddress
    ) public view returns (uint256) {
        uint256 selfStakeRecordCount = 0;

        LibStakingStorage.StakerVault storage userVault = s().vaults[
            stakerAddress
        ][stakerAddress];

        for (uint256 i = 0; i < userVault.stakes.length; i++) {
            if (userVault.stakes[i].loaded) {
                selfStakeRecordCount++;
            }
        }

        return selfStakeRecordCount;
    }

    /// @notice This function returns the list of permitted validators for a given realm.
    function permittedValidators(
        uint256 realmId
    ) external view returns (address[] memory) {
        return realm(realmId).permittedValidators.values();
    }

    /// @notice This function returns the list of realms for which a given validator is permitted.
    function permittedRealmsForValidator(
        address validator
    ) external view returns (uint256[] memory) {
        // First get the count of realms for which the validator is permitted.
        uint256 numPermittedRealms = 0;
        for (uint256 i = 0; i < s().realmIds.length(); i++) {
            uint256 realmId = s().realmIds.at(i);
            if (realm(realmId).permittedValidators.contains(validator)) {
                numPermittedRealms++;
            }
        }

        // Now, populate the array with the realm ids.
        uint256[] memory permittedRealms = new uint256[](numPermittedRealms);
        uint256 arrIndex = 0;
        for (uint256 i = 0; i < s().realmIds.length(); i++) {
            uint256 realmId = s().realmIds.at(i);
            if (realm(realmId).permittedValidators.contains(validator)) {
                permittedRealms[arrIndex] = realmId;
                arrIndex++;
            }
        }

        return permittedRealms;
    }

    function stakerToValidatorsTheyStakedTo(
        address staker
    ) external view returns (address[] memory) {
        return s().stakerToValidatorsTheyStakedTo[staker].values();
    }

    function getDelegatedStakersWithUnfreezingStakes(
        address validatorAddress,
        uint256 limit,
        uint256 offset
    ) external view returns (address[] memory) {
        return
            s()
                .validatorToDelegatedStakersWithUnfreezingStakes[
                    validatorAddress
                ]
                .limitedValuesFrom(limit, offset);
    }

    function getDelegatedStakersWithUnfreezingStakesCount(
        address validatorAddress
    ) external view returns (uint256) {
        return
            s()
                .validatorToDelegatedStakersWithUnfreezingStakes[
                    validatorAddress
                ]
                .length();
    }

    function getAttestedPubKey(
        address attestedAddress
    ) external view returns (bytes memory) {
        LibStakingStorage.UncompressedK256Key memory key = s()
            .attestedAddressToPubKey[attestedAddress];
        require(key.x != 0, "No key registered");
        require(key.y != 0, "No key registered");

        bytes memory pubKey = new bytes(65);
        pubKey[0] = bytes1(0x04);

        assembly {
            let x := mload(key)
            let y := mload(add(key, 32))

            mstore(add(pubKey, 33), x)
            mstore(add(pubKey, 65), y)
        }

        return pubKey;
    }

    error InvalidTimeLock();
}
