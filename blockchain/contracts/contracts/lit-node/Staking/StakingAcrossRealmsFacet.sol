//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import "hardhat/console.sol";

contract StakingAcrossRealmsFacet {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.UintSet;

    error RealmIdNotFound(address stakerAddress);

    /* ========== VIEWS ========== */

    function s()
        internal
        pure
        returns (LibStakingStorage.GlobalStakingStorage storage)
    {
        return LibStakingStorage.getStakingStorage();
    }

    function numRealms() public view returns (uint256) {
        return s().realmIds.length();
    }

    function getStakingAddress() internal view returns (address) {
        return
            s().contractResolver.getContract( // will 1 always be an existing realm?
                s().contractResolver.STAKING_CONTRACT(),
                s().env
            );
    }

    function stakingViewsFacet() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(getStakingAddress());
    }

    function isValidatorInCurrentEpoch(
        address stakerAddress
    ) public view returns (bool) {
        for (uint256 i = 0; i < s().realmIds.length(); i++) {
            uint256 realmId = s().realmIds.at(i);
            if (stakingViewsFacet().isActiveValidator(realmId, stakerAddress)) {
                return true;
            }
        }
        return false;
    }

    function isValidatorInNextEpoch(
        address stakerAddress
    ) public view returns (bool) {
        for (uint256 i = 0; i < s().realmIds.length(); i++) {
            uint256 realmId = s().realmIds.at(i);
            if (
                stakingViewsFacet().isActiveValidatorForNextEpoch(
                    realmId,
                    stakerAddress
                )
            ) {
                return true;
            }
        }
        return false;
    }

    function isValidatorInCurrentOrNextEpoch(
        address stakerAddress
    ) public view returns (bool) {
        return
            isValidatorInCurrentEpoch(stakerAddress) ||
            isValidatorInNextEpoch(stakerAddress);
    }

    /// @notice This function returns a non-zero value if the staker is in a currently active validator set.
    function getCurrentRealmIdForStakerAddress(
        address stakerAddress
    ) public view returns (uint256) {
        return s().validatorToCurrentRealmId[stakerAddress];
    }

    /// @notice This function returns a non-zero value if the staker is in either a
    /// currently active or upcoming validator set.
    /// @dev This function CAN return 0 if the staker is not in any realm. In that case,
    /// you can check for 0 and use `Validator.lastRealmId` to get the last realmId
    /// the staker was in.
    function getRealmIdForStakerAddress(
        address stakerAddress
    ) public view returns (uint256) {
        uint256 realmId = s().validatorToCurrentRealmId[stakerAddress];
        if (realmId == 0) {
            return s().validatorToNextRealmId[stakerAddress];
        }
        return realmId;
    }

    function getShadowRealmIdForStakerAddress(
        address stakerAddress
    ) public view returns (uint256) {
        uint256 baseRealmId = getRealmIdForStakerAddress(stakerAddress);
        if (baseRealmId == 0) {
            return 0;
        }

        uint256 shadowRealmId = s().realmToShadowRealm[baseRealmId];
        if (shadowRealmId == 0) {
            return 0;
        }

        if (
            stakingViewsFacet().isActiveShadowValidator(
                shadowRealmId,
                stakerAddress
            )
        ) {
            return shadowRealmId;
        }

        return 0;
    }

    function nodeAddressToStakerAddressAcrossRealms(
        address nodeAddress
    ) public view returns (address) {
        return s().nodeAddressToStakerAddress[nodeAddress];
    }

    function isRecentValidator(address nodeAddress) public view returns (bool) {
        for (uint256 i = 0; i < s().realmIds.length(); i++) {
            uint256 realmId = s().realmIds.at(i);
            if (stakingViewsFacet().isRecentValidator(realmId, nodeAddress)) {
                return true;
            }
        }
        return false;
    }

    function getAllUnkickedValidators() public view returns (address[] memory) {
        uint256 total_count = 0;
        for (uint256 i = 0; i < s().realmIds.length(); i++) {
            uint256 realmId = s().realmIds.at(i);
            total_count += stakingViewsFacet().getActiveUnkickedValidatorCount(
                realmId
            );
        }

        address[] memory validators = new address[](total_count);
        uint256 index = 0;

        for (uint256 i = 0; i < s().realmIds.length(); i++) {
            uint256 realmId = s().realmIds.at(i);
            address[] memory realm_validators = stakingViewsFacet()
                .getActiveUnkickedValidators(realmId);
            for (uint256 j = 0; j < realm_validators.length; j++) {
                validators[index] = realm_validators[j];
                index++;
            }
        }
        return validators;
    }

    function validator_by_staker_address(
        address stakerAddress
    ) public view returns (LibStakingStorage.Validator memory) {
        return s().validators[stakerAddress];
    }
}
