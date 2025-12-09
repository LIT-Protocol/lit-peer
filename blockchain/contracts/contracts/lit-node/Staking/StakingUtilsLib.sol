//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { StakingAcrossRealmsFacet } from "./StakingAcrossRealmsFacet.sol";
import { StakingValidatorFacet } from "./StakingValidatorFacet.sol";
import { FixedPointMathLib } from "solady/src/utils/FixedPointMathLib.sol";
import { console } from "hardhat/console.sol";

library StakingUtilsLib {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.UintSet;

    uint256 constant PRECISION = 1 ether;
    uint256 constant HUNDRED_PERCENT = 1 ether;

    /* ========== ERRORS ========== */
    error MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
        LibStakingStorage.States state
    );
    error MustBeInActiveOrUnlockedOrPausedState(LibStakingStorage.States state);
    error CallerNotOwner();
    error CallerNotOwnerOrDevopsAdmin();
    error NotEnoughValidatorsInNextEpoch(
        uint256 validatorCount,
        uint256 minimumValidatorCount
    );
    error StakeRecordNotFound(uint256 stakeRecordId);
    error StakeAmountNotMet(uint256 amount);
    error MinTimeLockNotMet(uint256 timeLock, uint256 minTimeLock);
    error InvalidNewSharePrice();
    error ValidatorIsNotInNextEpoch(
        address validator,
        address[] validatorsInNextEpoch
    );
    error InvalidSlashPercentage();
    error CannotStakeZero();
    error CannotMoveToLockedValidatorStateBeforeEpochEnds();

    /* ========== VIEWS ========== */

    function s()
        internal
        pure
        returns (LibStakingStorage.GlobalStakingStorage storage)
    {
        return LibStakingStorage.getStakingStorage();
    }

    function views() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(address(this));
    }

    function staking() internal view returns (StakingFacet) {
        return StakingFacet(address(this));
    }

    function realm(
        uint256 realmId
    ) internal view returns (LibStakingStorage.RealmStorage storage) {
        return LibStakingStorage.getRealmStorage(realmId);
    }

    function realms() internal view returns (StakingAcrossRealmsFacet) {
        return StakingAcrossRealmsFacet(address(this));
    }

    function mutableEpoch(
        uint256 realmId
    ) internal view returns (LibStakingStorage.Epoch storage) {
        return realm(realmId).epochs[0];
    }

    /**
     * @param recordId id of the record to pull
     * @param stakerAddress address of the staker who has this record
     */
    function getStakeRecord(
        address stakerAddress,
        uint256 recordId,
        address userStakerAddress
    ) internal view returns (LibStakingStorage.StakeRecord storage) {
        LibStakingStorage.StakerVault storage vault = s().vaults[stakerAddress][
            userStakerAddress
        ];
        for (uint256 i = 0; i < vault.stakes.length; i++) {
            if (vault.stakes[i].id == recordId) {
                return vault.stakes[i];
            }
        }
        revert StakeRecordNotFound(recordId);
    }

    function getRewardEpochGlobalStats(
        uint256 epochNumber
    ) internal view returns (LibStakingStorage.RewardEpochGlobalStats storage) {
        return s().rewardEpochsGlobalStats[epochNumber];
    }

    /* ========== INTERNAL MUTATIVE FUNCTIONS ========== */

    function clearEnumerableAddressSet(
        EnumerableSet.AddressSet storage set
    ) internal {
        while (set.length() > 0) {
            set.remove(set.at(0));
        }
    }

    function unlockEpoch(uint256 realmId) internal {
        LibStakingStorage.RealmStorage storage realmStorage = realm(realmId);
        // this should only be callable from the ReadyForNextEpoch state or the NextValidatorSetLocked state
        if (
            !(realmStorage.state ==
                LibStakingStorage.States.ReadyForNextEpoch ||
                realmStorage.state ==
                LibStakingStorage.States.NextValidatorSetLocked)
        ) {
            revert MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
                realmStorage.state
            );
        }
        // clear out readyForNextEpoch for current nodes
        uint256 validatorLength = realmStorage
            .validatorsInCurrentEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            realmStorage.readyForNextEpoch[
                realmStorage.validatorsInCurrentEpoch.at(i)
            ] = false;
        }

        // clear out readyForNextEpoch for next nodes
        validatorLength = realmStorage.validatorsInNextEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            realmStorage.readyForNextEpoch[
                realmStorage.validatorsInNextEpoch.at(i)
            ] = false;
        }

        realmStorage.state = LibStakingStorage.States.Unlocked;
        realmStorage.epochs[0].retries++;
        emit StateChanged(realmStorage.state);
    }

    function checkNextSetAboveThreshold(uint256 realmId) internal view {
        uint256 validatorsCnt = realm(realmId).validatorsInNextEpoch.length();
        checkValidatorCountAgainstKeySetsInRealm(realmId, validatorsCnt, 2);

        // never let the network go below 3
        if (validatorsCnt < s().globalConfig[0].minimumValidatorCount) {
            revert NotEnoughValidatorsInNextEpoch(
                validatorsCnt,
                s().globalConfig[0].minimumValidatorCount
            );
        }
    }

    function checkValidatorCountAgainstKeySetsInRealm(
        uint256 realmId,
        uint256 validatorCnt,
        uint256 reason
    ) internal view {
        bytes32[] memory keySetIds = s().keySetIds;
        for (uint256 i = 0; i < keySetIds.length; i++) {
            LibStakingStorage.KeySetConfig memory config = s().keySetsConfigs[
                keySetIds[i]
            ];
            for (uint256 j = 0; j < config.realms.length; j++) {
                if (config.realms[i] == realmId) {
                    if (validatorCnt < config.minimumThreshold) {
                        if (reason == 1) {
                            revert("Not enough validators for key set");
                        } else if (reason == 2) {
                            revert NotEnoughValidatorsInNextEpoch(
                                validatorCnt,
                                config.minimumThreshold
                            );
                        } else if (reason == 3) {
                            revert StakingValidatorFacet
                                .CannotKickBelowKeySetThreshold(
                                    config.identifier
                                );
                        }
                    }
                    break;
                }
            }
        }
    }

    /**
     * @notice Rewards validators in the current epoch
     * @param realmId The realmId of the realm
     * @param currentRewardEpoch The current reward epoch
     * @param currentRewardEpochGlobalStats The global stats for the current reward epoch
     */
    function rewardValidators(
        uint256 realmId,
        uint256 currentRewardEpoch,
        LibStakingStorage.RewardEpochGlobalStats
            memory currentRewardEpochGlobalStats
    ) internal {
        if (
            currentRewardEpoch < views().realmConfig(realmId).minEpochForRewards
        ) {
            return;
        }

        uint256 validatorLength = currentRewardEpochGlobalStats
            .validatorsInCurrentEpoch
            .length;
        uint256 dailyReward = views().calculateRewardsPerDay(
            currentRewardEpochGlobalStats
        );

        // If the epoch has already ended and we are visiting this function in order to
        // re-calculate the rewards, then we should use the epoch length that was already
        // stored. Otherwise, we can use the current block.timestamp to calculate the
        // actual epoch length.
        uint256 actualEpochLength;
        if (currentRewardEpochGlobalStats.actualEpochLength > 0) {
            actualEpochLength = currentRewardEpochGlobalStats.actualEpochLength;
        } else {
            actualEpochLength =
                block.timestamp -
                mutableEpoch(realmId).startTime;

            // Set the actual epoch length in the global stats
            s()
                .rewardEpochsGlobalStats[currentRewardEpoch]
                .actualEpochLength = actualEpochLength;
        }

        uint256 epochRewards = (dailyReward * actualEpochLength) / 1 days;

        // Validators earn some fixed amount for their infrastructure
        uint256 fixedCostRewards;
        {
            uint256 numberOfEpochsPerMonth = 30 days /
                s().globalConfig[0].rewardEpochDuration;
            uint256 fixedCostRewardsUSDPerEpoch = FixedPointMathLib.mulWad(
                s().globalConfig[0].usdCostPerMonth,
                s().globalConfig[0].profitMultiplier
            ) / numberOfEpochsPerMonth;
            // Adjust for the actual epoch length
            uint256 fixedCostRewardsUSDPerEpochProRated = (fixedCostRewardsUSDPerEpoch *
                    actualEpochLength) /
                    s().globalConfig[0].rewardEpochDuration;
            fixedCostRewards = FixedPointMathLib.mulWad(
                fixedCostRewardsUSDPerEpochProRated,
                s().globalConfig[0].tokenPrice
            );
        }

        uint256 totalEmitted = epochRewards +
            fixedCostRewards *
            validatorLength;

        // validators and delegators also receive staking rewards
        uint256 emissionRate = (totalEmitted * PRECISION) /
            views().getLitCirc() /
            actualEpochLength /
            365 days;

        if (emissionRate > s().globalConfig[0].maxEmissionRate) {
            // we clamp emissions in the case emissions get to high if price decreases
            epochRewards = 0;
            fixedCostRewards =
                (s().globalConfig[0].maxEmissionRate *
                    views().getLitCirc() *
                    actualEpochLength) /
                365 days /
                validatorLength /
                PRECISION;
        }

        for (uint256 i = 0; i < validatorLength; i++) {
            address stakerAddress = currentRewardEpochGlobalStats
                .validatorsInCurrentEpoch[i];
            LibStakingStorage.RewardEpoch storage rewardEpoch = s()
                .rewardEpochs[stakerAddress][currentRewardEpoch];

            uint256 totalNetworkStakeWeight = views()
                .getRewardEpochGlobalStats(currentRewardEpoch)
                .stakeWeight;

            if (
                (epochRewards == 0 && fixedCostRewards == 0) ||
                totalNetworkStakeWeight == 0 ||
                rewardEpoch.totalStakeWeight == 0
            ) {
                continue;
            }

            uint256 validatorRewards;
            {
                // We use c_max to prevent concentration of rewards towards any one validator (including their delegated stakes).
                // Here c_max is defined as max(5%, 2/N) where N is the number of validators in the current epoch. We use a max
                // function as a counterbalance to prevent the forcing equal distribution of stakeweights and rewards.
                uint256 cMax = max(
                    (5 * PRECISION) / 100,
                    (2 * PRECISION) / validatorLength
                );
                uint256 actualEligibleFraction = min(
                    (rewardEpoch.totalStakeWeight * PRECISION) /
                        totalNetworkStakeWeight,
                    cMax
                );
                validatorRewards =
                    (epochRewards * actualEligibleFraction) /
                    PRECISION; // Here we divide by PRECISION to get the rewards back to the same units as the epochRewards
            }

            uint256 commission = (validatorRewards *
                s().validators[stakerAddress].commissionRate) / PRECISION;
            s().validators[stakerAddress].lastRewardEpoch = currentRewardEpoch;
            validatorRewards -= commission;

            // Store the rewards in the reward epoch
            rewardEpoch.totalStakeRewards = validatorRewards;
            rewardEpoch.validatorCommission = commission;
            rewardEpoch.validatorFixedCostRewards = fixedCostRewards;
        }
    }

    /**
     * @notice Updates reward epoch (increments counter if new one is started) and rewards the validators
     * @dev Here are the scenarios to consider:
     * - Validator set no change
     *   - The next epoch's GS and RE are updated in the prior epoch before advancing
     * - Validator set kicking one node out in the next epoch
     *   - Upon slashing, GS is immediately updated for the current RE number / epoch in order to adjust
     *     the reward distributions for validators in the current epoch.
     * - Validator set dealing node in / out in the next epoch
     *   - Upon requesting to leave, next epoch's GS and RE are updated in the prior epoch before advancing
     * @dev This function assumes it will be called in advanceEpoch and that the next validator set will be
     *      what becomes the current validator set.
     */
    function updateRewardEpoch(uint256 realmId) internal {
        LibStakingStorage.Epoch memory epoch = mutableEpoch(realmId);
        uint256 currentRewardEpochNumber = epoch.rewardEpochNumber;
        uint256 nextRewardEpochNumber = epoch.nextRewardEpochNumber;

        // Get the global stats for the NEXT reward epoch for writing to.
        LibStakingStorage.RewardEpochGlobalStats
            storage nextRewardEpochGlobalStats = getRewardEpochGlobalStats(
                nextRewardEpochNumber
            );

        address[] memory currentValidators = views()
            .getValidatorsInCurrentEpoch(realmId);

        // Add the validators in the next epoch to the next global stats
        for (
            uint256 i = 0;
            i < realm(realmId).validatorsInNextEpoch.length();
            i++
        ) {
            address stakerAddress = realm(realmId).validatorsInNextEpoch.at(i);
            nextRewardEpochGlobalStats.validatorsInCurrentEpoch.push(
                stakerAddress
            );

            // If the validator is in the next validator set but not in the current, we set the next reward epoch slope
            // using the last reward epoch slope. This allows us to resume unfreezing that was paused when the validator
            // left the realm.
            if (!views().isActiveValidator(realmId, stakerAddress)) {
                uint256 lastRewardEpochNumber = s()
                    .validators[stakerAddress]
                    .lastRewardEpoch;
                LibStakingStorage.RewardEpoch
                    storage nextRewardEpoch = _getRewardEpoch(
                        stakerAddress,
                        nextRewardEpochNumber
                    );
                nextRewardEpoch.slope = s()
                .rewardEpochs[stakerAddress][lastRewardEpochNumber].slope;
            }
        }

        // Set the actual epoch length in the global stats
        s()
            .rewardEpochsGlobalStats[currentRewardEpochNumber]
            .actualEpochLength =
            block.timestamp -
            mutableEpoch(realmId).startTime;

        for (uint256 i = 0; i < currentValidators.length; i++) {
            _updateRewardEpochForValidator(
                realmId,
                currentValidators[i],
                currentRewardEpochNumber,
                nextRewardEpochNumber,
                nextRewardEpochGlobalStats
            );
        }

        // Reward the validators in the current epoch according to the current stats.
        rewardValidators(
            realmId,
            currentRewardEpochNumber,
            views().getRewardEpochGlobalStats(currentRewardEpochNumber)
        );
    }

    function _updateRewardEpochForValidator(
        uint256 realmId,
        address stakerAddress,
        uint256 currentRewardEpochNumber,
        uint256 nextRewardEpochNumber,
        LibStakingStorage.RewardEpochGlobalStats
            storage nextRewardEpochGlobalStats
    ) private {
        // If the validator is not in the next validator set, then we don't need to update the next reward epoch.
        if (!views().isActiveValidatorForNextEpoch(realmId, stakerAddress)) {
            return;
        }

        // Update the next reward epoch and global stats
        LibStakingStorage.RewardEpoch storage nextRewardEpoch = _getRewardEpoch(
            stakerAddress,
            nextRewardEpochNumber
        );
        LibStakingStorage.RewardEpoch
            memory currentRewardEpoch = _getRewardEpoch(
                stakerAddress,
                currentRewardEpochNumber
            );

        nextRewardEpoch.epochEnd =
            block.timestamp +
            s().globalConfig[0].rewardEpochDuration;
        nextRewardEpoch.validatorSharePrice = currentRewardEpoch
            .validatorSharePrice;

        // Update the next reward epoch - stake weight carries over from current epoch
        if (currentRewardEpoch.totalStakeWeight > 0) {
            // Adjust the slope for the NEXT reward epoch number for any stake records that have started
            // unfreezing in the past.
            // 1. Get all the unfreeze starts up until this moment for this validator.
            uint256[] memory unfreezeStartsToProcess = _getUnfreezeStartsInPast(
                stakerAddress
            );

            // 2. Adjust the slope accordingly, taking into account the latest validatorSharePrice.
            uint256 rawSlope = 0;
            for (uint256 i = 0; i < unfreezeStartsToProcess.length; i++) {
                uint256 unfreezeStart = unfreezeStartsToProcess[i];
                uint256 slope = s().validatorToUnfreezeStartToSlope[
                    stakerAddress
                ][unfreezeStart];
                rawSlope += slope;
            }
            uint256 actualSlope = (rawSlope *
                currentRewardEpoch.validatorSharePrice) / PRECISION;
            nextRewardEpoch.slope += actualSlope;

            // 3. Flush the processed unfreeze starts and update the tracking data structures.
            for (uint256 i = 0; i < unfreezeStartsToProcess.length; i++) {
                uint256 unfreezeStart = unfreezeStartsToProcess[i];
                s().validatorToUnfreezeStarts[stakerAddress].remove(
                    unfreezeStart
                );
                s().validatorToUnfreezeStartToSlope[stakerAddress][
                    unfreezeStart
                ] = 0;
            }

            // remove from total stake weight the stake weights that have been unfrozen
            uint256 actualEpochLength = views()
                .getRewardEpochGlobalStats(currentRewardEpochNumber)
                .actualEpochLength;
            uint256 stakeWeightCarriedOver = currentRewardEpoch
                .totalStakeWeight -
                (currentRewardEpoch.slope * actualEpochLength);
            nextRewardEpoch.totalStakeWeight += stakeWeightCarriedOver;
            nextRewardEpochGlobalStats.stakeWeight += stakeWeightCarriedOver;
        }

        // Update the next reward epoch - slope is adjusted for next epoch depending on freezes that might have ended
        if (currentRewardEpoch.slope > 0) {
            // Adjust the slope for the StakeRecords that have finished unfreezing
            // 1. Get all the unfreeze schedules ending up until this moment for this validator.
            uint256[] memory unfreezeEndsToProcess = _getUnfreezeEndsInPast(
                stakerAddress
            );

            // 2. Adjust the slope accordingly, taking into account the latest validatorSharePrice.
            uint256 rawSlopeIncrease = 0;
            for (uint256 i = 0; i < unfreezeEndsToProcess.length; i++) {
                uint256 unfreezeEnd = unfreezeEndsToProcess[i];
                uint256 slopeIncrease = s()
                    .validatorToUnfreezeEndToSlopeIncrease[stakerAddress][
                        unfreezeEnd
                    ];
                rawSlopeIncrease += slopeIncrease;
            }
            uint256 shareDifference = (currentRewardEpoch.validatorSharePrice *
                StakingUtilsLib.PRECISION) / StakingUtilsLib.PRECISION; // This is the initial share price.
            uint256 actualSlopeIncrease = (rawSlopeIncrease * shareDifference) /
                StakingUtilsLib.PRECISION;

            // Handle rounding errors - if actualSlopeIncrease is within 5 wei of the current slope, we can
            // just set the slope to 0.
            if (
                (currentRewardEpoch.slope > actualSlopeIncrease &&
                    currentRewardEpoch.slope - actualSlopeIncrease <= 5) ||
                (currentRewardEpoch.slope < actualSlopeIncrease &&
                    actualSlopeIncrease - currentRewardEpoch.slope <= 5)
            ) {
                nextRewardEpoch.slope = 0;
            } else {
                nextRewardEpoch.slope =
                    currentRewardEpoch.slope -
                    actualSlopeIncrease;
            }

            // 3. Flush the processed unfreeze schedules and update the tracking data structures.
            for (uint256 i = 0; i < unfreezeEndsToProcess.length; i++) {
                uint256 unfreezeEnd = unfreezeEndsToProcess[i];
                s().validatorToUnfreezeEnds[stakerAddress].remove(unfreezeEnd);
                s().validatorToUnfreezeEndToSlopeIncrease[stakerAddress][
                    unfreezeEnd
                ] = 0;
            }
        }

        nextRewardEpoch.stakeAmount += currentRewardEpoch.stakeAmount;
        nextRewardEpochGlobalStats.stakeAmount += currentRewardEpoch
            .stakeAmount;
    }

    /// @notice Gets the unfreeze ends for a validator up until the current block.timestamp
    /// @param stakerAddress The stakerAddress of the validator whose unfreeze schedules are being fetched
    /// @return unfreezeEndsToProcess The unfreeze ends that have been reached for the validator
    function _getUnfreezeEndsInPast(
        address stakerAddress
    ) internal view returns (uint256[] memory) {
        uint256[] memory unfreezeEnds = s()
            .validatorToUnfreezeEnds[stakerAddress]
            .values();
        uint256[] memory unfreezeEndsToProcessWithZeroes = new uint256[](
            unfreezeEnds.length
        );
        uint256 unfreezeEndsToProcessIndex = 0;
        for (uint256 i = 0; i < unfreezeEnds.length; i++) {
            uint256 unfreezeEnd = unfreezeEnds[i];
            if (unfreezeEnd > block.timestamp) {
                continue;
            }
            unfreezeEndsToProcessWithZeroes[
                unfreezeEndsToProcessIndex
            ] = unfreezeEnd;
            unfreezeEndsToProcessIndex++;
        }

        // Prune out all elements from the first 0 and onwards.
        uint256 actualValues = 0;
        for (uint256 i = 0; i < unfreezeEndsToProcessWithZeroes.length; i++) {
            if (unfreezeEndsToProcessWithZeroes[i] == 0) {
                break;
            }
            actualValues++;
        }
        uint256[] memory unfreezeEndsToProcess = new uint256[](actualValues);
        for (uint256 i = 0; i < actualValues; i++) {
            unfreezeEndsToProcess[i] = unfreezeEndsToProcessWithZeroes[i];
        }
        return unfreezeEndsToProcess;
    }

    /// @notice Gets the unfreeze starts for a validator up until the current block.timestamp
    /// @param stakerAddress The stakerAddress of the validator whose unfreeze starts are being fetched
    /// @return unfreezeStartsToProcess The unfreeze starts that have been reached for the validator
    /// TODO: This should be able to be optimised since all unfreeze starts are added to the set in order,
    /// meaning we can use an array instead of a set.
    function _getUnfreezeStartsInPast(
        address stakerAddress
    ) internal view returns (uint256[] memory) {
        uint256[] memory unfreezeStarts = s()
            .validatorToUnfreezeStarts[stakerAddress]
            .values();
        uint256[] memory unfreezeStartsToProcessWithZeroes = new uint256[](
            unfreezeStarts.length
        );
        uint256 unfreezeStartsToProcessIndex = 0;
        for (uint256 i = 0; i < unfreezeStarts.length; i++) {
            uint256 unfreezeStart = unfreezeStarts[i];
            if (unfreezeStart > block.timestamp) {
                continue;
            }
            unfreezeStartsToProcessWithZeroes[
                unfreezeStartsToProcessIndex
            ] = unfreezeStart;
            unfreezeStartsToProcessIndex++;
        }

        // Prune out all elements from the first 0 and onwards.
        uint256 actualValues = 0;
        for (uint256 i = 0; i < unfreezeStartsToProcessWithZeroes.length; i++) {
            if (unfreezeStartsToProcessWithZeroes[i] == 0) {
                break;
            }
            actualValues++;
        }
        uint256[] memory unfreezeStartsToProcess = new uint256[](actualValues);
        for (uint256 i = 0; i < actualValues; i++) {
            unfreezeStartsToProcess[i] = unfreezeStartsToProcessWithZeroes[i];
        }
        return unfreezeStartsToProcess;
    }

    /**
     * @notice Updates the share price of a validator and adjusts the associated reward epoch statistics.
     * @param stakerAddress The stakerAddress of the validator whose share price is being updated.
     * @param newValidatorSharePrice The new share price of the validator.
     * @param oldValidatorSharePrice The old share price of the validator.
     * @dev Reverts if the new share price is greater than the old share price.
     */
    function updateSharePrice(
        uint256 rewardEpochNumber,
        address stakerAddress,
        uint256 newValidatorSharePrice,
        uint256 oldValidatorSharePrice
    ) internal {
        // Ensure the new share price is not greater than the old share price
        if (newValidatorSharePrice > oldValidatorSharePrice) {
            revert InvalidNewSharePrice();
        }

        LibStakingStorage.RewardEpoch memory rewardEpoch = staking()
            .getRewardEpoch(stakerAddress, rewardEpochNumber);
        LibStakingStorage.RewardEpochGlobalStats
            storage rewardEpochGlobalStats = getRewardEpochGlobalStats(
                rewardEpochNumber
            );

        uint256 shareDifference = (newValidatorSharePrice * PRECISION) /
            oldValidatorSharePrice;

        // Calculate reductions based on the difference in share price
        uint256 stakeWeightReduction = rewardEpoch.totalStakeWeight -
            (shareDifference * rewardEpoch.totalStakeWeight) /
            PRECISION;

        uint256 slopeReduction = rewardEpoch.slope -
            (shareDifference * rewardEpoch.slope) /
            PRECISION;

        // Update local copies of the global stats for the current reward epoch
        rewardEpochGlobalStats.stakeWeight -= stakeWeightReduction;

        // Update the validator's reward epoch with new share price and reduced values
        rewardEpoch.totalStakeWeight -= stakeWeightReduction;
        rewardEpoch.slope -= slopeReduction;
        rewardEpoch.validatorSharePrice = newValidatorSharePrice;
        rewardEpoch.validatorSharePriceAtLastUpdate = newValidatorSharePrice;

        // Write back the updated values to storage
        s().rewardEpochs[stakerAddress][rewardEpochNumber] = rewardEpoch;
    }

    /*
     * @notice Internal version of getRewardEpoch for returning a storage reference
     */
    function _getRewardEpoch(
        address stakerAddress,
        uint256 rewardEpochNumber
    ) internal returns (LibStakingStorage.RewardEpoch storage) {
        LibStakingStorage.RewardEpoch storage rewardEpoch = s().rewardEpochs[
            stakerAddress
        ][rewardEpochNumber];

        if (rewardEpoch.epochEnd == 0) {
            staking().initializeRewardEpoch(
                stakerAddress,
                rewardEpochNumber,
                false
            );
            rewardEpoch = s().rewardEpochs[stakerAddress][rewardEpochNumber];
        }

        return rewardEpoch;
    }

    /**
     * @notice Slashes a validator by reducing their share price by a given percentage.
     * @param percentage The percentage by which to slash the validator's share price.
     * @param stakerAddress The address of the staker
     * @return The amount of stake slashed.
     * @dev This function updates the validator's share price and returns the slashed stake amount.
     * @dev Due to our mechanism for slashing validators who are part of the pendingRejoins array (and have not rejoined
     * within the timeout), it is possible that this function is called against a validator that is already kicked. This
     * results in the realmId being fetched as 0. When this happens, we need to adjust the validator share price from the
     * last reward epoch number of the last realm that the validator was in in order to deduct the rewards they can claim.
     */

    function slashValidator(
        uint256 percentage,
        address stakerAddress
    ) internal returns (uint256) {
        // Check that the percentage is within the bounds of 0% and 100%.
        if (percentage > 1 ether) {
            revert InvalidSlashPercentage();
        }

        uint256 realmId = realms().getCurrentRealmIdForStakerAddress(
            stakerAddress
        );
        uint256 rewardEpochNumber;
        bool isSlashingKickedValidator = false;
        if (realmId == 0) {
            // This is the special case as mentioned in the comments above (slashing a validator who is already kicked)
            realmId = s().validators[stakerAddress].lastRealmId;
            rewardEpochNumber = s().validators[stakerAddress].lastRewardEpoch;

            // If this validator has never been in an active set, then we don't slash them (since there is nothing to penalize
            // them for).
            if (rewardEpochNumber == 0) {
                return 0;
            }

            isSlashingKickedValidator = true;
        } else {
            // This is the normal case
            rewardEpochNumber = mutableEpoch(realmId).rewardEpochNumber;
        }

        uint256 initialSharePrice = s()
        .rewardEpochs[stakerAddress][rewardEpochNumber].validatorSharePrice;

        if (initialSharePrice == 0) {
            revert("no share price");
        }
        uint256 newSharePrice = initialSharePrice -
            FixedPointMathLib.mulWad(initialSharePrice, percentage);
        updateSharePrice(
            rewardEpochNumber,
            stakerAddress,
            newSharePrice,
            initialSharePrice
        );

        // Furthermore, if we are slashing an already-kicked validator, we need to retroactively update just the reward epoch
        // and global stats for the reward epoch number in which they got kicked. There is no need to update the subsequent reward
        // epochs and global stats, since this kicked validator is not even in the active validator set to contribute to either.
        if (isSlashingKickedValidator) {
            rewardValidators(
                realmId,
                rewardEpochNumber,
                views().getRewardEpochGlobalStats(rewardEpochNumber)
            );
        }

        uint256 amountSlashed = FixedPointMathLib.mulWad(
            s().rewardEpochs[stakerAddress][rewardEpochNumber].stakeAmount,
            percentage
        );

        return amountSlashed;
    }

    function removeValidatorFromNextEpoch(
        uint256 realmId,
        address staker
    ) internal {
        // If the validator is not in the next epoch, then revert.
        if (!realm(realmId).validatorsInNextEpoch.contains(staker)) {
            revert ValidatorIsNotInNextEpoch(
                staker,
                realm(realmId).validatorsInNextEpoch.values()
            );
        }

        // remove them
        realm(realmId).validatorsInNextEpoch.remove(staker);
        LibStakingStorage.Validator memory validator = s().validators[staker];
        bytes32 commsKeysHash = keccak256(
            abi.encodePacked(validator.senderPubKey, validator.receiverPubKey)
        );
        realm(realmId).usedCommsKeys[commsKeysHash] = false;

        s().validatorToNextRealmId[staker] = 0;
        s().validators[staker].lastRealmId = realm(realmId).realmId;

        // If the validator is not in the current epoch, then we need to remove the attributions
        // that we have made eagerly (in _setupNextRewardEpoch) for this validator.
        if (!realm(realmId).validatorsInCurrentEpoch.contains(staker)) {
            uint256 nextRewardEpochNumber = mutableEpoch(realmId)
                .nextRewardEpochNumber;
            LibStakingStorage.RewardEpochGlobalStats
                storage nextGlobalStats = getRewardEpochGlobalStats(
                    nextRewardEpochNumber
                );
            LibStakingStorage.RewardEpoch
                storage nextRewardEpoch = _getRewardEpoch(
                    staker,
                    nextRewardEpochNumber
                );
            LibStakingStorage.StakeRecord[] memory stakeRecords = views()
                .getStakeRecordsForUser(staker, staker);

            for (uint256 i = 0; i < stakeRecords.length; i++) {
                LibStakingStorage.StakeRecord memory stakeRecord = stakeRecords[
                    i
                ];

                uint256 stakeWeight = views().getStakeWeightInEpoch(
                    staker,
                    stakeRecord.id,
                    staker,
                    nextRewardEpochNumber
                );

                nextGlobalStats.stakeWeight -= stakeWeight;
                nextGlobalStats.stakeAmount -= stakeRecord.amount;
                nextRewardEpoch.totalStakeWeight -= stakeWeight;
                nextRewardEpoch.stakeAmount -= stakeRecord.amount;
            }

            // Handle delegated stakes
            nextGlobalStats.stakeWeight -= validator.delegatedStakeWeight;
            nextGlobalStats.stakeAmount -= validator.delegatedStakeAmount;
            nextRewardEpoch.totalStakeWeight -= validator.delegatedStakeWeight;
            nextRewardEpoch.stakeAmount -= validator.delegatedStakeAmount;
        }
    }

    /// @notice Function to set up reward epochs for a staker. This is meant to be called
    /// at the beginning of the executeRequestToJoin function.
    /// @notice This function is NOT idempotent. Use carefully.
    function setupNextRewardEpoch(
        uint256 realmId,
        address stakerAddress,
        bool isInitial
    ) internal {
        uint256 nextRewardEpochNumber = mutableEpoch(realmId)
            .nextRewardEpochNumber;

        console.log(
            "Attributing values to next RE number %s",
            nextRewardEpochNumber
        );

        // Initialize the next reward epoch
        staking().initializeRewardEpoch(
            stakerAddress,
            nextRewardEpochNumber,
            isInitial
        );
        LibStakingStorage.RewardEpoch storage nextRewardEpoch = _getRewardEpoch(
            stakerAddress,
            nextRewardEpochNumber
        );
        LibStakingStorage.RewardEpochGlobalStats storage nextGlobalStats = s()
            .rewardEpochsGlobalStats[nextRewardEpochNumber];
        LibStakingStorage.StakeRecord[] memory stakeRecords = views()
            .getStakeRecordsForUser(stakerAddress, stakerAddress);

        // Attribute ALL of the vault's stake record information into the global stats and
        // reward epoch, since the assumption is that this function is called when a validator is
        // inactive and not part of any realm / validator set, and so when they request to join a particular
        // realm / validator set, all of their stake records will start earning rewards against that realm.
        for (uint256 i = 0; i < stakeRecords.length; i++) {
            LibStakingStorage.StakeRecord memory stakeRecord = stakeRecords[i];

            uint256 stakeWeight = views().getStakeWeightInEpoch(
                stakerAddress,
                stakeRecord.id,
                stakerAddress,
                nextRewardEpochNumber
            );

            nextGlobalStats.stakeWeight += stakeWeight;
            nextGlobalStats.stakeAmount += stakeRecord.amount;
            nextRewardEpoch.totalStakeWeight += stakeWeight;
            nextRewardEpoch.stakeAmount += stakeRecord.amount;
        }

        // Attribute ALL of the stake weights and amounts that have been delegated to this validator
        nextGlobalStats.stakeWeight += s()
            .validators[stakerAddress]
            .delegatedStakeWeight;
        nextGlobalStats.stakeAmount += s()
            .validators[stakerAddress]
            .delegatedStakeAmount;

        nextRewardEpoch.totalStakeWeight += s()
            .validators[stakerAddress]
            .delegatedStakeWeight;
        nextRewardEpoch.stakeAmount += s()
            .validators[stakerAddress]
            .delegatedStakeAmount;
    }

    /// @notice Checks that the stake amount is within the min and max stake amounts
    /// @param amount The amount to check
    /// @param isSelfStake Whether the stake is a self-stake
    function checkStakeAmountMinMax(
        uint256 amount,
        bool isSelfStake
    ) internal view {
        if (
            amount < s().globalConfig[0].minStakeAmount ||
            amount > s().globalConfig[0].maxStakeAmount
        ) {
            revert StakeAmountNotMet(amount);
        } else if (isSelfStake && amount < s().globalConfig[0].minSelfStake) {
            revert StakeAmountNotMet(amount);
        }
    }

    function checkStakeParameters(
        bool isSelfStake,
        uint256 timeLock
    ) internal view {
        LibStakingStorage.GlobalConfig memory globalConfig = s().globalConfig[
            0
        ];

        if (isSelfStake) {
            if (timeLock < globalConfig.minSelfStakeTimelock) {
                revert MinTimeLockNotMet(
                    timeLock,
                    globalConfig.minSelfStakeTimelock
                );
            }
        } else {
            uint256 minTimeLock = globalConfig.minTimeLock;
            if (timeLock < minTimeLock) {
                revert MinTimeLockNotMet(timeLock, minTimeLock);
            }
        }
        if (timeLock > globalConfig.maxTimeLock) {
            timeLock = globalConfig.maxTimeLock;
        }
    }

    // Helper function to calculate the minimum of two values
    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    // Helper function to calculate the maximum of two values
    function max(uint256 a, uint256 b) internal pure returns (uint256) {
        return a > b ? a : b;
    }

    /* ========== EVENTS ========== */

    event StateChanged(LibStakingStorage.States newState);
    event ValidatorKickedFromNextEpoch(address indexed staker);
    event ValidatorBanned(address indexed staker);
}
