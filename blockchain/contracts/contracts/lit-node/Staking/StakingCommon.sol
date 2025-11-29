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
import { LibStakingNFT } from "./LibStakingNFT.sol";

contract StakingCommon is ReentrancyGuard {
    using EnumerableSet for EnumerableSet.UintSet;
    using EnumerableSetViewFriendly for EnumerableSetViewFriendly.AddressSet;

    /* ========== ERRORS ========== */
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

    function _createStakeRecord(
        uint256 stakeAmount,
        uint256 timeLock,
        address stakerAddress,
        address userStakerAddress,
        StakingUtilsLib.CreateStakeRecordOpts memory opts
    ) internal returns (uint256) {
        uint256 newStakeRecordId = StakingUtilsLib._createStakeRecord(
            stakeAmount,
            timeLock,
            stakerAddress,
            userStakerAddress,
            opts
        );

        LibStakingNFT.updateTokenToStakeRecord(
            opts.tokenIdToSet,
            stakerAddress,
            newStakeRecordId
        );

        return newStakeRecordId;
    }

    // EVENTS
    event Staked(address indexed staker, uint256 amount);
}
