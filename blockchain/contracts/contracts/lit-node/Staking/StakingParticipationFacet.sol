//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingAcrossRealmsFacet } from "./StakingAcrossRealmsFacet.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { console } from "hardhat/console.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { StakingValidatorFacet } from "./StakingValidatorFacet.sol";

contract StakingParticipationFacet {
    using EnumerableSet for EnumerableSet.AddressSet;
    // errors
    error CannotReuseCommsKeys(uint256 senderPubKey, uint256 receiverPubKey);
    error InvalidAttestedAddress();
    error ValidatorRegisterAttestedWalletDisabled();
    error InvalidUncompressedKeyLength(uint256 length);
    error InvalidUncompressedKeyPrefix(uint8 prefix);

    /* ========== VIEWS ========== */
    function s()
        internal
        pure
        returns (LibStakingStorage.GlobalStakingStorage storage)
    {
        return LibStakingStorage.getStakingStorage();
    }

    function validators() internal view returns (StakingValidatorFacet) {
        return StakingValidatorFacet(getStakingContractAddress());
    }

    function getStakingContractAddress() internal view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.STAKING_CONTRACT(),
                s().env
            );
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    /// @notice This will be called using the node operator wallet (unattested).
    function registerAttestedWallet(
        address stakerAddress,
        address attestedAddress,
        bytes calldata attestedPubKey,
        uint256 senderPubKey,
        uint256 receiverPubKey
    ) external {
        if (attestedPubKey.length != 65) {
            revert InvalidUncompressedKeyLength(attestedPubKey.length);
        }
        uint8 prefix = uint8(attestedPubKey[0]);
        if (prefix != 0x04) {
            revert InvalidUncompressedKeyPrefix(prefix);
        }

        // Check that the staker address is correct.
        address resolvedAddress = StakingUtilsLib
            .views()
            .operatorAddressToStakerAddress(msg.sender);
        if (resolvedAddress != stakerAddress) {
            revert StakingValidatorFacet.StakerAddressMismatch(
                msg.sender,
                resolvedAddress,
                stakerAddress
            );
        } else if (
            StakingUtilsLib.views().nodeAddressToStakerAddress(
                attestedAddress
            ) !=
            address(0) &&
            attestedAddress != msg.sender
        ) {
            revert InvalidAttestedAddress();
        }

        if (senderPubKey == 0) {
            revert StakingValidatorFacet.ValueMustBeNonzero("senderPubKey");
        }
        if (receiverPubKey == 0) {
            revert StakingValidatorFacet.ValueMustBeNonzero("receiverPubKey");
        }

        LibStakingStorage.Validator storage validator = s().validators[
            stakerAddress
        ];

        if (validator.registerAttestedWalletDisabled) {
            revert ValidatorRegisterAttestedWalletDisabled();
        }
        if (validator.lastRealmId != 0) {
            // Skip check if the keys are the same
            if (
                !(senderPubKey == validator.senderPubKey &&
                    receiverPubKey == validator.receiverPubKey)
            ) {
                LibStakingStorage.RealmStorage
                    storage realmStorage = StakingUtilsLib.realm(
                        validator.lastRealmId
                    );
                bytes32 commsKeysHash = keccak256(
                    abi.encodePacked(senderPubKey, receiverPubKey)
                );
                if (realmStorage.usedCommsKeys[commsKeysHash]) {
                    revert CannotReuseCommsKeys(senderPubKey, receiverPubKey);
                }
                realmStorage.usedCommsKeys[commsKeysHash] = true;
            }
        }

        uint256 x;
        uint256 y;
        assembly {
            x := calldataload(add(attestedPubKey.offset, 1))
            y := calldataload(add(attestedPubKey.offset, 33))
        }

        validator.senderPubKey = senderPubKey;
        validator.receiverPubKey = receiverPubKey;
        validator.nodeAddress = attestedAddress;
        s().stakerAddressToNodeAddress[stakerAddress] = attestedAddress;
        s().nodeAddressToStakerAddress[attestedAddress] = stakerAddress;
        s().attestedAddressToPubKey[attestedAddress] = LibStakingStorage
            .UncompressedK256Key(x, y);
        emit AttestedWalletRegistered(
            stakerAddress,
            attestedAddress,
            LibStakingStorage.UncompressedK256Key(x, y)
        );
    }

    /* ========== EVENTS ========== */

    event AttestedWalletRegistered(
        address indexed staker,
        address indexed attestedAddress,
        LibStakingStorage.UncompressedK256Key indexed attestedPubKey
    );
}
