//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ERC20Burnable } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { StakingCommon } from "./StakingCommon.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { ERC2771 } from "../../common/ERC2771.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { EnumerableSetViewFriendly } from "@lit-protocol/openzeppelin-contracts/utils/structs/EnumerableSetViewFriendly.sol";
import { console } from "hardhat/console.sol";

contract StakingFacet is StakingCommon, ERC2771 {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.UintSet;
    using EnumerableSetViewFriendly for EnumerableSetViewFriendly.AddressSet;

    error StakeAmountNotMet(uint256 amount);
    error TooSoonToWithdraw();
    error CannotWithdrawFrozen();
    error TimeLockNotMet();
    error CheckpointAheadOfCurrentEpoch(
        uint256 checkpoint,
        uint256 currentEpoch
    );
    error NewTimeLockMustBeGreaterThanCurrent();
    error CannotMigrateFromValidator();
    error RewardsMustBeClaimed();
    error InsufficientSelfStake(address stakerAddress);
    error InvalidRatio();
    error ValidatorNotRegistered(address validatorAddress);
    error CallerNotContract();
    error StakeMustBeGreaterThanMinimumStake(
        address stakerAddress,
        uint256 stakedAmount,
        uint256 minimumStake
    );
    error SlashingMustOccurInSameRealm(
        address slahedAddress,
        uint256 slashedRealmId,
        address senderAddress,
        uint256 senderRealmId
    );

    /* ========== MUTABLE FUNCTIONS ========== */

    /**
     * @notice Stake funds to a specific validator for a certain amount of time
     * @param amount The amount to stake
     * @param timeLock The amount of time to lock for
     * @dev Reverts if the time lock is less than the minimum time lock
     * @dev Reverts if the amount is less than the minimum stake amount or greater than the maximum stake amount
     * @dev Sets the time lock to the maximum time lock if it is greater than the maximum time lock
     */
    function stake(
        uint256 amount,
        uint256 timeLock,
        address operatorStakerAddress
    ) public {
        bool isSelfStake = LibERC2771._msgSender() == operatorStakerAddress;
        address userStakerAddress = (isSelfStake)
            ? operatorStakerAddress
            : LibERC2771._msgSender();

        StakingUtilsLib.checkStakeParameters(isSelfStake, timeLock);
        StakingUtilsLib.checkStakeAmountMinMax(amount, isSelfStake);
        _stake(amount, userStakerAddress);

        _createStakeRecord(
            amount,
            timeLock,
            operatorStakerAddress,
            userStakerAddress,
            CreateStakeRecordOpts({
                targetCurrentRewardEpoch: false,
                lastRewardEpochClaimedToSet: 0, // setting this to zero relies in the inner function to determine what this value should be
                unfreezeStartToSet: 0
            })
        );

        // if staking is done by a validator add them to the allValidators set
        if (isSelfStake) {
            if (!s().allValidators.contains(operatorStakerAddress)) {
                s().allValidators.add(operatorStakerAddress);
            }
        }

        // If this is the first stake this delegator has against this validator, then increment the unique delegating staker count.
        if (
            (userStakerAddress != operatorStakerAddress) &&
            (views().getStakeRecordCount(
                userStakerAddress,
                operatorStakerAddress
            ) == 1)
        ) {
            s().validators[operatorStakerAddress].uniqueDelegatingStakerCount++;
        }

        s().stakerToValidatorsTheyStakedTo[userStakerAddress].add(
            operatorStakerAddress
        );
    }

    /**
     * Unfreeze stake and start to decrease the stake weight
     * @param operatorStakerAddress address of the staker who has this record
     * @param stakeId ID of the stake
     */
    function unfreezeStake(
        address operatorStakerAddress,
        uint256 stakeId
    ) public {
        _unfreezeStake(LibERC2771._msgSender(), operatorStakerAddress, stakeId);
    }

    /**
     * @notice Get the reward epoch for a staker
     * @param stakerAddress The address of the staker
     * @param rewardEpochNumber The number of the reward epoch
     * @return rewardEpoch The reward epoch
     */
    function getRewardEpochView(
        address stakerAddress,
        uint256 rewardEpochNumber
    ) public view returns (LibStakingStorage.RewardEpoch memory) {
        return s().rewardEpochs[stakerAddress][rewardEpochNumber];
    }

    function getRewardEpoch(
        address stakerAddress,
        uint256 rewardEpochNumber
    ) public returns (LibStakingStorage.RewardEpoch memory) {
        LibStakingStorage.RewardEpoch memory rewardEpoch = s().rewardEpochs[
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
     * @notice Initialize reward epoch for a staker
     * @param stakerAddress The address of the staker
     * @param rewardEpochNumber The number of the reward epoch
     * @dev If the slope is already set, this function does NOT override it
     */
    function initializeRewardEpoch(
        address stakerAddress,
        uint256 rewardEpochNumber,
        bool isInitial
    ) public {
        require(
            LibERC2771._msgSender() == address(this),
            "Only the staking contract can initialize reward epochs"
        );

        console.log(
            "Initializing reward epoch %s for staker %s",
            rewardEpochNumber,
            stakerAddress
        );

        LibStakingStorage.RewardEpoch memory existingRewardEpoch = s()
            .rewardEpochs[stakerAddress][rewardEpochNumber];

        s().rewardEpochs[stakerAddress][rewardEpochNumber] = LibStakingStorage
            .RewardEpoch({
                epochEnd: block.timestamp +
                    s().globalConfig[0].rewardEpochDuration,
                totalStakeWeight: 0,
                totalStakeRewards: 0,
                validatorFixedCostRewards: 0,
                validatorCommission: 0,
                // The slope may have been set prior to this initialization, so let's
                // not override it if it is already set.
                slope: existingRewardEpoch.slope,
                validatorSharePrice: StakingUtilsLib.PRECISION,
                validatorSharePriceAtLastUpdate: StakingUtilsLib.PRECISION,
                stakeAmount: 0,
                initial: isInitial
            });
    }

    struct RemoveStakeRecordParams {
        address stakerAddress;
        uint256 stakeRecordId;
        address userStakerAddress;
        bool requireDecayed;
        /// @dev If this is address(0), then this stake record is not being removed as part of a migration.
        /// @dev If this is NOT address(0), then this stake record is being removed as part of a migration,
        /// and the new staker address is recorded here.
        address migrateToStakerAddress;
    }

    /**
     * @notice Remove stake record from staker vault
     * @param params The parameters for the remove stake record operation
     * @param params.stakerAddress address of the staker who has this record
     * @param params.stakeRecordId The record id of the stake
     * @param params.userStakerAddress The staker address of an end user who has delegated to the staker
     * @param params.migrateToStakerAddress The staker address to migrate the stake record to, if any
     * @return withdrawAmount The amount to withdraw
     * @dev Reverts if the stake record is frozen
     * @dev Reverts if the time lock has not been met
     * @dev Reverts if the rewards have not been claimed for the stake record
     */
    function removeStakeRecord(
        RemoveStakeRecordParams memory params
    ) internal returns (uint256 withdrawAmount) {
        LibStakingStorage.StakerVault storage userVault = s().vaults[
            params.stakerAddress
        ][params.userStakerAddress]; // get vault
        uint256 stakeRecordIdx;
        for (uint256 i = 0; i < userVault.stakes.length; i++) {
            if (userVault.stakes[i].id == params.stakeRecordId) {
                stakeRecordIdx = i;
                break;
            } else if (i == userVault.stakes.length - 1) {
                revert StakingUtilsLib.StakeRecordNotFound(
                    params.stakeRecordId
                );
            }
        }

        uint256 realmId = realms().getRealmIdForStakerAddress(
            params.stakerAddress
        );
        uint256 rewardEpochNumber;
        // If the realmId is 0, then the staker is not in any realm and we need to use the last realmId and last reward epoch number
        if (realmId == 0) {
            if (s().validators[params.stakerAddress].lastRealmId == 0) {
                (
                    uint256 realmIdWithLowestRewardEpochNumber,
                    uint256 currentRewardEpochNumber
                ) = views().getLowestRewardEpochNumber();
                realmId = realmIdWithLowestRewardEpochNumber;
                rewardEpochNumber = currentRewardEpochNumber;
            } else {
                realmId = s().validators[params.stakerAddress].lastRealmId;
                rewardEpochNumber = s()
                    .validators[params.stakerAddress]
                    .lastRewardEpoch;
            }
        } else {
            rewardEpochNumber = mutableEpoch(realmId).rewardEpochNumber;
        }

        LibStakingStorage.StakeRecord memory stakeRecord = userVault.stakes[
            stakeRecordIdx
        ];

        uint256 quantisedUnfreezeEnd = ((stakeRecord.unfreezeStart +
            stakeRecord.timeLock) / 1 days) * 1 days;

        if (params.requireDecayed) {
            if (stakeRecord.frozen) {
                revert CannotWithdrawFrozen();
            }

            if (
                (block.timestamp < stakeRecord.unfreezeStart) ||
                (block.timestamp - stakeRecord.unfreezeStart <
                    stakeRecord.timeLock)
            ) {
                revert TimeLockNotMet();
            }

            /// NOTE: We do NOT use the latest reward epoch number of any particular realm's epoch because
            /// the epoch may have advanced before this code block gets reached.
            if (
                stakeRecord.lastRewardEpochClaimed <
                s().validators[params.stakerAddress].lastRewardEpoch
            ) {
                revert RewardsMustBeClaimed();
            }
        }

        uint256 actualSharePrice = (getRewardEpoch(
            params.stakerAddress,
            rewardEpochNumber
        ).validatorSharePrice * StakingUtilsLib.PRECISION) /
            stakeRecord.initialSharePrice;

        uint256 slopeWithoutSharePrice = views().calculateStakeWeight(
            stakeRecord.timeLock,
            stakeRecord.amount
        ) / stakeRecord.timeLock;

        // If the unfreezeStart is still in the validatorToUnfreezeStarts set, then that means the validator never
        // got to process the unfreeze during an epoch advancement and this removal of the stake record is already
        // happening. The validator may either active or inactive. Either way, we need to remove the unfreezeStart
        // and the unfreezeEnd from the validatorToUnfreezeStarts and validatorToUnfreezeEnds sets. We purposely
        // DO NOT care about the validator share price here since it is as if the validator never got to see this
        // stake record. If this is part of a migration, we add the slope and slope increase to the mappings for the
        // new staker.
        //
        // Else, if the unfreezeEnd is still in the validatorToUnfreezeEnds set, then that means the validator never
        // got to process the unfreeze end during an epoch advancement and this removal of the stake record is already
        // happening. Similar to the above, the validator may be either active or inactive, but there is a difference
        // here if the validator is currently active. Since the unfreeze start DID get processed, we need to immediately
        // adjust the slope of the validator reward epoch to be less. For both cases though, we need to remove the
        // unfreezeEnd from the validatorToUnfreezeEnds set. The difference here is that we DO care about the validator
        // share price for determining the slope increase. In cases where the validator is slashed, the validator share
        // price and slope is immediately adjusted to be less. So, we need to correspondingly adjust the slope increase
        // to be less as well to match the adjusted slope. If this is part of a migration and the validator is inactive,
        // we need to add the slope immediately to the new staker, and add the slope increase to the mapping for the new
        // staker.
        if (
            s().validatorToUnfreezeStarts[params.stakerAddress].contains(
                stakeRecord.unfreezeStart
            )
        ) {
            // Deduct the slope from the validatorToUnfreezeStartToSlope mapping.
            s().validatorToUnfreezeStartToSlope[params.stakerAddress][
                stakeRecord.unfreezeStart
            ] -= slopeWithoutSharePrice;

            // If the slope is now 0, we can also remove the key altogether.
            if (
                s().validatorToUnfreezeStartToSlope[params.stakerAddress][
                    stakeRecord.unfreezeStart
                ] == 0
            ) {
                s().validatorToUnfreezeStarts[params.stakerAddress].remove(
                    stakeRecord.unfreezeStart
                );
            }

            // Deduct the slope from the slope increase for the unfreeze end.
            s().validatorToUnfreezeEndToSlopeIncrease[params.stakerAddress][
                quantisedUnfreezeEnd
            ] -= slopeWithoutSharePrice;

            // If the slope increase is now 0, we can also remove the key altogether.
            if (
                s().validatorToUnfreezeEndToSlopeIncrease[params.stakerAddress][
                    quantisedUnfreezeEnd
                ] == 0
            ) {
                s().validatorToUnfreezeEnds[params.stakerAddress].remove(
                    quantisedUnfreezeEnd
                );
            }

            // Regardless of whether the validator is active or inactive, if this is part of a migration, we need to add the slope
            // and slope increase to the mappings for the new staker.
            if (params.migrateToStakerAddress != address(0)) {
                s().validatorToUnfreezeStartToSlope[
                    params.migrateToStakerAddress
                ][stakeRecord.unfreezeStart] += slopeWithoutSharePrice;
                s()
                    .validatorToUnfreezeStarts[params.migrateToStakerAddress]
                    .add(stakeRecord.unfreezeStart);

                s().validatorToUnfreezeEndToSlopeIncrease[
                    params.migrateToStakerAddress
                ][quantisedUnfreezeEnd] += slopeWithoutSharePrice;
                s().validatorToUnfreezeEnds[params.migrateToStakerAddress].add(
                    quantisedUnfreezeEnd
                );
            }
        } else if (
            s().validatorToUnfreezeEnds[params.stakerAddress].contains(
                quantisedUnfreezeEnd
            )
        ) {
            {
                // Recover the slope that was added to the validatorToUnfreezeEndToSlopeIncrease mapping.
                // The elaborate logic is to avoid rounding errors as much as possible, though it is still
                // not completely avoidable.
                uint256 actualSlope;
                {
                    if (actualSharePrice == StakingUtilsLib.PRECISION) {
                        actualSlope = slopeWithoutSharePrice;
                    } else if (slopeWithoutSharePrice % 2 == 0) {
                        actualSlope =
                            (actualSharePrice * slopeWithoutSharePrice) /
                            StakingUtilsLib.PRECISION;
                    } else {
                        actualSlope =
                            ((actualSharePrice * slopeWithoutSharePrice) /
                                StakingUtilsLib.PRECISION) -
                            1;
                    }
                }

                // Deduct the slope from the validatorToUnfreezeEndToSlopeIncrease mapping.
                s().validatorToUnfreezeEndToSlopeIncrease[params.stakerAddress][
                    quantisedUnfreezeEnd
                ] -= actualSlope;

                // If the slope increase is now 0, we can also remove the key altogether.
                if (
                    s().validatorToUnfreezeEndToSlopeIncrease[
                        params.stakerAddress
                    ][quantisedUnfreezeEnd] == 0
                ) {
                    s().validatorToUnfreezeEnds[params.stakerAddress].remove(
                        quantisedUnfreezeEnd
                    );
                }
            }

            if (params.migrateToStakerAddress != address(0)) {
                uint256 newStakerRealmId = realms().getRealmIdForStakerAddress(
                    params.migrateToStakerAddress
                );
                // If the new validator is active, add slopeWithoutSharePrice to the new validator's reward epoch.
                // If the new validator is inactive, add the unfreeze start so it is picked up on their next epoch advancement.
                if (newStakerRealmId != 0) {
                    LibStakingStorage.RewardEpoch
                        storage newStakerRewardEpoch = s().rewardEpochs[
                            params.migrateToStakerAddress
                        ][mutableEpoch(newStakerRealmId).rewardEpochNumber];
                    newStakerRewardEpoch.slope += slopeWithoutSharePrice;
                } else {
                    s()
                        .validatorToUnfreezeStarts[
                            params.migrateToStakerAddress
                        ]
                        .add(stakeRecord.unfreezeStart);
                    s().validatorToUnfreezeStartToSlope[
                        params.migrateToStakerAddress
                    ][stakeRecord.unfreezeStart] += slopeWithoutSharePrice;
                }

                // Add the slope increase to the mapping for the new staker.
                s().validatorToUnfreezeEndToSlopeIncrease[
                    params.migrateToStakerAddress
                ][quantisedUnfreezeEnd] += slopeWithoutSharePrice;
                s().validatorToUnfreezeEnds[params.migrateToStakerAddress].add(
                    quantisedUnfreezeEnd
                );
            }

            // If this validator is active, we need to reduce the slope of the active validator.
            if (realms().isValidatorInCurrentEpoch(params.stakerAddress)) {
                LibStakingStorage.RewardEpoch
                    storage activeValidatorRewardEpoch = s().rewardEpochs[
                        params.stakerAddress
                    ][rewardEpochNumber];
                activeValidatorRewardEpoch.slope -= slopeWithoutSharePrice;
            }
        }

        withdrawAmount =
            (stakeRecord.amount * actualSharePrice) /
            StakingUtilsLib.PRECISION;

        // Only adjust staking details if the validator is part of the current validator set. Otherwise,
        // the reward epoch for this staker is not even initalized and populated with the correct values.
        // For the withdraw case, the validator needs to leave any active validator set before being able
        // to withdraw, and by not being part of any active validator set, less is contributed to the reward
        // epochs and global stats already. However, we explicitly need to decrement the reward epoch and global
        // stats here because the validator is still part of an active validator set.
        if (
            !params.requireDecayed &&
            realm(realmId).validatorsInCurrentEpoch.contains(
                params.stakerAddress
            )
        ) {
            uint256 stakeWeight = views().getStakeWeightInEpoch(
                params.stakerAddress,
                stakeRecord.id,
                params.userStakerAddress,
                rewardEpochNumber
            );
            s()
            .rewardEpochs[params.stakerAddress][rewardEpochNumber]
                .totalStakeWeight -= stakeWeight;
            s()
            .rewardEpochs[params.stakerAddress][rewardEpochNumber]
                .stakeAmount -= stakeRecord.amount;
            s()
                .rewardEpochsGlobalStats[rewardEpochNumber]
                .stakeWeight -= stakeWeight;
            s()
                .rewardEpochsGlobalStats[rewardEpochNumber]
                .stakeAmount -= stakeRecord.amount;
        }

        // Handle delegated stakes in the requireDecayed case by deducting the staking details from the reward epoch and global stats.
        // We do not need to deduct the stake weight because in the requireDecayed case, the delegated staker would have needed to
        // fully thaw their unfrozen stake before being able to withdraw. By unfreezing, their stake weight contribution to each successive
        // reward epoch and global stats would have been decaying towards 0 too, so by this time of this removal, the stake weight will be
        // APPROXIMATELY zero-ed out.
        if (
            params.requireDecayed &&
            (params.userStakerAddress != params.stakerAddress)
        ) {
            // Sometimes we hit this scenario for a dummy / irrelevant reward epoch number so we need to check for underflow errors.
            if (
                s()
                .rewardEpochs[params.stakerAddress][rewardEpochNumber]
                    .stakeAmount >= stakeRecord.amount
            ) {
                s()
                .rewardEpochs[params.stakerAddress][rewardEpochNumber]
                    .stakeAmount -= stakeRecord.amount;
            }

            if (
                s().rewardEpochsGlobalStats[rewardEpochNumber].stakeAmount >=
                stakeRecord.amount
            ) {
                s()
                    .rewardEpochsGlobalStats[rewardEpochNumber]
                    .stakeAmount -= stakeRecord.amount;
            }
        }

        // If the stake record has an attribution address, we need to be adjusting the validator's buffer fields accordingly
        // to be less.
        address attributionAddress = stakeRecord.attributionAddress;
        if (
            attributionAddress != address(0) &&
            s().validators[attributionAddress].delegatedStakeAmount > 0
        ) {
            s()
                .validators[attributionAddress]
                .delegatedStakeAmount -= stakeRecord.amount;
            s().validators[attributionAddress].delegatedStakeWeight -= views()
                .calculateStakeWeight(stakeRecord.timeLock, stakeRecord.amount);
        }

        userVault.stakes[stakeRecordIdx] = LibStakingStorage.StakeRecord({
            id: 0,
            amount: 0,
            unfreezeStart: 0,
            timeLock: 0,
            lastUpdateTimestamp: 0,
            lastRewardEpochClaimed: 0,
            loaded: false,
            frozen: false,
            initialSharePrice: 0,
            attributionAddress: address(0)
        });

        emit StakeRecordRemoved(params.stakerAddress, params.stakeRecordId);
    }

    /**
     * @notice Increase the time lock of a stake record
     * @param stakerAddress address of the staker who has this record
     * @param stakeRecordId The stake record ID
     * @param additionalTimeLock The time lock to add to the current time lock
     * @dev Reverts if the new time lock is less than the minimum time lock or greater than the maximum time lock
     * @dev Reverts if the rewards have not been claimed for the stake record
     * @dev Reverts if stake record is not frozen
     * @dev Reverts if the new time lock is less than the current time lock
     */
    function increaseStakeRecordTimelock(
        address stakerAddress,
        uint256 stakeRecordId,
        uint256 additionalTimeLock
    ) public {
        LibStakingStorage.StakeRecord memory stakeRecord = views()
            .getStakeRecord(
                stakerAddress,
                stakeRecordId,
                LibERC2771._msgSender()
            );

        if (!stakeRecord.frozen) {
            revert CannotModifyUnfrozen();
        }

        /// NOTE: We do NOT use the latest reward epoch number of any particular realm's epoch because
        /// the epoch may have advanced before this code block gets reached.
        if (
            stakeRecord.lastRewardEpochClaimed <
            s().validators[stakerAddress].lastRewardEpoch
        ) {
            revert RewardsMustBeClaimed();
        }

        uint256 newTimeLock = stakeRecord.timeLock + additionalTimeLock;
        if (
            newTimeLock < s().globalConfig[0].minTimeLock ||
            newTimeLock > s().globalConfig[0].maxTimeLock
        ) {
            revert TimeLockNotMet();
        }
        if (stakeRecord.timeLock >= newTimeLock) {
            revert NewTimeLockMustBeGreaterThanCurrent();
        }

        uint256 amountLeft = removeStakeRecord(
            RemoveStakeRecordParams({
                stakerAddress: stakerAddress,
                stakeRecordId: stakeRecordId,
                userStakerAddress: LibERC2771._msgSender(),
                requireDecayed: false,
                migrateToStakerAddress: address(0)
            })
        );

        _createStakeRecord(
            amountLeft,
            newTimeLock,
            stakerAddress,
            LibERC2771._msgSender(),
            CreateStakeRecordOpts({
                targetCurrentRewardEpoch: true,
                lastRewardEpochClaimedToSet: stakeRecord.lastRewardEpochClaimed,
                unfreezeStartToSet: 0
            })
        );
    }

    /**
     * @notice Increase the stake amount of a stake record
     * @param stakerAddress address of the staker who has this record
     * @param stakeRecordId The stake record ID
     * @param additionalAmount The amount to increase the stake record by
     * @dev Reverts if the stake record is not frozen
     * @dev Reverts if the rewards have not been claimed for the stake record
     */
    function increaseStakeRecordAmount(
        address stakerAddress,
        uint256 stakeRecordId,
        uint256 additionalAmount
    ) public {
        LibStakingStorage.StakeRecord memory stakeRecord = views()
            .getStakeRecord(
                stakerAddress,
                stakeRecordId,
                LibERC2771._msgSender()
            );

        if (!stakeRecord.frozen) {
            revert CannotModifyUnfrozen();
        }

        /// NOTE: We do NOT use the latest reward epoch number of any particular realm's epoch because
        /// the epoch may have advanced before this code block gets reached.
        if (
            stakeRecord.lastRewardEpochClaimed <
            s().validators[stakerAddress].lastRewardEpoch
        ) {
            revert RewardsMustBeClaimed();
        }

        _stake(additionalAmount, LibERC2771._msgSender());
        uint256 timeLock = stakeRecord.timeLock;
        uint256 amountLeft = removeStakeRecord(
            RemoveStakeRecordParams({
                stakerAddress: stakerAddress,
                stakeRecordId: stakeRecordId,
                userStakerAddress: LibERC2771._msgSender(),
                requireDecayed: false,
                migrateToStakerAddress: address(0)
            })
        );

        uint256 newAmount = amountLeft + additionalAmount;
        bool isSelfStake = stakerAddress == LibERC2771._msgSender();
        StakingUtilsLib.checkStakeAmountMinMax(newAmount, isSelfStake);

        _createStakeRecord(
            newAmount,
            timeLock,
            stakerAddress,
            LibERC2771._msgSender(),
            CreateStakeRecordOpts({
                targetCurrentRewardEpoch: true,
                lastRewardEpochClaimedToSet: stakeRecord.lastRewardEpochClaimed,
                unfreezeStartToSet: 0
            })
        );
    }

    /**
     * @notice Claims the reward from specifc stake record in the staker's vault
     * @param realmId The ID of the realm
     * @param stakerAddress The staker address
     * @param stakeRecordId The stake record ID to claim rewards from
     * @param maxNumberOfEpochsToClaim The user-defined maximum number of epochs to claim rewards for
     * @dev Reverts if the checkpoint epoch is ahead of the current epoch
     */
    function claimStakeRewards(
        uint256 realmId,
        address stakerAddress,
        uint256 stakeRecordId,
        uint256 maxNumberOfEpochsToClaim
    ) external nonReentrant {
        LibStakingStorage.StakeRecord storage stakeRecord = StakingUtilsLib
            .getStakeRecord(
                stakerAddress,
                stakeRecordId,
                LibERC2771._msgSender()
            );
        uint256 cumulativeReward = 0;
        uint256 currentRewardEpochNumber = mutableEpoch(realmId)
            .rewardEpochNumber;
        if (maxNumberOfEpochsToClaim == 0) {
            // Defaults to 50 epochs if no max is provided. 50 provides a reasonable bound such that
            // this function would not exceed the block gas limit.
            maxNumberOfEpochsToClaim = 50;
        }
        uint256 startEpochCheckpoint = stakeRecord.lastRewardEpochClaimed;
        if (currentRewardEpochNumber < startEpochCheckpoint) {
            return;
            // lets just return for now revert CheckpointAheadOfCurrentEpoch(startEpochCheckpoint, currentRewardEpochNumber);
        }
        uint256 nextEpochCheckpoint = currentRewardEpochNumber -
            startEpochCheckpoint >
            maxNumberOfEpochsToClaim
            ? startEpochCheckpoint + maxNumberOfEpochsToClaim
            : currentRewardEpochNumber;
        uint256 lastRewardEpochClaimed = nextEpochCheckpoint;

        for (uint256 i = startEpochCheckpoint; i < nextEpochCheckpoint; i++) {
            LibStakingStorage.RewardEpoch memory currentRewardEpoch = s()
                .rewardEpochs[stakerAddress][i];

            // If there are no rewards to claim, then we can skip this epoch.
            if (currentRewardEpoch.totalStakeRewards == 0) {
                continue;
            }

            // If this validator is on the pending rejoins watchlist and this reward epoch has an epoch ending time that
            // is more recent that the pendingRejoinTimeout, we end the iteration here without claiming rewards for this epoch.
            // This is because we must wait for the timeout to have elapsed to account for any necessary retroactive slashing
            // that may occur.
            if (
                s().isValidatorInPendingRejoin[stakerAddress] &&
                (currentRewardEpoch.epochEnd + s().pendingRejoinTimeout) >
                block.timestamp
            ) {
                lastRewardEpochClaimed = i;
                break;
            }

            // The reward epoch total stake rewards tracks the total rewards for the staked validator.
            // We need to calculate the fraction of the total stake weight that this staker (LibERC2771._msgSender())
            // has (the LibERC2771._msgSender() might be a delegated staker).

            // First calculate the reward rate per unit of stake weight
            uint256 rewardRate = (currentRewardEpoch.totalStakeRewards *
                StakingUtilsLib.PRECISION) /
                currentRewardEpoch.totalStakeWeight;

            uint256 stakeWeightInCurrentEpoch = views().getStakeWeightInEpoch(
                stakerAddress,
                stakeRecord.id,
                LibERC2771._msgSender(),
                i
            );
            // Then calculate the reward for the staker based on their stake weight in the current epoch
            cumulativeReward +=
                (rewardRate * stakeWeightInCurrentEpoch) /
                StakingUtilsLib.PRECISION;

            stakeRecord.lastRewardEpochClaimed = i;
        }

        stakeRecord.lastUpdateTimestamp = block.timestamp;
        stakeRecord.lastRewardEpochClaimed = lastRewardEpochClaimed;

        updateStakeRecord(LibERC2771._msgSender(), stakeRecord.id, stakeRecord);

        SafeERC20.safeTransfer(
            IERC20(views().getTokenContractAddress()),
            LibERC2771._msgSender(),
            cumulativeReward
        );
        emit StakeRewardsClaimed(
            stakerAddress,
            stakeRecord.id,
            cumulativeReward,
            startEpochCheckpoint,
            lastRewardEpochClaimed
        );
    }

    /// @notice Claims the fixed cost rewards for the validator
    /// @param realmId The ID of the realm
    /// @param maxNumberOfEpochsToClaim The user-defined maximum number of epochs to claim rewards for
    /// @dev In contrast to claimStakeRewards and claimValidatorCommission, this function does not check
    /// whether the validator is in the pending rejoins watchlist. This is because the fixed cost rewards are
    /// not associated with the stake rewards, whereas validator commissions are calculated as a percentage of
    /// the stake rewards.
    function claimFixedCostRewards(
        uint256 realmId,
        uint256 maxNumberOfEpochsToClaim
    ) external nonReentrant {
        uint256 cumulativeReward = 0;
        uint256 currentRewardEpochNumber = mutableEpoch(realmId)
            .rewardEpochNumber;
        if (maxNumberOfEpochsToClaim == 0) {
            // Defaults to 50 epochs if no max is provided. 50 provides a reasonable bound such that
            // this function would not exceed the block gas limit.
            maxNumberOfEpochsToClaim = 50;
        }
        uint256 startEpochCheckpoint = s()
            .validators[LibERC2771._msgSender()]
            .lastRewardEpochClaimedFixedCostRewards;
        if (currentRewardEpochNumber < startEpochCheckpoint) {
            return;
        }
        uint256 nextEpochCheckpoint = currentRewardEpochNumber -
            startEpochCheckpoint >
            maxNumberOfEpochsToClaim
            ? startEpochCheckpoint + maxNumberOfEpochsToClaim
            : currentRewardEpochNumber;
        uint256 lastRewardEpochClaimed = nextEpochCheckpoint;

        for (uint256 i = startEpochCheckpoint; i < nextEpochCheckpoint; i++) {
            LibStakingStorage.RewardEpoch memory currentRewardEpoch = s()
                .rewardEpochs[LibERC2771._msgSender()][i];

            // If there are no rewards to claim, then we can skip this epoch.
            if (currentRewardEpoch.validatorFixedCostRewards == 0) {
                continue;
            }

            cumulativeReward += currentRewardEpoch.validatorFixedCostRewards;
            s()
                .validators[LibERC2771._msgSender()]
                .lastRewardEpochClaimedFixedCostRewards = i;
        }

        s()
            .validators[LibERC2771._msgSender()]
            .lastRewardEpochClaimedFixedCostRewards = lastRewardEpochClaimed;

        SafeERC20.safeTransfer(
            IERC20(views().getTokenContractAddress()),
            LibERC2771._msgSender(),
            cumulativeReward
        );
        emit FixedCostRewardsClaimed(
            LibERC2771._msgSender(),
            cumulativeReward,
            startEpochCheckpoint,
            lastRewardEpochClaimed
        );
    }

    /// @notice Claims the commission for the validator
    /// @param realmId The ID of the realm
    /// @param maxNumberOfEpochsToClaim The user-defined maximum number of epochs to claim rewards for
    function claimValidatorCommission(
        uint256 realmId,
        uint256 maxNumberOfEpochsToClaim
    ) external nonReentrant {
        uint256 cumulativeReward = 0;
        uint256 currentRewardEpochNumber = mutableEpoch(realmId)
            .rewardEpochNumber;
        if (maxNumberOfEpochsToClaim == 0) {
            // Defaults to 50 epochs if no max is provided. 50 provides a reasonable bound such that
            // this function would not exceed the block gas limit.
            maxNumberOfEpochsToClaim = 50;
        }
        uint256 startEpochCheckpoint = s()
            .validators[LibERC2771._msgSender()]
            .lastRewardEpochClaimedCommission;
        if (currentRewardEpochNumber < startEpochCheckpoint) {
            return;
        }
        uint256 nextEpochCheckpoint = currentRewardEpochNumber -
            startEpochCheckpoint >
            maxNumberOfEpochsToClaim
            ? startEpochCheckpoint + maxNumberOfEpochsToClaim
            : currentRewardEpochNumber;
        uint256 lastRewardEpochClaimed = nextEpochCheckpoint;

        for (uint256 i = startEpochCheckpoint; i < nextEpochCheckpoint; i++) {
            LibStakingStorage.RewardEpoch memory currentRewardEpoch = s()
                .rewardEpochs[LibERC2771._msgSender()][i];

            // If there are no rewards to claim, then we can skip this epoch.
            if (currentRewardEpoch.validatorCommission == 0) {
                continue;
            }

            // If this validator is on the pending rejoins watchlist and this reward epoch has an epoch ending time that
            // is more recent that the pendingRejoinTimeout, we end the iteration here without claiming rewards for this epoch.
            // This is because we must wait for the timeout to have elapsed to account for any necessary retroactive slashing
            // that may occur.
            if (
                s().isValidatorInPendingRejoin[LibERC2771._msgSender()] &&
                (currentRewardEpoch.epochEnd + s().pendingRejoinTimeout) >
                block.timestamp
            ) {
                lastRewardEpochClaimed = i;
                break;
            }

            cumulativeReward += currentRewardEpoch.validatorCommission;
            s()
                .validators[LibERC2771._msgSender()]
                .lastRewardEpochClaimedCommission = i;
        }

        s()
            .validators[LibERC2771._msgSender()]
            .lastRewardEpochClaimedCommission = lastRewardEpochClaimed;

        SafeERC20.safeTransfer(
            IERC20(views().getTokenContractAddress()),
            LibERC2771._msgSender(),
            cumulativeReward
        );
        emit ValidatorCommissionClaimed(
            LibERC2771._msgSender(),
            cumulativeReward,
            startEpochCheckpoint,
            lastRewardEpochClaimed
        );
    }

    /**
     * @notice Updates the stake record in the staker's vault
     * @param userAddress The address of the staker
     * @param stakeId The ID of the stake record
     * @param newState The new state of the stake record
     */
    function updateStakeRecord(
        address userAddress,
        uint256 stakeId,
        LibStakingStorage.StakeRecord memory newState
    ) internal {
        address stakerAddress = s().userStakerAddressToStakerAddress[
            userAddress
        ];
        LibStakingStorage.StakeRecord[30] storage userStakes = s()
        .vaults[stakerAddress][userAddress].stakes;
        for (uint256 i = 0; i < userStakes.length; i++) {
            if (userStakes[i].id == stakeId) {
                userStakes[i] = newState;
                break;
            }
        }

        emit StakeRecordUpdated(stakerAddress, stakeId);
    }

    /**
     * @notice Migrates a stake record to a new validator
     * @param operatorAddressToMigrateFrom The address of the operator staker to migrate from
     * @param stakeRecordId The ID of the stake record
     * @param operatorAddressToMigrateTo The address of the operator staker to migrate to
     * @dev Reverts if the stake record will not expire
     * @dev Reverts if the rewards have not been claimed for the stake record
     */
    function migrateStakeRecord(
        address operatorAddressToMigrateFrom,
        uint256 stakeRecordId,
        address operatorAddressToMigrateTo
    ) external {
        LibStakingStorage.StakeRecord memory stakeRecord = views()
            .getStakeRecord(
                operatorAddressToMigrateFrom,
                stakeRecordId,
                LibERC2771._msgSender()
            );

        uint256 realmId = realms().getRealmIdForStakerAddress(
            operatorAddressToMigrateFrom
        );

        // If the realmId is not 0, we will need to check if the validator's stake will expire.
        if (
            realmId != 0 &&
            views().validatorSelfStakeWillExpire(
                realmId,
                operatorAddressToMigrateFrom,
                realm(realmId).validatorsInCurrentEpoch.contains(
                    operatorAddressToMigrateFrom
                )
            )
        ) {
            revert CannotMigrateFromValidator();
        }

        if (
            stakeRecord.lastRewardEpochClaimed <
            s().validators[operatorAddressToMigrateFrom].lastRewardEpoch
        ) {
            revert RewardsMustBeClaimed();
        }
        uint256 actualAmount = removeStakeRecord(
            RemoveStakeRecordParams({
                stakerAddress: operatorAddressToMigrateFrom,
                stakeRecordId: stakeRecordId,
                userStakerAddress: LibERC2771._msgSender(),
                requireDecayed: false,
                migrateToStakerAddress: operatorAddressToMigrateTo
            })
        );
        _createStakeRecord(
            actualAmount,
            stakeRecord.timeLock,
            operatorAddressToMigrateTo,
            LibERC2771._msgSender(),
            CreateStakeRecordOpts({
                targetCurrentRewardEpoch: true,
                lastRewardEpochClaimedToSet: stakeRecord.lastRewardEpochClaimed,
                unfreezeStartToSet: stakeRecord.unfreezeStart
            })
        );

        // If this was the last self-stake record, then remove the validator from the allValidators set.
        if (views().getSelfStakeRecordCount(LibERC2771._msgSender()) == 0) {
            s().allValidators.remove(LibERC2771._msgSender());
        }

        // If this was the last stake this delegator had against this validator, then decrement the unique delegating staker count.
        uint256 stakeRecordCountForOperatorStakerFrom = views()
            .getStakeRecordCount(
                LibERC2771._msgSender(),
                operatorAddressToMigrateFrom
            );
        if (
            (LibERC2771._msgSender() != operatorAddressToMigrateFrom) &&
            (stakeRecordCountForOperatorStakerFrom == 0)
        ) {
            s()
                .validators[operatorAddressToMigrateFrom]
                .uniqueDelegatingStakerCount--;
        }

        // If this is the first stake this delegator has against the new validator, then increment the unique delegating staker count.
        if (
            (LibERC2771._msgSender() != operatorAddressToMigrateTo) &&
            (views().getStakeRecordCount(
                LibERC2771._msgSender(),
                operatorAddressToMigrateTo
            ) == 1)
        ) {
            s()
                .validators[operatorAddressToMigrateTo]
                .uniqueDelegatingStakerCount++;
        }

        // If this was the last stake this staker had against the operator staker to migrate from, then adjust the global state accordingly.
        if (stakeRecordCountForOperatorStakerFrom == 0) {
            s().stakerToValidatorsTheyStakedTo[LibERC2771._msgSender()].remove(
                operatorAddressToMigrateFrom
            );
        }
        s().stakerToValidatorsTheyStakedTo[LibERC2771._msgSender()].add(
            operatorAddressToMigrateTo
        );

        // If this was the last unfreezing stake against the operator staker to migrate from, then remove the validator from the
        // validatorToDelegatedStakersWithUnfreezingStakes mapping.
        if (
            views().getUnfrozenStakeCountForUser(
                LibERC2771._msgSender(),
                operatorAddressToMigrateFrom
            ) == 0
        ) {
            s()
                .validatorToDelegatedStakersWithUnfreezingStakes[
                    operatorAddressToMigrateFrom
                ]
                .remove(LibERC2771._msgSender());
        }
    }

    /**
     * @notice Splits a stake record into two separate stake records
     * @param stakerAddress The address of the staker
     * @param stakeRecordId The ID of the stake record
     * @param ratio The percentage to allocate to the first split stake record
     * @dev Reverts if the rewards have not been claimed for the stake record
     * @dev Reverts if there is no empty staking slot
     */
    function splitStakeRecord(
        address stakerAddress,
        uint256 stakeRecordId,
        uint256 ratio
    ) external {
        LibStakingStorage.StakeRecord memory stakeRecord = views()
            .getStakeRecord(
                stakerAddress,
                stakeRecordId,
                LibERC2771._msgSender()
            );
        if (
            stakeRecord.lastRewardEpochClaimed <
            s().validators[stakerAddress].lastRewardEpoch
        ) {
            revert RewardsMustBeClaimed();
        }
        if (ratio < .001 ether || ratio > .999 ether) {
            revert InvalidRatio();
        }
        if (!stakeRecord.frozen) {
            revert CannotModifyUnfrozen();
        }
        bool hasEmptySlot;
        for (
            uint256 i = 0;
            i <
            s().vaults[stakerAddress][LibERC2771._msgSender()].stakes.length;
            i++
        ) {
            if (
                !s()
                .vaults[stakerAddress][LibERC2771._msgSender()].stakes[i].loaded
            ) {
                hasEmptySlot = true;
                break;
            }
        }
        if (!hasEmptySlot) {
            revert NoEmptyStakingSlot();
        }
        uint256 newTimeLock = stakeRecord.timeLock;
        uint256 amountLeft = removeStakeRecord(
            RemoveStakeRecordParams({
                stakerAddress: stakerAddress,
                stakeRecordId: stakeRecordId,
                userStakerAddress: LibERC2771._msgSender(),
                requireDecayed: false,
                migrateToStakerAddress: address(0)
            })
        );
        uint256 firstAmountSplit = (amountLeft * ratio) /
            StakingUtilsLib.PRECISION;
        uint256 secondAmountSplit = amountLeft - firstAmountSplit;

        bool isSelfStake = stakerAddress == LibERC2771._msgSender();
        StakingUtilsLib.checkStakeAmountMinMax(firstAmountSplit, isSelfStake);
        StakingUtilsLib.checkStakeAmountMinMax(secondAmountSplit, isSelfStake);

        _createStakeRecord(
            firstAmountSplit,
            newTimeLock,
            stakerAddress,
            LibERC2771._msgSender(),
            CreateStakeRecordOpts({
                targetCurrentRewardEpoch: true,
                lastRewardEpochClaimedToSet: stakeRecord.lastRewardEpochClaimed,
                unfreezeStartToSet: 0
            })
        );
        _createStakeRecord(
            secondAmountSplit,
            newTimeLock,
            stakerAddress,
            LibERC2771._msgSender(),
            CreateStakeRecordOpts({
                targetCurrentRewardEpoch: true,
                lastRewardEpochClaimedToSet: stakeRecord.lastRewardEpochClaimed,
                unfreezeStartToSet: 0
            })
        );
    }

    /**
     * @notice withdraws the stake from the validator
     * @param operatorStakerAddress address of the operator staker who has this record
     * @param stakeRecordId The ID of the stake record
     */
    function withdraw(
        address operatorStakerAddress,
        uint256 stakeRecordId
    ) public nonReentrant {
        // If this validator is on the pending rejoins watchlist and less than pendingRejoinTimeout has elapsed since
        // the validator's most recent epoch ending time, then we prevent the validator or any of its delegating stakers
        // from withdrawing their stake. This is because we must wait for the timeout to have elapsed to account for any
        // necessary retroactive slashing that may occur.
        if (s().isValidatorInPendingRejoin[operatorStakerAddress]) {
            // Get the last reward epoch for this validator
            LibStakingStorage.RewardEpoch
                memory lastRewardEpoch = getRewardEpoch(
                    operatorStakerAddress,
                    s().validators[operatorStakerAddress].lastRewardEpoch
                );
            if (
                lastRewardEpoch.epochEnd + s().pendingRejoinTimeout >
                block.timestamp
            ) {
                revert TooSoonToWithdraw();
            }
        }

        address userStakerAddress = LibERC2771._msgSender();
        uint256 withdrawAmount = removeStakeRecord(
            RemoveStakeRecordParams({
                stakerAddress: operatorStakerAddress,
                stakeRecordId: stakeRecordId,
                userStakerAddress: userStakerAddress,
                requireDecayed: true,
                migrateToStakerAddress: address(0)
            })
        );

        SafeERC20.safeTransfer(
            IERC20(views().getTokenContractAddress()),
            LibERC2771._msgSender(),
            withdrawAmount
        );

        // If this was the last self-stake record, then remove the validator from the allValidators set.
        if (views().getSelfStakeRecordCount(LibERC2771._msgSender()) == 0) {
            s().allValidators.remove(LibERC2771._msgSender());
        }

        // If this was the last stake this delegator had against this validator, then decrement the unique delegating staker count.
        uint256 stakeRecordCountForOperatorStakerFrom = views()
            .getStakeRecordCount(
                LibERC2771._msgSender(),
                operatorStakerAddress
            );
        if (
            (LibERC2771._msgSender() != operatorStakerAddress) &&
            (stakeRecordCountForOperatorStakerFrom == 0)
        ) {
            s().validators[operatorStakerAddress].uniqueDelegatingStakerCount--;
        }
        // If this was the last stake this staker had against the operator staker, then adjust the global state accordingly.
        if (stakeRecordCountForOperatorStakerFrom == 0) {
            s().stakerToValidatorsTheyStakedTo[LibERC2771._msgSender()].remove(
                operatorStakerAddress
            );
        }

        // If this was the last unfreezing stake against the operator staker, then remove the validator from the
        // validatorToDelegatedStakersWithUnfreezingStakes mapping.
        if (
            views().getUnfrozenStakeCountForUser(
                LibERC2771._msgSender(),
                operatorStakerAddress
            ) == 0
        ) {
            s()
                .validatorToDelegatedStakersWithUnfreezingStakes[
                    operatorStakerAddress
                ]
                .remove(LibERC2771._msgSender());
        }

        emit Withdrawn(operatorStakerAddress, withdrawAmount);
    }

    function checkStakingAmounts(
        address stakerAddress
    ) public view returns (bool) {
        uint256 minimumStake = s().globalConfig[0].minStakeAmount;

        // Get self-stake records.
        LibStakingStorage.StakeRecord[] memory selfStakeRecords = views()
            .getStakeRecordsForUser(stakerAddress, stakerAddress);

        uint256 totalSelfStake = 0;
        for (uint256 i = 0; i < selfStakeRecords.length; i++) {
            totalSelfStake += selfStakeRecords[i].amount;
        }

        if (totalSelfStake < minimumStake) {
            revert StakeMustBeGreaterThanMinimumStake(
                stakerAddress,
                totalSelfStake,
                minimumStake
            );
        }
        return true;
    }

    function balanceOf(address stakerAddress) external view returns (uint256) {
        LibStakingStorage.StakeRecord[] memory stakeRecords = views()
            .getStakeRecordsForUser(stakerAddress, stakerAddress);
        uint256 totalStake = 0;
        for (uint256 i = 0; i < stakeRecords.length; i++) {
            totalStake += stakeRecords[i].amount;
        }
        return totalStake;
    }

    function getMinimumStake() external view returns (uint256) {
        return s().globalConfig[0].minStakeAmount;
    }

    function getMinimumSelfStake() external view returns (uint256) {
        return s().globalConfig[0].minSelfStake;
    }

    function getMaximumStake() external view returns (uint256) {
        return s().globalConfig[0].maxStakeAmount;
    }

    function setValidatorCommissionRate(uint256 rate) external {
        require(rate < 1 ether, "Rate must be less than 100%");
        s().validators[LibERC2771._msgSender()].commissionRate = rate;
    }

    // EVENTS
    event StakeRecordRemoved(address userStakerAddress, uint256 recordId);
    event StakeRewardsClaimed(
        address stakerAddress,
        uint256 recordId,
        uint256 rewards,
        uint256 fromEpoch,
        uint256 toEpoch
    );
    event FixedCostRewardsClaimed(
        address stakerAddress,
        uint256 rewards,
        uint256 fromEpoch,
        uint256 toEpoch
    );
    event ValidatorCommissionClaimed(
        address stakerAddress,
        uint256 rewards,
        uint256 fromEpoch,
        uint256 toEpoch
    );
    event StakeRecordUpdated(address stakerAddress, uint256 recordId);
    event Withdrawn(address indexed staker, uint256 amount);
    event ValidatorRegistered(address indexed stakerAddress);
}
