// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import { Test, console2 } from "lib/forge-std/src/Test.sol";
import { StakingFacet } from "../Staking/StakingFacet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { StakingAdminFacet } from "../Staking/StakingAdminFacet.sol";
import { StakingValidatorFacet } from "../Staking/StakingValidatorFacet.sol";
import { StakingAcrossRealmsFacet } from "../Staking/StakingAcrossRealmsFacet.sol";
import { StakingUtilsLib } from "../Staking/StakingUtilsLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { Staking, StakingArgs } from "../Staking.sol";
import { DiamondInit } from "../../upgradeInitializers/DiamondInit.sol";
import { IDiamond } from "../../interfaces/IDiamond.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { LITToken } from "../LITToken.sol";
import { SetupAndUtils } from "./SetupAndUtils.t.sol";
import { console } from "lib/forge-std/src/console.sol";

contract StakingMigrateTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    /// @notice This test is when a validator with a delegated stake goes dark, that the
    /// delegating staker can migrate their stake to another validator.
    function testFuzz_DelegatedStakerMigrateFromValidatorWhoWentDark(
        uint256 operatorStakerThatWillGoDarkIndex,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        // Input assumptions
        operatorStakerThatWillGoDarkIndex = bound(
            operatorStakerThatWillGoDarkIndex,
            0,
            3
        );
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        {
            // Set complaint config for reason 1
            LibStakingStorage.ComplaintConfig
                memory complaintConfig = LibStakingStorage.ComplaintConfig({
                    tolerance: 6,
                    intervalSecs: 90,
                    kickPenaltyPercent: 0.5 ether, // 50%
                    kickPenaltyDemerits: 1 // set this low enough to prevent slashing of that validator
                });
            stakingAdminFacet.setComplaintConfig(1, complaintConfig);
        }

        // Setup validators
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _setupValidators(
            realmId,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with the validator that will go dark
        address validatorThatWillGoDark = randomOperatorStakers[
            operatorStakerThatWillGoDarkIndex
        ];
        // Find a validator that is not the validator that will go dark to migrate the stake to
        address validatorToMigrateTo;
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (i == operatorStakerThatWillGoDarkIndex) {
                continue;
            }
            validatorToMigrateTo = randomOperatorStakers[i];
            break;
        }

        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, validatorThatWillGoDark);

        // That validator goes dark in the 4th epoch - 3 validators all vote to kick the first validator
        _advanceEpochs(realmId, 3, randomOperatorStakers, 1);
        address[] memory validatorsNotKicked = new address[](4);
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (i == operatorStakerThatWillGoDarkIndex) {
                continue;
            }
            validatorsNotKicked[i] = randomOperatorStakers[i];
        }
        for (uint256 i = 0; i < validatorsNotKicked.length; i++) {
            address validator = validatorsNotKicked[i];
            if (validator == address(0)) {
                continue;
            }

            vm.prank(validator);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                validatorThatWillGoDark,
                1,
                ""
            );
        }

        // Advance epoch to deal out the validator who was kicked.
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        _advanceEpochs(realmId, 1, validatorsNotKicked, 4);

        // Assert that the delegating staker cannot migrate without claiming rewards
        vm.expectRevert(
            abi.encodeWithSelector(StakingFacet.RewardsMustBeClaimed.selector)
        );
        vm.prank(delegatingStaker);
        stakingFacet.migrateStakeRecord(
            validatorThatWillGoDark,
            1,
            validatorToMigrateTo
        );

        // Assert that the delegating staker also cannot claim rewards yet because
        // the pending rejoin timeout has not elapsed yet, and only when it does,
        // will the validator who went dark be slashed.
        uint256 balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(1, validatorThatWillGoDark, 1, 0);

        // Assert that the balance has not increased
        assertEq(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should not have increased"
        );

        // Assert that the delegating staker cannot withdraw their stake yet because
        // the validator who went dark is still on the pending rejoins watchlist.
        vm.expectRevert(StakingFacet.TooSoonToWithdraw.selector);
        vm.prank(delegatingStaker);
        stakingFacet.withdraw(validatorThatWillGoDark, 1);

        // Now, advance the epoch 24 times to slash the validator who went dark
        _advanceEpochs(realmId, 24, validatorsNotKicked, 5);

        // Delegating staker claims rewards
        balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(1, validatorThatWillGoDark, 1, 0);

        // Assert that the delegating staker has claimed rewards
        assertGt(token.balanceOf(delegatingStaker), balanceBefore);

        // Delegating staker migrates their stake to the second validator
        LibStakingStorage.RewardEpochGlobalStats
            memory epochGlobalStatsBeforeMigrating = stakingViewsFacet
                .getRewardEpochGlobalStats(29);

        vm.prank(delegatingStaker);
        stakingFacet.migrateStakeRecord(
            validatorThatWillGoDark,
            1,
            validatorToMigrateTo
        );

        // Assert that the global stats for this epoch has increased as a result of the migration
        assertGt(
            stakingViewsFacet.getRewardEpochGlobalStats(29).stakeWeight,
            epochGlobalStatsBeforeMigrating.stakeWeight,
            "Global stats stake weight should have increased"
        );
        assertGt(
            stakingViewsFacet.getRewardEpochGlobalStats(29).stakeAmount,
            epochGlobalStatsBeforeMigrating.stakeAmount,
            "Global stats stake amount should have increased"
        );

        // Assert that the old stake record is gone
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingUtilsLib.StakeRecordNotFound.selector,
                realmId
            )
        );
        stakingViewsFacet.getStakeRecord(
            validatorThatWillGoDark,
            1,
            delegatingStaker
        );

        // Assert the details of the new stake record
        LibStakingStorage.StakeRecord memory newStakeRecord = stakingViewsFacet
            .getStakeRecord(validatorToMigrateTo, 1, delegatingStaker);
        // Since the validator went dark, the delegated staker's stake is also penalized.
        uint256 expectedAmount = amount / 2;
        assertEq(
            newStakeRecord.amount,
            expectedAmount,
            "Stake amount should be penalized"
        );
        // Round down to the nearest day
        assertEq(newStakeRecord.timeLock, roundedTimeLock);

        // Assert that when the staker claims rewards immediately after migrating,
        // that there is nothing to claim against the new stake records.
        balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(1, validatorToMigrateTo, 1, 0);

        // Assert that the balance has not increased
        assertEq(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should not have increased"
        );

        // Advance one epoch to earn rewards against the new validator since the validator is already
        // active in epoch 29
        _advanceEpochs(realmId, 1, validatorsNotKicked, 29);

        // Assert that the delegating staker is earning rewards against the new validator
        balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(1, validatorToMigrateTo, 1, 0);

        // Assert that the delegating staker has claimed rewards
        assertGt(token.balanceOf(delegatingStaker), balanceBefore);
    }

    /// @notice This test is when a validator with a delegated stake goes dark after the delegated stake
    /// has been unfreezing for some time, that the delegating staker can migrate their stake to another
    /// validator, and that stake record continues unfreezing from where it was right before the migration.
    function testFuzz_DelegatedStakerMigrateFromValidatorWhoWentDark_UnfrozenStake(
        uint256 operatorStakerThatWillGoDarkIndex,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        operatorStakerThatWillGoDarkIndex = bound(
            operatorStakerThatWillGoDarkIndex,
            0,
            3
        );
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;
        uint256 epochLength = stakingViewsFacet.epoch(1).epochLength;

        {
            // Set complaint config for reason 1
            LibStakingStorage.ComplaintConfig
                memory complaintConfig = LibStakingStorage.ComplaintConfig({
                    tolerance: 6,
                    intervalSecs: 90,
                    kickPenaltyPercent: 0.5 ether, // 50%
                    kickPenaltyDemerits: 1 // set this low enough to prevent slashing of that validator
                });
            stakingAdminFacet.setComplaintConfig(1, complaintConfig);
        }

        // Setup validators
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _setupValidators(
            realmId,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with the validator that will go dark
        address validatorThatWillGoDark = randomOperatorStakers[
            operatorStakerThatWillGoDarkIndex
        ];
        // Find a validator that is not the validator that will go dark to migrate the stake to
        address validatorToMigrateTo;
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (i == operatorStakerThatWillGoDarkIndex) {
                continue;
            }
            validatorToMigrateTo = randomOperatorStakers[i];
            break;
        }

        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, validatorThatWillGoDark);

        // Delegating staker immediately starts unfreezing their stake
        vm.prank(delegatingStaker);
        stakingFacet.unfreezeStake(validatorThatWillGoDark, 1);

        // That validator goes dark in the 30 epochs later - 3 validators all vote to kick the first validator
        // 30 epochs is necessary to ensure at least a day is covered for the operator to pick up on the stake
        // record beginning to unfreeze.
        uint256 epochNumber = _advanceEpochsCustomEpochLength(
            realmId,
            30,
            randomOperatorStakers,
            1,
            epochLength
        );
        address[] memory validatorsNotKicked = new address[](4);
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (i == operatorStakerThatWillGoDarkIndex) {
                continue;
            }
            validatorsNotKicked[i] = randomOperatorStakers[i];
        }
        for (uint256 i = 0; i < validatorsNotKicked.length; i++) {
            address validator = validatorsNotKicked[i];
            if (validator == address(0)) {
                continue;
            }

            vm.prank(validator);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                validatorThatWillGoDark,
                1,
                ""
            );
        }

        // Advance epoch to deal out the validator who was kicked.
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        epochNumber = _advanceEpochsCustomEpochLength(
            realmId,
            1,
            validatorsNotKicked,
            epochNumber,
            epochLength
        );

        // Now, advance the epoch 24 times to slash the validator who went dark
        epochNumber = _advanceEpochsCustomEpochLength(
            realmId,
            24,
            validatorsNotKicked,
            epochNumber,
            epochLength
        );

        {
            // Delegating staker claims rewards
            uint256 balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(1, validatorThatWillGoDark, 1, 100);

            // Assert that the delegating staker has claimed rewards
            assertGt(token.balanceOf(delegatingStaker), balanceBefore);
        }

        {
            // Delegating staker migrates their stake to an active validator
            LibStakingStorage.RewardEpochGlobalStats
                memory epochGlobalStatsBeforeMigrating = stakingViewsFacet
                    .getRewardEpochGlobalStats(epochNumber);

            vm.prank(delegatingStaker);
            stakingFacet.migrateStakeRecord(
                validatorThatWillGoDark,
                1,
                validatorToMigrateTo
            );

            // Assert that the global stats for this epoch has increased as a result of the migration
            assertGt(
                stakingViewsFacet
                    .getRewardEpochGlobalStats(epochNumber)
                    .stakeWeight,
                epochGlobalStatsBeforeMigrating.stakeWeight,
                "Global stats stake weight should have increased"
            );
            assertGt(
                stakingViewsFacet
                    .getRewardEpochGlobalStats(epochNumber)
                    .stakeAmount,
                epochGlobalStatsBeforeMigrating.stakeAmount,
                "Global stats stake amount should have increased"
            );

            // Assert that the old stake record is gone
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingUtilsLib.StakeRecordNotFound.selector,
                    realmId
                )
            );
            stakingViewsFacet.getStakeRecord(
                validatorThatWillGoDark,
                1,
                delegatingStaker
            );

            // Assert the details of the new stake record
            LibStakingStorage.StakeRecord
                memory newStakeRecord = stakingViewsFacet.getStakeRecord(
                    validatorToMigrateTo,
                    1,
                    delegatingStaker
                );
            // Since the validator went dark, the delegated staker's stake is also penalized.
            uint256 expectedAmount = amount / 2;
            assertEq(
                newStakeRecord.amount,
                expectedAmount,
                "Stake amount should be penalized"
            );
            // Round down to the nearest day
            assertEq(newStakeRecord.timeLock, roundedTimeLock);
        }

        {
            // Advance just enough epochs to assert that the unfreezing progress with the first validator
            // is maintained after migration with the second validator.
            // +1 day to account for the unfreezing commencing the day after calling.
            // +1 day again for safety.
            uint256 remainingTimeToAdvance = (roundedTimeLock -
                (epochNumber * epochLength)) +
                1 days +
                1 days;
            skip(remainingTimeToAdvance);

            // Delegating staker withdraws their stake
            vm.prank(delegatingStaker);
            stakingFacet.withdraw(validatorToMigrateTo, 1);
        }

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Advance 2 epochs to ensure that the stake weights have stabilised on the new validator.
        epochNumber = _advanceEpochs(
            realmId,
            2,
            validatorsNotKicked,
            epochNumber
        );

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            validatorsNotKicked,
            epochNumber
        );
    }

    /// @notice This test is when a delegated staker first unfreezes their stake with a validator in realm 1, then
    /// migrates from that validator to another active validator in another realm, that the stake weights are
    /// decremented in the first realm and then the second realm before and after the migration respectively.
    function testFuzz_DelegatedStakerMigrateFromActiveValidator_UnfrozenStake_2Realms(
        uint256 amount,
        uint256 timeLock
    ) public {
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);

        // Add the second realm
        stakingAdminFacet.addRealm();
        // Also allocate rewards budget for realm 2
        stakingAdminFacet.increaseRewardPool(2, rewardsBudget);

        // Setup for Realm 1
        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        _setupValidators(
            1,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        address operatorStakerToStake = operatorStakersRealm1[0];

        // Setup for Realm 2
        address[] memory operatorStakersRealm2 = _generateAddressesWithOffset(
            4,
            4
        );
        _setupValidators(
            2,
            operatorStakersRealm2,
            amount * 10,
            amount,
            timeLock,
            _generateUint256sWithOffset(4, 4)
        );

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with an active validator in realm 1
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakerToStake);

        {
            // Advance both realms to epoch 4
            uint256 numEpochsToAdvance = 3;
            for (uint256 i = 0; i < numEpochsToAdvance; i++) {
                // Fast forward to the next day to guarantee rewards will be distributed even just for 1 epoch boundary.
                skip(1 days);

                // Realm 1 advances first.
                stakingValidatorFacet.lockValidatorsForNextEpoch(1);
                for (
                    uint256 realm1StakerIdx = 0;
                    realm1StakerIdx < operatorStakersRealm1.length;
                    realm1StakerIdx++
                ) {
                    vm.prank(operatorStakersRealm1[realm1StakerIdx]);
                    stakingValidatorFacet.signalReadyForNextEpoch(1, i + 1);
                }
                stakingValidatorFacet.advanceEpoch(1);

                // For the first iteration, introduce 2.5 hour delay before Realm 2 advances
                // to set Realm 2 to be 1.5 hours behind Realm 1. For all the remaining iterations,
                // keep the epoch length to be 1 day, same as realm 1.
                if (i == 0) {
                    skip(2.5 hours);
                } else {
                    skip(1 days);
                }

                // Realm 2 advances next.
                stakingValidatorFacet.lockValidatorsForNextEpoch(2);
                for (
                    uint256 realm2StakerIdx = 0;
                    realm2StakerIdx < operatorStakersRealm2.length;
                    realm2StakerIdx++
                ) {
                    vm.prank(operatorStakersRealm2[realm2StakerIdx]);
                    stakingValidatorFacet.signalReadyForNextEpoch(2, i + 1);
                }
                stakingValidatorFacet.advanceEpoch(2);
            }
        }

        // Delegating staker starts unfreezing their stake
        vm.prank(delegatingStaker);
        stakingFacet.unfreezeStake(operatorStakerToStake, 1);

        {
            // Advance both realms another 25 epochs to guarantee processing the unfreeze start
            _advanceEpochs(1, 25, operatorStakersRealm1, 4);
            _advanceEpochs(2, 25, operatorStakersRealm2, 4);
        }

        // From now on, every time realm 1 advances an epoch, stake weight for the first validator in realm 1
        // should be decreasing, and stake weight for the first validator in realm 2 should be constant.
        {
            // Advance both realms another 10 epochs
            uint256 numEpochsToAdvance = 10;
            LibStakingStorage.RewardEpoch memory oldRewardEpoch;
            LibStakingStorage.RewardEpochGlobalStats
                memory oldRewardEpochGlobalStats;

            for (uint256 i = 0; i < numEpochsToAdvance; i++) {
                // Get references to the current reward epoch and global stats for realm 1.
                uint256 currentRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(1);

                oldRewardEpoch = stakingFacet.getRewardEpoch(
                    operatorStakersRealm1[0],
                    currentRewardEpochNumber
                );
                oldRewardEpochGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentRewardEpochNumber);

                // Fast forward to the next day to guarantee rewards will be distributed even just for 1 epoch boundary.
                skip(1 days);

                // Realm 1 advances first.
                stakingValidatorFacet.lockValidatorsForNextEpoch(1);
                for (
                    uint256 realm1StakerIdx = 0;
                    realm1StakerIdx < operatorStakersRealm1.length;
                    realm1StakerIdx++
                ) {
                    vm.prank(operatorStakersRealm1[realm1StakerIdx]);
                    stakingValidatorFacet.signalReadyForNextEpoch(1, i + 29);
                }
                stakingValidatorFacet.advanceEpoch(1);

                // After each epoch advancement, assert that the stake weight is decreasing.
                currentRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(1);

                LibStakingStorage.RewardEpoch
                    memory newRewardEpoch = stakingFacet.getRewardEpoch(
                        operatorStakersRealm1[0],
                        currentRewardEpochNumber
                    );

                assertLt(
                    newRewardEpoch.totalStakeWeight,
                    oldRewardEpoch.totalStakeWeight,
                    "Reward epoch stake weight should have decreased"
                );

                LibStakingStorage.RewardEpochGlobalStats
                    memory newRewardEpochGlobalStats = stakingViewsFacet
                        .getRewardEpochGlobalStats(currentRewardEpochNumber);
                assertLt(
                    newRewardEpochGlobalStats.stakeWeight,
                    oldRewardEpochGlobalStats.stakeWeight,
                    "Global stats stake weight should have decreased"
                );

                // Get references to the current reward epoch and global stats for realm 2.
                currentRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(2);

                oldRewardEpoch = stakingFacet.getRewardEpoch(
                    operatorStakersRealm2[0],
                    currentRewardEpochNumber
                );
                oldRewardEpochGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentRewardEpochNumber);

                // Realm 2 advances next.
                skip(1 days);
                stakingValidatorFacet.lockValidatorsForNextEpoch(2);
                for (
                    uint256 realm2StakerIdx = 0;
                    realm2StakerIdx < operatorStakersRealm2.length;
                    realm2StakerIdx++
                ) {
                    vm.prank(operatorStakersRealm2[realm2StakerIdx]);
                    stakingValidatorFacet.signalReadyForNextEpoch(2, i + 29);
                }
                stakingValidatorFacet.advanceEpoch(2);

                // After each epoch advancement, assert that the stake weight is constant.
                currentRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(2);
                newRewardEpoch = stakingFacet.getRewardEpoch(
                    operatorStakersRealm2[0],
                    currentRewardEpochNumber
                );
                assertEq(
                    newRewardEpoch.totalStakeWeight,
                    oldRewardEpoch.totalStakeWeight,
                    "Reward epoch stake weight should remain constant"
                );
                newRewardEpochGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentRewardEpochNumber);
                assertEq(
                    newRewardEpochGlobalStats.stakeWeight,
                    oldRewardEpochGlobalStats.stakeWeight,
                    "Global stats stake weight should remain constant"
                );
            }
        }

        {
            // Delegating staker claims rewards
            uint256 balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(1, operatorStakerToStake, 1, 100);

            // Assert that the delegating staker has claimed rewards
            assertGt(token.balanceOf(delegatingStaker), balanceBefore);
        }

        // Delegating staker migrates their stake to the first active validator in realm 2
        vm.prank(delegatingStaker);
        stakingFacet.migrateStakeRecord(
            operatorStakerToStake,
            1,
            operatorStakersRealm2[0]
        );

        // From now on, every time realm 1 advances an epoch, stake weight for the first validator in realm 1
        // should be constant, and stake weight for the first validator in realm 2 should be decreasing.
        {
            // Advance both realms another 10 epochs
            uint256 numEpochsToAdvance = 10;
            LibStakingStorage.RewardEpoch memory oldRewardEpoch;
            LibStakingStorage.RewardEpochGlobalStats
                memory oldRewardEpochGlobalStats;

            for (uint256 i = 0; i < numEpochsToAdvance; i++) {
                // Get references to the current reward epoch and global stats for realm 1.
                uint256 currentRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(1);

                oldRewardEpoch = stakingFacet.getRewardEpoch(
                    operatorStakersRealm1[0],
                    currentRewardEpochNumber
                );
                oldRewardEpochGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentRewardEpochNumber);

                // Fast forward to the next day to guarantee rewards will be distributed even just for 1 epoch boundary.
                skip(1 days);

                // Realm 1 advances first.
                stakingValidatorFacet.lockValidatorsForNextEpoch(1);
                for (
                    uint256 realm1StakerIdx = 0;
                    realm1StakerIdx < operatorStakersRealm1.length;
                    realm1StakerIdx++
                ) {
                    vm.prank(operatorStakersRealm1[realm1StakerIdx]);
                    stakingValidatorFacet.signalReadyForNextEpoch(1, i + 39);
                }
                stakingValidatorFacet.advanceEpoch(1);

                // After each epoch advancement, assert that the stake weight is decreasing.
                currentRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(1);

                LibStakingStorage.RewardEpoch
                    memory newRewardEpoch = stakingFacet.getRewardEpoch(
                        operatorStakersRealm1[0],
                        currentRewardEpochNumber
                    );

                assertEq(
                    newRewardEpoch.totalStakeWeight,
                    oldRewardEpoch.totalStakeWeight,
                    "Reward epoch stake weight should remain constant"
                );

                LibStakingStorage.RewardEpochGlobalStats
                    memory newRewardEpochGlobalStats = stakingViewsFacet
                        .getRewardEpochGlobalStats(currentRewardEpochNumber);
                assertEq(
                    newRewardEpochGlobalStats.stakeWeight,
                    oldRewardEpochGlobalStats.stakeWeight,
                    "Global stats stake weight should remain constant"
                );

                // Get references to the current reward epoch and global stats for realm 2.
                currentRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(2);

                oldRewardEpoch = stakingFacet.getRewardEpoch(
                    operatorStakersRealm2[0],
                    currentRewardEpochNumber
                );
                oldRewardEpochGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentRewardEpochNumber);

                // Realm 2 advances next.
                skip(1 days);
                stakingValidatorFacet.lockValidatorsForNextEpoch(2);
                for (
                    uint256 realm2StakerIdx = 0;
                    realm2StakerIdx < operatorStakersRealm2.length;
                    realm2StakerIdx++
                ) {
                    vm.prank(operatorStakersRealm2[realm2StakerIdx]);
                    stakingValidatorFacet.signalReadyForNextEpoch(2, i + 39);
                }
                stakingValidatorFacet.advanceEpoch(2);

                // After each epoch advancement, assert that the stake weight is constant.
                currentRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(2);
                newRewardEpoch = stakingFacet.getRewardEpoch(
                    operatorStakersRealm2[0],
                    currentRewardEpochNumber
                );
                assertLt(
                    newRewardEpoch.totalStakeWeight,
                    oldRewardEpoch.totalStakeWeight,
                    "Reward epoch stake weight should have decreased"
                );
                newRewardEpochGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentRewardEpochNumber);
                assertLt(
                    newRewardEpochGlobalStats.stakeWeight,
                    oldRewardEpochGlobalStats.stakeWeight,
                    "Global stats stake weight should have decreased"
                );
            }
        }
    }

    /// @notice This test is when a delegated staker stakes with an inactive validator and is then
    /// able to migrate their stake to another inactive validator.
    function testFuzz_DelegatedStakerMigrateFromInactiveValidator(
        uint256 amount,
        uint256 timeLock
    ) public {
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        // Setup validators
        address[] memory operatorStakers = _generateAddresses(4);
        uint256[] memory operatorCommsKeys = _generateUint256s(4);

        // Have all validators stake but NOT join
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            _fundAddressWithTokensAndApprove(operatorStakers[i], amount * 10);

            // Set the IP and port of the validator
            vm.prank(operatorStakers[i]);
            stakingValidatorFacet.setIpPortNodeAddress(
                1,
                1,
                1,
                operatorStakers[i]
            );

            // Emit event check
            vm.expectEmit(true, true, true, true);
            emit StakeRecordCreated(
                operatorStakers[i],
                1,
                amount,
                operatorStakers[i]
            );

            // Stake and join
            vm.prank(operatorStakers[i]);
            stakingFacet.stake(amount, timeLock, operatorStakers[i]);
        }

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with an inactive validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakers[0]);

        // Assert that the stake record exists
        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(operatorStakers[0], 1, delegatingStaker);
        assertEq(stakeRecord.amount, amount);
        assertEq(stakeRecord.timeLock, roundedTimeLock);

        // Now, the delegating staker migrates their stake to another inactive validator
        vm.prank(delegatingStaker);
        stakingFacet.migrateStakeRecord(
            operatorStakers[0],
            1,
            operatorStakers[1]
        );

        // Assert that the stake record against the first validator is gone
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingUtilsLib.StakeRecordNotFound.selector,
                1
            )
        );
        stakingViewsFacet.getStakeRecord(
            operatorStakers[0],
            1,
            delegatingStaker
        );

        // Assert that the stake record against the second validator exists
        stakeRecord = stakingViewsFacet.getStakeRecord(
            operatorStakers[1],
            1,
            delegatingStaker
        );
        assertEq(stakeRecord.amount, amount);
    }
}
