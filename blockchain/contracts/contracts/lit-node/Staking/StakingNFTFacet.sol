//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ERC2771 } from "../../common/ERC2771.sol";
import { IERC721 } from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import { IERC721Receiver } from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { LibStakingNFT } from "./LibStakingNFT.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { EnumerableSetViewFriendly } from "@lit-protocol/openzeppelin-contracts/utils/structs/EnumerableSetViewFriendly.sol";

contract StakingNFTFacet is ERC2771, IERC721 {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSetViewFriendly for EnumerableSetViewFriendly.AddressSet;

    error ERC721ReceiverRejectedTokens();
    error ERC721TransferToNonERC721ReceiverImplementer();
    error NotApprovedOrOwner();
    error NotOwner();
    error ZeroAddress();
    error SameAddress();

    function s() internal pure returns (LibStakingStorage.NFTStorage storage) {
        return LibStakingStorage.getNFTStorage();
    }

    function views() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(address(this));
    }

    /* ========== ERC165: STANDARD FUNCTIONS ========== */
    function supportsInterface(
        bytes4 interfaceID
    ) external view returns (bool) {
        return s().supportedInterfaces[interfaceID];
    }

    /* ========== ERC721: STANDARD FUNCTIONS ========== */
    function balanceOf(address owner) public view override returns (uint256) {
        return s().ownerToTokenCount[owner];
    }

    function ownerOf(uint256 tokenId) public view override returns (address) {
        return s().tokenToOwner[tokenId];
    }

    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId,
        bytes memory data
    ) public override {
        address sender = LibERC2771._msgSender();
        _transferFrom(from, to, tokenId, sender);

        if (_isContract(to)) {
            // Throws if transfer destination is a contract which does not implement 'onERC721Received'
            try
                IERC721Receiver(to).onERC721Received(
                    sender,
                    from,
                    tokenId,
                    data
                )
            returns (bytes4 response) {
                if (response != IERC721Receiver(to).onERC721Received.selector) {
                    revert ERC721ReceiverRejectedTokens();
                }
            } catch (bytes memory reason) {
                if (reason.length == 0) {
                    revert ERC721TransferToNonERC721ReceiverImplementer();
                } else {
                    assembly {
                        revert(add(32, reason), mload(reason))
                    }
                }
            }
        }
    }

    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId
    ) public override {
        safeTransferFrom(from, to, tokenId, "");
    }

    function transferFrom(
        address from,
        address to,
        uint256 tokenId
    ) public override {
        _transferFrom(from, to, tokenId, LibERC2771._msgSender());
    }

    function approve(address approved, uint256 tokenId) external {
        address sender = LibERC2771._msgSender();
        address owner = ownerOf(tokenId);
        // Throws if `tokenId` is not a valid NFT
        if (owner == address(0)) revert ZeroAddress();
        // Throws if `approved` is the current owner
        if (owner == approved) revert SameAddress();
        // Check requirements
        bool senderIsOwner = (ownerOf(tokenId) == sender);
        bool senderIsApprovedForAll = (s().ownerToOperators[owner])[sender];
        if (!senderIsOwner && !senderIsApprovedForAll)
            revert NotApprovedOrOwner();
        // Set the approval
        s().tokenToApprovals[tokenId] = approved;
        emit Approval(owner, approved, tokenId);
    }

    function setApprovalForAll(address operator, bool approved) external {
        address sender = LibERC2771._msgSender();
        // Throws if `_operator` is the `msg.sender`
        if (operator == sender) revert SameAddress();
        s().ownerToOperators[sender][operator] = approved;
        emit ApprovalForAll(sender, operator, approved);
    }

    /* ========== ERC721: UTILITY FUNCTIONS ========== */

    function tokenOfOwnerByIndex(
        address owner,
        uint256 index
    ) public view returns (uint256) {
        return s().ownerToIndexToToken[owner][index];
    }

    function totalSupply() public view returns (uint256) {
        return s().tokenCount;
    }

    function getApproved(uint256 tokenId) external view returns (address) {
        return s().tokenToApprovals[tokenId];
    }

    function isApprovedForAll(
        address owner,
        address operator
    ) external view returns (bool) {
        return s().ownerToOperators[owner][operator];
    }

    function ownershipChange(uint256 tokenId) external view returns (uint256) {
        return s().ownershipChange[tokenId];
    }

    function tokenToStakeRecord(
        uint256 tokenId
    ) external view returns (LibStakingStorage.MappedStakeRecord memory) {
        return s().tokenToStakeRecord[tokenId];
    }

    /* ========== INTERNAL FUNCTIONS ========== */

    function _transferFrom(
        address from,
        address to,
        uint256 tokenId,
        address sender
    ) internal {
        // Check requirements
        if (!_isApprovedOrOwner(sender, tokenId)) revert NotApprovedOrOwner();

        // Throw if from is not the current owner
        if (ownerOf(tokenId) != from) revert NotOwner();

        // Check that to has empty stake record slots.
        // First, get the underlying stake record details for this NFT.
        LibStakingStorage.MappedStakeRecord memory mappedStakeRecord = s()
            .tokenToStakeRecord[tokenId];
        // Then, use this information to check for available stake record slots for the to address.
        assert(
            views().getEmptyStakeRecordSlots(
                to,
                mappedStakeRecord.operatorStakerAddress
            ) > 0
        );

        // Clear approvals
        delete s().tokenToApprovals[tokenId];

        // Remove NFT. Throws if `tokenId` is not a valid NFT
        LibStakingNFT._removeTokenFrom(from, tokenId);

        // Add NFT
        LibStakingNFT._addTokenTo(to, tokenId);
        // Set the block of ownership transfer (for Flash NFT protection)
        s().ownershipChange[tokenId] = block.number;
        // Log the transfer
        emit Transfer(from, to, tokenId);

        // Update staking related data.
        LibStakingNFT.handleNFTStakeRecordTransfer(
            from,
            to,
            tokenId,
            mappedStakeRecord
        );
    }

    function _isContract(address account) internal view returns (bool) {
        // This method relies on extcodesize, which returns 0 for contracts in
        // construction, since the code is only stored at the end of the
        // constructor execution.
        uint256 size;
        assembly {
            size := extcodesize(account)
        }
        return size > 0;
    }

    function _isApprovedOrOwner(
        address spender,
        uint256 tokenId
    ) internal view returns (bool) {
        address owner = ownerOf(tokenId);
        bool spenderIsOwner = owner == spender;
        bool spenderIsApproved = spender == s().tokenToApprovals[tokenId];
        bool spenderIsApprovedForAll = (s().ownerToOperators[owner])[spender];
        return spenderIsOwner || spenderIsApproved || spenderIsApprovedForAll;
    }
}
