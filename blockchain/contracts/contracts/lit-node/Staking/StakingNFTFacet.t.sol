// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import { Test, console2 } from "lib/forge-std/src/Test.sol";
import { SetupAndUtils } from "./SetupAndUtils.t.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { StakingNFTFacet } from "./StakingNFTFacet.sol";

contract StakingNFTFacetTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    function test_Stake() public {
        uint256 amount = 100 ether;
        uint256 timeLock = 100 days;

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

        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: operatorStakers[0],
                tokenId: 1,
                tokenBalance: 1,
                operatorStakerAddress: operatorStakers[0],
                tokenIndex: 0,
                totalSupply: 4,
                ownershipChanged: false,
                stakeRecordId: 1
            })
        );
    }

    function test_StakeAndTransfer(bool isSafeTransfer) public {
        uint256 amount = 100 ether;
        uint256 timeLock = 100 days;

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

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakers[0]);

        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: delegatingStaker,
                tokenId: 5,
                tokenBalance: 1,
                operatorStakerAddress: operatorStakers[0],
                tokenIndex: 0,
                totalSupply: 5,
                ownershipChanged: false,
                stakeRecordId: 1
            })
        );

        // Transfer the NFT to another address
        address tokenReceiver = address(0x888);
        _transferAndAssertNFTAssertions(
            isSafeTransfer,
            TransferNFTAssertions({
                tokenSender: delegatingStaker,
                tokenReceiver: tokenReceiver,
                tokenId: 5,
                operatorStakerAddress: operatorStakers[0]
            })
        );
    }

    function test_StakeAndTransfer_RevertNotOwner() public {
        uint256 amount = 100 ether;
        uint256 timeLock = 100 days;

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

        // Setup delegating staker
        address delegatingStaker = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker, amount);

        // Delegating staker stakes with a random validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakers[0]);

        // Transfer an NFT that is not owned by the delegating staker
        vm.expectRevert(StakingNFTFacet.NotApprovedOrOwner.selector);
        vm.prank(delegatingStaker);
        stakingNFTFacet.transferFrom(delegatingStaker, address(0x888), 1);
    }

    struct TransferNFTAssertions {
        address tokenSender;
        address tokenReceiver;
        uint256 tokenId;
        address operatorStakerAddress;
    }

    struct NFTAssertions {
        address tokenOwner;
        uint256 tokenId;
        uint256 tokenBalance;
        address operatorStakerAddress;
        uint256 tokenIndex;
        uint256 totalSupply;
        bool ownershipChanged;
        uint256 stakeRecordId;
    }

    function _transferAndAssertNFTAssertions(
        bool isSafeTransfer,
        TransferNFTAssertions memory transferNFTAssertions
    ) private {
        // Get sender and receiver details before the transfer
        uint256 balanceOfSenderBefore = stakingNFTFacet.balanceOf(
            transferNFTAssertions.tokenSender
        );
        uint256 balanceOfReceiverBefore = stakingNFTFacet.balanceOf(
            transferNFTAssertions.tokenReceiver
        );
        uint256 totalSupplyBefore = stakingNFTFacet.totalSupply();
        uint256 ownershipChangeBefore = stakingNFTFacet.ownershipChange(
            transferNFTAssertions.tokenId
        );
        uint256 stakeRecordIdBefore = stakingNFTFacet
            .tokenToStakeRecord(transferNFTAssertions.tokenId)
            .stakeRecordId;
        LibStakingStorage.StakeRecord
            memory stakeRecordSenderBefore = stakingViewsFacet.getStakeRecord(
                transferNFTAssertions.operatorStakerAddress,
                stakeRecordIdBefore,
                transferNFTAssertions.tokenSender
            );
        LibStakingStorage.StakeRecord
            memory mostRecentStakeRecordReceiverBefore = stakingViewsFacet
                .getMostRecentStakeRecord(
                    transferNFTAssertions.tokenSender,
                    transferNFTAssertions.tokenReceiver
                );

        // Transfer the NFT
        if (isSafeTransfer) {
            vm.prank(transferNFTAssertions.tokenSender);
            stakingNFTFacet.transferFrom(
                transferNFTAssertions.tokenSender,
                transferNFTAssertions.tokenReceiver,
                transferNFTAssertions.tokenId
            );
        } else {
            vm.prank(transferNFTAssertions.tokenSender);
            stakingNFTFacet.safeTransferFrom(
                transferNFTAssertions.tokenSender,
                transferNFTAssertions.tokenReceiver,
                transferNFTAssertions.tokenId
            );
        }

        // Assertions
        assertEq(
            stakingNFTFacet.balanceOf(transferNFTAssertions.tokenSender),
            balanceOfSenderBefore - 1
        );
        assertEq(
            stakingNFTFacet.balanceOf(transferNFTAssertions.tokenReceiver),
            balanceOfReceiverBefore + 1
        );
        assertEq(
            stakingNFTFacet.ownerOf(transferNFTAssertions.tokenId),
            transferNFTAssertions.tokenReceiver
        );
        assertEq(
            stakingNFTFacet.tokenOfOwnerByIndex(
                transferNFTAssertions.tokenReceiver,
                balanceOfReceiverBefore
            ),
            transferNFTAssertions.tokenId
        );
        assertEq(stakingNFTFacet.totalSupply(), totalSupplyBefore);
        assertGt(
            stakingNFTFacet.ownershipChange(transferNFTAssertions.tokenId),
            ownershipChangeBefore
        );
        assertEq(
            stakingNFTFacet
                .tokenToStakeRecord(transferNFTAssertions.tokenId)
                .operatorStakerAddress,
            transferNFTAssertions.operatorStakerAddress
        );
        assertEq(
            stakingNFTFacet
                .tokenToStakeRecord(transferNFTAssertions.tokenId)
                .stakeRecordId,
            mostRecentStakeRecordReceiverBefore.id + 1
        );

        // Expect revert when getting stake record for sender
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingUtilsLib.StakeRecordNotFound.selector,
                mostRecentStakeRecordReceiverBefore.id + 1
            )
        );
        stakingViewsFacet.getStakeRecord(
            transferNFTAssertions.operatorStakerAddress,
            stakeRecordIdBefore,
            transferNFTAssertions.tokenSender
        );

        // Expect success when getting stake record for receiver
        LibStakingStorage.StakeRecord
            memory stakeRecordReceiver = stakingViewsFacet.getStakeRecord(
                transferNFTAssertions.operatorStakerAddress,
                mostRecentStakeRecordReceiverBefore.id + 1,
                transferNFTAssertions.tokenReceiver
            );
        assertEq(stakeRecordReceiver.tokenId, transferNFTAssertions.tokenId);

        assertGe(
            stakeRecordReceiver.lastUpdateTimestamp,
            stakeRecordSenderBefore.lastUpdateTimestamp
        );

        // Assert stakerToValidatorsTheyStakedTo
        {
            address[]
                memory stakerToValidatorsTheyStakedToSender = stakingViewsFacet
                    .stakerToValidatorsTheyStakedTo(
                        transferNFTAssertions.tokenSender
                    );
            address[]
                memory stakerToValidatorsTheyStakedToReceiver = stakingViewsFacet
                    .stakerToValidatorsTheyStakedTo(
                        transferNFTAssertions.tokenReceiver
                    );
            for (
                uint256 i = 0;
                i < stakerToValidatorsTheyStakedToSender.length;
                i++
            ) {
                if (
                    stakerToValidatorsTheyStakedToSender[i] ==
                    transferNFTAssertions.operatorStakerAddress
                ) {
                    fail(
                        "Staker to validators they staked to sender contains operator staker address"
                    );
                }
            }
            bool receiverContainsOperatorStakerAddress = false;
            for (
                uint256 i = 0;
                i < stakerToValidatorsTheyStakedToReceiver.length;
                i++
            ) {
                if (
                    stakerToValidatorsTheyStakedToReceiver[i] ==
                    transferNFTAssertions.operatorStakerAddress
                ) {
                    receiverContainsOperatorStakerAddress = true;
                }
            }
            assertTrue(receiverContainsOperatorStakerAddress);
        }
    }

    function _assertNFTAssertions(NFTAssertions memory nftAssertions) private {
        assertEq(
            stakingNFTFacet.balanceOf(nftAssertions.tokenOwner),
            nftAssertions.tokenBalance
        );
        assertEq(
            stakingNFTFacet.ownerOf(nftAssertions.tokenId),
            nftAssertions.tokenOwner
        );
        assertEq(
            stakingNFTFacet.tokenOfOwnerByIndex(
                nftAssertions.tokenOwner,
                nftAssertions.tokenIndex
            ),
            nftAssertions.tokenId
        );
        assertEq(stakingNFTFacet.totalSupply(), nftAssertions.totalSupply);
        if (nftAssertions.ownershipChanged) {
            assertGt(stakingNFTFacet.ownershipChange(nftAssertions.tokenId), 0);
        } else {
            assertEq(stakingNFTFacet.ownershipChange(nftAssertions.tokenId), 0);
        }
        assertEq(
            stakingNFTFacet
                .tokenToStakeRecord(nftAssertions.tokenId)
                .operatorStakerAddress,
            nftAssertions.operatorStakerAddress
        );
        assertEq(
            stakingNFTFacet
                .tokenToStakeRecord(nftAssertions.tokenId)
                .stakeRecordId,
            nftAssertions.stakeRecordId
        );

        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(
                nftAssertions.operatorStakerAddress,
                nftAssertions.stakeRecordId,
                nftAssertions.tokenOwner
            );
        assertEq(stakeRecord.tokenId, nftAssertions.tokenId);
    }
}
