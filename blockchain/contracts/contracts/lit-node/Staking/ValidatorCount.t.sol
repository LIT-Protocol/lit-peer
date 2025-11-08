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
import { SetupAndUtils } from "./SetupAndUtils.t.sol";
import { console } from "lib/forge-std/src/console.sol";

contract ValidatorCountTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    /// @notice This test checks that the validator counts are correctly updated after a validator joins the validator set.
    function testFuzz_AfterJoining(
        uint256 numValidatorsStakeAndJoining,
        uint256 numValidatorsStakeOnly
    ) public {
        numValidatorsStakeAndJoining = bound(
            numValidatorsStakeAndJoining,
            3,
            10
        );
        numValidatorsStakeOnly = bound(
            numValidatorsStakeOnly,
            0,
            numValidatorsStakeAndJoining
        );
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;

        // Assert counts before anything happens.
        assertEq(stakingViewsFacet.getAllValidators().length, 0);
        assertEq(stakingViewsFacet.getAllReserveValidators().length, 0);

        // Setup validators who are staking and joining.
        address[] memory operatorStakers = _generateAddresses(
            numValidatorsStakeAndJoining
        );
        uint256[] memory commsKeys = _generateUint256s(
            numValidatorsStakeAndJoining
        );
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            commsKeys
        );

        // Setup validators who are staking only.
        _setupValidatorsStakeOnly(
            _generateAddressesWithOffset(
                numValidatorsStakeOnly,
                numValidatorsStakeAndJoining
            ),
            amount * 10,
            amount,
            timeLock
        );

        // Assert counts after validators join.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        assertEq(
            stakingViewsFacet.getAllReserveValidators().length,
            numValidatorsStakeOnly
        );
    }

    /// @notice This test checks that the counts for all validators, all active validators, and all reserve validators
    /// are correctly updated after a validator is kicked from the validator set.
    function testFuzz_AfterKicking(
        uint256 numValidatorsStakeAndJoining,
        uint256 numValidatorsStakeOnly,
        uint256 operatorStakerIndexToKick,
        bool withSlashing
    ) public {
        numValidatorsStakeAndJoining = bound(
            numValidatorsStakeAndJoining,
            4,
            10
        );
        numValidatorsStakeOnly = bound(
            numValidatorsStakeOnly,
            0,
            numValidatorsStakeAndJoining
        );
        operatorStakerIndexToKick = bound(
            operatorStakerIndexToKick,
            0,
            numValidatorsStakeAndJoining - 1
        );
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;

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

        // Setup validators who are staking and joining.
        address[] memory operatorStakers = _generateAddresses(
            numValidatorsStakeAndJoining
        );
        uint256[] memory commsKeys = _generateUint256s(
            numValidatorsStakeAndJoining
        );

        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            commsKeys
        );
        address operatorStakerToKick = operatorStakers[
            operatorStakerIndexToKick
        ];

        // Setup validators who are staking only.
        _setupValidatorsStakeOnly(
            _generateAddressesWithOffset(
                numValidatorsStakeOnly,
                numValidatorsStakeAndJoining
            ),
            amount * 10,
            amount,
            timeLock
        );

        // Advance to epoch 3.
        uint256 epochNumber = _advanceEpochs(1, 2, operatorStakers, 1);

        // Kick the validator.
        address[] memory remainingValidators = new address[](
            operatorStakers.length - 1
        );
        uint256 remainingValidatorsIndex = 0;
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            if (operatorStakers[i] == operatorStakerToKick) {
                continue;
            }
            remainingValidators[remainingValidatorsIndex] = operatorStakers[i];
            remainingValidatorsIndex++;
        }
        for (uint256 i = 0; i < remainingValidators.length; i++) {
            vm.prank(remainingValidators[i]);
            stakingValidatorFacet.kickValidatorInNextEpoch(
                operatorStakerToKick,
                1,
                ""
            );
        }

        // Assert counts before advancing epoch.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        assertEq(
            stakingViewsFacet.getAllReserveValidators().length,
            numValidatorsStakeOnly
        );

        // Advance epoch.
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        _advanceEpochs(1, 1, remainingValidators, epochNumber);

        // Assert counts after advancing epoch.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        if (withSlashing) {
            assertEq(
                stakingViewsFacet.getAllReserveValidators().length,
                numValidatorsStakeOnly
            );
        } else {
            assertEq(
                stakingViewsFacet.getAllReserveValidators().length,
                numValidatorsStakeOnly + 1
            );
        }
    }

    /// @notice This test checks that the counts for all validators, all active validators, and all reserve validators
    /// are correctly updated after a validator leaves the validator set.
    function testFuzz_AfterLeaving(
        uint256 numValidatorsStakeAndJoining,
        uint256 numValidatorsStakeOnly,
        uint256 operatorStakerIndexToLeave
    ) public {
        numValidatorsStakeAndJoining = bound(
            numValidatorsStakeAndJoining,
            4,
            10
        );
        numValidatorsStakeOnly = bound(
            numValidatorsStakeOnly,
            0,
            numValidatorsStakeAndJoining
        );
        operatorStakerIndexToLeave = bound(
            operatorStakerIndexToLeave,
            0,
            numValidatorsStakeAndJoining - 1
        );
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;

        // Setup validators who are staking and joining.
        address[] memory operatorStakers = _generateAddresses(
            numValidatorsStakeAndJoining
        );
        uint256[] memory commsKeys = _generateUint256s(
            numValidatorsStakeAndJoining
        );

        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            commsKeys
        );
        address operatorStakerToLeave = operatorStakers[
            operatorStakerIndexToLeave
        ];

        // Setup validators who are staking only.
        _setupValidatorsStakeOnly(
            _generateAddressesWithOffset(
                numValidatorsStakeOnly,
                numValidatorsStakeAndJoining
            ),
            amount * 10,
            amount,
            timeLock
        );

        // Advance to epoch 3.
        uint256 epochNumber = _advanceEpochs(1, 2, operatorStakers, 1);

        // The validator leaves the validator set.
        vm.prank(operatorStakerToLeave);
        stakingValidatorFacet.requestToLeave();

        address[] memory remainingValidators = new address[](
            operatorStakers.length - 1
        );
        uint256 remainingValidatorsIndex = 0;
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            if (operatorStakers[i] == operatorStakerToLeave) {
                continue;
            }
            remainingValidators[remainingValidatorsIndex] = operatorStakers[i];
            remainingValidatorsIndex++;
        }

        // Assert counts before advancing epoch.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        assertEq(
            stakingViewsFacet.getAllReserveValidators().length,
            numValidatorsStakeOnly
        );

        // Advance to epoch 4.
        _advanceEpochs(1, 1, remainingValidators, epochNumber);

        // Assert counts after advancing epoch.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        assertEq(
            stakingViewsFacet.getAllReserveValidators().length,
            numValidatorsStakeOnly + 1
        );
    }

    /// @notice This test checks that the counts for all validators, all active validators, and all reserve validators
    /// are correctly updated after an inactive validator migrates their self-stake to another validator, and in one
    /// case, they migrate their last self-stake and become a delegating staker.
    function testFuzz_AfterMigrating(
        uint256 numValidatorsStakeAndJoining,
        uint256 numValidatorsStakeOnly,
        uint256 operatorStakerIndexToMigrateFrom,
        uint256 operatorStakerIndexToMigrateTo,
        bool migrateLastSelfStake
    ) public {
        numValidatorsStakeAndJoining = bound(
            numValidatorsStakeAndJoining,
            4,
            10
        );
        numValidatorsStakeOnly = bound(
            numValidatorsStakeOnly,
            1,
            numValidatorsStakeAndJoining
        );
        operatorStakerIndexToMigrateFrom = bound(
            operatorStakerIndexToMigrateFrom,
            0,
            numValidatorsStakeOnly - 1
        );
        operatorStakerIndexToMigrateTo = bound(
            operatorStakerIndexToMigrateTo,
            0,
            numValidatorsStakeAndJoining - 1
        );
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;

        // Setup validators who are staking and joining.
        address[] memory operatorStakers = _generateAddresses(
            numValidatorsStakeAndJoining
        );
        uint256[] memory commsKeys = _generateUint256s(
            numValidatorsStakeAndJoining
        );
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            commsKeys
        );
        address operatorStakerToMigrateTo = operatorStakers[
            operatorStakerIndexToMigrateTo
        ];

        // Setup validators who are staking only.
        address operatorStakerToMigrateFrom;
        {
            address[]
                memory operatorStakersStakeOnly = _generateAddressesWithOffset(
                    numValidatorsStakeOnly,
                    numValidatorsStakeAndJoining
                );
            uint256[] memory commsKeysStakeOnly = _generateUint256sWithOffset(
                numValidatorsStakeOnly,
                numValidatorsStakeAndJoining
            );
            _setupValidatorsStakeOnly(
                operatorStakersStakeOnly,
                amount * 10,
                amount,
                timeLock
            );
            operatorStakerToMigrateFrom = operatorStakersStakeOnly[
                operatorStakerIndexToMigrateFrom
            ];

            // Set their node information
            for (uint256 i = 0; i < operatorStakersStakeOnly.length; i++) {
                vm.prank(operatorStakersStakeOnly[i]);
                stakingValidatorFacet.setIpPortNodeAddress(
                    1,
                    1,
                    1,
                    operatorStakersStakeOnly[i]
                );
            }
        }

        // Validator stakes once more.
        vm.prank(operatorStakerToMigrateFrom);
        stakingFacet.stake(amount, timeLock, operatorStakerToMigrateFrom);

        // Advance to epoch 3.
        _advanceEpochs(1, 2, operatorStakers, 1);

        // Assert counts before migrating.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        assertEq(
            stakingViewsFacet.getAllReserveValidators().length,
            numValidatorsStakeOnly
        );

        // Validator 1 migrates their stake to an active validator.
        vm.prank(operatorStakerToMigrateFrom);
        stakingFacet.migrateStakeRecord(
            operatorStakerToMigrateFrom,
            1,
            operatorStakerToMigrateTo
        );

        // Assert counts after migrating.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        assertEq(
            stakingViewsFacet.getAllReserveValidators().length,
            numValidatorsStakeOnly
        );

        if (migrateLastSelfStake) {
            // Validator 1 migrates their last self-stake and becomes a delegating staker.
            vm.prank(operatorStakerToMigrateFrom);
            stakingFacet.migrateStakeRecord(
                operatorStakerToMigrateFrom,
                2,
                operatorStakerToMigrateTo
            );
            // Assert counts after migrating.
            assertEq(
                stakingViewsFacet.getAllValidators().length,
                numValidatorsStakeAndJoining + numValidatorsStakeOnly - 1
            );
            assertEq(
                stakingViewsFacet.getAllReserveValidators().length,
                numValidatorsStakeOnly - 1
            );
        }
    }

    /// @notice This test checks that the counts for all validators, all active validators, and all reserve validators
    /// are correctly updated after a validator withdraws their self-stake, and in one case, they withdraw their last
    /// self-stake and no longer become a validator at all.
    function testFuzz_AfterWithdrawing(
        uint256 numValidatorsStakeAndJoining,
        uint256 numValidatorsStakeOnly,
        uint256 operatorStakerIndexToWithdraw,
        bool withdrawLastSelfStake
    ) public {
        numValidatorsStakeAndJoining = bound(
            numValidatorsStakeAndJoining,
            4,
            10
        );
        numValidatorsStakeOnly = bound(
            numValidatorsStakeOnly,
            1,
            numValidatorsStakeAndJoining
        );
        operatorStakerIndexToWithdraw = bound(
            operatorStakerIndexToWithdraw,
            0,
            numValidatorsStakeOnly - 1
        );
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;

        // Setup validators who are staking and joining.
        address[] memory operatorStakers = _generateAddresses(
            numValidatorsStakeAndJoining
        );
        uint256[] memory commsKeys = _generateUint256s(
            numValidatorsStakeAndJoining
        );
        _setupValidators(
            1,
            operatorStakers,
            amount * 10,
            amount,
            timeLock,
            commsKeys
        );

        // Setup validators who are staking only.
        address operatorStakerToWithdraw;
        {
            address[]
                memory operatorStakersStakeOnly = _generateAddressesWithOffset(
                    numValidatorsStakeOnly,
                    numValidatorsStakeAndJoining
                );
            uint256[] memory commsKeysStakeOnly = _generateUint256sWithOffset(
                numValidatorsStakeOnly,
                numValidatorsStakeAndJoining
            );
            _setupValidatorsStakeOnly(
                operatorStakersStakeOnly,
                amount * 10,
                amount,
                timeLock
            );
            operatorStakerToWithdraw = operatorStakersStakeOnly[
                operatorStakerIndexToWithdraw
            ];

            // Set their node information
            for (uint256 i = 0; i < operatorStakersStakeOnly.length; i++) {
                vm.prank(operatorStakersStakeOnly[i]);
                stakingValidatorFacet.setIpPortNodeAddress(
                    1,
                    1,
                    1,
                    operatorStakersStakeOnly[i]
                );
            }
        }

        // Validator stakes once more.
        vm.prank(operatorStakerToWithdraw);
        stakingFacet.stake(amount, timeLock, operatorStakerToWithdraw);

        // Advance to epoch 3.
        _advanceEpochs(1, 2, operatorStakers, 1);

        // Assert counts before withdrawing.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        assertEq(
            stakingViewsFacet.getAllReserveValidators().length,
            numValidatorsStakeOnly
        );

        // Unfreeze both the validator's stakes.
        vm.prank(operatorStakerToWithdraw);
        stakingFacet.unfreezeStake(operatorStakerToWithdraw, 1);
        vm.prank(operatorStakerToWithdraw);
        stakingFacet.unfreezeStake(operatorStakerToWithdraw, 2);

        // Fast forward to after the timelock has fully decayed.
        skip(800 days);

        // Withdraw the validator's stake.
        vm.prank(operatorStakerToWithdraw);
        stakingFacet.withdraw(operatorStakerToWithdraw, 1);

        // Assert counts after withdrawing.
        assertEq(
            stakingViewsFacet.getAllValidators().length,
            numValidatorsStakeAndJoining + numValidatorsStakeOnly
        );
        assertEq(
            stakingViewsFacet.getAllReserveValidators().length,
            numValidatorsStakeOnly
        );

        if (withdrawLastSelfStake) {
            // Withdraw the last self-stake record.
            vm.prank(operatorStakerToWithdraw);
            stakingFacet.withdraw(operatorStakerToWithdraw, 2);

            // Assert counts after withdrawing the last self-stake record.
            assertEq(
                stakingViewsFacet.getAllValidators().length,
                numValidatorsStakeAndJoining + numValidatorsStakeOnly - 1
            );
            assertEq(
                stakingViewsFacet.getAllReserveValidators().length,
                numValidatorsStakeOnly - 1
            );
        }
    }
}
