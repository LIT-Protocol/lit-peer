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
import { FunctionSelectorHelper } from "./FunctionSelectorHelper.t.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { LITToken } from "../LITToken.sol";
import { FixedPointMathLib } from "solady/src/utils/FixedPointMathLib.sol";
import { SetupAndUtils } from "./SetupAndUtils.t.sol";
import { console } from "lib/forge-std/src/console.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

contract StakingTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    /// Test that, with 4 validators in 1 realm, that after 5 epochs,
    /// the reward epoch values (and global stats) are correct.
    function testFuzz_rewardEpochAndGlobalStats_1Realm(
        uint256 amount,
        uint256 timeLock
    ) public {
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);
        uint256 realmId = 1;

        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);

        // Validators join realm 1
        _setupValidators(
            realmId,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm1
        );

        uint256 numEpochsToAdvance = 5;

        for (uint256 i = 0; i < numEpochsToAdvance; i++) {
            skip(1 days);
            stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

            for (uint256 j = 0; j < operatorStakersRealm1.length; j++) {
                vm.prank(operatorStakersRealm1[j]);
                stakingValidatorFacet.signalReadyForNextEpoch(realmId, i + 1);
            }
            stakingValidatorFacet.advanceEpoch(realmId);

            // Log the reward epoch and global stats after epoch has advanced
            // Get reward epoch number
            uint256 rewardEpochNumber = stakingViewsFacet.getRewardEpochNumber(
                realmId
            );

            // Assert the stake details in reward epoch remains constant - from epoch 2 onwards.
            if (i > 0) {
                rewardEpochAssertionsRealm1.push(rewardEpochNumber);
            }

            // Assert total rewards from epoch 3 onwards since epoch 1 and 2 are not meant to have rewards.
            // We also don't assert against the last reward epoch number because it will not have any rewards - the
            // current epoch number will only have total rewards populated once it has advanced, meaning that the largest
            // reward epoch number that we can assert against is always going to be the previous epoch number compared to
            // the current.
            if (i > 1 && (i < numEpochsToAdvance - 1)) {
                rewardEpochTotalRewardAssertionsRealm1.push(rewardEpochNumber);
            }

            // Assert the global stats from epoch 3 onwards, because due to epoch 1 not having any current validators,
            // it results in the global stats for epoch 2 being 0. The first assertion will be against epoch 3 and 4.
            if (i > 1) {
                rewardEpochGlobalStatsAssertionsRealm1.push(rewardEpochNumber);
            }
        }

        // Assert the reward epoch stake details
        _assertOldNewRewardEpochsConstant(
            rewardEpochAssertionsRealm1,
            operatorStakersRealm1
        );

        // Assert the reward epoch total rewards
        _assertOldNewRewardEpochTotalRewardsConstant(
            rewardEpochTotalRewardAssertionsRealm1,
            operatorStakersRealm1
        );

        // Assert the reward epoch global stats
        _assertOldNewRewardEpochGlobalStatsConstant(
            rewardEpochGlobalStatsAssertionsRealm1
        );
    }

    /// @notice This test is when there are 2 realms and a staker in one realm unfreezes,
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_UnfreezeStake_ActiveValidator_2Realms(
        uint256 operatorStakerIndexToUnfreeze,
        uint256 amount,
        uint256 timeLock,
        bool testDelegatedStaker
    ) public {
        operatorStakerIndexToUnfreeze = bound(
            operatorStakerIndexToUnfreeze,
            0,
            3
        );
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        // Add the second realm
        stakingAdminFacet.addRealm();
        // Also allocate rewards budget for realm 2
        stakingAdminFacet.increaseRewardPool(2, rewardsBudget);

        // Setup for Realm 1
        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        address operatorToUnfreeze = operatorStakersRealm1[
            operatorStakerIndexToUnfreeze
        ];

        // Setup for Realm 2
        address[] memory operatorStakersRealm2 = _generateAddressesWithOffset(
            4,
            4
        );

        // Validators join realms
        _setupValidators(
            1,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        _setupValidators(
            2,
            operatorStakersRealm2,
            amount * 10,
            amount,
            timeLock,
            _generateUint256sWithOffset(4, 4)
        );

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorToUnfreeze);
        }

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

        // At this point, the reward epoch number of realm 1 should be 1 less than realm 2 (and will consistently be so).
        // The staker unfreezes.
        if (testDelegatedStaker) {
            vm.prank(delegatingStaker);
        } else {
            vm.prank(operatorToUnfreeze);
        }
        stakingFacet.unfreezeStake(operatorToUnfreeze, 1);

        // Get the stake record unfreeze start time.
        uint256 unfreezeStart;
        {
            if (testDelegatedStaker) {
                unfreezeStart = stakingViewsFacet
                    .getStakeRecord(operatorToUnfreeze, 1, delegatingStaker)
                    .unfreezeStart;
            } else {
                unfreezeStart = stakingViewsFacet
                    .getStakeRecord(operatorToUnfreeze, 1, operatorToUnfreeze)
                    .unfreezeStart;
            }
        }

        // From here onwards, assert that the total stake weight decreases with every epoch for that validator. For efficiency,
        // we will only assert this over 50 epochs.
        uint256 startingEpochNumber = 4;
        numEpochsToAdvance = 50;
        uint256 lastStakeWeight = type(uint256).max;
        uint256 lastGlobalStakeWeight = type(uint256).max;
        for (
            uint256 i = startingEpochNumber;
            i < startingEpochNumber + numEpochsToAdvance;
            i++
        ) {
            skip(1 hours);

            // Obtain a reference to the reward epoch and global stats before advancing.
            uint256 realm1CurrentRewardEpochNumber = stakingViewsFacet
                .getRewardEpochNumber(1);
            lastStakeWeight = stakingFacet
                .getRewardEpoch(
                    operatorToUnfreeze,
                    realm1CurrentRewardEpochNumber
                )
                .totalStakeWeight;
            lastGlobalStakeWeight = stakingViewsFacet
                .getRewardEpochGlobalStats(realm1CurrentRewardEpochNumber)
                .stakeWeight;

            // Realm 1 advances first.
            stakingValidatorFacet.lockValidatorsForNextEpoch(1);
            for (uint256 j = 0; j < operatorStakersRealm1.length; j++) {
                vm.prank(operatorStakersRealm1[j]);
                stakingValidatorFacet.signalReadyForNextEpoch(1, i);
            }
            stakingValidatorFacet.advanceEpoch(1);

            skip(1 hours);

            // Realm 2 advances next.
            stakingValidatorFacet.lockValidatorsForNextEpoch(2);
            for (uint256 j = 0; j < operatorStakersRealm2.length; j++) {
                vm.prank(operatorStakersRealm2[j]);
                stakingValidatorFacet.signalReadyForNextEpoch(2, i);
            }
            stakingValidatorFacet.advanceEpoch(2);

            // Compare the last stake weight with the current stake weight
            // We only start asserting after the block timestamp has passed 2 hours after the unfreeze start time.
            // We can't assert right after the unfreeze start because that's when the epoch advancement just picks up
            // on the slope that will be used to decrement stake weights in the next reward epoch, and the next reward epoch
            // for this realm only comes 2 hours later due to the above logic - each realm is advancing one after the other.
            // So, we use 3 hours just to have enough buffer.
            if (block.timestamp > (unfreezeStart + 3 hours)) {
                uint256 realm1NewRewardEpochNumber = stakingViewsFacet
                    .getRewardEpochNumber(1);
                LibStakingStorage.RewardEpoch
                    memory newRewardEpoch = stakingFacet.getRewardEpoch(
                        operatorToUnfreeze,
                        realm1NewRewardEpochNumber
                    );
                LibStakingStorage.RewardEpochGlobalStats
                    memory newGlobalStats = stakingViewsFacet
                        .getRewardEpochGlobalStats(realm1NewRewardEpochNumber);
                assertLt(newRewardEpoch.totalStakeWeight, lastStakeWeight);
                assertLt(newGlobalStats.stakeWeight, lastGlobalStakeWeight);
                lastStakeWeight = newRewardEpoch.totalStakeWeight;
                lastGlobalStakeWeight = newGlobalStats.stakeWeight;
            }
        }
    }

    /// @notice Test that, with 4 validators in each of 2 realms, that after 5 epochs,
    /// the reward epoch values (and global stats) are correct.
    function testFuzz_rewardEpochAndGlobalStats_2Realms(
        uint256 amount,
        uint256 timeLock
    ) public {
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        // Add the second realm
        stakingAdminFacet.addRealm();
        // Also allocate rewards budget for realm 2
        stakingAdminFacet.increaseRewardPool(2, rewardsBudget);

        // Setup for Realm 1
        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);

        // Setup for Realm 2
        address[] memory operatorStakersRealm2 = _generateAddressesWithOffset(
            4,
            4
        );
        uint256[] memory commsKeysRealm2 = _generateUint256sWithOffset(4, 4);

        // Validators join realms
        _setupValidators(
            1,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm1
        );
        _setupValidators(
            2,
            operatorStakersRealm2,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm2
        );

        // The simulation for this test will involve Realm 1 and Realm 2 having epoch
        // boundaries that are consistently 2.5 hours apart. Realm 1 will have the first
        // epoch advancement, then Realm 2, then Realm 1, then Realm 2, etc. Each realm
        // will advance 5 epochs, and then the test will check that the reward values are
        // correct.
        uint256 numEpochsToAdvance = 5;
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

            // After the 2 realms have advanced 2 epochs, their reward epoch numbers should differ by 1.
            if (i > 1) {
                assertEq(
                    stakingViewsFacet.epoch(1).rewardEpochNumber,
                    stakingViewsFacet.epoch(2).rewardEpochNumber - 1,
                    "Reward epoch numbers should differ by 1"
                );
                assertEq(
                    stakingViewsFacet.epoch(1).nextRewardEpochNumber,
                    stakingViewsFacet.epoch(2).nextRewardEpochNumber - 1,
                    "Next reward epoch numbers should differ by 1"
                );
            }

            // Get respective reward epoch numbers for each realm
            uint256 rewardEpochNumberRealm1 = stakingViewsFacet
                .getRewardEpochNumber(1);
            uint256 rewardEpochNumberRealm2 = stakingViewsFacet
                .getRewardEpochNumber(2);

            // Assert the stake weight in epoch remains constant - from epoch 2 onwards.
            if (i > 0) {
                rewardEpochAssertionsRealm1.push(rewardEpochNumberRealm1);
                rewardEpochAssertionsRealm2.push(rewardEpochNumberRealm2);
            }

            // Assert total rewards from epoch 3 onwards since epoch 1 and 2 are not meant to have rewards.
            // We also don't assert against the last reward epoch number because it will not have any rewards - the
            // current epoch number will only have total rewards populated once it has advanced, meaning that the largest
            // reward epoch number that we can assert against is always going to be the previous epoch number compared to
            // the current.
            if (i > 1 && (i < numEpochsToAdvance - 1)) {
                rewardEpochTotalRewardAssertionsRealm1.push(
                    rewardEpochNumberRealm1
                );
                rewardEpochTotalRewardAssertionsRealm2.push(
                    rewardEpochNumberRealm2
                );
            }

            // Assert the global stats from epoch 3 onwards, because due to epoch 1 not having any current validators,
            // it results in the global stats for epoch 2 being 0. The first assertion will be against epoch 3 and 4.
            if (i > 1) {
                rewardEpochGlobalStatsAssertionsRealm1.push(
                    rewardEpochNumberRealm1
                );
                rewardEpochGlobalStatsAssertionsRealm2.push(
                    rewardEpochNumberRealm2
                );
            }
        }

        // Assert the reward epoch stake details
        _assertOldNewRewardEpochsConstant(
            rewardEpochAssertionsRealm1,
            operatorStakersRealm1
        );
        _assertOldNewRewardEpochsConstant(
            rewardEpochAssertionsRealm2,
            operatorStakersRealm2
        );

        // Assert the reward epoch total rewards
        _assertOldNewRewardEpochTotalRewardsConstant(
            rewardEpochTotalRewardAssertionsRealm1,
            operatorStakersRealm1
        );
        _assertOldNewRewardEpochTotalRewardsConstant(
            rewardEpochTotalRewardAssertionsRealm2,
            operatorStakersRealm2
        );

        // Assert the reward epoch global stats
        _assertOldNewRewardEpochGlobalStatsConstant(
            rewardEpochGlobalStatsAssertionsRealm1
        );
        _assertOldNewRewardEpochGlobalStatsConstant(
            rewardEpochGlobalStatsAssertionsRealm2
        );
    }

    /// @notice This test is when the validator participates in two realms and claims rewards from both realms,
    /// that the rewards are correctly distributed to the validator.
    function testFuzz_ClaimRewards_2Realms(
        uint256 operatorStakerIndexToSwitchRealms,
        uint256 amount,
        uint256 timeLock,
        bool testDelegatingStaker
    ) public {
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);
        operatorStakerIndexToSwitchRealms = bound(
            operatorStakerIndexToSwitchRealms,
            0,
            3
        );

        // Add the second realm
        stakingAdminFacet.addRealm();
        // Also allocate rewards budget for realm 2
        stakingAdminFacet.increaseRewardPool(2, rewardsBudget);

        // Setup for Realm 1
        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);
        address operatorStakerAddressToSwitchRealms = operatorStakersRealm1[
            operatorStakerIndexToSwitchRealms
        ];
        uint256 commsKeysForOperator = commsKeysRealm1[
            operatorStakerIndexToSwitchRealms
        ];

        // Setup for Realm 2
        address[] memory operatorStakersRealm2 = _generateAddressesWithOffset(
            4,
            4
        );
        uint256[] memory commsKeysRealm2 = _generateUint256sWithOffset(4, 4);

        // Validators join realms
        _setupValidators(
            1,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm1
        );
        _setupValidators(
            2,
            operatorStakersRealm2,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm2
        );

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        if (testDelegatingStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(
                amount,
                timeLock,
                operatorStakerAddressToSwitchRealms
            );
        }

        // Advance both realms to epoch 3
        uint256 numEpochsToAdvance = 2;
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

        // Validator requests to leave realm 1
        vm.prank(operatorStakerAddressToSwitchRealms);
        stakingValidatorFacet.requestToLeave();

        address[] memory remainingValidatorsRealm1 = new address[](4);
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            if (i == operatorStakerIndexToSwitchRealms) {
                continue;
            }
            remainingValidatorsRealm1[i] = operatorStakersRealm1[i];
        }

        // Advance 1 epoch for each realm
        uint256 startingEpochNumber = 3;
        numEpochsToAdvance = 1;
        for (
            uint256 i = startingEpochNumber;
            i < startingEpochNumber + numEpochsToAdvance;
            i++
        ) {
            // Fast forward to the next day to guarantee rewards will be distributed even just for 1 epoch boundary.
            skip(1 days);

            // Realm 1 advances first.
            stakingValidatorFacet.lockValidatorsForNextEpoch(1);
            for (
                uint256 realm1StakerIdx = 0;
                realm1StakerIdx < remainingValidatorsRealm1.length;
                realm1StakerIdx++
            ) {
                if (remainingValidatorsRealm1[realm1StakerIdx] == address(0)) {
                    continue;
                }

                vm.prank(remainingValidatorsRealm1[realm1StakerIdx]);
                stakingValidatorFacet.signalReadyForNextEpoch(1, i);
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
                stakingValidatorFacet.signalReadyForNextEpoch(2, i);
            }
            stakingValidatorFacet.advanceEpoch(2);
        }

        // Claim rewards from realm 1
        uint256 balanceBefore = token.balanceOf(
            operatorStakerAddressToSwitchRealms
        );

        vm.prank(operatorStakerAddressToSwitchRealms);
        stakingFacet.claimStakeRewards(
            1,
            operatorStakerAddressToSwitchRealms,
            1,
            0
        );

        // Assert that the balance has increased
        assertGt(
            token.balanceOf(operatorStakerAddressToSwitchRealms),
            balanceBefore
        );

        if (testDelegatingStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                1,
                operatorStakerAddressToSwitchRealms,
                1,
                0
            );

            // Assert that the balance has increased
            assertGt(token.balanceOf(delegatingStaker), balanceBefore);
        }

        // Now, the validator requests to join realm 2
        vm.prank(operatorStakerAddressToSwitchRealms);
        stakingValidatorFacet.requestToJoin(2);

        // Advance both realms by 2 epochs to earn rewards for 1 epoch in realm 2
        address[] memory newValidatorsRealm2 = new address[](5);
        for (uint256 i = 0; i < operatorStakersRealm2.length; i++) {
            newValidatorsRealm2[i] = operatorStakersRealm2[i];
        }
        newValidatorsRealm2[4] = operatorStakerAddressToSwitchRealms;

        startingEpochNumber = 4;
        numEpochsToAdvance = 2;
        for (
            uint256 i = startingEpochNumber;
            i < startingEpochNumber + numEpochsToAdvance;
            i++
        ) {
            // Fast forward to the next day to guarantee rewards will be distributed even just for 1 epoch boundary.
            skip(1 days);

            // Realm 1 advances first.
            stakingValidatorFacet.lockValidatorsForNextEpoch(1);
            for (
                uint256 realm1StakerIdx = 0;
                realm1StakerIdx < remainingValidatorsRealm1.length;
                realm1StakerIdx++
            ) {
                if (remainingValidatorsRealm1[realm1StakerIdx] == address(0)) {
                    continue;
                }

                vm.prank(remainingValidatorsRealm1[realm1StakerIdx]);
                stakingValidatorFacet.signalReadyForNextEpoch(1, i);
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
                realm2StakerIdx < newValidatorsRealm2.length;
                realm2StakerIdx++
            ) {
                vm.prank(newValidatorsRealm2[realm2StakerIdx]);
                stakingValidatorFacet.signalReadyForNextEpoch(2, i);
            }
            stakingValidatorFacet.advanceEpoch(2);
        }

        // Claim rewards from realm 2
        balanceBefore = token.balanceOf(operatorStakerAddressToSwitchRealms);

        vm.prank(operatorStakerAddressToSwitchRealms);
        stakingFacet.claimStakeRewards(
            2,
            operatorStakerAddressToSwitchRealms,
            1,
            0
        );

        // Assert that the balance has increased
        assertGt(
            token.balanceOf(operatorStakerAddressToSwitchRealms),
            balanceBefore
        );

        if (testDelegatingStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                2,
                operatorStakerAddressToSwitchRealms,
                1,
                0
            );

            // Assert that the balance has increased
            assertGt(token.balanceOf(delegatingStaker), balanceBefore);
        }
    }

    /// @notice This test is when a node operator / validator calls stakeAndJoin
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_StakeAndJoin_ValidatorState(
        uint256 operatorStakerIndex,
        uint256 amount,
        uint256 timeLock
    ) public {
        // Input assumptions
        uint256 realmId = 1;
        operatorStakerIndex = bound(operatorStakerIndex, 0, 3);
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);

        // Validators join realm 1
        _setupValidators(
            realmId,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm1
        );

        address operatorStakerAddressToAssert = operatorStakersRealm1[
            operatorStakerIndex
        ];

        // Assertions
        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(
                operatorStakerAddressToAssert,
                1,
                operatorStakerAddressToAssert
            );
        LibStakingStorage.Epoch memory epoch = stakingViewsFacet.epoch(1);
        uint256 currentRewardEpochNumber = epoch.rewardEpochNumber;
        assertEq(
            currentRewardEpochNumber,
            1,
            "currentRewardEpochNumber should be 1"
        );

        // Assert that the current reward epoch is not attributed to and is just initialized to zero values
        vm.prank(address(staking));
        LibStakingStorage.RewardEpoch memory currentRE = stakingFacet
            .getRewardEpoch(
                operatorStakerAddressToAssert,
                currentRewardEpochNumber
            );
        _assertRewardEpochIsZero(currentRE);

        // Advance to epoch 2 in order to assert the attribution of the stake against the next RE.
        _advanceEpochs(1, 1, operatorStakersRealm1, 1);

        // Assert that the next reward epoch number is incremented by 1
        uint256 nextRewardEpochNumber = stakingViewsFacet
            .epoch(1)
            .rewardEpochNumber;
        assertEq(nextRewardEpochNumber, currentRewardEpochNumber + 1);

        // Assert that the next reward epoch is attributed to by the staker
        LibStakingStorage.RewardEpoch memory nextRE = stakingFacet
            .getRewardEpoch(
                operatorStakerAddressToAssert,
                nextRewardEpochNumber
            );
        _assertNewRewardEpoch(nextRE, amount);
        assertEq(
            stakingViewsFacet
                .getRewardEpochGlobalStats(nextRewardEpochNumber)
                .stakeAmount,
            amount * 4 // 4 validators
        );
        assertEq(
            token.balanceOf(address(staking)),
            (amount * 4) + rewardsBudget
        );

        // Assert the stake record is created correctly
        assertEq(stakeRecord.amount, amount);
        assertEq(stakeRecord.initialSharePrice, nextRE.validatorSharePrice);
        assertEq(stakeRecord.frozen, true);
        assertEq(stakeRecord.loaded, true);
    }

    /// @notice This test is when 3 realms are created, and then the 2nd realm is removed,
    /// that validators are still able to stake and join the 3rd realm (now being the 2nd
    /// active realm).
    function testFuzz_StakeAndJoinAfterRemovingMiddleRealm(
        uint256 amount,
        uint256 timeLock
    ) public {
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        // Add the second realm
        stakingAdminFacet.addRealm();
        // Add the third realm
        stakingAdminFacet.addRealm();
        // Remove the second realm
        stakingAdminFacet.removeRealm(2);

        // Set up the validators for realm 1
        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);
        _setupValidators(
            1,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm1
        );

        // Set up the validators for realm 3
        address[] memory operatorStakersRealm3 = _generateAddressesWithOffset(
            4,
            4
        );
        uint256[] memory commsKeysRealm3 = _generateUint256sWithOffset(4, 4);
        _setupValidators(
            3,
            operatorStakersRealm3,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm3
        );
    }

    /// @notice This test is when a node operator uses invalid parameters for setting
    /// validator info after previously having requested to join a realm, that the
    /// call reverts.
    function testFuzz_SetValidatorInfoWithInvalidParameters(
        uint256 operatorStakerIndex,
        uint256 amount,
        uint256 timeLock
    ) public {
        // Input assumptions
        uint256 realmId = 1;
        operatorStakerIndex = bound(operatorStakerIndex, 0, 3);
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        // Set up the validators
        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeys = _generateUint256s(4);
        _setupValidators(
            realmId,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            commsKeys
        );
        address operatorStakerAddressToAssert = operatorStakersRealm1[
            operatorStakerIndex
        ];
        uint256 someOtherOperatorIndex = (operatorStakerIndex + 1) % 4;

        // Set validator information - fails for zero ip
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.ValueMustBeNonzero.selector,
                "ip"
            )
        );
        vm.prank(operatorStakerAddressToAssert);
        stakingValidatorFacet.setIpPortNodeAddress(0, 0, 0, address(0));

        // Set validator information - fails for zero port
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.ValueMustBeNonzero.selector,
                "port"
            )
        );
        vm.prank(operatorStakerAddressToAssert);
        stakingValidatorFacet.setIpPortNodeAddress(1, 0, 0, address(0));

        // Set validator information - fails for zero node address
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.ValueMustBeNonzero.selector,
                "operatorAddress"
            )
        );
        vm.prank(operatorStakerAddressToAssert);
        stakingValidatorFacet.setIpPortNodeAddress(1, 1, 1, address(0));
    }

    /// @notice This test is when a node operator / validator calls requestToLeave
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_RequestToLeave(
        uint256 operatorStakerIndex,
        uint256 amount,
        uint256 timeLock,
        bool testDelegatedStaker
    ) public {
        // Input assumptions
        uint256 realmId = 1;
        operatorStakerIndex = bound(operatorStakerIndex, 0, 3);
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);

        // Validators join realm 1
        _setupValidators(
            realmId,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm1
        );

        address operatorStakerAddressToAssert = operatorStakersRealm1[
            operatorStakerIndex
        ];

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorStakerAddressToAssert);
        }

        // Advance to epoch 2 to deal in the validators
        _advanceEpochs(1, 1, operatorStakersRealm1, 1);

        // Assert that the new reward epoch is attributed to by each staker
        uint256 rewardEpochNumber = stakingViewsFacet.getRewardEpochNumber(1);
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
                .getRewardEpoch(operatorStakersRealm1[i], rewardEpochNumber);

            if (i == operatorStakerIndex && testDelegatedStaker) {
                _assertNewRewardEpoch(rewardEpoch, amount * 2);
            } else {
                _assertNewRewardEpoch(rewardEpoch, amount);
            }
        }

        if (testDelegatedStaker) {
            // Assert the new global stats
            assertEq(
                stakingViewsFacet
                    .getRewardEpochGlobalStats(rewardEpochNumber)
                    .stakeAmount,
                amount * 5 // 5 stakers
            );
            // Assert the token balance
            assertEq(
                token.balanceOf(address(staking)),
                (amount * 5) + rewardsBudget
            );
        } else {
            // Assert the new global stats
            assertEq(
                stakingViewsFacet
                    .getRewardEpochGlobalStats(rewardEpochNumber)
                    .stakeAmount,
                amount * 4 // 4 validators
            );
            // Assert the token balance
            assertEq(
                token.balanceOf(address(staking)),
                (amount * 4) + rewardsBudget
            );
        }

        // Now, the staker requests to leave
        vm.prank(operatorStakerAddressToAssert);
        stakingValidatorFacet.requestToLeaveAsNode(1);

        // Advance to epoch 3 to deal out the staker who requested to leave
        // Only have the 3 other validators to signal ready
        address[] memory validatorsToSignalReady = new address[](4);
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            if (i == operatorStakerIndex) {
                continue;
            }
            validatorsToSignalReady[i] = operatorStakersRealm1[i];
        }
        _advanceEpochs(1, 1, validatorsToSignalReady, 2);

        // Assert that the new reward epoch is NOT attributed to the staker who requested to leave
        rewardEpochNumber = stakingViewsFacet.getRewardEpochNumber(1);
        LibStakingStorage.RewardEpoch memory rewardEpochZero = stakingFacet
            .getRewardEpoch(operatorStakerAddressToAssert, rewardEpochNumber);
        _assertRewardEpochIsZero(rewardEpochZero);

        // Assert the new global stats
        assertEq(
            stakingViewsFacet
                .getRewardEpochGlobalStats(rewardEpochNumber)
                .stakeAmount,
            amount * 3 // 3 validators
        );

        if (testDelegatedStaker) {
            // Assert the token balance is still the same (not withdrawn yet)
            assertEq(
                token.balanceOf(address(staking)),
                (amount * 5) + rewardsBudget
            );
        } else {
            // Assert the token balance is still the same (not withdrawn yet)
            assertEq(
                token.balanceOf(address(staking)),
                (amount * 4) + rewardsBudget
            );
        }
    }

    /// @notice This test is when a node operator / validator gets kicked
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_KickValidator(
        uint256 operatorStakerIndex,
        uint256 amount,
        uint256 timeLock,
        bool withSlashing,
        bool testDelegatedStaker
    ) public {
        // Input assumptions
        uint256 realmId = 1;
        operatorStakerIndex = bound(operatorStakerIndex, 0, 3);
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        // Set complaint config for reason 1
        LibStakingStorage.ComplaintConfig
            memory complaintConfig = LibStakingStorage.ComplaintConfig({
                tolerance: 6,
                intervalSecs: 90,
                kickPenaltyPercent: 0.5 ether, // 50%
                kickPenaltyDemerits: 1
            });
        if (withSlashing) {
            complaintConfig.kickPenaltyDemerits = 10; // = demerit threshold
        }
        stakingAdminFacet.setComplaintConfig(1, complaintConfig);

        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);

        // Validators join realm 1
        _setupValidators(
            realmId,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            commsKeysRealm1
        );

        address operatorStakerAddressToKick = operatorStakersRealm1[
            operatorStakerIndex
        ];

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorStakerAddressToKick);
        }

        // Advance to epoch 4:
        // - epoch 2 is necessary to deal in validators
        // - epoch 3 is necessary to get full round of rewards (epoch 2 does not have rewards)
        // - epoch 4 is when the validator gets kicked
        _advanceEpochs(1, 3, operatorStakersRealm1, 1);

        // Now, have all other validators vote to kick the validator
        address[] memory validatorsNotKicked = new address[](4);
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            if (i == operatorStakerIndex) {
                continue;
            }
            validatorsNotKicked[i] = operatorStakersRealm1[i];
        }

        for (uint256 i = 0; i < validatorsNotKicked.length; i++) {
            address validator = validatorsNotKicked[i];
            if (validator == address(0)) {
                continue;
            }

            vm.prank(validator);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                operatorStakerAddressToKick,
                1,
                ""
            );
        }

        // Advance to epoch 5 to deal out the validator who was kicked.
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        _advanceEpochs(realmId, 1, validatorsNotKicked, 4);

        // Assert rewards from epoch 4 and epoch 3 for this validator.
        LibStakingStorage.RewardEpoch memory epoch3RewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerAddressToKick, 3);
        LibStakingStorage.RewardEpoch memory epoch4RewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerAddressToKick, 4);
        if (withSlashing) {
            // Assert that the rewards from epoch 4 are roughly half than that of epoch 3 for this validator.
            assertApproxEqRel(
                epoch4RewardEpoch.totalStakeRewards,
                epoch3RewardEpoch.totalStakeRewards / 2,
                0.2 ether, // allow a large-ish delta (20%)
                "rewards from epoch 4 are not roughly half of that of epoch 3"
            );
        } else {
            // Assert that the rewards from epoch 4 are the same as that of epoch 3 for this validator.
            assertEq(
                epoch4RewardEpoch.totalStakeRewards,
                epoch3RewardEpoch.totalStakeRewards,
                "rewards from epoch 4 are not the same as that of epoch 3"
            );
        }

        // Assert that the new reward epoch is NOT attributed to the validator who was kicked.
        _assertRewardEpochIsZero(
            stakingFacet.getRewardEpoch(operatorStakerAddressToKick, 5)
        );

        // Assert the new global stats
        assertEq(
            stakingViewsFacet.getRewardEpochGlobalStats(5).stakeAmount,
            amount * 3 // 3 validators
        );

        // Assert the token balance is still the same (not withdrawn yet)
        if (testDelegatedStaker) {
            uint256 expectedTokenBalance = (amount * 5) + rewardsBudget;
            assertEq(token.balanceOf(address(staking)), expectedTokenBalance);
        } else {
            uint256 expectedTokenBalance = (amount * 4) + rewardsBudget;
            assertEq(token.balanceOf(address(staking)), expectedTokenBalance);
        }

        // Now, let's have the kicked validator unfreeze and withdraw in order to assert the stake and rewards claimed.
        vm.prank(operatorStakerAddressToKick);
        stakingFacet.unfreezeStake(operatorStakerAddressToKick, 1);

        if (testDelegatedStaker) {
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(operatorStakerAddressToKick, 1);
        }

        // Fast forward to after the timelock has fully decayed.
        skip(800 days);

        // Assert that the rewards claimed for this validator is correct (epoch 3 and 4's rewards)
        {
            uint256 expectedRewardToClaimPerStaker;
            {
                if (testDelegatedStaker) {
                    expectedRewardToClaimPerStaker =
                        (epoch3RewardEpoch.totalStakeRewards +
                            epoch4RewardEpoch.totalStakeRewards) /
                        2;
                } else {
                    expectedRewardToClaimPerStaker =
                        epoch3RewardEpoch.totalStakeRewards +
                        epoch4RewardEpoch.totalStakeRewards;
                }
            }
            uint256 balanceBefore = token.balanceOf(
                operatorStakerAddressToKick
            );

            vm.prank(operatorStakerAddressToKick);
            stakingFacet.claimStakeRewards(
                1,
                operatorStakerAddressToKick,
                1,
                0
            );

            // Note that this will not be exact due to the lack of precision in the reward calculations in claimStakeRewards.
            assertApproxEqRel(
                token.balanceOf(operatorStakerAddressToKick),
                balanceBefore + expectedRewardToClaimPerStaker,
                0.001 ether // use a small delta (0.1%)
            );

            if (testDelegatedStaker) {
                balanceBefore = token.balanceOf(delegatingStaker);

                vm.prank(delegatingStaker);
                stakingFacet.claimStakeRewards(
                    1,
                    operatorStakerAddressToKick,
                    1,
                    0
                );

                // Note that this will not be exact due to the lack of precision in the reward calculations in claimStakeRewards.
                assertApproxEqRel(
                    token.balanceOf(delegatingStaker),
                    balanceBefore + expectedRewardToClaimPerStaker,
                    0.001 ether // use a small delta (0.1%)
                );
            }
        }

        // Assert that the stake withdrawn for this validator is correct.
        uint256 balanceBefore = token.balanceOf(operatorStakerAddressToKick);

        vm.prank(operatorStakerAddressToKick);
        stakingFacet.withdraw(operatorStakerAddressToKick, 1);

        uint256 expectedWithdrawAmount;
        if (withSlashing) {
            expectedWithdrawAmount = amount / 2;
        } else {
            expectedWithdrawAmount = amount;
        }
        assertEq(
            token.balanceOf(operatorStakerAddressToKick),
            balanceBefore + expectedWithdrawAmount
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.withdraw(operatorStakerAddressToKick, 1);

            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore + expectedWithdrawAmount
            );
        }

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Get the current epoch number for realm 1
        uint256 realm1CurrentEpochNumber = stakingViewsFacet.epoch(1).number;

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            validatorsNotKicked,
            realm1CurrentEpochNumber
        );
    }

    /// @notice This test is when a staker tries to stake an invalid amount.
    function testFuzz_StakeInvalidAmount(
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        bool invalidAmountRevert;

        {
            // Set the min and max stake amount to be within the bounds of amount.
            LibStakingStorage.GlobalConfig
                memory globalConfig = stakingViewsFacet.globalConfig();
            globalConfig.minStakeAmount = 60 ether;
            globalConfig.maxStakeAmount = 80 ether;
            stakingAdminFacet.setConfig(globalConfig);

            if (amount < 60 ether || amount > 80 ether) {
                invalidAmountRevert = true;
            }
        }

        address staker = address(0x1);
        _fundAddressWithTokensAndApprove(staker, amount * 2);

        if (invalidAmountRevert) {
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingFacet.StakeAmountNotMet.selector,
                    amount
                )
            );
        } else {
            vm.expectEmit(true, true, true, true);
            emit StakeRecordCreated(staker, 1, amount, staker);
        }

        // Stake
        vm.prank(staker);
        stakingFacet.stake(amount, timeLock, staker);
    }

    /// @notice This test is when a validator tries to increase the stake amount of a stake record
    /// beyond the configured max stake amount, that it will revert.
    function testFuzz_IncreaseStakeRecordAmountInvalidAmount(
        uint256 operatorStakerIndexToIncreaseAmount,
        uint256 amount,
        uint256 timeLock,
        uint256 additionalAmount
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 100 ether);
        additionalAmount = bound(additionalAmount, 1 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToIncreaseAmount = bound(
            operatorStakerIndexToIncreaseAmount,
            0,
            3
        );
        bool invalidAmountRevert;

        {
            // Set the max stake amount so that increasing the stake amount by an amount that
            // exceeds the max stake amount will revert.
            LibStakingStorage.GlobalConfig
                memory globalConfig = stakingViewsFacet.globalConfig();
            globalConfig.maxStakeAmount = 100 ether + 20 ether;
            stakingAdminFacet.setConfig(globalConfig);

            if (amount + additionalAmount > globalConfig.maxStakeAmount) {
                invalidAmountRevert = true;
            }
        }

        // Set up the validators
        address[] memory operatorStakers = _generateAddresses(4);
        _setupValidators(
            realmId,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        address operatorStakerAddressToIncreaseAmount = operatorStakers[
            operatorStakerIndexToIncreaseAmount
        ];

        if (invalidAmountRevert) {
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingFacet.StakeAmountNotMet.selector,
                    amount + additionalAmount
                )
            );
        }

        // Validator increases the stake amount
        vm.prank(operatorStakerAddressToIncreaseAmount);
        stakingFacet.increaseStakeRecordAmount(
            operatorStakerAddressToIncreaseAmount,
            1,
            additionalAmount
        );
    }

    /// @notice This test is when a validator tries to split a stake record into two separate stake records
    /// with a ratio that results in a stake record amount that is less than the configured min stake amount, that it will revert.
    function testFuzz_SplitStakeRecordInvalidRatio(
        uint256 operatorStakerIndexToSplit,
        uint256 amount,
        uint256 timeLock,
        uint256 ratio
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        ratio = bound(ratio, 0.001 ether, 0.999 ether);
        operatorStakerIndexToSplit = bound(operatorStakerIndexToSplit, 0, 3);
        uint256 willRevert; // 0 = will not revert, 1 = first stake record will revert, 2 = second stake record will revert

        uint256 firstAmountSplit = (amount * ratio) / 1 ether;
        uint256 secondAmountSplit = amount - firstAmountSplit;

        {
            // Set the min stake amount so that splitting the stake amount by a ratio that
            // results in a stake amount that is less than the min stake amount will revert.
            LibStakingStorage.GlobalConfig
                memory globalConfig = stakingViewsFacet.globalConfig();
            globalConfig.minStakeAmount = 16 ether;
            stakingAdminFacet.setConfig(globalConfig);

            if (firstAmountSplit < globalConfig.minStakeAmount) {
                willRevert = 1;
            }
            if (secondAmountSplit < globalConfig.minStakeAmount) {
                willRevert = 2;
            }
        }

        // Set up the validators
        address[] memory operatorStakers = _generateAddresses(4);
        _setupValidators(
            realmId,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        address operatorStakerAddressToSplit = operatorStakers[
            operatorStakerIndexToSplit
        ];

        if (willRevert == 1) {
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingFacet.StakeAmountNotMet.selector,
                    firstAmountSplit
                )
            );
        }
        if (willRevert == 2) {
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingFacet.StakeAmountNotMet.selector,
                    secondAmountSplit
                )
            );
        }

        // Validator splits the stake amount
        vm.prank(operatorStakerAddressToSplit);
        stakingFacet.splitStakeRecord(operatorStakerAddressToSplit, 1, ratio);
    }

    /// @notice This test is when a staker tries to withdraw
    /// a stake that is frozen, they fail.
    function testFuzz_CannotWithdrawFrozen(
        uint256 operatorStakerIndex,
        uint256 amount,
        uint256 timeLock,
        uint256 randomKey,
        bool testDelegatedStaker
    ) public {
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _fundAddressesWithTokensAndApprove(randomOperatorStakers, 1_000 ether);

        // Input assumptions
        uint256 realmId = 1;
        operatorStakerIndex = bound(operatorStakerIndex, 0, 3);
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);
        randomKey = bound(randomKey, 1, 1000000000);

        address operatorStakerAddress = randomOperatorStakers[
            operatorStakerIndex
        ];

        // Set the IP and port of the validator
        vm.prank(operatorStakerAddress);
        stakingValidatorFacet.setIpPortNodeAddress(
            1,
            1,
            1,
            operatorStakerAddress
        );

        // Emit event check
        vm.expectEmit(true, true, true, true);
        emit StakeRecordCreated(
            operatorStakerAddress,
            1,
            amount,
            operatorStakerAddress
        );

        // Call stakeAndJoin
        vm.prank(operatorStakerAddress);
        stakingFacet.stake(amount, timeLock, operatorStakerAddress);
        vm.prank(operatorStakerAddress);
        stakingValidatorFacet.requestToJoin(realmId);

        // Set up delegated staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorStakerAddress);
        }

        // Expect revert when trying to withdraw
        vm.expectRevert(StakingFacet.CannotWithdrawFrozen.selector);
        vm.prank(operatorStakerAddress);
        stakingFacet.withdraw(operatorStakerAddress, 1);

        if (testDelegatedStaker) {
            vm.expectRevert(StakingFacet.CannotWithdrawFrozen.selector);
            vm.prank(delegatingStaker);
            stakingFacet.withdraw(operatorStakerAddress, 1);
        }
    }

    /// @notice This test is when a node operator / validator calls unfreezeStake
    /// that the stake record is unfrozen.
    function testFuzz_Unfreeze(
        uint256 operatorStakerIndex,
        uint256 amount,
        uint256 timeLock,
        uint256 randomKey,
        bool testDelegatedStaker
    ) public {
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _fundAddressesWithTokensAndApprove(randomOperatorStakers, 1_000 ether);

        // Input assumptions
        uint256 realmId = 1;
        operatorStakerIndex = bound(operatorStakerIndex, 0, 3);
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);
        randomKey = bound(randomKey, 1, 1000000000);

        address operatorStakerAddress = randomOperatorStakers[
            operatorStakerIndex
        ];

        // Set the IP and port of the validator
        vm.prank(operatorStakerAddress);
        stakingValidatorFacet.setIpPortNodeAddress(
            1,
            1,
            1,
            operatorStakerAddress
        );

        // Emit event check
        vm.expectEmit(true, true, true, true);
        emit StakeRecordCreated(
            operatorStakerAddress,
            1,
            amount,
            operatorStakerAddress
        );

        // Call stakeAndJoin
        vm.prank(operatorStakerAddress);
        stakingFacet.stake(amount, timeLock, operatorStakerAddress);
        vm.prank(operatorStakerAddress);
        stakingValidatorFacet.requestToJoin(realmId);

        // Set up delegated staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorStakerAddress);
        }

        vm.prank(operatorStakerAddress);
        stakingFacet.unfreezeStake(operatorStakerAddress, 1);

        bool unfreezeStatus = stakingViewsFacet
            .getStakeRecord(operatorStakerAddress, 1, operatorStakerAddress)
            .frozen;
        assertEq(unfreezeStatus, false);

        if (testDelegatedStaker) {
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(operatorStakerAddress, 1);

            unfreezeStatus = stakingViewsFacet
                .getStakeRecord(operatorStakerAddress, 1, delegatingStaker)
                .frozen;
            assertEq(unfreezeStatus, false);
        }
    }

    /// @notice This test is when a node operator / validator calls withdraw
    /// that the stake record is withdrawn.
    function testFuzz_Withdraw(
        uint256 operatorStakerIndexToLeave,
        uint256 amount,
        uint256 timeLock,
        bool testDelegatedStaker
    ) public {
        uint256 realmId = 1;
        // Input assumptions
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToLeave = bound(operatorStakerIndexToLeave, 0, 3);

        address[] memory randomOperatorStakers = _generateAddresses(4);
        uint256[] memory randomCommsKeys = _generateUint256s(4);
        _setupValidators(
            realmId,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            randomCommsKeys
        );
        address operatorStakerAddressToLeave = randomOperatorStakers[
            operatorStakerIndexToLeave
        ];

        // Set up delegated staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorStakerAddressToLeave);
        }

        // Advance to epoch 3
        uint256 epochNumber = _advanceEpochs(
            realmId,
            2,
            randomOperatorStakers,
            1
        );

        // The validator will request to leave in the 3rd epoch.
        vm.prank(operatorStakerAddressToLeave);
        stakingValidatorFacet.requestToLeaveAsNode(realmId);

        // Remaining validators advance 1 more epoch to epoch 4.
        address[] memory remainingValidators = new address[](4);
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (i == operatorStakerIndexToLeave) {
                continue;
            }
            remainingValidators[i] = randomOperatorStakers[i];
        }
        epochNumber = _advanceEpochs(
            realmId,
            1,
            remainingValidators,
            epochNumber
        );

        // After the validator has left, unfreeze the validator
        vm.prank(operatorStakerAddressToLeave);
        stakingFacet.unfreezeStake(operatorStakerAddressToLeave, 1);
        bool unfreezeStatus = stakingViewsFacet
            .getStakeRecord(
                operatorStakerAddressToLeave,
                1,
                operatorStakerAddressToLeave
            )
            .frozen;
        assertEq(unfreezeStatus, false);

        if (testDelegatedStaker) {
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(operatorStakerAddressToLeave, 1);

            unfreezeStatus = stakingViewsFacet
                .getStakeRecord(
                    operatorStakerAddressToLeave,
                    1,
                    delegatingStaker
                )
                .frozen;
            assertEq(unfreezeStatus, false);
        }

        {
            // Fast forward to JUST after the timelock has fully decayed.
            uint256 epochLength = stakingViewsFacet.epoch(realmId).epochLength;
            uint256 roundedTimeLock = (timeLock / 86400) * 86400;
            uint256 remainingTimeToAdvance = roundedTimeLock + 1 days;
            skip(remainingTimeToAdvance);
        }

        // Assert that the rewards claimed for this validator is correct (just epoch 3's rewards)
        LibStakingStorage.RewardEpoch memory epoch3RewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerAddressToLeave, 3);
        uint256 expectedRewardToClaim;
        {
            if (testDelegatedStaker) {
                expectedRewardToClaim = epoch3RewardEpoch.totalStakeRewards / 2;
            } else {
                expectedRewardToClaim = epoch3RewardEpoch.totalStakeRewards;
            }
        }
        uint256 balanceBefore = token.balanceOf(operatorStakerAddressToLeave);

        vm.prank(operatorStakerAddressToLeave);
        stakingFacet.claimStakeRewards(1, operatorStakerAddressToLeave, 1, 0);

        // Note that this will not be exact due to the lack of precision in the reward calculations in claimStakeRewards.
        assertApproxEqRel(
            token.balanceOf(operatorStakerAddressToLeave),
            balanceBefore + expectedRewardToClaim,
            0.001 ether // use a small delta (0.1%)
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                1,
                operatorStakerAddressToLeave,
                1,
                0
            );

            assertApproxEqRel(
                token.balanceOf(delegatingStaker),
                balanceBefore + expectedRewardToClaim,
                0.001 ether // use a small delta (0.1%)
            );
        }

        // Assert exact withdrawn stake amount.
        balanceBefore = token.balanceOf(operatorStakerAddressToLeave);

        vm.prank(operatorStakerAddressToLeave);
        stakingFacet.withdraw(operatorStakerAddressToLeave, 1);

        assertEq(
            token.balanceOf(operatorStakerAddressToLeave),
            balanceBefore + amount
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.withdraw(operatorStakerAddressToLeave, 1);

            assertEq(token.balanceOf(delegatingStaker), balanceBefore + amount);
        }

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Get the current epoch number for realm 1
        uint256 realm1CurrentEpochNumber = stakingViewsFacet.epoch(1).number;

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            remainingValidators,
            realm1CurrentEpochNumber
        );
    }

    /// @notice This test is when a delegating staker withdraws their stake from an active validator
    /// that the reward epoch and global stats are updated correctly.
    function test_DelegatingStakerWithdraws_ActiveValidator() public {
        uint256 realmId = 1;
        uint256 amount = 100 ether;
        uint256 timeLock = 15 days;
        uint256 operatorStakerIndexToStakeAgainst = 0;
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;
        uint256 currentEpochNumber = 1;

        vm.pauseGasMetering();
        {
            // Set the config
            LibStakingStorage.GlobalConfig
                memory globalConfig = stakingViewsFacet.globalConfig();
            globalConfig.minTimeLock = timeLock - 1 days;
            stakingAdminFacet.setConfig(globalConfig);
        }

        // Set up the validators
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _setupValidators(
            realmId,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        address operatorStakerAddressToStakeAgainst = randomOperatorStakers[
            operatorStakerIndexToStakeAgainst
        ];

        // Set up delegated staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(
            amount,
            timeLock,
            operatorStakerAddressToStakeAgainst
        );

        // Advance to epoch 3
        currentEpochNumber = _advanceEpochs(
            realmId,
            2,
            randomOperatorStakers,
            currentEpochNumber
        );

        // Assert that the reward epoch and global stats are updated correctly
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerAddressToStakeAgainst, 3);
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(3);
        uint256 stakeWeightPerValidator = stakingViewsFacet
            .calculateStakeWeight(roundedTimeLock, amount);
        assertEq(rewardEpoch.totalStakeWeight, stakeWeightPerValidator * 2);
        assertEq(rewardEpoch.stakeAmount, amount * 2);
        assertEq(globalStats.stakeWeight, stakeWeightPerValidator * 5);
        assertEq(globalStats.stakeAmount, amount * 5);

        // Delegating staker unfreezes their stake.
        vm.prank(delegatingStaker);
        stakingFacet.unfreezeStake(operatorStakerAddressToStakeAgainst, 1);

        // Delegating staker fails to withdraw their stake immediately after unfreezing
        vm.expectRevert(StakingFacet.TimeLockNotMet.selector);
        vm.prank(delegatingStaker);
        stakingFacet.withdraw(operatorStakerAddressToStakeAgainst, 1);

        {
            // Delegating staker withdraws their stake after the timelock has fully decayed.
            uint256 epochLength = stakingViewsFacet.epoch(realmId).epochLength;
            uint256 epochsToAdvance = ((roundedTimeLock + 1 days) /
                epochLength) + 1; // Add additional day because unfreezing starts at the beginning of the next day
            vm.pauseGasMetering();
            currentEpochNumber = _advanceEpochs(
                realmId,
                epochsToAdvance,
                randomOperatorStakers,
                currentEpochNumber
            );
            vm.resumeGasMetering();
        }

        // Delegating staker claims their rewards
        uint256 balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerAddressToStakeAgainst,
            1,
            type(uint256).max
        );

        assertGt(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Delegating staker should have received rewards"
        );

        vm.prank(delegatingStaker);
        stakingFacet.withdraw(operatorStakerAddressToStakeAgainst, 1);

        // Assert that the reward epoch and global stats are updated correctly
        rewardEpoch = stakingFacet.getRewardEpoch(
            operatorStakerAddressToStakeAgainst,
            currentEpochNumber
        );
        globalStats = stakingViewsFacet.getRewardEpochGlobalStats(
            currentEpochNumber
        );
        assertApproxEqRel(
            rewardEpoch.totalStakeWeight,
            stakeWeightPerValidator,
            0.000001 ether,
            "reward epoch stake weight should be from 1 validator"
        );
        assertEq(
            rewardEpoch.stakeAmount,
            amount,
            "reward epoch stake amount should be from 1 validator"
        );
        assertApproxEqRel(
            globalStats.stakeWeight,
            stakeWeightPerValidator * 4,
            0.000001 ether,
            "global stats stake weight should be from 4 validators"
        );
        assertEq(
            globalStats.stakeAmount,
            amount * 4,
            "global stats stake amount should be from 4 validators"
        );

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Get the current epoch number for realm 1
        uint256 realm1CurrentEpochNumber = stakingViewsFacet.epoch(1).number;

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            randomOperatorStakers,
            realm1CurrentEpochNumber
        );
    }

    /// @notice This test checks the admin slash validator function for a validator who has already been
    /// kicked and slashed from the validator set previously. It is not intuitive why admins may wish to
    /// manually slash a validator who has already been kicked and slashed, but regardless this test is
    /// necessary to ensure that this manual function call will not result in duplicated rewards for
    /// the validator.
    function testFuzz_AdminSlashValidator_SlashedValidator(
        uint256 operatorStakerIndexToSlash,
        uint256 amount,
        uint256 timeLock,
        uint256 commissionRate
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        commissionRate = bound(commissionRate, 1_000_000, (1 ether - 1));
        operatorStakerIndexToSlash = bound(operatorStakerIndexToSlash, 0, 3);

        // Set up the validators
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _setupValidators(
            realmId,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        address operatorStakerAddressToSlash = randomOperatorStakers[
            operatorStakerIndexToSlash
        ];

        // Set the commission rate
        vm.prank(operatorStakerAddressToSlash);
        stakingFacet.setValidatorCommissionRate(commissionRate);

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Validator gets kicked and slashed
        address[] memory remainingValidators = new address[](4);
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (randomOperatorStakers[i] == operatorStakerAddressToSlash) {
                continue;
            }
            remainingValidators[i] = randomOperatorStakers[i];
        }
        for (uint256 i = 0; i < remainingValidators.length; i++) {
            if (remainingValidators[i] == address(0)) {
                continue;
            }

            vm.prank(remainingValidators[i]);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                operatorStakerAddressToSlash,
                1,
                ""
            );
        }

        // Advance to epoch 4
        // We expect a revert here because the validator is not in the active validator set
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        _advanceEpochs(realmId, 1, remainingValidators, 3);

        // Validator claims the commission and fixed cost rewards
        {
            uint256 balanceBefore = token.balanceOf(
                operatorStakerAddressToSlash
            );
            vm.prank(operatorStakerAddressToSlash);
            stakingFacet.claimValidatorCommission(realmId, 0);

            // Assert that the balance has increased by the commission
            assertGt(
                token.balanceOf(operatorStakerAddressToSlash),
                balanceBefore,
                "Validator should have claimed non-zero commission in epoch 3"
            );
        }
        {
            uint256 balanceBefore = token.balanceOf(
                operatorStakerAddressToSlash
            );
            vm.prank(operatorStakerAddressToSlash);
            stakingFacet.claimFixedCostRewards(realmId, 0);

            // Assert that the balance has increased by the fixed cost rewards
            assertGt(
                token.balanceOf(operatorStakerAddressToSlash),
                balanceBefore,
                "Validator should have claimed non-zero fixed cost rewards in epoch 3"
            );
        }

        // Slash the validator
        stakingAdminFacet.adminSlashValidator(
            0.5 ether, // 5% penalty
            operatorStakerAddressToSlash
        );

        // Validator attempts to claim the commission and fixed cost rewards again, but
        // they should not get anything.
        {
            uint256 balanceBefore = token.balanceOf(
                operatorStakerAddressToSlash
            );
            vm.prank(operatorStakerAddressToSlash);
            stakingFacet.claimValidatorCommission(realmId, 0);

            assertEq(
                token.balanceOf(operatorStakerAddressToSlash),
                balanceBefore,
                "Validator should not have claimed any commission"
            );
        }
        {
            uint256 balanceBefore = token.balanceOf(
                operatorStakerAddressToSlash
            );
            vm.prank(operatorStakerAddressToSlash);
            stakingFacet.claimFixedCostRewards(realmId, 0);

            assertEq(
                token.balanceOf(operatorStakerAddressToSlash),
                balanceBefore,
                "Validator should not have claimed any fixed cost rewards"
            );
        }
    }

    /// @notice This test checks the reward epoch and global stats are correct when an active validator gets kicked without being slashed,
    /// then when the epoch advances multiple times, the validator is eventually slashed since it did not rejoin within the pending rejoin timeout.
    function testFuzz_ActiveValidatorSlashedViaRejoinTimeout(
        uint256 operatorStakerIndexToKick,
        uint256 amount,
        uint256 timeLock,
        bool testDelegatedStaker
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToKick = bound(operatorStakerIndexToKick, 0, 3);
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        {
            // Set complaint config for reason 1
            LibStakingStorage.ComplaintConfig
                memory complaintConfig = LibStakingStorage.ComplaintConfig({
                    tolerance: 6,
                    intervalSecs: 90,
                    kickPenaltyPercent: 0.5 ether, // 50%
                    kickPenaltyDemerits: 1 // set this low enough to prevent slashing of that validator during kicking
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

        address operatorStakerAddressToKick = randomOperatorStakers[
            operatorStakerIndexToKick
        ];

        // Set up delegated staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorStakerAddressToKick);
        }

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Kick validator without slashing them
        randomOperatorStakers[operatorStakerIndexToKick] = address(0);
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            address validator = randomOperatorStakers[i];
            if (validator == address(0)) {
                continue;
            }

            vm.prank(validator);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                operatorStakerAddressToKick,
                1,
                ""
            );
        }

        // We MUST NOT fast forward time and instead actually try to advance multiple epochs so that we can test the slashing logic
        // which should use the last realmId + reward epoch number for the validator who was kicked.
        // First, advance 1 epoch to epoch 4 so that rewards for epoch 3 are calculated, and then get a reference for the reward epoch
        // and global stats for epoch 3. Then, advance 24 more epochs to epoch 28.
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        _advanceEpochs(realmId, 1, randomOperatorStakers, 3);

        // Assert that the validator cannot claim rewards yet because
        // the pending rejoin timeout has not elapsed yet, and only when it does,
        // will that validator be slashed.
        uint256 balanceBefore = token.balanceOf(operatorStakerAddressToKick);

        vm.prank(operatorStakerAddressToKick);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerAddressToKick,
            1,
            0
        );

        // Assert that the balance has not increased
        assertEq(
            token.balanceOf(operatorStakerAddressToKick),
            balanceBefore,
            "Balance should not have increased"
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                realmId,
                operatorStakerAddressToKick,
                1,
                0
            );

            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore,
                "Balance should not have increased"
            );
        }

        // Assert that the validator cannot withdraw their stake yet because they are still on the pending rejoins watchlist.
        vm.expectRevert(StakingFacet.TooSoonToWithdraw.selector);
        vm.prank(operatorStakerAddressToKick);
        stakingFacet.withdraw(operatorStakerAddressToKick, 1);

        if (testDelegatedStaker) {
            vm.expectRevert(StakingFacet.TooSoonToWithdraw.selector);
            vm.prank(delegatingStaker);
            stakingFacet.withdraw(operatorStakerAddressToKick, 1);
        }

        // Obtain a reference for the reward epoch and global stats for epoch 3.
        LibStakingStorage.RewardEpoch
            memory rewardEpochBeforeSlashing = stakingFacet.getRewardEpoch(
                operatorStakerAddressToKick,
                3
            );
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStatsBeforeKicking = stakingViewsFacet
                .getRewardEpochGlobalStats(3);
        _advanceEpochs(realmId, 24, randomOperatorStakers, 4);

        {
            // Assert that the rewards from epoch 3 are roughly halved for this validator.
            LibStakingStorage.RewardEpoch
                memory epoch3RewardEpoch = stakingFacet.getRewardEpoch(
                    operatorStakerAddressToKick,
                    3
                );
            assertApproxEqRel(
                epoch3RewardEpoch.totalStakeRewards,
                rewardEpochBeforeSlashing.totalStakeRewards / 2,
                0.2 ether, // allow a large-ish delta (20%),
                "rewards from epoch 3 are not roughly halved"
            );
            assertEq(
                epoch3RewardEpoch.totalStakeWeight,
                rewardEpochBeforeSlashing.totalStakeWeight / 2,
                "stake weight from epoch 3 is not roughly halved"
            );
            assertEq(
                epoch3RewardEpoch.slope,
                0,
                "slope from epoch 3 should be zero because the stake is still frozen"
            );
            assertEq(
                epoch3RewardEpoch.validatorSharePrice,
                rewardEpochBeforeSlashing.validatorSharePrice / 2,
                "validator share price from epoch 3 should be halved"
            );
        }

        {
            // Assert global stats for epoch 3. Even though stake amount has not changed, the validator who was kicked
            // would have had their stake weight adjusted.
            assertEq(
                stakingViewsFacet.getRewardEpochGlobalStats(3).stakeAmount,
                globalStatsBeforeKicking.stakeAmount,
                "stake amount for epoch 3 should not have changed"
            );
            uint256 singleValidatorOriginalStakeWeight = stakingViewsFacet
                .calculateStakeWeight(roundedTimeLock, amount);
            if (testDelegatedStaker) {
                assertApproxEqAbs(
                    stakingViewsFacet.getRewardEpochGlobalStats(3).stakeWeight,
                    globalStatsBeforeKicking.stakeWeight -
                        singleValidatorOriginalStakeWeight,
                    100,
                    "stake weight for epoch 3 should have reduced by a single validator's stake weight (half of validator + half of delegating staker)"
                );
            } else {
                assertApproxEqAbs(
                    stakingViewsFacet.getRewardEpochGlobalStats(3).stakeWeight,
                    globalStatsBeforeKicking.stakeWeight -
                        (singleValidatorOriginalStakeWeight / 2),
                    100,
                    "stake weight for epoch 3 should have reduced by half of a single validator's stake weight"
                );
            }

            // Assert that the reward epoch 4 is NOT attributed to the validator who was kicked.
            LibStakingStorage.RewardEpoch
                memory epoch4RewardEpoch = stakingFacet.getRewardEpoch(
                    operatorStakerAddressToKick,
                    4
                );
            _assertRewardEpochIsZero(epoch4RewardEpoch);

            // Assert that the global stats from epoch 4 are for 3 validators.
            assertEq(
                stakingViewsFacet.getRewardEpochGlobalStats(4).stakeAmount,
                amount * 3,
                "stake amount for epoch 4 should be from 3 validators"
            );
            assertEq(
                stakingViewsFacet.getRewardEpochGlobalStats(4).stakeWeight,
                singleValidatorOriginalStakeWeight * 3,
                "stake weight for epoch 4 should be from 3 validators"
            );
        }

        // Start the unfreeze and withdrawal process
        vm.prank(operatorStakerAddressToKick);
        stakingFacet.unfreezeStake(operatorStakerAddressToKick, 1);

        if (testDelegatedStaker) {
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(operatorStakerAddressToKick, 1);
        }

        // Fast forward to after the timelock has fully decayed.
        skip(800 days);

        // Assert that the rewards claimed for this validator is correct (epoch 3's rewards)
        uint256 expectedAmountIncreasePerStaker;
        {
            uint256 epoch3Rewards = stakingFacet
                .getRewardEpoch(operatorStakerAddressToKick, 3)
                .totalStakeRewards;
            if (testDelegatedStaker) {
                expectedAmountIncreasePerStaker = epoch3Rewards / 2;
            } else {
                expectedAmountIncreasePerStaker = epoch3Rewards;
            }
        }
        balanceBefore = token.balanceOf(operatorStakerAddressToKick);

        vm.prank(operatorStakerAddressToKick);
        stakingFacet.claimStakeRewards(1, operatorStakerAddressToKick, 1, 0);

        assertApproxEqRel(
            token.balanceOf(operatorStakerAddressToKick),
            balanceBefore + expectedAmountIncreasePerStaker,
            0.001 ether, // use a small delta (0.1%),
            "rewards claimed for epoch 3 are not correct"
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                realmId,
                operatorStakerAddressToKick,
                1,
                0
            );

            assertApproxEqRel(
                token.balanceOf(delegatingStaker),
                balanceBefore + expectedAmountIncreasePerStaker,
                0.001 ether, // use a small delta (0.1%),
                "rewards claimed for epoch 3 are not correct"
            );
        }

        // Assert that the amount they can withdraw is correct.
        balanceBefore = token.balanceOf(operatorStakerAddressToKick);

        vm.prank(operatorStakerAddressToKick);
        stakingFacet.withdraw(operatorStakerAddressToKick, 1);

        expectedAmountIncreasePerStaker = amount / 2;

        assertEq(
            token.balanceOf(operatorStakerAddressToKick),
            balanceBefore + expectedAmountIncreasePerStaker,
            "withdraw amount is not correct"
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.withdraw(operatorStakerAddressToKick, 1);

            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore + expectedAmountIncreasePerStaker,
                "withdraw amount is not correct"
            );
        }

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Get the current epoch number for realm 1
        uint256 realm1CurrentEpochNumber = stakingViewsFacet.epoch(1).number;

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            randomOperatorStakers,
            realm1CurrentEpochNumber
        );
    }

    /// @notice This test checks the reward epoch and global stats are correct when a validator who has requested to join a realm
    /// gets kicked without being slashed, then when the epoch advances multiple times, the validator is eventually slashed since it did not rejoin
    /// within the pending rejoin timeout.
    function testFuzz_ValidatorSlashedViaRejoinTimeout_JoiningValidator(
        uint256 amount,
        uint256 timeLock,
        bool testDelegatedStaker
    ) public {
        uint256 realmId = 1;
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
                    kickPenaltyDemerits: 1 // set this low enough to prevent slashing of that validator during kicking
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

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Set up new validator
        address[] memory newValidators = new address[](1);
        newValidators[0] = address(0x111);
        _setupValidators(
            realmId,
            newValidators,
            amount * 10,
            amount,
            timeLock,
            _generateUint256sWithOffset(1, 4)
        );

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, newValidators[0]);
        }

        // Assert that the next reward epoch is being attributed to by the new validator
        LibStakingStorage.RewardEpoch memory nextRewardEpoch = stakingFacet
            .getRewardEpoch(newValidators[0], 4);
        LibStakingStorage.RewardEpochGlobalStats
            memory nextGlobalStats = stakingViewsFacet
                .getRewardEpochGlobalStats(4);
        uint256 stakeWeightPerValidator = stakingViewsFacet
            .calculateStakeWeight(roundedTimeLock, amount);

        {
            uint256 expectedStakeWeightAttribution;
            if (testDelegatedStaker) {
                expectedStakeWeightAttribution = stakeWeightPerValidator * 2;
            } else {
                expectedStakeWeightAttribution = stakeWeightPerValidator;
            }

            uint256 expectedStakeAmountAttribution;
            if (testDelegatedStaker) {
                expectedStakeAmountAttribution = amount * 2;
            } else {
                expectedStakeAmountAttribution = amount;
            }

            assertEq(
                nextRewardEpoch.totalStakeWeight,
                expectedStakeWeightAttribution,
                "Next reward epoch total stake weight should be tentatively attributed to"
            );
            assertEq(
                nextRewardEpoch.stakeAmount,
                expectedStakeAmountAttribution,
                "Next reward epoch stake amount should be tentatively attributed to"
            );
            assertEq(
                nextGlobalStats.stakeAmount,
                // The 4 existing validators will attribute to epoch 4 once the epoch advances from 3 to 4.
                expectedStakeAmountAttribution,
                "Next global stats stake amount should be tentatively attributed to by the new validator"
            );
            assertEq(
                nextGlobalStats.stakeWeight,
                expectedStakeWeightAttribution,
                "Next global stats stake weight should be tentatively attributed to by the new validator"
            );
        }

        // Lock the validator set
        skip(1 hours);
        stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

        // Kick the new validator without slashing them
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            address validator = randomOperatorStakers[i];
            vm.prank(validator);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                newValidators[0],
                1,
                ""
            );
        }

        // We MUST NOT fast forward time and instead actually try to advance multiple epochs so that we can test the slashing logic
        // which should use the last realmId + reward epoch number for the validator who was kicked.
        // First, advance 1 epoch to epoch 4 so that rewards for epoch 3 are calculated, and then get a reference for the reward epoch
        // and global stats for epoch 3. Then, advance 24 more epochs to epoch 28.
        _advanceEpochs(realmId, 1, randomOperatorStakers, 3);

        // Immediately assert that the global stats for epoch 4 are not attributed to by the new validator or their delegated stakes
        LibStakingStorage.RewardEpochGlobalStats
            memory epoch4GlobalStats = stakingViewsFacet
                .getRewardEpochGlobalStats(4);
        assertEq(
            epoch4GlobalStats.stakeAmount,
            amount * 4,
            "stake amount for epoch 4 should be from 4 validators"
        );
        assertEq(
            epoch4GlobalStats.stakeWeight,
            stakeWeightPerValidator * 4,
            "stake weight for epoch 4 should be from 4 validators"
        );

        // Assert that the kicked validator cannot claim rewards yet because
        // the pending rejoin timeout has not elapsed yet, and only when it does,
        // will that validator be slashed.
        uint256 balanceBefore = token.balanceOf(newValidators[0]);

        vm.prank(newValidators[0]);
        stakingFacet.claimStakeRewards(realmId, newValidators[0], 1, 0);

        // Assert that the balance has not increased
        assertEq(
            token.balanceOf(newValidators[0]),
            balanceBefore,
            "Balance should not have increased"
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(realmId, newValidators[0], 1, 0);

            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore,
                "Balance should not have increased"
            );
        }

        // Assert that the validator cannot withdraw their stake yet because they are still on the pending rejoins watchlist.
        vm.expectRevert(StakingFacet.TooSoonToWithdraw.selector);
        vm.prank(newValidators[0]);
        stakingFacet.withdraw(newValidators[0], 1);

        if (testDelegatedStaker) {
            vm.expectRevert(StakingFacet.TooSoonToWithdraw.selector);
            vm.prank(delegatingStaker);
            stakingFacet.withdraw(newValidators[0], 1);
        }

        _advanceEpochs(realmId, 24, randomOperatorStakers, 4);

        // The kicked validator starts the unfreeze and withdrawal process
        vm.prank(newValidators[0]);
        stakingFacet.unfreezeStake(newValidators[0], 1);

        if (testDelegatedStaker) {
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(newValidators[0], 1);
        }

        // Fast forward to after the timelock has fully decayed.
        skip(800 days);

        // Assert that the kicked validator will not have any rewards to claim
        balanceBefore = token.balanceOf(newValidators[0]);

        vm.prank(newValidators[0]);
        stakingFacet.claimStakeRewards(realmId, newValidators[0], 1, 0);

        assertEq(
            token.balanceOf(newValidators[0]),
            balanceBefore,
            "Balance should not have increased"
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(realmId, newValidators[0], 1, 0);

            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore,
                "Balance should not have increased"
            );
        }

        // Assert that the kicked validator can now withdraw their original amount in full
        balanceBefore = token.balanceOf(newValidators[0]);

        vm.prank(newValidators[0]);
        stakingFacet.withdraw(newValidators[0], 1);

        assertEq(
            token.balanceOf(newValidators[0]),
            balanceBefore + amount,
            "Validator should be able to withdraw their full stake"
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.withdraw(newValidators[0], 1);

            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore + amount,
                "Delegating staker should be able to withdraw their full stake"
            );
        }

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Get the current epoch number for realm 1
        uint256 realm1CurrentEpochNumber = stakingViewsFacet.epoch(1).number;

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            randomOperatorStakers,
            realm1CurrentEpochNumber
        );
    }

    /// @notice This test checks the reward epoch and global stats are correct when a validator who has requested to join a realm
    /// gets slashed before the epoch advances. To avoid any weirdness with the initial few epochs, we deal in a new validator in epoch 3.
    function testFuzz_ValidatorSlashedViaKickBeforeAdvanceEpoch_JoiningValidator(
        uint256 amount,
        uint256 timeLock,
        bool testDelegatedStaker
    ) public {
        uint256 realmId = 1;
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
                    kickPenaltyDemerits: 10000
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

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // New validator stakes and requests to join
        address newValidator = address(0x111);
        address[] memory newValidators = new address[](1);
        newValidators[0] = newValidator;
        _setupValidators(
            realmId,
            newValidators,
            amount * 10,
            amount,
            timeLock,
            _generateUint256sWithOffset(1, 4)
        );

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        if (testDelegatedStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, newValidators[0]);
        }

        // Validator set gets locked
        skip(1 hours);
        stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

        // New validator gets kicked and slashed
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            address validator = randomOperatorStakers[i];

            vm.prank(validator);
            stakingValidatorFacet.kickValidatorInNextEpoch(newValidator, 1, "");
        }

        // Validator set gets locked again
        stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

        // Advance epoch
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        _advanceEpochs(realmId, 1, randomOperatorStakers, 3);

        {
            // Assert that the global stats for epoch 4 are NOT contributed to by the new validator
            LibStakingStorage.RewardEpochGlobalStats
                memory globalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(4);
            uint256 stakeWeightPerValidator = stakingViewsFacet
                .calculateStakeWeight(roundedTimeLock, amount);
            assertEq(
                globalStats.stakeAmount,
                amount * 4,
                "stake amount for epoch 4 should be from 4 validators"
            );
            assertEq(
                globalStats.stakeWeight,
                stakeWeightPerValidator * 4,
                "stake weight for epoch 4 should be from 4 validators"
            );
        }

        // Kicked staker should be able to withdraw their FULL stake (we do not penalize stakers
        // who have not been in an active set)
        vm.prank(newValidator);
        stakingFacet.unfreezeStake(newValidator, 1);

        if (testDelegatedStaker) {
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(newValidator, 1);
        }

        // Fast forward to after the timelock has fully decayed.
        skip(800 days);

        uint256 balanceBefore = token.balanceOf(newValidator);

        vm.prank(newValidator);
        stakingFacet.withdraw(newValidator, 1);

        assertEq(
            token.balanceOf(newValidator),
            balanceBefore + amount,
            "validator should be able to withdraw their full stake"
        );

        if (testDelegatedStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.withdraw(newValidator, 1);

            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore + amount,
                "delegating staker should be able to withdraw their full stake"
            );
        }

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Get the current epoch number for realm 1
        uint256 realm1CurrentEpochNumber = stakingViewsFacet.epoch(1).number;

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            randomOperatorStakers,
            realm1CurrentEpochNumber
        );
    }

    /// @notice This test checks that the reward epoch and global stats are correctly attributed to when a validator who
    /// gets kicked but not slashed, decides to rejoin the realm later.
    function testFuzz_ValidatorKickedButRejoins(
        uint256 operatorStakerIndexToKick,
        uint256 amount,
        uint256 timeLock,
        bool testDelegatingStaker
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToKick = bound(operatorStakerIndexToKick, 0, 3);
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        // Set complaint config for reason 1
        stakingAdminFacet.setComplaintConfig(
            1,
            LibStakingStorage.ComplaintConfig({
                tolerance: 6,
                intervalSecs: 90,
                kickPenaltyPercent: 0.5 ether, // 50%
                kickPenaltyDemerits: 1 // set this low enough to prevent slashing of that validator
            })
        );

        // Setup validators
        uint256 commsKeysOfKickedValidator;
        address[] memory randomOperatorStakers = _generateAddresses(4);
        {
            uint256[] memory randomCommsKeys = _generateUint256s(4);
            _setupValidators(
                realmId,
                randomOperatorStakers,
                amount * 10,
                amount,
                timeLock,
                randomCommsKeys
            );
            commsKeysOfKickedValidator = randomCommsKeys[
                operatorStakerIndexToKick
            ];
        }
        address operatorStakerAddressToKick = randomOperatorStakers[
            operatorStakerIndexToKick
        ];

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        if (testDelegatingStaker) {
            _fundAddressWithTokensAndApprove(delegatingStaker, amount);
            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorStakerAddressToKick);
        }

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Validator gets kicked
        {
            address[] memory validatorsNotKicked = new address[](4);
            for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
                if (i == operatorStakerIndexToKick) {
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
                    operatorStakerAddressToKick,
                    1,
                    ""
                );
            }

            // Advance the epoch 3 times
            // It's already locked at this point, so we expect a revert.
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingValidatorFacet
                        .MustBeInActiveOrUnlockedState
                        .selector,
                    1
                )
            );
            _advanceEpochs(realmId, 3, validatorsNotKicked, 3);

            // Validator rejoins the realm
            vm.prank(operatorStakerAddressToKick);
            stakingValidatorFacet.requestToJoin(realmId);
        }

        // Advance 1 epoch to deal in the validator
        _advanceEpochs(realmId, 1, randomOperatorStakers, 6);

        // Assert that the reward epoch and global stats are correctly attributed to.
        {
            LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
                .getRewardEpoch(operatorStakerAddressToKick, 7);
            LibStakingStorage.RewardEpochGlobalStats
                memory globalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(7);
            uint256 stakeWeightPerValidator = stakingViewsFacet
                .calculateStakeWeight(roundedTimeLock, amount);
            if (testDelegatingStaker) {
                assertEq(
                    rewardEpoch.totalStakeWeight,
                    stakeWeightPerValidator * 2
                );
                assertEq(rewardEpoch.stakeAmount, amount * 2);
                assertEq(globalStats.stakeWeight, stakeWeightPerValidator * 5);
                assertEq(globalStats.stakeAmount, amount * 5);
            } else {
                assertEq(rewardEpoch.totalStakeWeight, stakeWeightPerValidator);
                assertEq(rewardEpoch.stakeAmount, amount);
                assertEq(globalStats.stakeWeight, stakeWeightPerValidator * 4);
                assertEq(globalStats.stakeAmount, amount * 4);
            }
        }

        // Claim rewards for the kicked validator
        uint256 balanceBefore = token.balanceOf(operatorStakerAddressToKick);

        vm.prank(operatorStakerAddressToKick);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerAddressToKick,
            1,
            0
        );

        // Assert that the balance has increased
        assertGt(
            token.balanceOf(operatorStakerAddressToKick),
            balanceBefore,
            "Balance should have increased"
        );

        if (testDelegatingStaker) {
            balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                realmId,
                operatorStakerAddressToKick,
                1,
                0
            );

            assertGt(
                token.balanceOf(delegatingStaker),
                balanceBefore,
                "Balance should have increased"
            );
        }

        // Choose another validator for comparing rewards against.
        address validatorWhoWasNotKicked;
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (i == operatorStakerIndexToKick) {
                continue;
            }
            validatorWhoWasNotKicked = randomOperatorStakers[i];
            break;
        }

        // Claim all the rewards for the validator who never got kicked
        balanceBefore = token.balanceOf(validatorWhoWasNotKicked);

        vm.prank(validatorWhoWasNotKicked);
        stakingFacet.claimStakeRewards(realmId, validatorWhoWasNotKicked, 1, 0);

        // Assert that the balance has increased
        assertGt(
            token.balanceOf(validatorWhoWasNotKicked),
            balanceBefore,
            "Balance should have increased"
        );

        // Advance 1 epoch to earn rewards for all validators
        _advanceEpochs(realmId, 1, randomOperatorStakers, 7);

        // Assert that the rewards claimed for the kicked and non-kicked validators are correct.
        balanceBefore = token.balanceOf(operatorStakerAddressToKick);

        vm.prank(operatorStakerAddressToKick);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerAddressToKick,
            1,
            0
        );

        {
            uint256 rewardsClaimedForKickedValidator = token.balanceOf(
                operatorStakerAddressToKick
            ) - balanceBefore;
            assertGt(
                rewardsClaimedForKickedValidator,
                0,
                "Kicked validator should have earned rewards"
            );

            balanceBefore = token.balanceOf(validatorWhoWasNotKicked);

            vm.prank(validatorWhoWasNotKicked);
            stakingFacet.claimStakeRewards(
                realmId,
                validatorWhoWasNotKicked,
                1,
                0
            );

            uint256 rewardsClaimedForNonKickedValidator = token.balanceOf(
                validatorWhoWasNotKicked
            ) - balanceBefore;
            assertApproxEqAbs(
                rewardsClaimedForKickedValidator,
                rewardsClaimedForNonKickedValidator,
                100,
                "Rewards claimed for kicked and non-kicked validators should be the same"
            );
            if (testDelegatingStaker) {
                balanceBefore = token.balanceOf(delegatingStaker);

                vm.prank(delegatingStaker);
                stakingFacet.claimStakeRewards(
                    realmId,
                    operatorStakerAddressToKick,
                    1,
                    0
                );

                uint256 rewardsClaimedForDelegatingStaker = token.balanceOf(
                    delegatingStaker
                ) - balanceBefore;

                assertEq(
                    rewardsClaimedForKickedValidator,
                    rewardsClaimedForDelegatingStaker,
                    "Rewards claimed for delegating staker should be the same as the kicked validator's rewards"
                );
            }
        }
    }

    /// @notice This test checks that the attributions are correct when a delegating staker stakes with a validator before the validator
    /// requests to join a realm.
    function testFuzz_DelegatedStakeBeforeValidatorRequestsToJoin(
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndexToDelegateTo
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToDelegateTo = bound(
            operatorStakerIndexToDelegateTo,
            0,
            3
        );

        address[] memory randomOperatorStakers = _generateAddresses(4);
        uint256[] memory randomCommsKeys = _generateUint256s(4);
        _setupValidatorsStakeOnly(
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock
        );

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(
            amount,
            timeLock,
            randomOperatorStakers[operatorStakerIndexToDelegateTo]
        );

        // All validators now request to join a realm and the network becomes active
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            // Set the IP and port of the validator
            vm.prank(randomOperatorStakers[i]);
            stakingValidatorFacet.setIpPortNodeAddress(
                1,
                1,
                1,
                randomOperatorStakers[i]
            );

            // Request to join the realm
            vm.prank(randomOperatorStakers[i]);
            stakingValidatorFacet.requestToJoin(realmId);
        }
        stakingAdminFacet.setEpochState(
            realmId,
            LibStakingStorage.States.Active
        );

        // Advance 3 epochs
        _advanceEpochs(realmId, 3, randomOperatorStakers, 1);

        // Assert that the current reward epoch and global stats is not attributed to and is just initialized to zero values
        address[5] memory allStakers;
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            allStakers[i] = randomOperatorStakers[i];
        }
        allStakers[4] = delegatingStaker;
        for (uint256 i = 0; i < allStakers.length; i++) {
            LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
                .getRewardEpoch(allStakers[i], 1);
            _assertRewardEpochIsZero(rewardEpoch);
        }

        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(1);
        assertEq(globalStats.stakeAmount, 0);
        assertEq(globalStats.stakeWeight, 0);

        // Assert the 2nd reward epoch is correctly attributed to
        for (uint256 i = 0; i < allStakers.length; i++) {
            LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
                .getRewardEpoch(allStakers[i], 2);

            if (allStakers[i] == delegatingStaker) {
                // Reward epochs are always at the granularity of the validator, so the delegating staker should not
                // have any reward epochs initialized.
                _assertRewardEpochIsZero(rewardEpoch);
            } else if (
                allStakers[i] ==
                randomOperatorStakers[operatorStakerIndexToDelegateTo]
            ) {
                // The validator that the delegating staker delegated to should have a reward epoch initialized with an
                // amount that includes its self-stake as well the delegating staker's stake.
                _assertNewRewardEpoch(rewardEpoch, amount * 2);
            }
        }

        // Assert the 2nd global stats is correctly attributed to
        LibStakingStorage.RewardEpochGlobalStats
            memory epoch2GlobalStats = stakingViewsFacet
                .getRewardEpochGlobalStats(2);
        assertEq(epoch2GlobalStats.stakeAmount, amount * 5);
        assertEq(
            epoch2GlobalStats.stakeWeight,
            stakingViewsFacet.calculateStakeWeight(
                (timeLock / 86400) * 86400,
                amount
            ) * 5
        );
    }

    /// @notice This test is when a validator increases the timelock of their stake record
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_IncreaseStakeRecordTimelock(
        uint256 operatorStakerIndexToIncreaseTimelock,
        uint256 amount,
        uint256 timeLock,
        bool makeValidatorInactive
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 364 days); // 1 less than max timelock
        operatorStakerIndexToIncreaseTimelock = bound(
            operatorStakerIndexToIncreaseTimelock,
            0,
            3
        );
        uint256 currentEpochNumber;

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
        address validatorToIncreaseTimelock = randomOperatorStakers[
            operatorStakerIndexToIncreaseTimelock
        ];

        // Advance to epoch 4 to earn rewards for epoch 3
        currentEpochNumber = _advanceEpochs(
            realmId,
            3,
            randomOperatorStakers,
            1
        );

        // Expect increasing stake timelock without claiming rewards will revert
        uint256 newTimeLock = timeLock + 1 days;
        vm.expectRevert(
            abi.encodeWithSelector(StakingFacet.RewardsMustBeClaimed.selector)
        );
        vm.prank(validatorToIncreaseTimelock);
        stakingFacet.increaseStakeRecordTimelock(
            validatorToIncreaseTimelock,
            1,
            1 days
        );

        if (makeValidatorInactive) {
            // Validator requests to leave
            vm.prank(validatorToIncreaseTimelock);
            stakingValidatorFacet.requestToLeave();

            randomOperatorStakers[
                operatorStakerIndexToIncreaseTimelock
            ] = address(0);

            // Advance 1 epoch to deal out the validator
            currentEpochNumber = _advanceEpochs(
                realmId,
                1,
                randomOperatorStakers,
                currentEpochNumber
            );
        }

        // Claim rewards
        uint256 balanceBefore = token.balanceOf(validatorToIncreaseTimelock);

        vm.prank(validatorToIncreaseTimelock);
        stakingFacet.claimStakeRewards(1, validatorToIncreaseTimelock, 1, 0);

        // Assert balance has increased
        assertGt(token.balanceOf(validatorToIncreaseTimelock), balanceBefore);

        // Increase the timelock of the stake record
        LibStakingStorage.RewardEpoch
            memory epoch4RewardEpochBeforeUpdating = stakingFacet
                .getRewardEpoch(
                    validatorToIncreaseTimelock,
                    currentEpochNumber
                );
        LibStakingStorage.RewardEpochGlobalStats
            memory epoch4GlobalStatsBeforeUpdating = stakingViewsFacet
                .getRewardEpochGlobalStats(currentEpochNumber);

        vm.prank(validatorToIncreaseTimelock);
        stakingFacet.increaseStakeRecordTimelock(
            validatorToIncreaseTimelock,
            1,
            1 days
        );

        // Assert stake record
        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(
                validatorToIncreaseTimelock,
                2, // new stake record id
                validatorToIncreaseTimelock
            );
        // Round down to the nearest day
        assertEq(stakeRecord.timeLock, (newTimeLock / 86400) * 86400);
        assertEq(stakeRecord.amount, amount);

        if (makeValidatorInactive) {
            _assertOldNewRewardEpochsConstantForStaker(
                epoch4GlobalStatsBeforeUpdating,
                epoch4RewardEpochBeforeUpdating,
                currentEpochNumber,
                validatorToIncreaseTimelock,
                false
            );
        } else {
            // Assert reward epoch and global stats increased
            uint256 additionalStakeWeight = stakingViewsFacet
                .calculateStakeWeight(
                    (newTimeLock / 86400) *
                        86400 -
                        ((timeLock / 86400) * 86400),
                    amount
                );

            _assertOldNewRewardEpochsAndGlobalStatsIncreased(
                OldNewForRewardEpoch(
                    epoch4GlobalStatsBeforeUpdating,
                    epoch4RewardEpochBeforeUpdating,
                    validatorToIncreaseTimelock,
                    currentEpochNumber
                ),
                0,
                additionalStakeWeight,
                true
            );
        }

        // Assert that when the staker claims rewards immediately after increasing the timelock,
        // that there is nothing to claim against the new stake records.
        balanceBefore = token.balanceOf(validatorToIncreaseTimelock);

        vm.prank(validatorToIncreaseTimelock);
        stakingFacet.claimStakeRewards(1, validatorToIncreaseTimelock, 2, 0);

        // Assert that the balance has not increased
        assertEq(
            token.balanceOf(validatorToIncreaseTimelock),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is when a delegating staker of a validator increases the timelock of their stake record
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_IncreaseDelegatedStakeRecordTimelock(
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndexToDelegateTo,
        bool makeValidatorInactive
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 364 days); // 1 less than max timelock
        operatorStakerIndexToDelegateTo = bound(
            operatorStakerIndexToDelegateTo,
            0,
            3
        );
        uint256 currentEpochNumber;

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
        address validatorToIncreaseTimelock = randomOperatorStakers[
            operatorStakerIndexToDelegateTo
        ];

        // Advance to epoch 4 to earn rewards for epoch 3
        currentEpochNumber = _advanceEpochs(
            realmId,
            3,
            randomOperatorStakers,
            1
        );

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        {
            // Delegating staker stakes with a random validator
            LibStakingStorage.RewardEpoch
                memory rewardEpochBeforeUpdating = stakingFacet.getRewardEpoch(
                    validatorToIncreaseTimelock,
                    4
                );
            LibStakingStorage.RewardEpochGlobalStats
                memory globalStatsBeforeUpdating = stakingViewsFacet
                    .getRewardEpochGlobalStats(4);

            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, validatorToIncreaseTimelock);

            // Assert that current reward epoch and global stats have not changed
            _assertOldNewRewardEpochsConstantForStaker(
                globalStatsBeforeUpdating,
                rewardEpochBeforeUpdating,
                4,
                validatorToIncreaseTimelock,
                false
            );

            // Advance to epoch 6 to attribute reward epoch and global stats and earn rewards for epoch 5
            currentEpochNumber = _advanceEpochs(
                realmId,
                2,
                randomOperatorStakers,
                currentEpochNumber
            );

            // Assert reward epoch and global stats are correct for epoch 5
            uint256 additionalStakeWeight = stakingViewsFacet
                .calculateStakeWeight((timeLock / 86400) * 86400, amount);
            _assertOldNewRewardEpochsAndGlobalStatsIncreased(
                OldNewForRewardEpoch(
                    globalStatsBeforeUpdating,
                    rewardEpochBeforeUpdating,
                    validatorToIncreaseTimelock,
                    currentEpochNumber
                ),
                amount,
                additionalStakeWeight,
                false
            );
        }

        // Expect increasing delegated stake timelock without claiming rewards will revert
        vm.expectRevert(
            abi.encodeWithSelector(StakingFacet.RewardsMustBeClaimed.selector)
        );
        vm.prank(delegatingStaker);
        stakingFacet.increaseStakeRecordTimelock(
            validatorToIncreaseTimelock,
            1,
            1 days
        );

        if (makeValidatorInactive) {
            // Validator requests to leave
            vm.prank(validatorToIncreaseTimelock);
            stakingValidatorFacet.requestToLeave();

            randomOperatorStakers[operatorStakerIndexToDelegateTo] = address(0);

            // Advance 1 epoch to deal out the validator
            currentEpochNumber = _advanceEpochs(
                realmId,
                1,
                randomOperatorStakers,
                currentEpochNumber
            );
        }

        {
            // Claim rewards
            uint256 balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                1,
                validatorToIncreaseTimelock,
                1,
                0
            );

            // Assert balance has increased
            assertGt(token.balanceOf(delegatingStaker), balanceBefore);
        }

        // Increase the timelock of the delegated stake record
        LibStakingStorage.RewardEpoch
            memory rewardEpochBeforeUpdating = stakingFacet.getRewardEpoch(
                validatorToIncreaseTimelock,
                currentEpochNumber
            );
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStatsBeforeUpdating = stakingViewsFacet
                .getRewardEpochGlobalStats(currentEpochNumber);

        {
            uint256 newTimeLock = timeLock + 1 days;
            vm.prank(delegatingStaker);
            stakingFacet.increaseStakeRecordTimelock(
                validatorToIncreaseTimelock,
                1,
                1 days
            );

            // Assert stake record is correct
            LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
                .getStakeRecord(
                    validatorToIncreaseTimelock,
                    2, // new stake record id
                    delegatingStaker
                );
            assertEq(stakeRecord.timeLock, (newTimeLock / 86400) * 86400);
            assertEq(stakeRecord.amount, amount);
        }

        // Assert reward epoch and global stats are correct
        if (makeValidatorInactive) {
            _assertOldNewRewardEpochsConstantForStaker(
                globalStatsBeforeUpdating,
                rewardEpochBeforeUpdating,
                currentEpochNumber,
                validatorToIncreaseTimelock,
                false
            );
        } else {
            uint256 newTimeLock = timeLock + 1 days;
            uint256 additionalStakeWeight = stakingViewsFacet
                .calculateStakeWeight(
                    (newTimeLock / 86400) *
                        86400 -
                        ((timeLock / 86400) * 86400),
                    amount
                );
            _assertOldNewRewardEpochsAndGlobalStatsIncreased(
                OldNewForRewardEpoch(
                    globalStatsBeforeUpdating,
                    rewardEpochBeforeUpdating,
                    validatorToIncreaseTimelock,
                    currentEpochNumber
                ),
                0,
                additionalStakeWeight,
                true
            );
        }

        {
            // Assert that when the staker claims rewards immediately after increasing the timelock,
            // that there is nothing to claim against the new stake records.
            uint256 balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                1,
                validatorToIncreaseTimelock,
                2,
                0
            );

            // Assert that the balance has not increased
            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore,
                "Balance should not have increased"
            );
        }
    }

    /// @notice This test is when a validator increases the amount of their stake record
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_IncreaseStakeRecordAmount(
        uint256 operatorStakerIndexToIncreaseAmount,
        uint256 amount,
        uint256 timeLock,
        bool makeValidatorInactive
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToIncreaseAmount = bound(
            operatorStakerIndexToIncreaseAmount,
            0,
            3
        );
        uint256 currentEpochNumber;
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

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
        address validatorToIncreaseAmount = randomOperatorStakers[
            operatorStakerIndexToIncreaseAmount
        ];

        // Advance to epoch 4 to earn rewards for epoch 3
        currentEpochNumber = _advanceEpochs(
            realmId,
            3,
            randomOperatorStakers,
            1
        );

        // Expect increasing stake record amount without claiming rewards will revert
        uint256 newAmount = amount + 1 ether;
        vm.expectRevert(
            abi.encodeWithSelector(StakingFacet.RewardsMustBeClaimed.selector)
        );
        vm.prank(validatorToIncreaseAmount);
        stakingFacet.increaseStakeRecordAmount(
            validatorToIncreaseAmount,
            1,
            newAmount
        );

        if (makeValidatorInactive) {
            // Validator requests to leave
            vm.prank(validatorToIncreaseAmount);
            stakingValidatorFacet.requestToLeave();

            randomOperatorStakers[
                operatorStakerIndexToIncreaseAmount
            ] = address(0);

            // Advance 1 epoch to deal out the validator
            currentEpochNumber = _advanceEpochs(
                realmId,
                1,
                randomOperatorStakers,
                currentEpochNumber
            );
        }

        // Claim rewards
        uint256 balanceBefore = token.balanceOf(validatorToIncreaseAmount);

        vm.prank(validatorToIncreaseAmount);
        stakingFacet.claimStakeRewards(1, validatorToIncreaseAmount, 1, 0);

        // Assert balance has increased
        assertGt(token.balanceOf(validatorToIncreaseAmount), balanceBefore);

        // Increase the amount of the stake record
        LibStakingStorage.RewardEpoch
            memory epoch4RewardEpochBeforeUpdating = stakingFacet
                .getRewardEpoch(validatorToIncreaseAmount, currentEpochNumber);
        LibStakingStorage.RewardEpochGlobalStats
            memory epoch4GlobalStatsBeforeUpdating = stakingViewsFacet
                .getRewardEpochGlobalStats(currentEpochNumber);

        vm.prank(validatorToIncreaseAmount);
        stakingFacet.increaseStakeRecordAmount(
            validatorToIncreaseAmount,
            1,
            newAmount
        );

        // Assert stake record
        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(
                validatorToIncreaseAmount,
                2, // new stake record id
                validatorToIncreaseAmount
            );
        assertEq(stakeRecord.amount, amount + newAmount);
        // Round down to the nearest day
        assertEq(stakeRecord.timeLock, roundedTimeLock);

        if (makeValidatorInactive) {
            _assertOldNewRewardEpochsConstantForStaker(
                epoch4GlobalStatsBeforeUpdating,
                epoch4RewardEpochBeforeUpdating,
                currentEpochNumber,
                validatorToIncreaseAmount,
                false
            );
        } else {
            // Assert reward epoch
            LibStakingStorage.RewardEpoch
                memory epoch4RewardEpochAfterUpdating = stakingFacet
                    .getRewardEpoch(
                        validatorToIncreaseAmount,
                        currentEpochNumber
                    );
            assertApproxEqAbs(
                epoch4RewardEpochAfterUpdating.totalStakeWeight,
                epoch4RewardEpochBeforeUpdating.totalStakeWeight +
                    stakingViewsFacet.calculateStakeWeight(
                        roundedTimeLock,
                        newAmount
                    ),
                100,
                "Total stake weight did not increase properly"
            );
            assertEq(
                epoch4RewardEpochAfterUpdating.stakeAmount,
                epoch4RewardEpochBeforeUpdating.stakeAmount + newAmount,
                "Stake amount should have increased"
            );

            // Assert global stats
            LibStakingStorage.RewardEpochGlobalStats
                memory epoch4GlobalStatsAfterUpdating = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentEpochNumber);
            assertApproxEqAbs(
                epoch4GlobalStatsAfterUpdating.stakeWeight,
                epoch4GlobalStatsBeforeUpdating.stakeWeight +
                    stakingViewsFacet.calculateStakeWeight(
                        roundedTimeLock,
                        newAmount
                    ),
                100,
                "Global stats stake weight did not increase properly"
            );
            assertEq(
                epoch4GlobalStatsAfterUpdating.stakeAmount,
                epoch4GlobalStatsBeforeUpdating.stakeAmount + newAmount,
                "Global stats stake amount should have increased"
            );
        }

        // Assert that when the staker claims rewards immediately after increasing the amount,
        // that there is nothing to claim against the new stake records.
        balanceBefore = token.balanceOf(validatorToIncreaseAmount);

        vm.prank(validatorToIncreaseAmount);
        stakingFacet.claimStakeRewards(1, validatorToIncreaseAmount, 2, 0);

        // Assert that the balance has not increased
        assertEq(
            token.balanceOf(validatorToIncreaseAmount),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is when a delegating staker of an active validator increases the amount of their stake record
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_IncreaseDelegatedStakeRecordAmount(
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndexToIncreaseAmount,
        bool makeValidatorInactive
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToIncreaseAmount = bound(
            operatorStakerIndexToIncreaseAmount,
            0,
            3
        );
        uint256 currentEpochNumber;
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

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
        address validatorToIncreaseAmount = randomOperatorStakers[
            operatorStakerIndexToIncreaseAmount
        ];

        // Advance to epoch 4 to earn rewards for epoch 3
        currentEpochNumber = _advanceEpochs(
            realmId,
            3,
            randomOperatorStakers,
            1
        );

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount * 2);

        {
            // Delegating staker stakes with a random validator
            LibStakingStorage.RewardEpoch
                memory rewardEpochBeforeUpdating = stakingFacet.getRewardEpoch(
                    validatorToIncreaseAmount,
                    currentEpochNumber
                );
            LibStakingStorage.RewardEpochGlobalStats
                memory globalStatsBeforeUpdating = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentEpochNumber);

            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, validatorToIncreaseAmount);

            // Assert that current reward epoch and global stats have not changed
            _assertOldNewRewardEpochsConstantForStaker(
                globalStatsBeforeUpdating,
                rewardEpochBeforeUpdating,
                currentEpochNumber,
                validatorToIncreaseAmount,
                false
            );

            // Advance to epoch 6 to attribute reward epoch and global stats and earn rewards for epoch 5
            currentEpochNumber = _advanceEpochs(
                realmId,
                2,
                randomOperatorStakers,
                currentEpochNumber
            );

            // Assert reward epoch and global stats are correct for epoch 5
            uint256 additionalStakeWeight = stakingViewsFacet
                .calculateStakeWeight(roundedTimeLock, amount);
            _assertOldNewRewardEpochsAndGlobalStatsIncreased(
                OldNewForRewardEpoch(
                    globalStatsBeforeUpdating,
                    rewardEpochBeforeUpdating,
                    validatorToIncreaseAmount,
                    currentEpochNumber
                ),
                amount,
                additionalStakeWeight,
                false
            );
        }

        {
            // Expect increasing delegated stake record amount without claiming rewards will revert
            uint256 additionalAmount = 1 ether;
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingFacet.RewardsMustBeClaimed.selector
                )
            );
            vm.prank(delegatingStaker);
            stakingFacet.increaseStakeRecordAmount(
                validatorToIncreaseAmount,
                1,
                additionalAmount
            );
        }

        if (makeValidatorInactive) {
            // Validator requests to leave
            vm.prank(validatorToIncreaseAmount);
            stakingValidatorFacet.requestToLeave();

            randomOperatorStakers[
                operatorStakerIndexToIncreaseAmount
            ] = address(0);

            // Advance 1 epoch to deal out the validator
            currentEpochNumber = _advanceEpochs(
                realmId,
                1,
                randomOperatorStakers,
                currentEpochNumber
            );
        }

        {
            // Claim rewards
            uint256 balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(1, validatorToIncreaseAmount, 1, 0);

            // Assert balance has increased
            assertGt(token.balanceOf(delegatingStaker), balanceBefore);
        }

        // Increase the amount of the delegated stake record
        LibStakingStorage.RewardEpoch
            memory epoch4RewardEpochBeforeUpdating = stakingFacet
                .getRewardEpoch(validatorToIncreaseAmount, currentEpochNumber);
        LibStakingStorage.RewardEpochGlobalStats
            memory epoch4GlobalStatsBeforeUpdating = stakingViewsFacet
                .getRewardEpochGlobalStats(currentEpochNumber);

        {
            uint256 additionalAmount = amount;
            uint256 newAmount = amount + additionalAmount;
            vm.prank(delegatingStaker);
            stakingFacet.increaseStakeRecordAmount(
                validatorToIncreaseAmount,
                1,
                additionalAmount
            );

            // Assert stake record
            LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
                .getStakeRecord(
                    validatorToIncreaseAmount,
                    2, // new stake record id
                    delegatingStaker
                );
            assertEq(stakeRecord.amount, newAmount);
            assertEq(stakeRecord.timeLock, roundedTimeLock);
        }

        if (makeValidatorInactive) {
            _assertOldNewRewardEpochsConstantForStaker(
                epoch4GlobalStatsBeforeUpdating,
                epoch4RewardEpochBeforeUpdating,
                currentEpochNumber,
                validatorToIncreaseAmount,
                false
            );
        } else {
            // Assert reward epoch
            uint256 additionalAmount = amount;
            LibStakingStorage.RewardEpoch
                memory epoch4RewardEpochAfterUpdating = stakingFacet
                    .getRewardEpoch(
                        validatorToIncreaseAmount,
                        currentEpochNumber
                    );
            assertApproxEqAbs(
                epoch4RewardEpochAfterUpdating.totalStakeWeight,
                epoch4RewardEpochBeforeUpdating.totalStakeWeight +
                    stakingViewsFacet.calculateStakeWeight(
                        roundedTimeLock,
                        additionalAmount
                    ),
                100,
                "Total stake weight did not increase properly"
            );
            assertEq(
                epoch4RewardEpochAfterUpdating.stakeAmount,
                epoch4RewardEpochBeforeUpdating.stakeAmount + additionalAmount,
                "Stake amount should have increased"
            );

            // Assert global stats
            LibStakingStorage.RewardEpochGlobalStats
                memory epoch4GlobalStatsAfterUpdating = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentEpochNumber);
            assertApproxEqAbs(
                epoch4GlobalStatsAfterUpdating.stakeWeight,
                epoch4GlobalStatsBeforeUpdating.stakeWeight +
                    stakingViewsFacet.calculateStakeWeight(
                        roundedTimeLock,
                        additionalAmount
                    ),
                100,
                "Global stats stake weight did not increase properly"
            );
            assertEq(
                epoch4GlobalStatsAfterUpdating.stakeAmount,
                epoch4GlobalStatsBeforeUpdating.stakeAmount + additionalAmount,
                "Global stats stake amount should have increased"
            );
        }

        // Assert that when the staker claims rewards immediately after increasing the amount,
        // that there is nothing to claim against the new stake records.
        uint256 balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            validatorToIncreaseAmount,
            2,
            0
        );

        // Assert that the balance has not increased
        assertEq(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is for the case when a delegating staker splits their stake against an
    /// active validator.
    function testFuzz_DelegatedStakeSplitsStake_ActiveValidator(
        uint256 amount,
        uint256 timeLock,
        uint256 ratio,
        uint256 operatorStakerIndexToDelegateTo
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        ratio = bound(ratio, .001 ether, .999 ether);
        operatorStakerIndexToDelegateTo = bound(
            operatorStakerIndexToDelegateTo,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        {
            // Set the bounds for min stake amount to be beyond what the ratio can split into.
            LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
                .globalConfig();
            config.minStakeAmount = 1;
            stakingAdminFacet.setConfig(config);
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
        address randomOperatorStakerToDelegateTo = randomOperatorStakers[
            operatorStakerIndexToDelegateTo
        ];

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, randomOperatorStakerToDelegateTo);

        // Advance to epoch 4 to earn rewards for at least 1 epoch (epoch 3)
        _advanceEpochs(realmId, 3, randomOperatorStakers, 1);

        // Assert that splitting the stake now will revert due to unclaimed rewards
        vm.expectRevert(
            abi.encodeWithSelector(StakingFacet.RewardsMustBeClaimed.selector)
        );
        vm.prank(delegatingStaker);
        stakingFacet.splitStakeRecord(
            randomOperatorStakerToDelegateTo,
            1,
            ratio
        );

        // Claim rewards
        uint256 balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            1,
            randomOperatorStakerToDelegateTo,
            1,
            0
        );

        // Assert balance has increased
        assertGt(token.balanceOf(delegatingStaker), balanceBefore);

        // Obtain references to reward epochs and global stats before stake splits
        LibStakingStorage.RewardEpoch
            memory epoch4RewardEpochBefore = stakingFacet.getRewardEpoch(
                randomOperatorStakerToDelegateTo,
                4
            );
        LibStakingStorage.RewardEpochGlobalStats
            memory epoch4GlobalStatsBefore = stakingViewsFacet
                .getRewardEpochGlobalStats(4);

        // Split the stake
        vm.prank(delegatingStaker);
        stakingFacet.splitStakeRecord(
            randomOperatorStakerToDelegateTo,
            1,
            ratio
        );

        // Assert that the first stake record is now gone
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingUtilsLib.StakeRecordNotFound.selector,
                realmId
            )
        );
        stakingViewsFacet.getStakeRecord(
            randomOperatorStakerToDelegateTo,
            1,
            delegatingStaker
        );

        // Assert that the stake amounts for the two new stake records are correct
        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(
                randomOperatorStakerToDelegateTo,
                2,
                delegatingStaker
            );
        uint256 expectedSplitAmount = (amount * ratio) / 1 ether;
        assertEq(stakeRecord.amount, expectedSplitAmount);
        assertEq(stakeRecord.timeLock, roundedTimeLock);

        stakeRecord = stakingViewsFacet.getStakeRecord(
            randomOperatorStakerToDelegateTo,
            3,
            delegatingStaker
        );
        uint256 expectedRemainingAmount = amount - expectedSplitAmount;
        assertApproxEqAbs(
            stakeRecord.amount,
            expectedRemainingAmount,
            100,
            "Original amount did not split properly"
        );
        assertEq(stakeRecord.timeLock, roundedTimeLock);

        // Assert reward epochs and global stats have not changed
        _assertOldNewRewardEpochsConstantForStaker(
            epoch4GlobalStatsBefore,
            epoch4RewardEpochBefore,
            4,
            randomOperatorStakerToDelegateTo,
            true
        );

        // Assert that when the staker claims rewards immediately after splitting,
        // that there is nothing to claim against the new stake records.
        balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            randomOperatorStakerToDelegateTo,
            2,
            0
        );
        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            randomOperatorStakerToDelegateTo,
            3,
            0
        );

        // Assert balance has not increased
        assertEq(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is when a delegating staker splits their stake of an inactive validator
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_DelegatedStakerSplitsStake(
        uint256 amount,
        uint256 timeLock,
        uint256 ratio,
        uint256 operatorStakerIndexToSplitStake,
        bool makeValidatorInactive
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        ratio = bound(ratio, .001 ether, .999 ether);
        operatorStakerIndexToSplitStake = bound(
            operatorStakerIndexToSplitStake,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        {
            // Set the bounds for min stake amount to be beyond what the ratio can split into.
            LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
                .globalConfig();
            config.minStakeAmount = 1;
            stakingAdminFacet.setConfig(config);
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
        address randomOperatorStakerToSplitStake = randomOperatorStakers[
            operatorStakerIndexToSplitStake
        ];

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, randomOperatorStakerToSplitStake);

        // Advance to epoch 4 to earn rewards for epoch 3
        uint256 currentEpochNumber = _advanceEpochs(
            realmId,
            3,
            randomOperatorStakers,
            1
        );

        // Assert that splitting the stake now will revert due to unclaimed rewards
        vm.expectRevert(
            abi.encodeWithSelector(StakingFacet.RewardsMustBeClaimed.selector)
        );
        vm.prank(delegatingStaker);
        stakingFacet.splitStakeRecord(
            randomOperatorStakerToSplitStake,
            1,
            ratio
        );

        if (makeValidatorInactive) {
            // Validator requests to leave
            vm.prank(randomOperatorStakerToSplitStake);
            stakingValidatorFacet.requestToLeave();

            randomOperatorStakers[operatorStakerIndexToSplitStake] = address(0);

            // Advance 1 epoch to deal out the validator
            currentEpochNumber = _advanceEpochs(
                realmId,
                1,
                randomOperatorStakers,
                currentEpochNumber
            );
        }

        {
            // Claim rewards
            uint256 balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                1,
                randomOperatorStakerToSplitStake,
                1,
                0
            );

            // Assert balance has increased
            assertGt(token.balanceOf(delegatingStaker), balanceBefore);
        }

        // Split the stake
        vm.prank(delegatingStaker);
        stakingFacet.splitStakeRecord(
            randomOperatorStakerToSplitStake,
            1,
            ratio
        );

        // Assert that the first stake record is now gone
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingUtilsLib.StakeRecordNotFound.selector,
                realmId
            )
        );
        stakingViewsFacet.getStakeRecord(
            randomOperatorStakerToSplitStake,
            1,
            delegatingStaker
        );

        // Assert that the stake amounts for the two new stake records are correct
        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(
                randomOperatorStakerToSplitStake,
                2,
                delegatingStaker
            );
        uint256 expectedSplitAmount = (amount * ratio) / 1 ether;
        assertEq(stakeRecord.amount, expectedSplitAmount);
        assertEq(stakeRecord.timeLock, roundedTimeLock);

        stakeRecord = stakingViewsFacet.getStakeRecord(
            randomOperatorStakerToSplitStake,
            3,
            delegatingStaker
        );
        uint256 expectedRemainingAmount = amount - expectedSplitAmount;
        assertApproxEqAbs(
            stakeRecord.amount,
            expectedRemainingAmount,
            100,
            "Original amount did not split properly"
        );

        // Assert that when the staker claims rewards immediately after splitting,
        // that there is nothing to claim against the new stake records.
        uint256 balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            randomOperatorStakerToSplitStake,
            2,
            0
        );
        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            randomOperatorStakerToSplitStake,
            3,
            0
        );

        // Assert balance has not increased
        assertEq(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is for the case when an active validator splits their stake
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_SplitsStake(
        uint256 amount,
        uint256 timeLock,
        uint256 ratio,
        uint256 operatorStakerIndexToSplitStake,
        bool makeValidatorInactive
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        ratio = bound(ratio, .001 ether, .999 ether);
        operatorStakerIndexToSplitStake = bound(
            operatorStakerIndexToSplitStake,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        {
            // Set the bounds for min stake amount to be beyond what the ratio can split into.
            LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
                .globalConfig();
            config.minStakeAmount = 1;
            config.minSelfStake = 1;
            stakingAdminFacet.setConfig(config);
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
        address randomOperatorStakerToSplitStake = randomOperatorStakers[
            operatorStakerIndexToSplitStake
        ];

        // Advance to epoch 4 to earn rewards for at least 1 epoch (epoch 3)
        uint256 currentEpochNumber = _advanceEpochs(
            realmId,
            3,
            randomOperatorStakers,
            1
        );

        // Assert that splitting the stake now will revert due to unclaimed rewards
        vm.expectRevert(
            abi.encodeWithSelector(StakingFacet.RewardsMustBeClaimed.selector)
        );
        vm.prank(randomOperatorStakerToSplitStake);
        stakingFacet.splitStakeRecord(
            randomOperatorStakerToSplitStake,
            1,
            ratio
        );

        if (makeValidatorInactive) {
            // Validator requests to leave
            vm.prank(randomOperatorStakerToSplitStake);
            stakingValidatorFacet.requestToLeave();

            randomOperatorStakers[operatorStakerIndexToSplitStake] = address(0);

            // Advance 1 epoch to deal out the validator
            currentEpochNumber = _advanceEpochs(
                realmId,
                1,
                randomOperatorStakers,
                currentEpochNumber
            );
        }

        {
            // Claim rewards
            uint256 balanceBefore = token.balanceOf(
                randomOperatorStakerToSplitStake
            );

            vm.prank(randomOperatorStakerToSplitStake);
            stakingFacet.claimStakeRewards(
                1,
                randomOperatorStakerToSplitStake,
                1,
                0
            );

            // Assert balance has increased
            assertGt(
                token.balanceOf(randomOperatorStakerToSplitStake),
                balanceBefore
            );
        }

        // Obtain references to reward epochs and global stats before stake splits
        LibStakingStorage.RewardEpoch
            memory epoch4RewardEpochBefore = stakingFacet.getRewardEpoch(
                randomOperatorStakerToSplitStake,
                currentEpochNumber
            );
        LibStakingStorage.RewardEpochGlobalStats
            memory epoch4GlobalStatsBefore = stakingViewsFacet
                .getRewardEpochGlobalStats(currentEpochNumber);

        // Split the stake
        vm.prank(randomOperatorStakerToSplitStake);
        stakingFacet.splitStakeRecord(
            randomOperatorStakerToSplitStake,
            1,
            ratio
        );

        // Assert that the first stake record is now gone
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingUtilsLib.StakeRecordNotFound.selector,
                realmId
            )
        );
        stakingViewsFacet.getStakeRecord(
            randomOperatorStakerToSplitStake,
            1,
            randomOperatorStakerToSplitStake
        );

        // Assert that the stake amounts for the two new stake records are correct
        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(
                randomOperatorStakerToSplitStake,
                2,
                randomOperatorStakerToSplitStake
            );
        uint256 expectedSplitAmount = (amount * ratio) / 1 ether;
        assertEq(stakeRecord.amount, expectedSplitAmount);
        assertEq(stakeRecord.timeLock, roundedTimeLock);

        stakeRecord = stakingViewsFacet.getStakeRecord(
            randomOperatorStakerToSplitStake,
            3,
            randomOperatorStakerToSplitStake
        );
        uint256 expectedRemainingAmount = amount - expectedSplitAmount;
        assertApproxEqAbs(
            stakeRecord.amount,
            expectedRemainingAmount,
            100,
            "Original amount did not split properly"
        );

        // Assert reward epochs and global stats have not changed
        _assertOldNewRewardEpochsConstantForStaker(
            epoch4GlobalStatsBefore,
            epoch4RewardEpochBefore,
            currentEpochNumber,
            randomOperatorStakerToSplitStake,
            true
        );

        {
            // Assert that when the staker claims rewards immediately after splitting,
            // that there is nothing to claim against the new stake records.
            uint256 balanceBefore = token.balanceOf(
                randomOperatorStakerToSplitStake
            );

            vm.prank(randomOperatorStakerToSplitStake);
            stakingFacet.claimStakeRewards(
                realmId,
                randomOperatorStakerToSplitStake,
                2,
                0
            );
            vm.prank(randomOperatorStakerToSplitStake);
            stakingFacet.claimStakeRewards(
                realmId,
                randomOperatorStakerToSplitStake,
                3,
                0
            );

            // Assert balance has not increased
            assertEq(
                token.balanceOf(randomOperatorStakerToSplitStake),
                balanceBefore,
                "Balance should not have increased"
            );
        }
    }

    /// @notice This test is when an active validator sets their commission rate
    /// that the reward epoch and global stats are updated correctly.
    function testFuzz_SetValidatorCommissionRate_ActiveValidator(
        uint256 rate,
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndex
    ) public {
        uint256 realmId = 1;
        rate = bound(rate, 1, (1 ether) - 1);
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndex = bound(operatorStakerIndex, 0, 3);

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

        address operatorStakerToSetCommissionRate = randomOperatorStakers[
            operatorStakerIndex
        ];

        // Advance to epoch 4 to earn commission for one epoch
        _advanceEpochs(realmId, 3, randomOperatorStakers, 1);

        // Assert that the validator has claimed fixed cost rewards for epoch 3
        uint256 balanceBeforeClaimingEpoch3Commission = token.balanceOf(
            operatorStakerToSetCommissionRate
        );

        vm.prank(operatorStakerToSetCommissionRate);
        stakingFacet.claimValidatorCommission(realmId, 0);

        uint256 balanceAfterClaimingEpoch3Commission = token.balanceOf(
            operatorStakerToSetCommissionRate
        );
        uint256 epoch3ActualCommissionObtained = balanceAfterClaimingEpoch3Commission -
                balanceBeforeClaimingEpoch3Commission;
        assertEq(
            epoch3ActualCommissionObtained,
            0,
            "Validator should have claimed zero commision in epoch 3"
        );

        // Set the commission rate to non-zero
        vm.prank(operatorStakerToSetCommissionRate);
        stakingFacet.setValidatorCommissionRate(rate);

        // Advance to epoch 5 to earn non-zero commission for one epoch
        _advanceEpochs(realmId, 1, randomOperatorStakers, 4);

        // Validator claims commission
        uint256 balanceBeforeClaimingEpoch4Commission = token.balanceOf(
            operatorStakerToSetCommissionRate
        );

        vm.prank(operatorStakerToSetCommissionRate);
        stakingFacet.claimValidatorCommission(realmId, 0);

        // Assert balance has increased
        uint256 balanceAfterClaimingEpoch4Commission = token.balanceOf(
            operatorStakerToSetCommissionRate
        );
        uint256 epoch4ActualCommissionObtained = balanceAfterClaimingEpoch4Commission -
                balanceBeforeClaimingEpoch4Commission;
        assertGt(
            epoch4ActualCommissionObtained,
            epoch3ActualCommissionObtained,
            "Validator should have claimed more commision in epoch 4 (includes commission rate set by validator)"
        );

        // Assert that the validator total rewards have decreased from epoch 3 to 4.
        LibStakingStorage.RewardEpoch memory epoch3RewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerToSetCommissionRate, 3);
        LibStakingStorage.RewardEpoch memory epoch4RewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerToSetCommissionRate, 4);
        uint256 variableCommissionEarned = epoch4ActualCommissionObtained -
            epoch3ActualCommissionObtained;
        assertEq(
            epoch4RewardEpoch.totalStakeRewards,
            epoch3RewardEpoch.totalStakeRewards - variableCommissionEarned,
            "Total rewards should have decreased by the amount of commission obtained"
        );
    }

    /// @notice This test is when an active validator earns rewards, creates a new stake and cannot
    /// immediately claim rewards for that stake per the creation of a new stake record
    function testFuzz_OperatorStakesAgain_ActiveValidator(
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndexToStakeAgain
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeAgain = bound(
            operatorStakerIndexToStakeAgain,
            0,
            3
        );

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
        address operatorStakerToStakeAgain = randomOperatorStakers[
            operatorStakerIndexToStakeAgain
        ];

        // Advance to epoch 4 to earn rewards for at least 1 epoch (epoch 3)
        _advanceEpochs(realmId, 3, randomOperatorStakers, 1);

        // Claim rewards
        uint256 balanceBefore = token.balanceOf(operatorStakerToStakeAgain);

        vm.prank(operatorStakerToStakeAgain);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerToStakeAgain,
            1,
            0
        );

        // Assert balance has increased
        assertGt(
            token.balanceOf(operatorStakerToStakeAgain),
            balanceBefore,
            "Balance should have increased"
        );

        // Fund the operator with more tokens and stake again
        token.transfer(operatorStakerToStakeAgain, amount);

        vm.expectEmit(true, true, true, true);
        emit StakeRecordCreated(
            operatorStakerToStakeAgain,
            2,
            amount,
            operatorStakerToStakeAgain
        );
        vm.prank(operatorStakerToStakeAgain);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeAgain);

        // Assert that when the staker claims rewards immediately after staking again,
        // that there is nothing to claim against the new stake records.
        balanceBefore = token.balanceOf(operatorStakerToStakeAgain);

        vm.prank(operatorStakerToStakeAgain);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerToStakeAgain,
            2,
            0
        );

        // Assert balance has not increased
        assertEq(
            token.balanceOf(operatorStakerToStakeAgain),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is when a delegating staker stakes with an active validator, earns rewards,
    /// and then the delegator creates a new stake and cannot immediately claim rewards for that stake
    /// per the creation of a new stake record.
    function testFuzz_DelegatingStakerStakesAgain_ActiveValidator(
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndexToStakeAgainst
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeAgainst = bound(
            operatorStakerIndexToStakeAgainst,
            0,
            3
        );

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
        address operatorStakerToStakeAgainst = randomOperatorStakers[
            operatorStakerIndexToStakeAgainst
        ];

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount * 2);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeAgainst);

        // Advance to epoch 4 to earn rewards for at least 1 epoch (epoch 3)
        _advanceEpochs(realmId, 3, randomOperatorStakers, 1);

        // Claim rewards
        uint256 balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerToStakeAgainst,
            1,
            0
        );

        // Assert balance has increased
        assertGt(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should have increased"
        );

        // Fund the delegating staker with more tokens and stake again
        token.transfer(delegatingStaker, amount);

        vm.expectEmit(true, true, true, true);
        emit StakeRecordCreated(
            operatorStakerToStakeAgainst,
            2,
            amount,
            delegatingStaker
        );
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeAgainst);

        // Assert that when the delegating staker claims rewards immediately after staking again,
        // that there is nothing to claim against the new stake records.
        balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerToStakeAgainst,
            2,
            0
        );

        // Assert balance has not increased
        assertEq(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is when an active validator earns rewards, becomes inactive (leaves set),
    /// creates a new stake and cannot immediately claim rewards for that stake per the creation of
    /// a new stake record.
    function testFuzz_OperatorStakesAgain_InactiveValidator(
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndexToStakeAgain
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeAgain = bound(
            operatorStakerIndexToStakeAgain,
            0,
            3
        );

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
        address operatorStakerToStakeAgain = randomOperatorStakers[
            operatorStakerIndexToStakeAgain
        ];

        // Advance to epoch 3 before requesting to leave to earn rewards for at least 1 epoch (epoch 3)
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Request to leave
        vm.prank(operatorStakerToStakeAgain);
        stakingValidatorFacet.requestToLeave();

        // Advance epoch to deal out validator
        address[] memory remainingValidators = new address[](4);
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (randomOperatorStakers[i] != operatorStakerToStakeAgain) {
                remainingValidators[i] = randomOperatorStakers[i];
            }
        }
        _advanceEpochs(realmId, 1, remainingValidators, 3);

        // Claim rewards
        uint256 balanceBefore = token.balanceOf(operatorStakerToStakeAgain);

        vm.prank(operatorStakerToStakeAgain);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerToStakeAgain,
            1,
            0
        );

        // Assert balance has increased
        assertGt(
            token.balanceOf(operatorStakerToStakeAgain),
            balanceBefore,
            "Balance should have increased"
        );

        // Fund the operator with more tokens and stake again
        token.transfer(operatorStakerToStakeAgain, amount);

        vm.expectEmit(true, true, true, true);
        emit StakeRecordCreated(
            operatorStakerToStakeAgain,
            2,
            amount,
            operatorStakerToStakeAgain
        );
        vm.prank(operatorStakerToStakeAgain);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeAgain);

        // Assert that when the staker claims rewards immediately after staking again,
        // that there is nothing to claim against the new stake records.
        balanceBefore = token.balanceOf(operatorStakerToStakeAgain);

        vm.prank(operatorStakerToStakeAgain);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerToStakeAgain,
            2,
            0
        );

        // Assert balance has not increased
        assertEq(
            token.balanceOf(operatorStakerToStakeAgain),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is when a delegating staker stakes with active validator, earns rewards,
    /// then validator becomes inactive (leaves set), and then the delegator creates a new stake and
    /// cannot immediately claim rewards for that stake per the creation of a new stake record
    function testFuzz_DelegatingStakerStakesAgain_InactiveValidator(
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndexToStakeAgainst
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeAgainst = bound(
            operatorStakerIndexToStakeAgainst,
            0,
            3
        );

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
        address operatorStakerToStakeAgainst = randomOperatorStakers[
            operatorStakerIndexToStakeAgainst
        ];

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount * 2);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeAgainst);

        // Advance to epoch 3 before requesting to leave to earn rewards for at least 1 epoch (epoch 3)
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Request to leave
        vm.prank(operatorStakerToStakeAgainst);
        stakingValidatorFacet.requestToLeave();

        // Advance epoch to deal out validator
        address[] memory remainingValidators = new address[](4);
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (randomOperatorStakers[i] != operatorStakerToStakeAgainst) {
                remainingValidators[i] = randomOperatorStakers[i];
            }
        }
        _advanceEpochs(realmId, 1, remainingValidators, 3);

        // Claim rewards
        uint256 balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerToStakeAgainst,
            1,
            0
        );

        // Assert balance has increased
        assertGt(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should have increased"
        );

        // Fund the delegating staker with more tokens and stake again
        token.transfer(delegatingStaker, amount);

        vm.expectEmit(true, true, true, true);
        emit StakeRecordCreated(
            operatorStakerToStakeAgainst,
            2,
            amount,
            delegatingStaker
        );
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeAgainst);

        // Assert that when the delegating staker claims rewards immediately after staking again,
        // that there is nothing to claim against the new stake records.
        balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.claimStakeRewards(
            realmId,
            operatorStakerToStakeAgainst,
            2,
            0
        );

        // Assert balance has not increased
        assertEq(
            token.balanceOf(delegatingStaker),
            balanceBefore,
            "Balance should not have increased"
        );
    }

    /// @notice This test is when a validator stakes twice at the genesis epoch before any epoch
    /// advancements and the reward epoch and global stats are updated correctly.
    function testFuzz_ValidatorStakesTwice_JoiningValidator(
        uint256 operatorStakerIndexToStakeTwice,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeTwice = bound(
            operatorStakerIndexToStakeTwice,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

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
        address operatorStakerToStakeTwice = randomOperatorStakers[
            operatorStakerIndexToStakeTwice
        ];

        // Operator stakes again
        vm.prank(operatorStakerToStakeTwice);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeTwice);

        // Assert that the operator staker has two stake records
        assertEq(
            stakingViewsFacet.getStakeRecordCount(
                operatorStakerToStakeTwice,
                operatorStakerToStakeTwice
            ),
            2,
            "Operator staker should have two stake records"
        );

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Assert that the reward epoch and global stats are updated correctly
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerToStakeTwice, 3);
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(3);

        uint256 stakeWeightPerValidator = stakingViewsFacet
            .calculateStakeWeight(roundedTimeLock, amount);
        assertEq(
            rewardEpoch.stakeAmount,
            amount * 2,
            "Reward epoch stake amount should be double the original amount"
        );
        assertEq(
            rewardEpoch.totalStakeWeight,
            stakeWeightPerValidator * 2,
            "Reward epoch total stake weight should be double that of other validator's stake weight"
        );
        assertEq(
            globalStats.stakeAmount,
            amount * 5,
            "Global stats stake amount should be 5x the original amount"
        );
        assertEq(
            globalStats.stakeWeight,
            stakeWeightPerValidator * 5,
            "Global stats stake weight should be 5x that of each validator's stake weight"
        );
    }

    /// @notice This test is when a validator stakes once at the genesis epoch, then the epoch
    /// advances to epoch 3 and the validator stakes once more and the reward epoch and global stats
    /// are updated correctly.
    function testFuzz_ValidatorStakesTwice_ActiveValidator(
        uint256 operatorStakerIndexToStakeTwice,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeTwice = bound(
            operatorStakerIndexToStakeTwice,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

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
        address operatorStakerToStakeTwice = randomOperatorStakers[
            operatorStakerIndexToStakeTwice
        ];

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Operator stakes again
        vm.prank(operatorStakerToStakeTwice);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeTwice);

        // Assert that the operator staker has two stake records
        assertEq(
            stakingViewsFacet.getStakeRecordCount(
                operatorStakerToStakeTwice,
                operatorStakerToStakeTwice
            ),
            2,
            "Operator staker should have two stake records"
        );

        // Assert that the reward epoch and global stats for epoch 3 are correct
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerToStakeTwice, 3);
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(3);
        uint256 stakeWeightPerValidator = stakingViewsFacet
            .calculateStakeWeight(roundedTimeLock, amount);

        assertEq(
            rewardEpoch.stakeAmount,
            amount,
            "Reward epoch stake amount should be the original amount"
        );
        assertEq(
            rewardEpoch.totalStakeWeight,
            stakeWeightPerValidator,
            "Reward epoch total stake weight should be the original stake weight"
        );
        assertEq(
            globalStats.stakeAmount,
            amount * 4,
            "Global stats stake amount should be 4x the original amount"
        );
        assertEq(
            globalStats.stakeWeight,
            stakeWeightPerValidator * 4,
            "Global stats stake weight should be 4x that of each validator's stake weight"
        );

        // Advance to epoch 4
        _advanceEpochs(realmId, 1, randomOperatorStakers, 3);

        // Assert that the reward epoch and global stats are updated correctly
        _assertOldNewRewardEpochsAndGlobalStatsIncreased(
            OldNewForRewardEpoch(
                globalStats,
                rewardEpoch,
                operatorStakerToStakeTwice,
                4
            ),
            amount,
            stakeWeightPerValidator,
            false
        );
    }

    /// @notice This test is when a delegating staker stakes twice at the genesis epoch before any epoch
    /// advancements and the reward epoch and global stats are updated correctly.
    function testFuzz_DelegatingStakerStakesTwice_JoiningValidator(
        uint256 operatorStakerIndexToStakeAgainst,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeAgainst = bound(
            operatorStakerIndexToStakeAgainst,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

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
        _fundAddressWithTokensAndApprove(delegatingStaker, amount * 2);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(
            amount,
            timeLock,
            randomOperatorStakers[operatorStakerIndexToStakeAgainst]
        );

        // Delegating staker stakes again
        vm.prank(delegatingStaker);
        stakingFacet.stake(
            amount,
            timeLock,
            randomOperatorStakers[operatorStakerIndexToStakeAgainst]
        );

        // Assert that the delegating staker has two stake records
        assertEq(
            stakingViewsFacet.getStakeRecordCount(
                delegatingStaker,
                randomOperatorStakers[operatorStakerIndexToStakeAgainst]
            ),
            2,
            "Delegating staker should have two stake records"
        );

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Assert that the reward epoch and global stats are updated correctly
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(
                randomOperatorStakers[operatorStakerIndexToStakeAgainst],
                3
            );
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(3);
        uint256 stakeWeightPerStaker = stakingViewsFacet.calculateStakeWeight(
            roundedTimeLock,
            amount
        );
        assertEq(
            rewardEpoch.stakeAmount,
            amount * 3,
            "Reward epoch stake amount should be triple the original amount"
        );
        assertEq(
            rewardEpoch.totalStakeWeight,
            stakeWeightPerStaker * 3,
            "Reward epoch total stake weight should be triple that of each staker's stake weight"
        );
        assertEq(
            globalStats.stakeAmount,
            amount * 6,
            "Global stats stake amount should be 6x the original amount"
        );
        assertEq(
            globalStats.stakeWeight,
            stakeWeightPerStaker * 6,
            "Global stats stake weight should be 6x that of each staker's stake weight"
        );
    }

    /// @notice This test is when a delegating staker stakes once at the genesis epoch, then the epoch
    /// advances to epoch 3 and the delegating staker stakes once more and the reward epoch and global stats
    /// are updated correctly.
    function testFuzz_DelegatingStakerStakesTwice_ActiveValidator(
        uint256 operatorStakerIndexToStakeAgainst,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeAgainst = bound(
            operatorStakerIndexToStakeAgainst,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

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
        _fundAddressWithTokensAndApprove(delegatingStaker, amount * 2);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(
            amount,
            timeLock,
            randomOperatorStakers[operatorStakerIndexToStakeAgainst]
        );

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Delegating staker stakes again
        vm.prank(delegatingStaker);
        stakingFacet.stake(
            amount,
            timeLock,
            randomOperatorStakers[operatorStakerIndexToStakeAgainst]
        );

        // Assert that the delegating staker has two stake records
        assertEq(
            stakingViewsFacet.getStakeRecordCount(
                delegatingStaker,
                randomOperatorStakers[operatorStakerIndexToStakeAgainst]
            ),
            2,
            "Delegating staker should have two stake records"
        );

        // Assert that the reward epoch and global stats for epoch 3 are correct
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(
                randomOperatorStakers[operatorStakerIndexToStakeAgainst],
                3
            );
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(3);
        uint256 stakeWeightPerStaker = stakingViewsFacet.calculateStakeWeight(
            roundedTimeLock,
            amount
        );

        assertEq(
            rewardEpoch.stakeAmount,
            amount * 2,
            "Reward epoch stake amount should be double the original amount"
        );
        assertEq(
            rewardEpoch.totalStakeWeight,
            stakeWeightPerStaker * 2,
            "Reward epoch total stake weight should be double that of each staker's stake weight"
        );
        assertEq(
            globalStats.stakeAmount,
            amount * 5,
            "Global stats stake amount should be 5x the original amount"
        );
        assertEq(
            globalStats.stakeWeight,
            stakeWeightPerStaker * 5,
            "Global stats stake weight should be 5x that of each staker's stake weight"
        );

        // Advance to epoch 4
        _advanceEpochs(realmId, 1, randomOperatorStakers, 3);

        // Assert that the reward epoch and global stats are updated correctly
        _assertOldNewRewardEpochsAndGlobalStatsIncreased(
            OldNewForRewardEpoch(
                globalStats,
                rewardEpoch,
                randomOperatorStakers[operatorStakerIndexToStakeAgainst],
                4
            ),
            amount,
            stakeWeightPerStaker,
            false
        );
    }

    /// @notice This test checks that when a joining validator who has a delegating staker is kicked before
    /// the epoch advances, that the reward epoch and global stats are correctly updated.
    function testFuzz_ValidatorWithDelegatingStakerKickedBeforeAdvanceEpoch_JoiningValidator(
        uint256 operatorStakerIndexToStakeAgainst,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeAgainst = bound(
            operatorStakerIndexToStakeAgainst,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        // Setup complaint config for reason 1
        LibStakingStorage.ComplaintConfig
            memory complaintConfig = LibStakingStorage.ComplaintConfig({
                tolerance: 6,
                intervalSecs: 90,
                kickPenaltyPercent: 0.5 ether, // 50%
                kickPenaltyDemerits: 1 // set this low enough to prevent slashing of that validator during kicking
            });
        stakingAdminFacet.setComplaintConfig(1, complaintConfig);

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

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Set up new validator
        address[] memory newValidators = new address[](1);
        newValidators[0] = address(0x999);
        _setupValidators(
            realmId,
            newValidators,
            amount * 10,
            amount,
            timeLock,
            _generateUint256sWithOffset(1, 4)
        );

        // Set up delegating staker
        address delegatingStaker = address(0x111);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with new joining validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, newValidators[0]);

        // Assert that the reward epoch for this kicked validator is updated
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(newValidators[0], 4);
        uint256 stakeWeightPerStaker = stakingViewsFacet.calculateStakeWeight(
            roundedTimeLock,
            amount
        );
        assertEq(
            rewardEpoch.totalStakeWeight,
            stakeWeightPerStaker * 2,
            "Reward epoch total stake weight should be tentatively attributed to"
        );
        assertEq(
            rewardEpoch.stakeAmount,
            amount * 2,
            "Reward epoch stake amount should be tentatively attributed to"
        );

        // Kick the new validator without slashing them
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            address validator = randomOperatorStakers[i];
            vm.prank(validator);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                newValidators[0],
                1,
                ""
            );
        }

        // Advance one epoch
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        _advanceEpochs(realmId, 1, randomOperatorStakers, 3);

        // Assert that the global stats are correctly updated
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(3);
        assertEq(
            globalStats.stakeWeight,
            stakeWeightPerStaker * 4,
            "Reward epoch total stake weight should be attributed to by the original 4 validators"
        );
        assertEq(
            globalStats.stakeAmount,
            amount * 4,
            "Reward epoch total stake amount should be attributed to by the original 4 validators"
        );
    }

    /// @notice This test checks that when a delegating staker withdraws before the validator joins a realm,
    /// that the reward epoch and global stats are correctly updated.
    function testFuzz_DelegatingStakerWithdrawsBeforeValidatorJoins(
        uint256 operatorStakerIndexToStakeAgainst,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToStakeAgainst = bound(
            operatorStakerIndexToStakeAgainst,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

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

        // Advance to epoch 3
        _advanceEpochs(realmId, 2, randomOperatorStakers, 1);

        // Setup new validator
        address[] memory newValidators = new address[](1);
        newValidators[0] = address(0x999);
        _setupValidatorsStakeOnly(newValidators, amount * 10, amount, timeLock);

        // Setup delegating staker
        address delegatingStaker = address(0x111);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with the new validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, newValidators[0]);

        // Delegating staker withdraws
        vm.prank(delegatingStaker);
        stakingFacet.unfreezeStake(newValidators[0], 1);

        // Fast forward to after the timelock has fully decayed.
        skip(800 days);

        uint256 balanceBefore = token.balanceOf(delegatingStaker);

        vm.prank(delegatingStaker);
        stakingFacet.withdraw(newValidators[0], 1);

        assertEq(token.balanceOf(delegatingStaker), balanceBefore + amount);

        // Set the IP and port of the validator
        vm.prank(newValidators[0]);
        stakingValidatorFacet.setIpPortNodeAddress(1, 1, 1, newValidators[0]);

        // The new validator joins a realm
        vm.prank(newValidators[0]);
        stakingValidatorFacet.requestToJoin(realmId);

        // Advance the epoch
        address[] memory allValidators = new address[](5);
        allValidators[0] = newValidators[0];
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            allValidators[i + 1] = randomOperatorStakers[i];
        }

        _advanceEpochs(realmId, 1, allValidators, 3);

        // Assert that the reward epoch are correctly updated
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(newValidators[0], 4);
        uint256 stakeWeightPerStaker = stakingViewsFacet.calculateStakeWeight(
            roundedTimeLock,
            amount
        );
        assertEq(rewardEpoch.totalStakeWeight, stakeWeightPerStaker);
        assertEq(rewardEpoch.stakeAmount, amount);

        // Assert that the global stats stake weight and amount only accounts for 5 stakes.
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(4);
        assertEq(globalStats.stakeWeight, stakeWeightPerStaker * 5);
        assertEq(globalStats.stakeAmount, amount * 5);

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Get the current epoch number for realm 1
        uint256 realm1CurrentEpochNumber = stakingViewsFacet.epoch(1).number;

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            allValidators,
            realm1CurrentEpochNumber
        );
    }

    /// @notice This test checks that when a staker unfreezes their stake, that their stake weight decreases
    /// properly.
    function testFuzz_UnfreezeSlope_ActiveValidator(
        uint256 operatorStakerIndexToUnfreeze,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 180 days); // use a small upper bound to help with gas for this test
        operatorStakerIndexToUnfreeze = bound(
            operatorStakerIndexToUnfreeze,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;
        uint256 epochLength = 10 days; // lengthen this so we can run this test without out of gas.

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
        address operatorToUnfreeze = randomOperatorStakers[
            operatorStakerIndexToUnfreeze
        ];

        // Advance to epoch 3
        _advanceEpochsCustomEpochLength(
            realmId,
            2,
            randomOperatorStakers,
            1,
            epochLength
        );

        // Validator unfreezes their stake
        vm.prank(operatorToUnfreeze);
        stakingFacet.unfreezeStake(operatorToUnfreeze, 1);

        // Get stake record
        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(operatorToUnfreeze, 1, operatorToUnfreeze);

        // From here onwards, assert that the total stake weight decreases with every epoch for that validator.
        uint256 startingEpochNumber = 3;
        uint256 numEpochsToAdvance = roundedTimeLock / epochLength;
        uint256 lastStakeWeight = type(uint256).max;
        uint256 lastGlobalStakeWeight = type(uint256).max;
        uint256 nextEpochToSignalReady = 0;
        for (
            uint256 i = startingEpochNumber;
            i < startingEpochNumber + numEpochsToAdvance;
            i++
        ) {
            skip(epochLength);
            stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);

            // Check when the timelock has reduced to the point where it no longer meets the minimum timelock configuration.
            uint256 remainingTimelock = stakingViewsFacet.getTimelockInEpoch(
                operatorToUnfreeze,
                stakeRecord,
                nextEpochToSignalReady
            );
            if (
                remainingTimelock <
                stakingViewsFacet.globalConfig().minSelfStakeTimelock
            ) {
                break;
            }

            for (uint256 j = 0; j < randomOperatorStakers.length; j++) {
                vm.prank(randomOperatorStakers[j]);
                stakingValidatorFacet.signalReadyForNextEpoch(realmId, i);
            }

            stakingValidatorFacet.advanceEpoch(realmId);

            // Compare the last stake weight with the current stake weight
            LibStakingStorage.RewardEpoch memory newRewardEpoch = stakingFacet
                .getRewardEpoch(operatorToUnfreeze, i + 1);
            LibStakingStorage.RewardEpochGlobalStats
                memory newGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(i + 1);
            assertLt(newRewardEpoch.totalStakeWeight, lastStakeWeight);
            assertLt(newGlobalStats.stakeWeight, lastGlobalStakeWeight);
            lastStakeWeight = newRewardEpoch.totalStakeWeight;
            lastGlobalStakeWeight = newGlobalStats.stakeWeight;
            nextEpochToSignalReady = i + 1;
        }

        // The immediate next attempt to signal ready for that validator should fail.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingFacet.InsufficientSelfStake.selector,
                operatorToUnfreeze
            )
        );
        vm.prank(operatorToUnfreeze);
        stakingValidatorFacet.signalReadyForNextEpoch(
            realmId,
            nextEpochToSignalReady
        );
    }

    /// @notice This test is when an active validator unfreezes, lets it thaw for some time before requesting to leave,
    /// that the reward epoch and global stats are correctly updated.
    function testFuzz_RequestToLeaveHalfwayUnfreeze_ActiveValidator(
        uint256 operatorStakerIndexToUnfreeze,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToUnfreeze = bound(
            operatorStakerIndexToUnfreeze,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

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
        address operatorToUnfreeze = randomOperatorStakers[
            operatorStakerIndexToUnfreeze
        ];

        // Advance to epoch 3
        uint256 currentEpoch = _advanceEpochs(
            realmId,
            2,
            randomOperatorStakers,
            1
        );

        // Validator unfreezes their stake
        vm.prank(operatorToUnfreeze);
        stakingFacet.unfreezeStake(operatorToUnfreeze, 1);

        // Advance 35 epochs before the validator requests to leave
        // 35 so that the unfreeze start is in the past, allowing the slope to be used for decrementing stake weights.
        currentEpoch = _advanceEpochs(
            realmId,
            35,
            randomOperatorStakers,
            currentEpoch
        );

        // Assert that the global stats are correctly updated
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(
                currentEpoch
            );
        uint256 stakeWeightPerValidator = stakingViewsFacet
            .calculateStakeWeight(roundedTimeLock, amount);
        assertLt(globalStats.stakeWeight, stakeWeightPerValidator * 4);
        assertGt(globalStats.stakeWeight, stakeWeightPerValidator * 3);
        assertEq(globalStats.stakeAmount, amount * 4);

        // Validator requests to leave
        vm.prank(operatorToUnfreeze);
        stakingValidatorFacet.requestToLeave();

        // Advance 1 epoch to deal out validator
        address[] memory remainingValidators = new address[](4);
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            if (randomOperatorStakers[i] != operatorToUnfreeze) {
                remainingValidators[i] = randomOperatorStakers[i];
            }
        }
        currentEpoch = _advanceEpochs(
            realmId,
            1,
            remainingValidators,
            currentEpoch
        );

        // Assert that the reward epoch and global stats are correctly updated
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(operatorToUnfreeze, currentEpoch);
        globalStats = stakingViewsFacet.getRewardEpochGlobalStats(currentEpoch);
        _assertRewardEpochIsZero(rewardEpoch);
        assertEq(globalStats.stakeWeight, stakeWeightPerValidator * 3);
        assertEq(globalStats.stakeAmount, amount * 3);

        // Now, let's process the unfreeze end and make sure that epoch advancements no longer cause the stake weights
        // to decrease.

        // Get the current epoch number for realm 1
        uint256 realm1CurrentEpochNumber = stakingViewsFacet.epoch(1).number;

        _advanceEpochsAndAssertConstantStakeWeight(
            1,
            10,
            remainingValidators,
            realm1CurrentEpochNumber
        );
    }

    /// @notice This test is when an active validator unfreezes and then stakes again,
    /// that the reward epoch and global stats are correctly updated.
    function testFuzz_ValidatorStakeAgainHalfwayUnfreeze_ActiveValidator(
        uint256 operatorStakerIndexToUnfreeze,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 minSelfStakeTimelock = stakingViewsFacet
            .globalConfig()
            .minSelfStakeTimelock;
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(
            timeLock,
            minSelfStakeTimelock,
            minSelfStakeTimelock + 15 days
        ); // use smaller upper bound to help with gas for this test
        operatorStakerIndexToUnfreeze = bound(
            operatorStakerIndexToUnfreeze,
            0,
            3
        );
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;
        uint256 epochLength = 1 days;
        stakingAdminFacet.setEpochLength(realmId, epochLength);

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
        address operatorToUnfreeze = randomOperatorStakers[
            operatorStakerIndexToUnfreeze
        ];

        // Advance to epoch 3
        _advanceEpochsCustomEpochLength(
            realmId,
            2,
            randomOperatorStakers,
            1,
            epochLength
        );

        // Validator unfreezes their stake
        vm.prank(operatorToUnfreeze);
        stakingFacet.unfreezeStake(operatorToUnfreeze, 1);

        // Validator stakes again
        vm.prank(operatorToUnfreeze);
        stakingFacet.stake(amount, timeLock, operatorToUnfreeze);

        // Advance enough epochs to make sure the validator's first stake is fully thawed and has no stake weight contributions,
        // and then assert that the reward epoch and global stats are correctly updated.
        // Advance many, many epochs to make sure the global stake weight converges to 4 validator's worth.
        uint256 numEpochsToAdvance = (roundedTimeLock / epochLength) + 10;
        for (uint256 i = 3; i < 3 + numEpochsToAdvance; i++) {
            skip(epochLength);
            stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);
            for (uint256 j = 0; j < randomOperatorStakers.length; j++) {
                vm.prank(randomOperatorStakers[j]);
                stakingValidatorFacet.signalReadyForNextEpoch(realmId, i);
            }

            stakingValidatorFacet.advanceEpoch(realmId);
        }

        uint256 currentRewardEpochNumber = numEpochsToAdvance + 3;
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(operatorToUnfreeze, currentRewardEpochNumber);
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(
                currentRewardEpochNumber
            );
        uint256 stakeWeightPerValidator = stakingViewsFacet
            .calculateStakeWeight(roundedTimeLock, amount);
        assertApproxEqRel(
            rewardEpoch.totalStakeWeight,
            stakeWeightPerValidator,
            0.000000001 ether // 0.0000001% delta
        );
        assertEq(rewardEpoch.stakeAmount, amount * 2);
        assertApproxEqRel(
            globalStats.stakeWeight,
            stakeWeightPerValidator * 4,
            0.000000001 ether // 0.0000001% delta
        );
        assertEq(globalStats.stakeAmount, amount * 5);
    }

    /// @notice This test checks that the unique delegating staker count is correctly updated when different delegating
    /// stakers stake and unstake.
    function testFuzz_UniqueDelegatingStakerCount(
        uint256 operatorStakerIndexToDelegateTo,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 numValidators = 6;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToDelegateTo = bound(
            operatorStakerIndexToDelegateTo,
            0,
            numValidators - 1
        );

        // Setup validators
        address[] memory operatorStakers = _generateAddresses(numValidators);
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(numValidators)
        );
        address operatorToDelegateTo = operatorStakers[
            operatorStakerIndexToDelegateTo
        ];

        // Assert that the unique delegating staker count is 0 for each validator.
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            assertEq(
                stakingViewsFacet
                    .validators(operatorStakers[i])
                    .uniqueDelegatingStakerCount,
                0
            );
        }

        // Set up the delegating stakers

        address delegatingStaker1 = address(0x999);
        address delegatingStaker2 = address(0x888);
        address delegatingStaker3 = address(0x777);

        _fundAddressWithTokensAndApprove(delegatingStaker1, amount);
        _fundAddressWithTokensAndApprove(delegatingStaker2, amount * 2);
        _fundAddressWithTokensAndApprove(delegatingStaker3, amount * 3);

        // Stake and join. The plan is the following:
        // - delegatingStaker1 stakes once
        // - delegatingStaker2 stakes twice
        // - delegatingStaker3 stakes thrice
        // - delegatingStaker1 unstakes once
        // - delegatingStaker2 unstakes twice
        // - delegatingStaker3 unstakes thrice
        // Assertions will be done after each step.

        vm.prank(delegatingStaker1);
        stakingFacet.stake(amount, timeLock, operatorToDelegateTo);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            1
        );

        vm.prank(delegatingStaker2);
        stakingFacet.stake(amount, timeLock, operatorToDelegateTo);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            2
        );

        vm.prank(delegatingStaker2);
        stakingFacet.stake(amount, timeLock, operatorToDelegateTo);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            2
        );

        vm.prank(delegatingStaker3);
        stakingFacet.stake(amount, timeLock, operatorToDelegateTo);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            3
        );

        vm.prank(delegatingStaker3);
        stakingFacet.stake(amount, timeLock, operatorToDelegateTo);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            3
        );

        vm.prank(delegatingStaker3);
        stakingFacet.stake(amount, timeLock, operatorToDelegateTo);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            3
        );

        console.log("operatorToDelegateTo", operatorToDelegateTo);

        // Unfreeze all stakes
        vm.prank(delegatingStaker1);
        stakingFacet.unfreezeStake(operatorToDelegateTo, 1);
        vm.prank(delegatingStaker2);
        stakingFacet.unfreezeStake(operatorToDelegateTo, 1);
        vm.prank(delegatingStaker2);
        stakingFacet.unfreezeStake(operatorToDelegateTo, 2);
        vm.prank(delegatingStaker3);
        stakingFacet.unfreezeStake(operatorToDelegateTo, 1);
        vm.prank(delegatingStaker3);
        stakingFacet.unfreezeStake(operatorToDelegateTo, 2);
        vm.prank(delegatingStaker3);
        stakingFacet.unfreezeStake(operatorToDelegateTo, 3);

        skip(800 days);

        // Withdraw all stakes
        vm.prank(delegatingStaker1);
        stakingFacet.withdraw(operatorToDelegateTo, 1);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            2
        );

        vm.prank(delegatingStaker2);
        stakingFacet.withdraw(operatorToDelegateTo, 1);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            2
        );
        vm.prank(delegatingStaker2);
        stakingFacet.withdraw(operatorToDelegateTo, 2);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            1
        );

        vm.prank(delegatingStaker3);
        stakingFacet.withdraw(operatorToDelegateTo, 1);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            1
        );
        vm.prank(delegatingStaker3);
        stakingFacet.withdraw(operatorToDelegateTo, 2);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            1
        );
        vm.prank(delegatingStaker3);
        stakingFacet.withdraw(operatorToDelegateTo, 3);
        assertEq(
            stakingViewsFacet
                .validators(operatorToDelegateTo)
                .uniqueDelegatingStakerCount,
            0
        );
    }

    /// @notice This test checks that the registerAttestedWalletDisabled flag is correctly set for active validators,
    /// which prevents them from registering an attested wallet.
    function testFuzz_RegisterAttestedWalletDisabledForActiveValidators(
        uint256 operatorStakerIndexToLeave,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 numValidators = 6;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToLeave = bound(
            operatorStakerIndexToLeave,
            0,
            numValidators - 1
        );

        // Setup validators
        address[] memory operatorStakers = _generateAddresses(numValidators);
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(numValidators)
        );
        address operatorStakerToLeave = operatorStakers[
            operatorStakerIndexToLeave
        ];

        // Assert that each validator can register an attested wallet multiple times before they
        // become active validators.
        bytes[] memory walletKeys = _generatePubKeys(operatorStakers.length);

        for (uint256 i = 0; i < operatorStakers.length; i++) {
            // Register the first time.
            vm.prank(operatorStakers[i]);
            stakingValidatorFacet.registerAttestedWallet(
                operatorStakers[i],
                vm.randomAddress(),
                walletKeys[i],
                50 + i,
                50 + i
            );

            // Register the second time.
            vm.prank(operatorStakers[i]);
            stakingValidatorFacet.registerAttestedWallet(
                operatorStakers[i],
                vm.randomAddress(),
                walletKeys[i],
                50 + i,
                50 + i
            );
        }

        // Now, advance the epoch so the validators become active validators.
        _advanceEpochs(1, 1, operatorStakers, 1);

        // Assert that each validator cannot register an attested wallet after they become active validators.
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            vm.prank(operatorStakers[i]);
            vm.expectRevert(
                StakingValidatorFacet
                    .ValidatorRegisterAttestedWalletDisabled
                    .selector
            );
            stakingValidatorFacet.registerAttestedWallet(
                operatorStakers[i],
                vm.randomAddress(),
                walletKeys[i],
                50 + i,
                50 + i
            );
        }

        // Now, a random operator staker requests to leave.
        vm.prank(operatorStakerToLeave);
        stakingValidatorFacet.requestToLeave();

        address[] memory remainingValidators = new address[](
            operatorStakers.length
        );
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            if (operatorStakers[i] == operatorStakerToLeave) {
                continue;
            }
            remainingValidators[i] = operatorStakers[i];
        }

        // Advance the epoch to deal out this validator.
        _advanceEpochs(1, 1, remainingValidators, 2);

        // Assert that the validator CAN register an attested wallet after they are dealt out.
        // For the remaining active validators, they should still not be able to register.
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            if (operatorStakers[i] == operatorStakerToLeave) {
                vm.prank(operatorStakers[i]);
                stakingValidatorFacet.registerAttestedWallet(
                    operatorStakers[i],
                    vm.randomAddress(),
                    walletKeys[i],
                    50 + i,
                    50 + i
                );
            } else {
                vm.prank(operatorStakers[i]);
                vm.expectRevert(
                    StakingValidatorFacet
                        .ValidatorRegisterAttestedWalletDisabled
                        .selector
                );
                stakingValidatorFacet.registerAttestedWallet(
                    operatorStakers[i],
                    vm.randomAddress(),
                    walletKeys[i],
                    50 + i,
                    50 + i
                );
            }
        }
    }

    /// @notice This test checks that the stakerToValidatorsTheyStakedTo mapping is correctly
    /// updated when a delegating staker stakes and unstakes.
    function testFuzz_StakerToValidatorsTheyStakedTo(
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 numValidators = 6;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);

        // Setup validators
        address[] memory operatorStakers = _generateAddresses(numValidators);
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(numValidators)
        );

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(
            delegatingStaker,
            amount * numValidators
        );

        // Delegating staker stakes against each validator, and we assert at each step.
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            vm.prank(delegatingStaker);
            stakingFacet.stake(amount, timeLock, operatorStakers[i]);
            assertEq(
                stakingViewsFacet
                    .stakerToValidatorsTheyStakedTo(delegatingStaker)
                    .length,
                i + 1
            );
        }

        // Delegating staker unfreezes all their stake.
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(operatorStakers[i], 1);
        }

        skip(800 days);

        // Delegating staker withdraws all their stake, and we assert at each step.
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            vm.prank(delegatingStaker);
            stakingFacet.withdraw(operatorStakers[i], 1);
            assertEq(
                stakingViewsFacet
                    .stakerToValidatorsTheyStakedTo(delegatingStaker)
                    .length,
                operatorStakers.length - 1 - i
            );
        }
    }

    /// @notice This test checks that the delegated stakers with unfreezing stakes mapping is correctly updated when a staker unfreezes their stake.
    function testFuzz_DelegatedStakersWithUnfreezingStakes(
        uint256 amount,
        uint256 timeLock,
        uint256 operatorStakerIndex
    ) public {
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _fundAddressesWithTokensAndApprove(randomOperatorStakers, 1_000 ether);

        // Input assumptions
        uint256 realmId = 1;
        operatorStakerIndex = bound(operatorStakerIndex, 0, 3);
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        address operatorStakerAddress = randomOperatorStakers[
            operatorStakerIndex
        ];

        // Set the IP and port of the validator
        vm.prank(operatorStakerAddress);
        stakingValidatorFacet.setIpPortNodeAddress(
            1,
            1,
            1,
            operatorStakerAddress
        );

        // Emit event check
        vm.expectEmit(true, true, true, true);
        emit StakeRecordCreated(
            operatorStakerAddress,
            1,
            amount,
            operatorStakerAddress
        );

        // Call stakeAndJoin
        vm.prank(operatorStakerAddress);
        stakingFacet.stake(amount, timeLock, operatorStakerAddress);
        vm.prank(operatorStakerAddress);
        stakingValidatorFacet.requestToJoin(realmId);

        // Set up delegated staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes twice with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakerAddress);

        // Assert that the delegated stakers count is 0.
        {
            uint256 delegatedStakersWithUnfreezingStakesCount = stakingViewsFacet
                    .getDelegatedStakersWithUnfreezingStakesCount(
                        operatorStakerAddress
                    );
            assertEq(delegatedStakersWithUnfreezingStakesCount, 0);
        }

        // Unfreeze the first stake
        vm.prank(delegatingStaker);
        stakingFacet.unfreezeStake(operatorStakerAddress, 1);

        // Assert that the delegated stakers count is 1.
        {
            uint256 delegatedStakersWithUnfreezingStakesCount = stakingViewsFacet
                    .getDelegatedStakersWithUnfreezingStakesCount(
                        operatorStakerAddress
                    );
            assertEq(delegatedStakersWithUnfreezingStakesCount, 1);
            address[]
                memory delegatedStakersWithUnfreezingStakes = stakingViewsFacet
                    .getDelegatedStakersWithUnfreezingStakes(
                        operatorStakerAddress,
                        delegatedStakersWithUnfreezingStakesCount,
                        0
                    );
            assertEq(delegatedStakersWithUnfreezingStakes.length, 1);
            assertEq(delegatedStakersWithUnfreezingStakes[0], delegatingStaker);
        }

        // Fast forward and withdraw the first stake
        skip(800 days);

        vm.prank(delegatingStaker);
        stakingFacet.withdraw(operatorStakerAddress, 1);

        // Assert that the delegated stakers count is 0.
        {
            uint256 delegatedStakersWithUnfreezingStakesCount = stakingViewsFacet
                    .getDelegatedStakersWithUnfreezingStakesCount(
                        operatorStakerAddress
                    );
            assertEq(delegatedStakersWithUnfreezingStakesCount, 0);
        }
    }

    /// @notice This test checks that when a delegated staker unfreezes their stake against an active validator,
    /// that when the unfreeze start has been processed, and after some time the validator leaves the realm, the
    /// realm advances some epochs before the validator rejoins the realm, that the unfreeze continues to result
    /// the stake weight decrementing, until the unfreeze end is reached.
    function testFuzz_UnfreezeContinuesAfterRejoining(
        uint256 amount,
        uint256 timeLock
    ) public {
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        // Setup validators
        address[] memory operatorStakers = _generateAddresses(4);
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        address operatorStakerToStakeAgainst = operatorStakers[0];

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes against the operator staker
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeAgainst);

        // Advance the epoch to 4
        uint256 epochNumber = _advanceEpochs(1, 4, operatorStakers, 1);

        // Get the stake weight for the validator before unfreezing.
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(
                operatorStakerToStakeAgainst,
                stakingViewsFacet.getRewardEpochNumber(1)
            );
        uint256 originalStakeWeight = rewardEpoch.totalStakeWeight;

        // Unfreeze the stake
        vm.prank(delegatingStaker);
        stakingFacet.unfreezeStake(operatorStakerToStakeAgainst, 1);

        // Advance the epoch another 30 epochs to make sure the unfreeze start has been processed
        epochNumber = _advanceEpochs(1, 30, operatorStakers, epochNumber);

        // Assert that the stake weight has decremented.
        {
            LibStakingStorage.RewardEpoch memory newRewardEpoch = stakingFacet
                .getRewardEpoch(
                    operatorStakerToStakeAgainst,
                    stakingViewsFacet.getRewardEpochNumber(1)
                );
            assertLt(
                newRewardEpoch.totalStakeWeight,
                originalStakeWeight,
                "stake weight should have decremented"
            );
        }

        // Now, the validator leaves the realm.
        vm.prank(operatorStakerToStakeAgainst);
        stakingValidatorFacet.requestToLeave();

        // Advance 5 epochs to deal out the validator and make sure there is some progress before they rejoin.
        address[] memory remainingValidators = new address[](
            operatorStakers.length
        );
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            if (operatorStakers[i] == operatorStakerToStakeAgainst) {
                continue;
            }
            remainingValidators[i] = operatorStakers[i];
        }
        epochNumber = _advanceEpochs(1, 5, remainingValidators, epochNumber);

        // Now, the validator rejoins the realm.
        vm.prank(operatorStakerToStakeAgainst);
        stakingValidatorFacet.requestToJoin(1);

        // Advance 1 epoch to deal in the validator.
        epochNumber = _advanceEpochs(1, 1, operatorStakers, epochNumber);

        // Get the stake weight for the validator.
        rewardEpoch = stakingFacet.getRewardEpoch(
            operatorStakerToStakeAgainst,
            stakingViewsFacet.getRewardEpochNumber(1)
        );

        // Advance 5 epochs to allow the stake weight to decrement.
        epochNumber = _advanceEpochs(1, 5, operatorStakers, epochNumber);

        // Assert that the stake weight has decremented.
        {
            LibStakingStorage.RewardEpoch memory newRewardEpoch = stakingFacet
                .getRewardEpoch(
                    operatorStakerToStakeAgainst,
                    stakingViewsFacet.getRewardEpochNumber(1)
                );
            assertLt(
                newRewardEpoch.totalStakeWeight,
                rewardEpoch.totalStakeWeight,
                "stake weight should have decremented"
            );
        }
    }

    /// @notice This test checks that when a delegated staker unfreezes their stake against a validator
    /// and then that validator gets slashed, that the delegated staker can still withdraw their stake
    /// eventually. Then, when they are admin-rejoined, that their stake weights are constant.
    function testFuzz_WithdrawAfterUnfreezingAgainstSlashedValidator(
        uint256 amount,
        uint256 timeLock
    ) public {
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        {
            // Set complaint config for reason 1
            LibStakingStorage.ComplaintConfig
                memory complaintConfig = LibStakingStorage.ComplaintConfig({
                    tolerance: 6,
                    intervalSecs: 90,
                    kickPenaltyPercent: 0.5 ether, // 50%
                    kickPenaltyDemerits: 10
                });
            stakingAdminFacet.setComplaintConfig(1, complaintConfig);
        }

        // Setup validators
        address[] memory operatorStakers = _generateAddresses(4);
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        address operatorStakerToStakeAgainst = operatorStakers[0];

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes against the operator staker
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakerToStakeAgainst);

        // Advance the epoch to 4
        uint256 epochNumber = _advanceEpochs(1, 4, operatorStakers, 1);

        // Unfreeze the stake
        vm.prank(delegatingStaker);
        stakingFacet.unfreezeStake(operatorStakerToStakeAgainst, 1);

        // Advance the epoch another 30 epochs to make sure the unfreeze start has been processed
        epochNumber = _advanceEpochs(1, 30, operatorStakers, epochNumber);

        // Kick and slash the validator
        address[] memory validatorsNotKicked = new address[](
            operatorStakers.length
        );
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            if (operatorStakers[i] == operatorStakerToStakeAgainst) {
                continue;
            }
            validatorsNotKicked[i] = operatorStakers[i];
        }
        for (uint256 i = 0; i < validatorsNotKicked.length; i++) {
            address validator = validatorsNotKicked[i];
            if (validator == address(0)) {
                continue;
            }

            vm.prank(validator);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                operatorStakerToStakeAgainst,
                1,
                ""
            );
        }

        // Advance some epochs to deal out the validator who was kicked.
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        epochNumber = _advanceEpochs(1, 30, validatorsNotKicked, epochNumber);

        // Now, we fast forward 800 days to allow the delegated staker to withdraw their stake.
        skip(800 days);

        {
            // Delegated staker claims their rewards
            uint256 balanceBefore = stakingFacet.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.claimStakeRewards(
                1,
                operatorStakerToStakeAgainst,
                1,
                type(uint256).max
            );

            assertGt(
                token.balanceOf(delegatingStaker),
                balanceBefore,
                "delegated staker should have claimed their rewards"
            );
        }

        // Delegated staker withdraws their stake.
        vm.prank(delegatingStaker);
        stakingFacet.withdraw(operatorStakerToStakeAgainst, 1);

        // Now, when the validator is admin-rejoined, the stake weight should be constant.
        stakingAdminFacet.adminRejoinValidator(1, operatorStakerToStakeAgainst);

        // Advance 3 epochs to deal in the validator.
        epochNumber = _advanceEpochs(1, 3, operatorStakers, epochNumber);

        // Get the stake weight for the validator.
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(
                operatorStakerToStakeAgainst,
                stakingViewsFacet.getRewardEpochNumber(1)
            );
        uint256 originalStakeWeight = rewardEpoch.totalStakeWeight;

        // Advance 5 epochs before checking the stake weight again.
        epochNumber = _advanceEpochs(1, 5, operatorStakers, epochNumber);

        // Assert that the stake weight is constant.
        {
            LibStakingStorage.RewardEpoch memory newRewardEpoch = stakingFacet
                .getRewardEpoch(
                    operatorStakerToStakeAgainst,
                    stakingViewsFacet.getRewardEpochNumber(1)
                );
            assertEq(
                newRewardEpoch.totalStakeWeight,
                originalStakeWeight,
                "stake weight should be constant"
            );
        }
    }

    /// @notice This test checks that, after a stake has started unfreezing, and given 2 realms and epoch transitions for 1
    /// realm gets stuck for some time, that this does not cause the stake weight to continue depleting past where it should,
    /// until it renders the validator no longer able to continue participating anymore.
    function testFuzz_UnfreezePastUnfrozen_2Realms_1Stuck(
        uint256 amount,
        uint256 delegatingStakerAmount,
        bool testDelegatingStaker
    ) public {
        uint256 numValidators = 4;
        uint256 operatorStakerIndexToUnfreeze = 0;
        // Choose a low amount for the initial self stake amount so it's faster to test the decrementing stake weight.
        amount = bound(amount, 2 ether, 1_000 ether);
        delegatingStakerAmount = bound(
            delegatingStakerAmount,
            100 ether,
            1_000 ether
        );
        uint256 timeLock = 2 days;

        // Add the second realm
        stakingAdminFacet.addRealm();
        // Also allocate rewards budget for realm 2
        stakingAdminFacet.increaseRewardPool(2, rewardsBudget);

        {
            // // Set the minimum self stake timelock to be 2 days.
            LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
                .globalConfig();
            config.minSelfStakeTimelock = 2 days;
            config.minTimeLock = 2 days;
            stakingAdminFacet.setConfig(config);
        }

        // Setup for Realm 1
        address[] memory operatorStakersRealm1 = _generateAddresses(
            numValidators
        );
        address operatorStakerToUnfreeze = operatorStakersRealm1[
            operatorStakerIndexToUnfreeze
        ];

        // Setup for Realm 2
        address[] memory operatorStakersRealm2 = _generateAddressesWithOffset(
            numValidators,
            numValidators
        );

        // Validators join realms
        _setupValidators(
            1,
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(numValidators)
        );
        _setupValidators(
            2,
            operatorStakersRealm2,
            amount * 10,
            amount,
            timeLock,
            _generateUint256sWithOffset(numValidators, numValidators)
        );

        // Get the original stake weight for the operator staker from the reward epoch
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(operatorStakerToUnfreeze, 2); // The attribution is on the next epoch number
        uint256 originalStakeWeight = rewardEpoch.totalStakeWeight;

        address delegatingStaker = address(0x999);
        if (testDelegatingStaker) {
            // Set up the delegating staker with significant stake weight
            _fundAddressWithTokensAndApprove(
                delegatingStaker,
                delegatingStakerAmount
            );

            // Delegating staker stakes with a random validator
            vm.prank(delegatingStaker);
            stakingFacet.stake(
                delegatingStakerAmount,
                timeLock,
                operatorStakerToUnfreeze
            );
        } else {
            vm.prank(operatorStakerToUnfreeze);
            stakingFacet.stake(amount, timeLock, operatorStakerToUnfreeze);
        }

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

        if (testDelegatingStaker) {
            // Unfreeze the stake
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(operatorStakerToUnfreeze, 1);
        } else {
            // Unfreeze the stake
            vm.prank(operatorStakerToUnfreeze);
            stakingFacet.unfreezeStake(operatorStakerToUnfreeze, 2);
        }

        // Advance enough epochs for realm 2 to take over the endRewardEpochNumber that the unfreeze would be targeting
        // if it were to assume perfectly regular intervals for each epoch advancement.
        uint256 epochLength = stakingViewsFacet.epoch(1).epochLength;
        uint256 epochNumberRealm1 = stakingViewsFacet.epoch(1).number;
        uint256 epochNumberRealm2 = stakingViewsFacet.epoch(2).number;

        numEpochsToAdvance = timeLock / epochLength;

        epochNumberRealm2 = _advanceEpochs(
            2,
            numEpochsToAdvance,
            operatorStakersRealm2,
            epochNumberRealm2
        );

        // Now, advance the same number of epochs for realm 1
        epochNumberRealm1 = _advanceEpochs(
            1,
            numEpochsToAdvance,
            operatorStakersRealm1,
            epochNumberRealm1
        );

        // And, continue advancing epochs for realm 1 and assert that the stake weight should no longer be depleting.
        uint256 rewardEpochNumberBeforeAdvance = stakingViewsFacet
            .getRewardEpochNumber(1);
        LibStakingStorage.RewardEpoch
            memory rewardEpochBeforeAdvance = stakingFacet.getRewardEpoch(
                operatorStakerToUnfreeze,
                rewardEpochNumberBeforeAdvance
            );
        _advanceEpochs(1, 50, operatorStakersRealm1, epochNumberRealm1);

        uint256 rewardEpochNumberAfterAdvance = stakingViewsFacet
            .getRewardEpochNumber(1);
        LibStakingStorage.RewardEpoch
            memory rewardEpochAfterAdvance = stakingFacet.getRewardEpoch(
                operatorStakerToUnfreeze,
                rewardEpochNumberAfterAdvance
            );

        // Assert that the stake weight is not depleting
        assertEq(
            rewardEpochAfterAdvance.totalStakeWeight,
            rewardEpochBeforeAdvance.totalStakeWeight,
            "stake weight should not be depleting"
        );

        // Also assert that the stake weight does not deplete more than the original amount.
        assertGt(
            rewardEpochAfterAdvance.totalStakeWeight,
            originalStakeWeight,
            "stake weight should not be depleting more than the original amount"
        );
    }

    /// @notice Test to check that the rewards calculations are correct against known inputs and outputs, over a single epoch.
    /// Refer to "LIT example rewards" Google Sheets for the expected rewards per epoch.
    function test_CalcRewards_Vector1() public {
        uint256 p = 0.5 ether;
        uint256 k = 0.5 ether;
        uint256 b_min = 0.004 ether;
        uint256 b_max = 0.0165 ether;
        uint256 maxTimeLock = 364 days;
        uint256 expectedRewardsPerDay = 108_333.333 ether;

        // Set the config
        LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
            .globalConfig();
        config.p = p;
        config.k = k;
        config.bmin = b_min;
        config.bmax = b_max;
        config.maxTimeLock = maxTimeLock;
        stakingAdminFacet.setConfig(config);

        // Create 10 validators and keys and join them.
        address[] memory addresses = _generateAddresses(10);
        uint256[] memory commsKeys = _generateUint256s(10);
        _setupValidators(
            1,
            addresses,
            2_000_000 ether,
            1_000_000 ether,
            182 days,
            commsKeys
        );

        // Now fetch the global stats and rewards
        LibStakingStorage.RewardEpochGlobalStats memory g = stakingViewsFacet
            .getRewardEpochGlobalStats(2); // advancing the epoch to 2 results in attribution to epoch 2 stake weights

        uint256 actualRewardsPerDay = stakingViewsFacet.calculateRewardsPerDay(
            g
        );

        assertApproxEqRel(
            actualRewardsPerDay,
            expectedRewardsPerDay,
            0.0000001 ether, // 0.00001% delta
            "reward calculation did not match expected"
        );
    }

    /// @notice Test to check that the rewards calculations are correct against known inputs and outputs, over multiple epochs.
    /// Refer to "LIT example rewards" Google Sheets for the expected rewards per epoch.
    function test_CalcRewards_Vector2() public {
        uint256 p = 0.5 ether;
        uint256 k = 0.5 ether;
        uint256 b_min = 0.004 ether;
        uint256 b_max = 0.0165 ether;
        uint256 maxTimeLock = 364 days; // even numbers for good vibes ;)

        // Set the config
        LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
            .globalConfig();
        config.p = p;
        config.k = k;
        config.bmin = b_min;
        config.bmax = b_max;
        config.maxTimeLock = maxTimeLock;
        stakingAdminFacet.setConfig(config);

        uint256 realmId = 1;

        uint256[10] memory amountToStakePerNode = [
            uint256(1_300_000 ether),
            uint256(500_000 ether),
            uint256(260_000 ether),
            uint256(240_000 ether),
            uint256(210_000 ether),
            uint256(200_000 ether),
            uint256(150_000 ether),
            uint256(120_000 ether),
            uint256(100_000 ether),
            uint256(100_000 ether)
        ];
        uint256[10] memory timeLockPerNode = [
            (0.5 ether * maxTimeLock) / 1 ether,
            maxTimeLock,
            (0.5 ether * maxTimeLock) / 1 ether,
            (0.5 ether * maxTimeLock) / 1 ether,
            maxTimeLock,
            maxTimeLock,
            (0.5 ether * maxTimeLock) / 1 ether,
            (0.5 ether * maxTimeLock) / 1 ether,
            (0.1 ether * maxTimeLock) / 1 ether,
            (0.1 ether * maxTimeLock) / 1 ether
        ];

        // Setup validators
        address[] memory randomOperatorStakers = _generateAddresses(10);
        uint256[] memory randomCommsKeys = _generateUint256s(10);
        // Have all validators stake and join
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            uint256 perOperatorAmountToTransferAndApprove = amountToStakePerNode[
                    i
                ] * 10;
            uint256 perOperatorAmountToStake = amountToStakePerNode[i];
            uint256 timeLock = timeLockPerNode[i];

            _fundAddressWithTokensAndApprove(
                randomOperatorStakers[i],
                perOperatorAmountToTransferAndApprove
            );

            // Set the IP and port of the validator
            vm.prank(randomOperatorStakers[i]);
            stakingValidatorFacet.setIpPortNodeAddress(
                1,
                1,
                1,
                randomOperatorStakers[i]
            );

            // Emit event check
            vm.expectEmit(true, true, true, true);
            emit StakeRecordCreated(
                randomOperatorStakers[i],
                1,
                perOperatorAmountToStake,
                randomOperatorStakers[i]
            );

            // Stake and join
            vm.prank(randomOperatorStakers[i]);
            stakingFacet.stake(
                perOperatorAmountToStake,
                timeLock,
                randomOperatorStakers[i]
            );
            vm.prank(randomOperatorStakers[i]);
            stakingValidatorFacet.requestToJoin(realmId);
        }

        stakingAdminFacet.setEpochState(
            realmId,
            LibStakingStorage.States.Active
        );

        // Advance the epoch to 2
        _advanceEpochs(realmId, 1, randomOperatorStakers, 1);

        // Node 8 unfreezes
        vm.prank(randomOperatorStakers[7]);
        stakingFacet.unfreezeStake(randomOperatorStakers[7], 1);

        // Advance the epoch to 13 so that rewards have been earned for 10 epochs (epoch 3 -> 13)
        _advanceEpochs(realmId, 11, randomOperatorStakers, 2);

        // Assert global stats
        LibStakingStorage.RewardEpochGlobalStats memory g = stakingViewsFacet
            .getRewardEpochGlobalStats(13);
        assertEq(
            g.stakeAmount,
            3_180_000 ether,
            "stake amount did not match expected"
        );
        assertApproxEqAbs(
            g.stakeWeight,
            1_964_863 ether,
            200_000 ether,
            "stake weight did not match expected"
        );

        // Assert the rewards earned for the validators
        // The rewards are multiplied by 10 because they earned rewards over 10 epochs
        uint256[10] memory rewardOver10EpochsPerValidator = [
            // Without c_max taking effect, the first validator would have effective stake weight of 33%,
            // and their rewards would be 1_516 * 10 ether. Since c_max is in effect, the effective stake weight
            // is capped at 20%.
            uint256((uint256(1_516 * 0.2 ether) / 0.33 ether) * 10 ether),
            // Without c_max taking effect, the second validator would have effective stake weight of 25%,
            // and their rewards would be 1_166 * 10 ether. Since c_max is in effect, the effective stake weight
            // is capped at 20%.
            uint256((uint256(1_166 * 0.2 ether) / 0.25 ether) * 10 ether),
            uint256(303 * 10 ether),
            uint256(280 * 10 ether),
            uint256(490 * 10 ether),
            uint256(467 * 10 ether),
            uint256(175 * 10 ether),
            uint256(140 * 10 ether),
            uint256(23 * 10 ether),
            uint256(23 * 10 ether)
        ];

        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            uint256 balanceBefore = token.balanceOf(randomOperatorStakers[i]);

            // Validator claim stake rewards
            vm.prank(randomOperatorStakers[i]);
            stakingFacet.claimStakeRewards(
                realmId,
                randomOperatorStakers[i],
                1,
                0
            );

            uint256 balanceAfter = token.balanceOf(randomOperatorStakers[i]);
            assertApproxEqRel(
                balanceAfter - balanceBefore,
                rewardOver10EpochsPerValidator[i],
                0.02 ether, // 2% delta
                "reward did not match expected"
            );
        }
    }

    /// @notice This test checks that the fixed cost rewards are correctly calculated.
    function testFuzz_CalcRewards_Vector3(
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);

        uint256 profitMultiplier = 2 ether;

        // Set the config
        LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
            .globalConfig();
        config.profitMultiplier = profitMultiplier;
        stakingAdminFacet.setConfig(config);

        // Setup validators
        address[] memory randomOperatorStakers = _generateAddresses(10);
        _setupValidators(
            realmId,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(10)
        );

        // Advance the epoch to 4 to earn rewards for epoch 3.
        _advanceEpochs(realmId, 3, randomOperatorStakers, 1);

        // Assert that the commission earned is correct
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            uint256 balanceBefore = token.balanceOf(randomOperatorStakers[i]);

            // Claim commission
            vm.prank(randomOperatorStakers[i]);
            stakingFacet.claimFixedCostRewards(realmId, 0);

            uint256 fixedCostRewards = token.balanceOf(
                randomOperatorStakers[i]
            ) - balanceBefore;

            // Convert to USD
            uint256 fixedCostRewardsUSD = FixedPointMathLib.divWad(
                fixedCostRewards,
                config.tokenPrice
            );

            assertApproxEqAbs(
                fixedCostRewardsUSD,
                2.77777777777777777 ether,
                0.00000000000000001 ether,
                "fixed cost rewards did not match expected"
            );
        }
    }

    /// Test to check that the rewards calculations are consistent between the contract and the python script.
    /// forge-config: default.fuzz.runs = 3
    function testFuzz_CalcRewards(
        uint256 p,
        uint256 k,
        uint256 b_min,
        uint256 b_max,
        uint256 stakeAmount,
        uint256 timeLock
    ) public {
        // Set fuzzing boundaries
        p = bound(p, .4 ether, .5 ether);
        k = bound(k, .4 ether, .5 ether);
        b_min = bound(b_min, .001 ether, .01 ether);
        b_max = bound(b_max, .015 ether, .05 ether);
        stakeAmount = bound(stakeAmount, 50_000 ether, 10_000_000 ether);
        timeLock = bound(timeLock, 90 days, 365 * 2 days);

        // Set the config
        LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
            .globalConfig();
        config.p = p;
        config.k = k;
        config.bmin = b_min;
        config.bmax = b_max;
        stakingAdminFacet.setConfig(config);

        // Create 10 validators and keys and join them.
        address[] memory addresses = _generateAddresses(10);
        uint256[] memory commsKeys = _generateUint256s(10);
        _setupValidators(
            1,
            addresses,
            11_000_000 ether,
            stakeAmount,
            timeLock,
            commsKeys
        );

        // Get token total supply
        uint256 tokenTotalSupply = token.totalSupply();

        // Use dummy values to calculate the actual rewards.
        uint256 dummyStakeWeight = 1_000_000 ether;
        LibStakingStorage.RewardEpochGlobalStats memory g = stakingViewsFacet
            .getRewardEpochGlobalStats(2); // advancing the epoch to 2 results in attribution to epoch 2 stake weights
        g.stakeWeight = dummyStakeWeight;
        uint256 actual = stakingViewsFacet.calculateRewardsPerDay(g);

        uint256 expected = _callPythonCalcRewards(
            p,
            k,
            b_min,
            b_max,
            tokenTotalSupply,
            dummyStakeWeight,
            g.stakeAmount
        );

        assertApproxEqRel(
            actual,
            expected,
            10 ** 10,
            "reward calculation did not match expected"
        );
    }

    /// Test to check that the stake weight calculations are correct against known inputs and outputs.
    /// Test Case: Timelock longer than 2 weeks but shorter than 2 years results in proportional stake weight
    function test_CalcStakeWeight_Vector1() public {
        uint256 amount = 100 ether;
        uint256 timeLock = 2 * 365 days;
        uint256 expectedStakeWeight = 50 ether;

        address[] memory randomOperatorStakers = _generateAddresses(4);
        uint256[] memory randomCommsKeys = _generateUint256s(4);
        _setupValidators(
            1,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            randomCommsKeys
        );

        // Assert for each validator
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            uint256 stakeWeight = stakingViewsFacet.getStakeWeightInEpoch(
                randomOperatorStakers[i],
                1,
                randomOperatorStakers[i],
                2 // advancing the epoch to 2 results in attribution to epoch 2 stake weights
            );
            assertEq(
                stakeWeight,
                expectedStakeWeight,
                "stake weight did not match expected"
            );
        }
    }

    /// Test to check that the stake weight calculations are correct against known inputs and outputs.
    /// Test Case: Timelock longer than 2 years results in max stake weight
    function test_CalcStakeWeight_Vector2() public {
        uint256 amount = 100 ether;
        uint256 timeLock = 4 * 365 days + 1;
        uint256 expectedStakeWeight = 100 ether;

        address[] memory randomOperatorStakers = _generateAddresses(4);
        uint256[] memory randomCommsKeys = _generateUint256s(4);
        _setupValidators(
            1,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            randomCommsKeys
        );

        // Assert for each validator
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            uint256 stakeWeight = stakingViewsFacet.getStakeWeightInEpoch(
                randomOperatorStakers[i],
                1,
                randomOperatorStakers[i],
                2 // advancing the epoch to 2 results in attribution to epoch 2 stake weights
            );
            assertEq(
                stakeWeight,
                expectedStakeWeight,
                "stake weight did not match expected"
            );
        }
    }

    /// Test to check that the inputs and outputs are consistent.
    function testFuzz_CalcStakeweight(uint256 amount, uint256 timeLock) public {
        // Input assumptions
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);

        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        address[] memory randomOperatorStakers = _generateAddresses(4);
        uint256[] memory randomCommsKeys = _generateUint256s(4);
        _setupValidators(
            1,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            randomCommsKeys
        );

        // Assert for each validator
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            uint256 stakeWeight = stakingViewsFacet.getStakeWeightInEpoch(
                randomOperatorStakers[i],
                1,
                randomOperatorStakers[i],
                2 // advancing the epoch to 2 results in attribution to epoch 2 stake weights
            );
            assertEq(
                stakeWeight,
                stakingViewsFacet.calculateStakeWeight(roundedTimeLock, amount),
                "stake weight did not match expected"
            );
        }
    }

    /// @notice This test is for testing the admin slash validator function with various percentages.
    /// An invalid percentage is defined as a percentage that is outside of 0.001% and 100%.
    function testFuzz_AdminSlashValidator(
        uint256 percentage,
        uint256 amount,
        uint256 timeLock
    ) public {
        uint256 realmId = 1;
        percentage = bound(percentage, 0, 2 ether);
        amount = bound(amount, 32 ether, 100 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        bool invalidPercentageRevert;
        if (percentage > 1 ether) {
            invalidPercentageRevert = true;
        }

        // Setup validators
        address[] memory operatorStakers = _generateAddresses(4);
        _setupValidators(
            realmId,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );

        // Advance the epoch to 3
        _advanceEpochs(realmId, 2, operatorStakers, 1);

        if (invalidPercentageRevert) {
            vm.expectRevert(StakingUtilsLib.InvalidSlashPercentage.selector);
            stakingAdminFacet.adminSlashValidator(
                percentage,
                operatorStakers[0]
            );
        } else {
            // Slash the validator
            stakingAdminFacet.adminSlashValidator(
                percentage,
                operatorStakers[0]
            );
        }
    }

    // TODO: Remove the "Skip" suffix once we have an optimization for this.
    /// @notice This test is for simulating a delegating staker claiming three months worth of rewards.
    function test_ClaimMonthsOfRewards_Skip() public {
        // Pause the gas metering to allow as much gas as possible for use in the claimStakeRewards call.
        vm.pauseGasMetering();

        uint256 realmId = 1;
        uint256 amount = 100 ether;
        uint256 timeLock = 2 * 365 days;
        uint256 operatorStakerIndexToDelegateTo = 0;
        uint256 desiredTimeline = 1 * 30 days; /// was 3 months

        // Setup validators
        address[] memory operatorStakers = _generateAddresses(4);
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );

        // Set up the delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, delegatingStaker);

        // Advance as many epochs as needed to reach three months worth.
        uint256 epochLength = stakingViewsFacet.epoch(realmId).epochLength;
        uint256 epochsToAdvance = desiredTimeline / epochLength;
        _advanceEpochs(realmId, epochsToAdvance, operatorStakers, 1);

        // Delegating staker claims rewards
        vm.prank(delegatingStaker);
        vm.resumeGasMetering();
        stakingFacet.claimStakeRewards(
            realmId,
            delegatingStaker,
            1,
            type(uint256).max
        );

        // Make sure gas usage is not higher than the block gas limit.
        uint256 gasUsed = vm.snapshotGasLastCall("claimRewardsOver3Months");
        assertLt(gasUsed, 32_000_000, "gas used was too high");
    }

    /// @notice This test is for testing the getAllReserveValidators function.
    function testFuzz_GetAllReserveValidators(
        uint256 numActiveValidators
    ) public {
        vm.pauseGasMetering();

        uint256 numStakingValidators = 100;
        numActiveValidators = bound(
            numActiveValidators,
            0,
            numStakingValidators
        );
        uint256 numReserveValidators = numStakingValidators -
            numActiveValidators;
        uint256 amount = 100 ether;
        uint256 timeLock = 365 days;

        // Setup validators that are joining the validator set
        _setupValidators(
            1,
            _generateAddresses(numActiveValidators),
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(numActiveValidators)
        );

        // Setup validators that are just staking only.
        _setupValidatorsStakeOnly(
            _generateAddressesWithOffset(
                numReserveValidators,
                numActiveValidators
            ),
            amount * 10,
            amount,
            timeLock
        );

        // Get all reserve validators
        vm.resumeGasMetering();
        address[] memory actualReserveValidators = stakingViewsFacet
            .getAllReserveValidators();
        vm.snapshotGasLastCall("getAllReserveValidatorsFor100Stakers");

        assertEq(
            actualReserveValidators.length,
            numReserveValidators,
            "number of reserve validators did not match expected"
        );
    }
}
