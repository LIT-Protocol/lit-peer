//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { StakingAcrossRealmsFacet } from "./StakingAcrossRealmsFacet.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { ReentrancyGuard } from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import { console } from "lib/forge-std/src/console.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { EnumerableSetViewFriendly } from "@lit-protocol/openzeppelin-contracts/utils/structs/EnumerableSetViewFriendly.sol";
import { LibPubkeyRouterStorage } from "../PubkeyRouter/LibPubkeyRouterStorage.sol";

contract StakingCommon is ReentrancyGuard {
    using EnumerableSet for EnumerableSet.UintSet;
    using EnumerableSetViewFriendly for EnumerableSetViewFriendly.AddressSet;

    /* ========== ERRORS ========== */
    error NoEmptyStakingSlot();
    error CannotModifyUnfrozen();

    /* ========== VIEWS ========== */

    function views() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(address(this));
    }

    function staking() internal view returns (StakingFacet) {
        return StakingFacet(address(this));
    }

    /* ========== INTERNAL FUNCTIONS ========== */

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

    function pubkeyRouter()
        internal
        pure
        returns (LibPubkeyRouterStorage.PubkeyRouterStorage storage)
    {
        return LibPubkeyRouterStorage.getStorage();
    }

    function realms() internal view returns (StakingAcrossRealmsFacet) {
        return StakingAcrossRealmsFacet(address(this));
    }

    function mutableEpoch(
        uint256 realmId
    ) internal view returns (LibStakingStorage.Epoch storage) {
        return realm(realmId).epochs[0];
    }

    function _stake(uint256 amount, address account) internal nonReentrant {
        if (amount == 0) {
            revert StakingUtilsLib.CannotStakeZero();
        }
        SafeERC20.safeTransferFrom(
            IERC20(views().getTokenContractAddress()),
            account,
            address(this),
            amount
        );
        emit Staked(account, amount);
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
    ) internal {
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
                attributionAddress: address(0)
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
    }

    /**
     * Unfreeze stake and start to decrease the stake weight
     * @param userStakerAddress address of the staker who has this record
     * @param operatorStakerAddress address of the validator
     * @param stakeId ID of the stake
     */
    function _unfreezeStake(
        address userStakerAddress,
        address operatorStakerAddress,
        uint256 stakeId
    ) internal {
        uint256 realmId = realms().getRealmIdForStakerAddress(
            operatorStakerAddress
        );
        uint256 currentRewardEpochNumber;
        uint256 nextRewardEpochNumber;
        if (realmId == 0) {
            if (s().validators[operatorStakerAddress].lastRealmId == 0) {
                // Validator has never been in a realm before, so we just use the lowest reward epoch number
                uint256 realmIdWithLowestRewardEpochNumber;
                (
                    realmIdWithLowestRewardEpochNumber,
                    currentRewardEpochNumber
                ) = views().getLowestRewardEpochNumber();
                nextRewardEpochNumber = views()
                    .epoch(realmIdWithLowestRewardEpochNumber)
                    .nextRewardEpochNumber;
            } else {
                realmId = s().validators[operatorStakerAddress].lastRealmId;
                currentRewardEpochNumber = mutableEpoch(realmId)
                    .rewardEpochNumber;
                nextRewardEpochNumber = mutableEpoch(realmId)
                    .nextRewardEpochNumber;
            }
        } else {
            currentRewardEpochNumber = mutableEpoch(realmId).rewardEpochNumber;
            nextRewardEpochNumber = mutableEpoch(realmId).nextRewardEpochNumber;
        }

        LibStakingStorage.StakeRecord[30] storage userStakes = s()
        .vaults[operatorStakerAddress][userStakerAddress].stakes;

        uint256 validatorSharePrice = StakingUtilsLib
            ._getRewardEpoch(operatorStakerAddress, currentRewardEpochNumber)
            .validatorSharePrice;

        for (uint256 i = 0; i < userStakes.length; i++) {
            LibStakingStorage.StakeRecord storage userStake = userStakes[i];
            if (userStake.id == stakeId) {
                if (userStake.frozen == false) {
                    revert CannotModifyUnfrozen();
                }
                userStake.frozen = false;
                // Unfreezing starts at the beginning of the next day. This is necessary to ensure that the
                // unfreezing process results in the most accurate stake weight decrements with minimal "dust"
                // left behind for the validator, by having both the unfreeze start and the unfreeze end both
                // be quantised to the start of a day.
                userStake.unfreezeStart =
                    (block.timestamp / 1 days) *
                    1 days +
                    1 days;
                uint256 slope = 0;
                {
                    uint256 snapshotSharePrice = userStake.initialSharePrice;
                    uint256 actualSharePrice = (validatorSharePrice *
                        StakingUtilsLib.PRECISION) / snapshotSharePrice;
                    // To calculate the slope, divide the starting stake weight divided by the total timelock
                    // the slope will be applied over. The starting stake weight should be calculated using the
                    // `calculateStakeWeight` function.
                    uint256 startingStakeWeight = views().calculateStakeWeight(
                        userStake.timeLock,
                        userStake.amount
                    );
                    slope =
                        (startingStakeWeight * actualSharePrice) /
                        userStake.timeLock /
                        StakingUtilsLib.PRECISION;
                }

                // Register this stake record's unfreeze schedule
                // The quantised unfreeze end is the unfreeze start + timelock rounded down to the start of that day.
                // Note that this quantisation means that the actual number of stake weight decrements will always be
                // less than or equal to the expected number.
                uint256 quantisedUnfreezeEnd = ((userStake.unfreezeStart +
                    userStake.timeLock) / 1 days) * 1 days;
                s().validatorToUnfreezeEndToSlopeIncrease[
                    operatorStakerAddress
                ][quantisedUnfreezeEnd] += slope;
                s().validatorToUnfreezeEnds[operatorStakerAddress].add(
                    quantisedUnfreezeEnd
                );

                // Store the gradient (negative number represented as uint) - the reduction of stake weight per second.
                // Instead of registering the slope at the next reward epoch number, we store it and let the validator
                // pick up this variable whenever the epoch advancements through this time. This is necessary because
                // we don't know for sure what the reward epoch number will be that coincides with the quantised
                // unfreeze start. This should be reasonably cost-efficient to do since we can exploit the fact
                // that successive unfreeze calls will always add to the tracking data structures resulting in
                // ordered unfreeze starts.
                s().validatorToUnfreezeStartToSlope[operatorStakerAddress][
                    userStake.unfreezeStart
                ] += slope;
                s().validatorToUnfreezeStarts[operatorStakerAddress].add(
                    userStake.unfreezeStart
                );

                break;
            }
        }

        if (userStakerAddress != operatorStakerAddress) {
            // Track this delegated staker in the validator mapping.
            s()
                .validatorToDelegatedStakersWithUnfreezingStakes[
                    operatorStakerAddress
                ]
                .add(userStakerAddress);
        }
    }

    // EVENTS
    event Staked(address indexed staker, uint256 amount);
    event StakeRecordCreated(
        address stakerAddress,
        uint256 recordId,
        uint256 amount,
        address stakerAddressClient
    );
}
