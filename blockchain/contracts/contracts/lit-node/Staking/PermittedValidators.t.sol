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

contract PermittedValidatorsTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    /// @notice This test checks that, when permittedValidatorsOn is set, that the chosen validator
    /// is not permitted to join the validator set.
    function testFuzz_PermittedValidators(
        uint256 operatorStakerIndexToNotPermit
    ) public {
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;
        operatorStakerIndexToNotPermit = bound(
            operatorStakerIndexToNotPermit,
            0,
            3
        );

        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);

        // Set permittedValidatorsOn to true for this realm.
        stakingAdminFacet.setPermittedValidatorsOn(1, true);

        // Permit all validators except for the one that is not permitted
        address[] memory validatorsToPermit = new address[](
            operatorStakersRealm1.length - 1
        );
        uint256 validatorsToPermitIndex = 0;
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            if (i != operatorStakerIndexToNotPermit) {
                validatorsToPermit[
                    validatorsToPermitIndex
                ] = operatorStakersRealm1[i];
                validatorsToPermitIndex++;
            }
        }
        stakingAdminFacet.setPermittedValidators(1, validatorsToPermit);

        // Assert permitted validators.
        address[] memory actualPermittedValidators = stakingViewsFacet
            .permittedValidators(1);
        assertEq(actualPermittedValidators.length, 3);

        // Assert permitted realms for validator.
        uint256[] memory actualPermittedRealms = stakingViewsFacet
            .permittedRealmsForValidator(
                operatorStakersRealm1[operatorStakerIndexToNotPermit]
            );
        assertEq(actualPermittedRealms.length, 0);

        // Validators stake
        _setupValidatorsStakeOnly(
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock
        );

        // Validators set node information
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            vm.prank(operatorStakersRealm1[i]);
            stakingValidatorFacet.setIpPortNodeAddress(
                1,
                1,
                1,
                operatorStakersRealm1[i]
            );
        }

        // Validators join except for the one that is not permitted
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            if (i == operatorStakerIndexToNotPermit) {
                vm.expectRevert(
                    abi.encodeWithSelector(
                        StakingValidatorFacet.ValidatorNotPermitted.selector,
                        operatorStakersRealm1[i],
                        1
                    )
                );
            }
            vm.prank(operatorStakersRealm1[i]);
            stakingValidatorFacet.requestToJoin(1);
        }
    }

    /// @notice This test checks that, when permittedValidatorsOn is set for each realm,
    /// that the permitted validators are different for each realm.
    function testFuzz_PermittedValidators_2Realms(
        uint256 operatorStakerIndexToNotPermitRealm1,
        uint256 operatorStakerIndexToNotPermitRealm2
    ) public {
        uint256 amount = 100 ether;
        uint256 timeLock = 90 days;
        operatorStakerIndexToNotPermitRealm1 = bound(
            operatorStakerIndexToNotPermitRealm1,
            0,
            3
        );
        operatorStakerIndexToNotPermitRealm2 = bound(
            operatorStakerIndexToNotPermitRealm2,
            0,
            3
        );

        address[] memory operatorStakersRealm1 = _generateAddresses(4);
        uint256[] memory commsKeysRealm1 = _generateUint256s(4);
        address[] memory operatorStakersRealm2 = _generateAddressesWithOffset(
            4,
            4
        );
        uint256[] memory commsKeysRealm2 = _generateUint256sWithOffset(4, 4);

        // Add realm 2.
        stakingAdminFacet.addRealm();

        // Set permittedValidatorsOn to true for both realms.
        stakingAdminFacet.setPermittedValidatorsOn(1, true);
        stakingAdminFacet.setPermittedValidatorsOn(2, true);

        // Permit all validators except for the one that is not permitted
        address[] memory validatorsToPermitRealm1 = new address[](
            operatorStakersRealm1.length
        );
        address[] memory validatorsToPermitRealm2 = new address[](
            operatorStakersRealm2.length
        );
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            if (i != operatorStakerIndexToNotPermitRealm1) {
                validatorsToPermitRealm1[i] = operatorStakersRealm1[i];
            }
        }
        for (uint256 i = 0; i < operatorStakersRealm2.length; i++) {
            if (i != operatorStakerIndexToNotPermitRealm2) {
                validatorsToPermitRealm2[i] = operatorStakersRealm2[i];
            }
        }
        stakingAdminFacet.setPermittedValidators(1, validatorsToPermitRealm1);
        stakingAdminFacet.setPermittedValidators(2, validatorsToPermitRealm2);

        // Assert permitted realms for each realm 1 validator.
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            uint256[] memory actualPermittedRealms = stakingViewsFacet
                .permittedRealmsForValidator(operatorStakersRealm1[i]);
            if (i == operatorStakerIndexToNotPermitRealm1) {
                assertEq(actualPermittedRealms.length, 0);
            } else {
                assertEq(actualPermittedRealms.length, 1);
                assertEq(actualPermittedRealms[0], 1);
            }
        }

        // Assert permitted realms for each realm 2 validator.
        for (uint256 i = 0; i < operatorStakersRealm2.length; i++) {
            uint256[] memory actualPermittedRealms = stakingViewsFacet
                .permittedRealmsForValidator(operatorStakersRealm2[i]);
            if (i == operatorStakerIndexToNotPermitRealm2) {
                assertEq(actualPermittedRealms.length, 0);
            } else {
                assertEq(actualPermittedRealms.length, 1);
                assertEq(actualPermittedRealms[0], 2);
            }
        }

        // Validators stake
        _setupValidatorsStakeOnly(
            operatorStakersRealm1,
            amount * 10,
            amount,
            timeLock
        );
        _setupValidatorsStakeOnly(
            operatorStakersRealm2,
            amount * 10,
            amount,
            timeLock
        );

        // Validators set node information
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            vm.prank(operatorStakersRealm1[i]);
            stakingValidatorFacet.setIpPortNodeAddress(
                1,
                1,
                1,
                operatorStakersRealm1[i]
            );
        }
        for (uint256 i = 0; i < operatorStakersRealm2.length; i++) {
            vm.prank(operatorStakersRealm2[i]);
            stakingValidatorFacet.setIpPortNodeAddress(
                2,
                1,
                1,
                operatorStakersRealm2[i]
            );
        }

        // Validators join except for the one that is not permitted
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            if (i == operatorStakerIndexToNotPermitRealm1) {
                vm.expectRevert(
                    abi.encodeWithSelector(
                        StakingValidatorFacet.ValidatorNotPermitted.selector,
                        operatorStakersRealm1[i],
                        1
                    )
                );
            }
            vm.prank(operatorStakersRealm1[i]);
            stakingValidatorFacet.requestToJoin(1);
        }
        for (uint256 i = 0; i < operatorStakersRealm2.length; i++) {
            if (i == operatorStakerIndexToNotPermitRealm2) {
                vm.expectRevert(
                    abi.encodeWithSelector(
                        StakingValidatorFacet.ValidatorNotPermitted.selector,
                        operatorStakersRealm2[i],
                        2
                    )
                );
            }
            vm.prank(operatorStakersRealm2[i]);
            stakingValidatorFacet.requestToJoin(2);
        }

        // Realm 1 validators are not permitted to join realm 2.
        for (uint256 i = 0; i < operatorStakersRealm1.length; i++) {
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingValidatorFacet.ValidatorNotPermitted.selector,
                    operatorStakersRealm1[i],
                    2
                )
            );
            vm.prank(operatorStakersRealm1[i]);
            stakingValidatorFacet.requestToJoin(2);
        }
        // Realm 2 validators are not permitted to join realm 1.
        for (uint256 i = 0; i < operatorStakersRealm2.length; i++) {
            vm.expectRevert(
                abi.encodeWithSelector(
                    StakingValidatorFacet.ValidatorNotPermitted.selector,
                    operatorStakersRealm2[i],
                    1
                )
            );
            vm.prank(operatorStakersRealm2[i]);
            stakingValidatorFacet.requestToJoin(1);
        }
    }
}
