//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { EnumerableSetViewFriendly } from "@lit-protocol/openzeppelin-contracts/utils/structs/EnumerableSetViewFriendly.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { StakingAcrossRealmsFacet } from "./StakingAcrossRealmsFacet.sol";
import { FixedPointMathLib } from "solady/src/utils/FixedPointMathLib.sol";
import { console } from "hardhat/console.sol";
import { StakingNFTFacet } from "./StakingNFTFacet.sol";

library LibStakingNFT {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSetViewFriendly for EnumerableSetViewFriendly.AddressSet;

    function s() internal pure returns (LibStakingStorage.NFTStorage storage) {
        return LibStakingStorage.getNFTStorage();
    }

    function staking_s()
        internal
        pure
        returns (LibStakingStorage.GlobalStakingStorage storage)
    {
        return LibStakingStorage.getStakingStorage();
    }

    function views() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(address(this));
    }

    function nft() internal view returns (StakingNFTFacet) {
        return StakingNFTFacet(address(this));
    }

    /// @notice Handles the transfer of a stake record / NFT by updating the ownership
    /// of the stake record and the corresponding staking related data structures.
    /// @dev We choose to implement logic within a library instead of
    /// introducing any remove / add functions in StakingFacet to avoid
    /// potential security issues.
    function handleNFTStakeRecordTransfer(
        address from,
        address to,
        uint256 tokenId,
        LibStakingStorage.MappedStakeRecord memory mappedStakeRecord
    ) internal {
        // First get a reference to the stake record.
        LibStakingStorage.StakerVault storage fromUserStakerVault = staking_s()
            .vaults[mappedStakeRecord.operatorStakerAddress][from];
        LibStakingStorage.StakeRecord memory fromUserStakerRecord;
        uint256 stakeRecordIdx;
        for (uint256 i = 0; i < fromUserStakerVault.stakes.length; i++) {
            if (
                fromUserStakerVault.stakes[i].id ==
                mappedStakeRecord.stakeRecordId
            ) {
                fromUserStakerRecord = fromUserStakerVault.stakes[i];
                stakeRecordIdx = i;
                break;
            }
        }

        // Add the stake record to the new staker vault. Find the index of the first empty slot.
        LibStakingStorage.StakerVault storage toUserStakerVault = staking_s()
            .vaults[mappedStakeRecord.operatorStakerAddress][to];
        uint256 toStakerVaultFreeSlotIdx;
        for (uint256 i = 0; i < toUserStakerVault.stakes.length; i++) {
            if (!toUserStakerVault.stakes[i].loaded) {
                toStakerVaultFreeSlotIdx = i;
                break;
            }
        }
        // Update the stake record id and lastUpdateTimestamp before setting in to staker vault
        toUserStakerVault.lastStakeRecordId += 1;
        fromUserStakerRecord.id = toUserStakerVault.lastStakeRecordId;
        fromUserStakerRecord.lastUpdateTimestamp = block.timestamp;
        toUserStakerVault.stakes[
            toStakerVaultFreeSlotIdx
        ] = fromUserStakerRecord;

        // Delete the stake record from the from staker vault.
        fromUserStakerVault.stakes[stakeRecordIdx] = LibStakingStorage
            .StakeRecord({
                id: 0,
                amount: 0,
                unfreezeStart: 0,
                timeLock: 0,
                lastUpdateTimestamp: 0,
                lastRewardEpochClaimed: 0,
                loaded: false,
                frozen: false,
                initialSharePrice: 0,
                attributionAddress: address(0),
                tokenId: 0
            });

        // Update staking-related data structures.
        // If this was the last stake this delegator had against this validator, then
        // remove this validator from stakerToValidatorsTheyStakedTo
        uint256 stakeRecordCountForOperatorStakerFrom = views()
            .getStakeRecordCount(from, mappedStakeRecord.operatorStakerAddress);
        if (stakeRecordCountForOperatorStakerFrom == 0) {
            staking_s().stakerToValidatorsTheyStakedTo[from].remove(
                mappedStakeRecord.operatorStakerAddress
            );
        }
        staking_s().stakerToValidatorsTheyStakedTo[to].add(
            mappedStakeRecord.operatorStakerAddress
        );

        // If this was the last unfreezing stake this delegator had against this validator, then
        // remove this validator from validatorToDelegatedStakersWithUnfreezingStakes
        uint256 unfrozenStakeCountForUserFrom = views()
            .getUnfrozenStakeCountForUser(
                from,
                mappedStakeRecord.operatorStakerAddress
            );
        if (unfrozenStakeCountForUserFrom == 0) {
            staking_s()
                .validatorToDelegatedStakersWithUnfreezingStakes[
                    mappedStakeRecord.operatorStakerAddress
                ]
                .remove(from);
        }
        if (fromUserStakerRecord.unfreezeStart > 0) {
            staking_s()
                .validatorToDelegatedStakersWithUnfreezingStakes[
                    mappedStakeRecord.operatorStakerAddress
                ]
                .add(to);
        }

        s().tokenToStakeRecord[tokenId] = LibStakingStorage.MappedStakeRecord({
            operatorStakerAddress: mappedStakeRecord.operatorStakerAddress,
            stakeRecordId: toUserStakerVault.lastStakeRecordId
        });
    }

    /// @notice Adds a new token to a given address and returns the token ID.
    function addNewTokenTo(address to) internal returns (uint256 tokenId) {
        s().tokenCount += 1;
        tokenId = s().tokenCount;
        _addTokenTo(to, tokenId);
    }

    /// @dev Add a NFT to a given address
    ///      Throws if `tokenId` is owned by someone.
    function _addTokenTo(address to, uint256 tokenId) internal {
        // Throws if `_tokenId` is owned by someone
        assert(nft().ownerOf(tokenId) == address(0));
        // Change the owner
        s().tokenToOwner[tokenId] = to;
        // Update owner token index tracking
        uint256 currentCount = s().ownerToTokenCount[to];

        s().ownerToIndexToToken[to][currentCount] = tokenId;
        s().tokenToOwnerIndex[tokenId] = currentCount;
        // Change count tracking
        s().ownerToTokenCount[to] += 1;
    }

    /// @dev Remove a NFT from a given address
    ///      Throws if `from` is not the current owner.
    function _removeTokenFrom(address from, uint256 tokenId) internal {
        // Throws if `from` is not the current owner
        assert(nft().ownerOf(tokenId) == from);
        // Change the owner
        s().tokenToOwner[tokenId] = address(0);

        // Update owner token index tracking using swap-and-pop pattern
        uint256 currentIndex = s().tokenToOwnerIndex[tokenId];
        uint256 lastIndex = s().ownerToTokenCount[from] - 1;

        if (currentIndex != lastIndex) {
            // Move the last token to the position of the token being removed
            uint256 lastTokenId = s().ownerToIndexToToken[from][lastIndex];
            s().ownerToIndexToToken[from][currentIndex] = lastTokenId;
            s().tokenToOwnerIndex[lastTokenId] = currentIndex;
        }

        // Delete the token from mappings
        delete s().ownerToIndexToToken[from][lastIndex];
        delete s().tokenToOwnerIndex[tokenId];

        // Change count tracking
        s().ownerToTokenCount[from] -= 1;
    }

    /// @notice Updates the token to stake record mapping.
    function updateTokenToStakeRecord(
        uint256 tokenId,
        address operatorStakerAddress,
        uint256 stakeRecordId
    ) internal {
        s().tokenToStakeRecord[tokenId] = LibStakingStorage.MappedStakeRecord({
            operatorStakerAddress: operatorStakerAddress,
            stakeRecordId: stakeRecordId
        });
    }
}
