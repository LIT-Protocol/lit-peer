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

contract StakingAdminTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    struct VestedTokens {
        uint256 vestedAmount;
        /// @notice Time taken to vest the amount
        uint256 vestingTime;
    }

    /// @notice This test is to check that an admin can stake and start the unfreezing
    /// process on behalf of a delegating staker (to simulate vesting schedules with a lockup
    /// period), and then the delegating staker can migrate their thawing stake to an active
    /// validator later, so that they earn rewards while their stake is thawing.
    /// @dev This is the vesting schedule:
    /// - 50% released after 1 year
    /// - 1/24th released every month after that
    function testFuzz_AdminStakesUnfreezesForUser(
        uint256 operatorStakerIndexToMigrateTo,
        uint256 amount,
        uint256 timeLock
    ) public {
        amount = bound(amount, 32 ether, 1_000_000 ether);
        timeLock = bound(timeLock, 90 days, 2 * 365 days);
        operatorStakerIndexToMigrateTo = bound(
            operatorStakerIndexToMigrateTo,
            0,
            3
        );

        // Setup validators
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _setupValidators(
            1,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );
        // Setup "dummy" operator for temporary delegation.
        address dummyOperator = address(0x123);
        {
            uint256 dummyCommsKeys = 123;
            _fundAddressWithTokensAndApprove(dummyOperator, amount * 10);
            vm.prank(dummyOperator);
            stakingFacet.stake(amount, timeLock, dummyOperator);
            vm.prank(dummyOperator);
            stakingValidatorFacet.setIpPortNodeAddress(1, 1, 1, dummyOperator);
        }

        address delegatingStaker = address(0x999);
        address operatorStakerToMigrateTo = randomOperatorStakers[
            operatorStakerIndexToMigrateTo
        ];

        uint256 totalAmountToVest = 100_000 ether;
        VestedTokens[] memory vestingSchedule = new VestedTokens[](13);
        {
            // Define the vesting schedule
            uint256 totalAmountToVestPerMonth = totalAmountToVest / 24; // Vesting schedule is 2 years total.
            vestingSchedule[0] = VestedTokens({
                vestedAmount: (totalAmountToVest * 50) / 100,
                vestingTime: 365 days
            });
            // Calculate each month's vesting schedule
            for (uint256 i = 1; i < 13; i++) {
                vestingSchedule[i] = VestedTokens({
                    vestedAmount: totalAmountToVestPerMonth,
                    vestingTime: 365 days + (i * 30 days)
                });
            }

            // Now with the entire vesting schedule defined, admins can create a stake record for each vesting schedule
            // item on behalf of the delegating staker.
            for (uint256 i = 0; i < vestingSchedule.length; i++) {
                stakingAdminFacet.adminStakeForUser(
                    delegatingStaker,
                    dummyOperator, // we can use a dummy address here to start.
                    vestingSchedule[i].vestingTime,
                    vestingSchedule[i].vestedAmount
                );
                // Immediately unfreeze the stake to start the thawing process.
                stakingAdminFacet.adminUnfreezeForUser(
                    delegatingStaker,
                    dummyOperator, // we can use a dummy address here to start.
                    i + 1
                );
            }
        }

        {
            // The delegating staker asserts that all their stake records sum to the total amount correctly.
            uint256 cumulativeAmount = 0;
            for (uint256 i = 0; i < vestingSchedule.length; i++) {
                LibStakingStorage.StakeRecord
                    memory stakeRecord = stakingViewsFacet.getStakeRecord(
                        dummyOperator,
                        i + 1,
                        delegatingStaker
                    );
                cumulativeAmount += stakeRecord.amount;
            }
            assertApproxEqAbs(
                cumulativeAmount,
                totalAmountToVest,
                100,
                "Cumulative amount should be approximately equal to the total amount to vest"
            );

            // Assert that there are no extra stake records created.
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingUtilsLib.StakeRecordNotFound.selector,
                    vestingSchedule.length + 1
                )
            );
            stakingViewsFacet.getStakeRecord(
                dummyOperator,
                vestingSchedule.length + 1,
                delegatingStaker
            );
        }

        // The validators all advance to epoch 3 as active validators.
        uint256 epochNumber = _advanceEpochs(1, 2, randomOperatorStakers, 1);

        {
            // The delegating staker now migrates ALL of their stakes to an active validator.
            for (uint256 i = 1; i < vestingSchedule.length + 1; i++) {
                vm.prank(delegatingStaker);
                stakingFacet.migrateStakeRecord(
                    dummyOperator,
                    i,
                    operatorStakerToMigrateTo
                );
            }
        }

        // Advance 10 epochs to assert that rewards can be earned.
        epochNumber = _advanceEpochs(1, 10, randomOperatorStakers, epochNumber);

        {
            // The delegating staker can now claim their rewards for ALL their stake records.
            for (uint256 i = 1; i < vestingSchedule.length + 1; i++) {
                uint256 balanceBefore = token.balanceOf(delegatingStaker);

                vm.prank(delegatingStaker);
                stakingFacet.claimStakeRewards(
                    1,
                    operatorStakerToMigrateTo,
                    i,
                    0
                );

                // Assert that the balance has increased.
                assertGt(token.balanceOf(delegatingStaker), balanceBefore);
            }
        }

        // Assert that none of the stake records can be withdrawn at this moment.
        for (uint256 i = 1; i < vestingSchedule.length + 1; i++) {
            vm.expectRevert(StakingFacet.TimeLockNotMet.selector);
            vm.prank(delegatingStaker);
            stakingFacet.withdraw(operatorStakerToMigrateTo, i);
        }

        // Now, fast forward time successively to assert each milestone of the vesting schedule can
        // be correctly withdrawn against.

        // We first skip 2 days to account for the initial offset of establishing the vesting schedule.
        skip(2 days);

        for (uint256 i = 1; i < vestingSchedule.length + 1; i++) {
            skip(vestingSchedule[i - 1].vestingTime);

            uint256 balanceBefore = token.balanceOf(delegatingStaker);

            vm.prank(delegatingStaker);
            stakingFacet.withdraw(operatorStakerToMigrateTo, i);

            // Assert that the balance has increased by the exact amount of the vested amount.
            assertEq(
                token.balanceOf(delegatingStaker),
                balanceBefore + vestingSchedule[i - 1].vestedAmount,
                "Balance should increase by the exact amount of the vested amount"
            );
        }
    }

    /// @notice This test is to check that, after a validator has been kicked, an admin
    /// rejoins the validator, the validator can be kicked again, either by other nodes
    /// kicking it, or by the admin kicking it again.
    function testFuzz_KickBeforeAndAfterAdminRejoinValidator(
        uint256 operatorStakerIndexToKick,
        bool secondKickIsByAdmin
    ) public {
        uint256 numValidators = 5;
        uint256 amount = 1 ether;
        uint256 timeLock = 365 days;
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;
        operatorStakerIndexToKick = bound(
            operatorStakerIndexToKick,
            0,
            numValidators - 1
        );

        {
            // Set minimum stake
            LibStakingStorage.GlobalConfig memory config = stakingViewsFacet
                .globalConfig();
            config.minStakeAmount = amount;
            config.minSelfStake = amount;
            stakingAdminFacet.setConfig(config);
        }

        // Setup validators
        address[] memory randomOperatorStakers = _generateAddresses(
            numValidators
        );
        _setupValidators(
            1,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(numValidators)
        );
        address operatorStakerAddressToKick = randomOperatorStakers[
            operatorStakerIndexToKick
        ];

        // Advance the epoch to 4.
        uint256 epochNumber = _advanceEpochs(1, 3, randomOperatorStakers, 1);

        // All nodes kick the validator
        address[] memory validatorsNotKicked = new address[](numValidators);
        for (uint256 i = 0; i < numValidators; i++) {
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

        // Advanced the epoch to deal out the validator.
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        epochNumber = _advanceEpochs(1, 1, validatorsNotKicked, epochNumber);

        // Admin now rejoins ALL the desired validators.
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            stakingAdminFacet.adminRejoinValidator(1, randomOperatorStakers[i]);
        }

        if (secondKickIsByAdmin) {
            stakingAdminFacet.adminKickValidatorInNextEpoch(
                operatorStakerAddressToKick
            );
        } else {
            // All nodes kick the validator again.
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
        }

        // Assert global stats
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(
                epochNumber
            );
        uint256 stakeWeightPerValidator = stakingViewsFacet
            .calculateStakeWeight(roundedTimeLock, amount);
        assertEq(globalStats.stakeAmount, amount * (numValidators - 1));
        assertEq(
            globalStats.stakeWeight,
            stakeWeightPerValidator * (numValidators - 1)
        );

        // Advance epoch
        if (!secondKickIsByAdmin) {
            // It's already locked at this point, so we expect a revert.
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingValidatorFacet
                        .MustBeInActiveOrUnlockedState
                        .selector,
                    1
                )
            );
        }
        epochNumber = _advanceEpochs(1, 1, validatorsNotKicked, epochNumber);

        // Assert global stats
        globalStats = stakingViewsFacet.getRewardEpochGlobalStats(epochNumber);
        stakeWeightPerValidator = stakingViewsFacet.calculateStakeWeight(
            roundedTimeLock,
            amount
        );
        assertEq(globalStats.stakeAmount, amount * (numValidators - 1));
        assertEq(
            globalStats.stakeWeight,
            stakeWeightPerValidator * (numValidators - 1)
        );
    }
}
