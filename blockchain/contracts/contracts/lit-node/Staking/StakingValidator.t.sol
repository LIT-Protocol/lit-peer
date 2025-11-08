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

contract StakingValidatorTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    /// @notice This test checks that a realm cannot be removed if it has active validators.
    function test_CannotRemoveRealmWithActiveValidators() public {
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;

        // Add the second realm
        stakingAdminFacet.addRealm();
        // Also allocate rewards budget for realm 2
        stakingAdminFacet.increaseRewardPool(2, rewardsBudget);

        // Setup validators
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

        // Advance to epoch 3 to load the validators into the current set.
        _advanceEpochs(1, 2, operatorStakersRealm1, 1);

        // Try to remove realm
        vm.expectRevert("Realm has validators");
        stakingAdminFacet.removeRealm(1);
    }

    /// @notice This test checks that the final realm cannot be removed since this would mean
    /// lost keyshares.
    function testFuzz_CannotRemoveFinalRealm(
        uint256 realmIdToRemoveFirst
    ) public {
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;
        realmIdToRemoveFirst = bound(realmIdToRemoveFirst, 1, 2);

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

        // Advance both realms to epoch 3
        _advanceEpochs(1, 2, operatorStakersRealm1, 1);
        _advanceEpochs(2, 2, operatorStakersRealm2, 1);

        // Remove the first realm by first admin setting the current validator set.
        address[] memory validatorsForCurrentEpoch = new address[](0);
        stakingAdminFacet.adminSetValidatorsInCurrentEpoch(
            realmIdToRemoveFirst,
            validatorsForCurrentEpoch
        );
        stakingAdminFacet.removeRealm(realmIdToRemoveFirst);

        // Try to remove the last realm.
        uint256 otherRealmId = realmIdToRemoveFirst == 1 ? 2 : 1;
        stakingAdminFacet.adminSetValidatorsInCurrentEpoch(
            otherRealmId,
            validatorsForCurrentEpoch
        );
        vm.expectRevert("Cannot remove the last realm");
        stakingAdminFacet.removeRealm(otherRealmId);
    }

    /// @notice This test checks that a validator banned from realm 1 can join realm 2 after
    /// their demerits have been reset.
    function testFuzz_BannedValidatorJoinsAnotherRealm(
        uint256 operatorStakerIndexToKick
    ) public {
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;
        operatorStakerIndexToKick = bound(operatorStakerIndexToKick, 0, 3);

        // Add the second realm
        stakingAdminFacet.addRealm();
        // Also allocate rewards budget for realm 2
        stakingAdminFacet.increaseRewardPool(2, rewardsBudget);

        {
            // Set complaint config for reason 1
            LibStakingStorage.ComplaintConfig
                memory complaintConfig = LibStakingStorage.ComplaintConfig({
                    tolerance: 6,
                    intervalSecs: 90,
                    kickPenaltyPercent: 0 ether, // 0% because we want to simplify the staked amount checks.
                    kickPenaltyDemerits: 10
                });
            stakingAdminFacet.setComplaintConfig(1, complaintConfig);
        }

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

        // Advance both epochs to epoch 3
        uint256 epochNumber1 = _advanceEpochs(1, 2, operatorStakersRealm1, 1);
        uint256 epochNumber2 = _advanceEpochs(2, 2, operatorStakersRealm2, 1);
        // Kick the validator
        address operatorStakerAddressToKick = operatorStakersRealm1[
            operatorStakerIndexToKick
        ];
        address[] memory validatorsNotKicked = new address[](4);
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            if (i == operatorStakerIndexToKick) {
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

        // Advance realm 1 to epoch 4 to deal out the validator who was kicked.
        // It's already locked at this point, so we expect a revert.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.MustBeInActiveOrUnlockedState.selector,
                1
            )
        );
        epochNumber1 = _advanceEpochs(1, 1, validatorsNotKicked, epochNumber1);

        // Advance realm 2 to epoch 4 for consistency.
        epochNumber2 = _advanceEpochs(
            2,
            1,
            operatorStakersRealm2,
            epochNumber2
        );

        // Now that the validator is perma-banned, assert that it is unable to join realm 2.
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingValidatorFacet.CannotRejoinBecauseBanned.selector,
                operatorStakerAddressToKick
            )
        );
        vm.prank(operatorStakerAddressToKick);
        stakingValidatorFacet.requestToJoin(2);

        // Admins remove the demerits and joins the validator to the next epoch in realm 2.
        stakingAdminFacet.adminRejoinValidator(2, operatorStakerAddressToKick);

        // Assert that the validator is now in the next epoch of realm 2.
        address[] memory validatorsInNextEpoch = stakingViewsFacet
            .getValidatorsInNextEpoch(2);
        bool validatorIsInNextEpoch = false;
        for (uint256 i = 0; i < validatorsInNextEpoch.length; i++) {
            if (validatorsInNextEpoch[i] == operatorStakerAddressToKick) {
                validatorIsInNextEpoch = true;
                break;
            }
        }
        assertTrue(validatorIsInNextEpoch);
    }
}
