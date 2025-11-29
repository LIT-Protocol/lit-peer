//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { StakingAcrossRealmsFacet } from "./StakingAcrossRealmsFacet.sol";
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
    error NoEmptyStakingSlot();

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
        // never let the network go below 3
        if (
            realm(realmId).validatorsInNextEpoch.length() <
            s().globalConfig[0].minimumValidatorCount
        ) {
            revert NotEnoughValidatorsInNextEpoch(
                realm(realmId).validatorsInNextEpoch.length(),
                s().globalConfig[0].minimumValidatorCount
            );
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

    struct CreateStakeRecordOpts {
        /// @notice Whether to (forcibly) target the current reward epoch. This is sometimes necessary
        /// for flows such as migrating stake, increasing stake record amount / timelock etc.
        bool targetCurrentRewardEpoch;
        /// @notice The last reward epoch claimed to set for the new stake record. If this is set to 0,
        /// it means that the caller is relying on this function to determine the correct value to be set accordingly.
        uint256 lastRewardEpochClaimedToSet;
        /// @notice The start time of the unfreezing process. If this is non-zero, this means that this stake record
        /// is created in order to resume the unfreezing process from a previously removed stake record, eg. for flows
        /// such as migrating an already unfreezing stake record to a new validator.
        uint256 unfreezeStartToSet;
        /// @notice The token ID of the NFT that the stake record is associated with.
        uint256 tokenIdToSet;
    }

    /**
     * @notice Create stake record in staker vault
     * @param stakeAmount The stake amount
     * @param timeLock The stake time lock in seconds
     * @param userStakerAddress The staker address
     * @param stakerAddress address of the main staker for the node
     * @param opts The options for the new stake record
     * @dev Reverts if there is no empty staking slot
     */
    function _createStakeRecord(
        uint256 stakeAmount,
        uint256 timeLock,
        address stakerAddress,
        address userStakerAddress,
        CreateStakeRecordOpts memory opts
    ) internal returns (uint256) {
        LibStakingStorage.StakerVault storage userVault = s().vaults[
            stakerAddress
        ][userStakerAddress];
        uint256 freeSlotIndex = 2 ** 256 - 1; // max uint
        for (uint256 i = 0; i < userVault.stakes.length; i++) {
            if (!userVault.stakes[i].loaded) {
                freeSlotIndex = i;
                break;
            }
        }
        if (freeSlotIndex == 2 ** 256 - 1) {
            revert NoEmptyStakingSlot();
        }

        timeLock = timeLock > s().globalConfig[0].maxTimeLock
            ? s().globalConfig[0].maxTimeLock
            : timeLock;
        // Round down to the nearest day
        timeLock = (timeLock / 1 days) * 1 days;

        userVault.lastStakeRecordId += 1;
        LibStakingStorage.StakeRecord memory stakeRecord = LibStakingStorage
            .StakeRecord({
                id: userVault.lastStakeRecordId,
                amount: stakeAmount,
                unfreezeStart: opts.unfreezeStartToSet,
                timeLock: timeLock,
                lastUpdateTimestamp: block.timestamp,
                lastRewardEpochClaimed: opts.lastRewardEpochClaimedToSet,
                loaded: true,
                frozen: opts.unfreezeStartToSet == 0,
                // This is fine to always be set to PRECISION. The validator share price only ever decreases
                // when a validator gets slashed, however, when a validator gets slashed, they are perma-banned
                // from joining any realm / validator set, and pretty much the only way for that validator to
                // re-join any realm is via admin intervention by reducing their demerit counters. When this
                // validator proceeds to re-join a realm, their slate is "wiped clean" and they should have a
                // validator share price reset to PRECISION initially. Hence, there is no need to set this
                // initialSharePrice to anything else other than PRECISION always.
                initialSharePrice: StakingUtilsLib.PRECISION,
                attributionAddress: address(0),
                tokenId: opts.tokenIdToSet
            });

        // There is actually a bit of a hack we can use to calculate the stake weight to add to
        // the validator's buffer without knowing exactly which reward epoch number will be used
        // in the future (this is preferred for simplicity). Within StakingViewsFacet.getTimelockInEpoch,
        // the purpose of needing the reward epoch number is to compare `RewardEpoch.epochEnd` with
        // `StakeRecord.unfreezeStart`. However, since this is an entirely new stake record, the timelock we should
        // be using is simply the timelock that we have here. Similarly, in StakingViewsFacet.getTokensStaked, the
        // purpose of needing the reward epoch number is to get the `RewardEpoch.validatorSharePrice`. Since all non-slashed-and-active,
        // joining, or yet-to-join validators will always have the initialized (not penalized) validator share price (at 1 ether), the
        // stake amount we should be using is simply the stake amount that we have here. This means that the stake weight calculation
        // can be done directly using `StakingViewsFacet.calculateStakeWeight`.
        uint256 stakeWeight = views().calculateStakeWeight(
            timeLock,
            stakeAmount
        );

        // If validator is in current or next set, then we already know the next reward epoch which
        // will need to be updated, so we proceed to update right away.
        // If validator is not in current or next set, then we should update GS and RE once we know which
        // realmId the validator is joining, which we should be able to do conveniently inside the
        // requestToJoin (or related) functions.
        uint256 realmId = realms().getRealmIdForStakerAddress(stakerAddress);
        if (realmId != 0) {
            uint256 rewardEpochNumberToUpdate;
            if (opts.targetCurrentRewardEpoch) {
                rewardEpochNumberToUpdate = mutableEpoch(realmId)
                    .rewardEpochNumber;
            } else {
                rewardEpochNumberToUpdate = mutableEpoch(realmId)
                    .nextRewardEpochNumber;
            }

            LibStakingStorage.RewardEpoch storage rewardEpoch = StakingUtilsLib
                ._getRewardEpoch(stakerAddress, rewardEpochNumberToUpdate);

            // If lastRewardEpochClaimedToSet is set to 0, we need to be setting the lastRewardEpochClaimed
            // to be (rewardEpochNumberToUpdate - 1)
            if (opts.lastRewardEpochClaimedToSet == 0) {
                stakeRecord.lastRewardEpochClaimed =
                    rewardEpochNumberToUpdate -
                    1;
            }

            userVault.stakes[freeSlotIndex] = (stakeRecord);

            LibStakingStorage.RewardEpochGlobalStats storage globalStats = s()
                .rewardEpochsGlobalStats[rewardEpochNumberToUpdate];
            globalStats.stakeWeight += stakeWeight;
            globalStats.stakeAmount += stakeAmount;

            rewardEpoch.totalStakeWeight += stakeWeight;
            rewardEpoch.stakeAmount += stakeAmount;
        } else {
            // If lastRewardEpochClaimedToSet is set to 0, we need to be setting the lastRewardEpochClaimed to be the
            // lowest reward epoch number across all realms. Why does this work? Well, if we are here, it means that the validator
            // is not yet part of any realm, and since reward epoch numbers are monotonically increasing across all realms, setting
            // the lastRewardEpochClaimed to the currently lowest reward epoch number across all realms will ensure that this value
            // is high enough to prevent the validator (or any of its delegated stakes) from being able to illegally double-claim
            // rewards for historical epochs, and be low enough to ensure that the validator will be able to earn rewards as soon as
            // they join any realm in the future. Note that we do not have to make the reward epoch number have exactly correct semantics
            // here (validator ultimately joins a realm at epoch X so this parameter must be set to X-1), and setting the value this way
            // allows us to keep the logic sufficiently simple while achieving the same desired outcome.
            // Note that this assumes that the epoch progression is not delayed and always advances as expected (or in a timely manner). If
            // this is not the case, then there is a possibility that this parameter is not set high enough and the staker is still able to
            // claim rewards for historical epochs more than once.
            if (opts.lastRewardEpochClaimedToSet == 0) {
                (, stakeRecord.lastRewardEpochClaimed) = views()
                    .getLowestRewardEpochNumber();
            }
        }

        // Handle delegated stakes
        if (userStakerAddress != stakerAddress) {
            // We need to attribute the stake amount and weight to the validator's struct.
            stakeRecord.attributionAddress = stakerAddress;
            s().validators[stakerAddress].delegatedStakeAmount += stakeAmount;
            s().validators[stakerAddress].delegatedStakeWeight += stakeWeight;
        }

        userVault.stakes[freeSlotIndex] = (stakeRecord);

        emit StakeRecordCreated(
            stakerAddress,
            stakeRecord.id,
            stakeAmount,
            userStakerAddress
        );

        return stakeRecord.id;
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
    event StakeRecordCreated(
        address stakerAddress,
        uint256 recordId,
        uint256 amount,
        address stakerAddressClient
    );
}
