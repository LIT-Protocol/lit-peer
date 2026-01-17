// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import { Test, console2 } from "lib/forge-std/src/Test.sol";
import { SetupAndUtils } from "./SetupAndUtils.t.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { StakingNFTFacet } from "./StakingNFTFacet.sol";
import { console } from "lib/forge-std/src/console.sol";

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
                tokenOwner: delegatingStaker,
                tokenSender: delegatingStaker,
                tokenReceiver: tokenReceiver,
                tokenId: 5,
                operatorStakerAddress: operatorStakers[0],
                expectedStakerToValidatorsTheyStakedToContainsOperator: false
            })
        );
    }

    function test_StakeAndTransfer_RevertNotOwner(bool isSafeTransfer) public {
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
        if (isSafeTransfer) {
            stakingNFTFacet.safeTransferFrom(
                delegatingStaker,
                address(0x888),
                1
            );
        } else {
            stakingNFTFacet.transferFrom(delegatingStaker, address(0x888), 1);
        }
    }

    /// @notice This tests that when A transfers their NFT to B, B can mutate the stake record associated with the NFT.
    function test_StakeAndTransfer_ReceiverMutatesStakeRecord(
        bool isSafeTransfer,
        uint256 operation
    ) public {
        uint256 amount = 100 ether;
        uint256 timeLock = 100 days;
        operation = bound(operation, 0, 3); // 4 operations - increase stake amount, increase stake timelock, split and migrate stake

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
        _fundAddressWithTokensAndApprove(tokenReceiver, amount);

        _transferAndAssertNFTAssertions(
            isSafeTransfer,
            TransferNFTAssertions({
                tokenOwner: delegatingStaker,
                tokenSender: delegatingStaker,
                tokenReceiver: tokenReceiver,
                tokenId: 5,
                operatorStakerAddress: operatorStakers[0],
                expectedStakerToValidatorsTheyStakedToContainsOperator: false
            })
        );

        if (operation == 0) {
            _increaseStakeRecordAmountAndAssert(
                IncreaseStakeRecordAmountAndAssertParams({
                    userStakerAddress: tokenReceiver,
                    operatorStakerAddress: operatorStakers[0],
                    stakeRecordId: 1,
                    tokenId: 5,
                    isTransferredToken: true
                })
            );
        } else if (operation == 1) {
            _increaseStakeRecordTimelockAndAssert(
                IncreaseStakeRecordTimelockAndAssertParams({
                    userStakerAddress: tokenReceiver,
                    operatorStakerAddress: operatorStakers[0],
                    stakeRecordId: 1,
                    tokenId: 5,
                    isTransferredToken: true
                })
            );
        } else if (operation == 2) {
            _splitStakeRecordAndAssert(
                SplitStakeRecordAndAssertParams({
                    userStakerAddress: tokenReceiver,
                    operatorStakerAddress: operatorStakers[0],
                    stakeRecordId: 1,
                    tokenId: 5,
                    isTransferredToken: true
                })
            );
        } else if (operation == 3) {
            _migrateStakeRecordAndAssert(
                MigrateStakeRecordAndAssertParams({
                    userStakerAddress: tokenReceiver,
                    operatorStakerAddress: operatorStakers[0],
                    operatorStakerAddressToMigrateTo: operatorStakers[1],
                    stakeRecordId: 1,
                    tokenId: 5,
                    isTransferredToken: true
                })
            );
        }
    }

    /// @notice This tests that when a staker mutates their stake record, the NFT-related details are updated correctly.
    function test_Stake_MutateStakeRecord(uint256 operation) public {
        uint256 amount = 100 ether;
        uint256 timeLock = 100 days;
        operation = bound(operation, 0, 3); // 4 operations - increase stake amount, increase stake timelock, split and migrate stake

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
        _fundAddressWithTokensAndApprove(delegatingStaker, amount * 2);

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

        if (operation == 0) {
            _increaseStakeRecordAmountAndAssert(
                IncreaseStakeRecordAmountAndAssertParams({
                    userStakerAddress: delegatingStaker,
                    operatorStakerAddress: operatorStakers[0],
                    stakeRecordId: 1,
                    tokenId: 5,
                    isTransferredToken: false
                })
            );
        } else if (operation == 1) {
            _increaseStakeRecordTimelockAndAssert(
                IncreaseStakeRecordTimelockAndAssertParams({
                    userStakerAddress: delegatingStaker,
                    operatorStakerAddress: operatorStakers[0],
                    stakeRecordId: 1,
                    tokenId: 5,
                    isTransferredToken: false
                })
            );
        } else if (operation == 2) {
            _splitStakeRecordAndAssert(
                SplitStakeRecordAndAssertParams({
                    userStakerAddress: delegatingStaker,
                    operatorStakerAddress: operatorStakers[0],
                    stakeRecordId: 1,
                    tokenId: 5,
                    isTransferredToken: false
                })
            );
        } else if (operation == 3) {
            _migrateStakeRecordAndAssert(
                MigrateStakeRecordAndAssertParams({
                    userStakerAddress: delegatingStaker,
                    operatorStakerAddress: operatorStakers[0],
                    operatorStakerAddressToMigrateTo: operatorStakers[1],
                    stakeRecordId: 1,
                    tokenId: 5,
                    isTransferredToken: false
                })
            );
        }
    }

    /// @notice This tests that when A transfers their NFT to B, B can claim past & future staking rewards.
    function test_StakeAndTransfer_ClaimRewards(bool isSafeTransfer) public {
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

        // Advance 5 epochs to produce staking rewards
        uint256 epochNumber = _advanceEpochs(1, 5, operatorStakers, 1);

        // Transfer the NFT to another address
        address tokenReceiver = address(0x888);
        _transferAndAssertNFTAssertions(
            isSafeTransfer,
            TransferNFTAssertions({
                tokenOwner: delegatingStaker,
                tokenSender: delegatingStaker,
                tokenReceiver: tokenReceiver,
                tokenId: 5,
                operatorStakerAddress: operatorStakers[0],
                expectedStakerToValidatorsTheyStakedToContainsOperator: false
            })
        );

        // Receiver can instantly claim rewards
        uint256 balanceBefore = token.balanceOf(tokenReceiver);
        vm.prank(tokenReceiver);
        stakingFacet.claimStakeRewards(1, operatorStakers[0], 1, 0);
        assertGt(token.balanceOf(tokenReceiver), balanceBefore);

        // Advance another 5 epochs to produce more staking rewards
        epochNumber = _advanceEpochs(1, 5, operatorStakers, epochNumber);

        // Receiver can claim rewards again
        balanceBefore = token.balanceOf(tokenReceiver);
        vm.prank(tokenReceiver);
        stakingFacet.claimStakeRewards(1, operatorStakers[0], 1, 0);
        assertGt(token.balanceOf(tokenReceiver), balanceBefore);
    }

    /// @notice This tests that when A transfers their NFT to B, whether A unfrezzes or B does,
    /// B can eventually withdraw their stake.
    function test_StakeAndTransfer_Withdraw(
        bool isSafeTransfer,
        bool isUnfreezeBeforeTransfer
    ) public {
        uint256 amount = 100 ether;
        uint256 timeLock = 15 days;
        uint256 roundedTimeLock = (timeLock / 86400) * 86400;

        {
            // Set the config
            LibStakingStorage.GlobalConfig
                memory globalConfig = stakingViewsFacet.globalConfig();
            globalConfig.minTimeLock = timeLock - 1 days;
            stakingAdminFacet.setConfig(globalConfig);
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

        // Advance 5 epochs to initialize the reward epochs
        uint256 currentEpochNumber = _advanceEpochs(1, 5, operatorStakers, 1);
        address tokenReceiver = address(0x888);
        if (isUnfreezeBeforeTransfer) {
            // Unfreeze the stake before transferring the NFT
            vm.prank(delegatingStaker);
            stakingFacet.unfreezeStake(operatorStakers[0], 1);

            // Transfer the NFT to another address
            _transferAndAssertNFTAssertions(
                isSafeTransfer,
                TransferNFTAssertions({
                    tokenOwner: delegatingStaker,
                    tokenSender: delegatingStaker,
                    tokenReceiver: tokenReceiver,
                    tokenId: 5,
                    operatorStakerAddress: operatorStakers[0],
                    expectedStakerToValidatorsTheyStakedToContainsOperator: false
                })
            );
        } else {
            // Transfer the NFT to another address
            _transferAndAssertNFTAssertions(
                isSafeTransfer,
                TransferNFTAssertions({
                    tokenOwner: delegatingStaker,
                    tokenSender: delegatingStaker,
                    tokenReceiver: tokenReceiver,
                    tokenId: 5,
                    operatorStakerAddress: operatorStakers[0],
                    expectedStakerToValidatorsTheyStakedToContainsOperator: false
                })
            );

            // Unfreeze the stake
            vm.prank(tokenReceiver);
            stakingFacet.unfreezeStake(operatorStakers[0], 1);
        }

        // Assert validatorToDelegatedStakersWithUnfreezingStakes
        {
            assertEq(
                stakingViewsFacet.getDelegatedStakersWithUnfreezingStakesCount(
                    operatorStakers[0]
                ),
                1
            );

            address[]
                memory validatorToDelegatedStakersWithUnfreezingStakes = stakingViewsFacet
                    .getDelegatedStakersWithUnfreezingStakes(
                        operatorStakers[0],
                        1,
                        0
                    );
            assertEq(validatorToDelegatedStakersWithUnfreezingStakes.length, 1);
            assertEq(
                validatorToDelegatedStakersWithUnfreezingStakes[0],
                tokenReceiver
            );
        }

        // Advance enough epochs to fully unfreeze the stake and withdraw
        {
            uint256 epochLength = stakingViewsFacet.epoch(1).epochLength;
            uint256 epochsToAdvance = ((roundedTimeLock + 1 days) /
                epochLength) + 1; // Add additional day because unfreezing starts at the beginning of the next day
            vm.pauseGasMetering();
            currentEpochNumber = _advanceEpochs(
                1,
                epochsToAdvance,
                operatorStakers,
                currentEpochNumber
            );
            vm.resumeGasMetering();
        }

        // Rewards must be claimed before withdrawing
        uint256 balanceBefore = token.balanceOf(tokenReceiver);
        vm.prank(tokenReceiver);
        stakingFacet.claimStakeRewards(
            1,
            operatorStakers[0],
            1,
            type(uint256).max
        );
        assertGt(token.balanceOf(tokenReceiver), balanceBefore);

        // Withdraw the stake
        _withdrawAndAssertNFTAssertions(
            WithdrawAndAssertParams({
                userStakerAddress: tokenReceiver,
                operatorStakerAddress: operatorStakers[0],
                stakeRecordId: 1
            })
        );
    }

    /// @notice This tests that when A approves B and then B transfers to C, that approval is revoked from B.
    function test_StakeApproveAndTransfer(bool isSafeTransfer) public {
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

        // Approve B
        address tokenReceiverB = address(0x888);
        address tokenReceiverC = address(0x777);
        vm.prank(delegatingStaker);
        stakingNFTFacet.approve(tokenReceiverB, 5);

        // Assert approvals
        assertEq(stakingNFTFacet.getApproved(5), tokenReceiverB);

        // B transfers the NFT to C
        _transferAndAssertNFTAssertions(
            isSafeTransfer,
            TransferNFTAssertions({
                tokenOwner: delegatingStaker,
                tokenSender: tokenReceiverB,
                tokenReceiver: tokenReceiverC,
                tokenId: 5,
                operatorStakerAddress: operatorStakers[0],
                expectedStakerToValidatorsTheyStakedToContainsOperator: false
            })
        );

        // Assert approvals are cleared
        assertEq(stakingNFTFacet.getApproved(5), address(0));
    }

    /// @notice This tests that when A approves for all, and then B transfers both tokens to C and D,
    /// that all approvals have NOT changed.
    function test_StakeApproveForAllAndTransfer(bool isSafeTransfer) public {
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
        _fundAddressWithTokensAndApprove(delegatingStaker, amount * 2);

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

        // Delegating staker stakes again with the same validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakers[0]);
        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: delegatingStaker,
                tokenId: 6,
                tokenBalance: 2,
                operatorStakerAddress: operatorStakers[0],
                tokenIndex: 1,
                totalSupply: 6,
                ownershipChanged: false,
                stakeRecordId: 2
            })
        );

        // Approve B for all tokens
        address tokenReceiverB = address(0x888);
        address tokenReceiverC = address(0x777);
        address tokenReceiverD = address(0x666);
        vm.prank(delegatingStaker);
        stakingNFTFacet.setApprovalForAll(tokenReceiverB, true);

        // Assert approvals
        assertTrue(
            stakingNFTFacet.isApprovedForAll(delegatingStaker, tokenReceiverB)
        );
        assertFalse(
            stakingNFTFacet.isApprovedForAll(delegatingStaker, tokenReceiverC)
        );
        assertFalse(
            stakingNFTFacet.isApprovedForAll(delegatingStaker, tokenReceiverD)
        );

        // B transfers both tokens to C and D
        _transferAndAssertNFTAssertions(
            isSafeTransfer,
            TransferNFTAssertions({
                tokenOwner: delegatingStaker,
                tokenSender: tokenReceiverB,
                tokenReceiver: tokenReceiverC,
                tokenId: 5,
                operatorStakerAddress: operatorStakers[0],
                expectedStakerToValidatorsTheyStakedToContainsOperator: true
            })
        );
        _transferAndAssertNFTAssertions(
            isSafeTransfer,
            TransferNFTAssertions({
                tokenOwner: delegatingStaker,
                tokenSender: tokenReceiverB,
                tokenReceiver: tokenReceiverD,
                tokenId: 6,
                operatorStakerAddress: operatorStakers[0],
                expectedStakerToValidatorsTheyStakedToContainsOperator: false
            })
        );

        // Assert approvals have NOT changed (it is impossible to clear entirely)
        assertTrue(
            stakingNFTFacet.isApprovedForAll(delegatingStaker, tokenReceiverB)
        );
        assertFalse(
            stakingNFTFacet.isApprovedForAll(delegatingStaker, tokenReceiverC)
        );
        assertFalse(
            stakingNFTFacet.isApprovedForAll(delegatingStaker, tokenReceiverD)
        );
    }

    /// @notice This tests that when a staker stakes multiple times and then transfers the first one,
    /// that the token owner by index is updated correctly.
    function test_StakeAndTransfer_TokenOwnerByIndex(
        bool isSafeTransfer
    ) public {
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
        _fundAddressWithTokensAndApprove(delegatingStaker, amount * 2);

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

        // Delegating staker stakes again with the same validator
        vm.prank(delegatingStaker);
        stakingFacet.stake(amount, timeLock, operatorStakers[0]);
        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: delegatingStaker,
                tokenId: 6,
                tokenBalance: 2,
                operatorStakerAddress: operatorStakers[0],
                tokenIndex: 1,
                totalSupply: 6,
                ownershipChanged: false,
                stakeRecordId: 2
            })
        );

        // Assert token owner by index is correct before transferring
        assertEq(stakingNFTFacet.tokenOfOwnerByIndex(delegatingStaker, 0), 5);
        assertEq(stakingNFTFacet.tokenOfOwnerByIndex(delegatingStaker, 1), 6);

        // Transfer the first token
        address tokenReceiver = address(0x888);
        _transferAndAssertNFTAssertions(
            isSafeTransfer,
            TransferNFTAssertions({
                tokenOwner: delegatingStaker,
                tokenSender: delegatingStaker,
                tokenReceiver: tokenReceiver,
                tokenId: 5,
                operatorStakerAddress: operatorStakers[0],
                expectedStakerToValidatorsTheyStakedToContainsOperator: true
            })
        );

        // Assert token owner by index is updated correctly
        assertEq(stakingNFTFacet.balanceOf(delegatingStaker), 1);
        assertEq(stakingNFTFacet.tokenOfOwnerByIndex(delegatingStaker, 0), 6);
    }

    /// @notice This tests that when a staker stakes the maximum number of times (30)
    /// and attempts to obtain another NFT to themselves, that the transfer should fail.
    function test_StakeMax_TransferShouldFail(bool isSafeTransfer) public {
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

        // Setup delegating staker #1
        address delegatingStaker1 = address(0x999);
        _fundAddressWithTokensAndApprove(delegatingStaker1, amount * 30);

        // Delegating staker #1 stakes 30 times with the same validator
        for (uint256 i = 0; i < 30; i++) {
            vm.prank(delegatingStaker1);
            stakingFacet.stake(amount, timeLock, operatorStakers[0]);
            _assertNFTAssertions(
                NFTAssertions({
                    tokenOwner: delegatingStaker1,
                    tokenId: 5 + i,
                    tokenBalance: 1 + i,
                    operatorStakerAddress: operatorStakers[0],
                    tokenIndex: i,
                    totalSupply: 5 + i,
                    ownershipChanged: false,
                    stakeRecordId: 1 + i
                })
            );
        }

        // Setup delegating staker #2
        address delegatingStaker2 = address(0x888);
        _fundAddressWithTokensAndApprove(delegatingStaker2, amount);

        // Delegating staker #2 stakes with the same validator
        vm.prank(delegatingStaker2);
        stakingFacet.stake(amount, timeLock, operatorStakers[0]);
        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: delegatingStaker2,
                tokenId: 35,
                tokenBalance: 1,
                operatorStakerAddress: operatorStakers[0],
                tokenIndex: 0,
                totalSupply: 35,
                ownershipChanged: false,
                stakeRecordId: 1
            })
        );

        // Delegating staker #2 attempts to transfer the NFT to delegating staker #1
        vm.prank(delegatingStaker2);
        vm.expectRevert(
            abi.encodeWithSelector(
                StakingNFTFacet.NoEmptyStakeRecordSlots.selector
            )
        );
        stakingNFTFacet.transferFrom(delegatingStaker2, delegatingStaker1, 35);
    }

    /* ========== PRIVATE FUNCTIONS ========== */

    struct WithdrawAndAssertParams {
        address userStakerAddress;
        address operatorStakerAddress;
        uint256 stakeRecordId;
    }

    function _withdrawAndAssertNFTAssertions(
        WithdrawAndAssertParams memory params
    ) private {
        // Get the stake record before the withdraw
        LibStakingStorage.StakeRecord
            memory stakeRecordBefore = stakingViewsFacet.getStakeRecord(
                params.operatorStakerAddress,
                params.stakeRecordId,
                params.userStakerAddress
            );
        (
            uint256 stakeRecordTotalStakeByUserBefore,
            uint256 _stakeRecordTotalStakeWeightByUserBefore
        ) = stakingViewsFacet.getTotalStakeByUser(
                params.operatorStakerAddress,
                params.userStakerAddress
            );
        uint256 nftTokenBalanceBefore = stakingNFTFacet.balanceOf(
            params.userStakerAddress
        );
        uint256 tokenBalanceBefore = token.balanceOf(params.userStakerAddress);
        uint256 totalSupplyBefore = stakingNFTFacet.totalSupply();

        // Withdraw the stake
        vm.prank(params.userStakerAddress);
        stakingFacet.withdraw(
            params.operatorStakerAddress,
            params.stakeRecordId
        );

        // Assert that the stake was withdrawn
        (
            uint256 stakeRecordTotalStakeByUserAfter,
            uint256 _stakeRecordTotalStakeWeightByUserAfter
        ) = stakingViewsFacet.getTotalStakeByUser(
                params.operatorStakerAddress,
                params.userStakerAddress
            );
        assertEq(
            stakeRecordTotalStakeByUserAfter,
            stakeRecordTotalStakeByUserBefore - stakeRecordBefore.amount
        );
        assertEq(
            token.balanceOf(params.userStakerAddress),
            tokenBalanceBefore + stakeRecordBefore.amount
        );
        assertEq(
            stakingNFTFacet.balanceOf(params.userStakerAddress),
            nftTokenBalanceBefore - 1
        );
        assertEq(
            stakingNFTFacet.ownerOf(stakeRecordBefore.tokenId),
            address(0)
        );
        assertEq(stakingNFTFacet.totalSupply(), totalSupplyBefore); // total supply never decreases

        // We do not currently clear this data but we could.
        assertEq(
            stakingNFTFacet
                .tokenToStakeRecord(stakeRecordBefore.tokenId)
                .operatorStakerAddress,
            params.operatorStakerAddress
        );
        assertEq(
            stakingNFTFacet
                .tokenToStakeRecord(stakeRecordBefore.tokenId)
                .stakeRecordId,
            params.stakeRecordId
        );
    }

    struct TransferNFTAssertions {
        address tokenOwner; // Owner of NFT before transfer
        address tokenSender;
        address tokenReceiver;
        uint256 tokenId;
        address operatorStakerAddress;
        bool expectedStakerToValidatorsTheyStakedToContainsOperator;
    }

    function _transferAndAssertNFTAssertions(
        bool isSafeTransfer,
        TransferNFTAssertions memory transferNFTAssertions
    ) private {
        // Get sender and receiver details before the transfer
        uint256 balanceOfOwnerBefore = stakingNFTFacet.balanceOf(
            transferNFTAssertions.tokenOwner
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
            memory stakeRecordOwnerBefore = stakingViewsFacet.getStakeRecord(
                transferNFTAssertions.operatorStakerAddress,
                stakeRecordIdBefore,
                transferNFTAssertions.tokenOwner
            );
        LibStakingStorage.StakeRecord
            memory mostRecentStakeRecordReceiverBefore = stakingViewsFacet
                .getMostRecentStakeRecord(
                    transferNFTAssertions.tokenReceiver,
                    transferNFTAssertions.operatorStakerAddress
                );

        // Transfer the NFT
        vm.prank(transferNFTAssertions.tokenSender);
        if (isSafeTransfer) {
            stakingNFTFacet.transferFrom(
                transferNFTAssertions.tokenOwner,
                transferNFTAssertions.tokenReceiver,
                transferNFTAssertions.tokenId
            );
        } else {
            stakingNFTFacet.safeTransferFrom(
                transferNFTAssertions.tokenOwner,
                transferNFTAssertions.tokenReceiver,
                transferNFTAssertions.tokenId
            );
        }

        // Assertions
        assertEq(
            stakingNFTFacet.balanceOf(transferNFTAssertions.tokenOwner),
            balanceOfOwnerBefore - 1
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
                stakeRecordIdBefore
            )
        );
        stakingViewsFacet.getStakeRecord(
            transferNFTAssertions.operatorStakerAddress,
            stakeRecordIdBefore,
            transferNFTAssertions.tokenOwner
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
            stakeRecordOwnerBefore.lastUpdateTimestamp
        );

        // Assert stakerToValidatorsTheyStakedTo
        {
            address[]
                memory stakerToValidatorsTheyStakedToOwner = stakingViewsFacet
                    .stakerToValidatorsTheyStakedTo(
                        transferNFTAssertions.tokenOwner
                    );
            address[]
                memory stakerToValidatorsTheyStakedToReceiver = stakingViewsFacet
                    .stakerToValidatorsTheyStakedTo(
                        transferNFTAssertions.tokenReceiver
                    );
            bool ownerContainsOperatorStakerAddress = false;
            for (
                uint256 i = 0;
                i < stakerToValidatorsTheyStakedToOwner.length;
                i++
            ) {
                if (
                    stakerToValidatorsTheyStakedToOwner[i] ==
                    transferNFTAssertions.operatorStakerAddress
                ) {
                    ownerContainsOperatorStakerAddress = true;
                }
            }
            assertEq(
                ownerContainsOperatorStakerAddress,
                transferNFTAssertions
                    .expectedStakerToValidatorsTheyStakedToContainsOperator
            );
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

    struct IncreaseStakeRecordAmountAndAssertParams {
        address userStakerAddress;
        address operatorStakerAddress;
        uint256 stakeRecordId;
        uint256 tokenId;
        bool isTransferredToken;
    }

    function _increaseStakeRecordAmountAndAssert(
        IncreaseStakeRecordAmountAndAssertParams memory params
    ) private {
        // Get the stake record before the increase
        LibStakingStorage.StakeRecord
            memory stakeRecordBefore = stakingViewsFacet.getStakeRecord(
                params.operatorStakerAddress,
                params.stakeRecordId,
                params.userStakerAddress
            );
        uint256 existingStakeRecordAmount = stakeRecordBefore.amount;
        uint256 totalSupplyBefore = stakingNFTFacet.totalSupply();

        vm.prank(params.userStakerAddress);
        stakingFacet.increaseStakeRecordAmount(
            params.operatorStakerAddress,
            params.stakeRecordId,
            1 ether // always increase by 1 ether
        );

        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: params.userStakerAddress,
                tokenId: params.tokenId,
                tokenBalance: 1,
                operatorStakerAddress: params.operatorStakerAddress,
                tokenIndex: 0,
                totalSupply: totalSupplyBefore,
                ownershipChanged: params.isTransferredToken,
                stakeRecordId: params.stakeRecordId + 1
            })
        );
    }

    struct IncreaseStakeRecordTimelockAndAssertParams {
        address userStakerAddress;
        address operatorStakerAddress;
        uint256 stakeRecordId;
        uint256 tokenId;
        bool isTransferredToken;
    }

    function _increaseStakeRecordTimelockAndAssert(
        IncreaseStakeRecordTimelockAndAssertParams memory params
    ) private {
        // Get the stake record before the increase
        LibStakingStorage.StakeRecord
            memory stakeRecordBefore = stakingViewsFacet.getStakeRecord(
                params.operatorStakerAddress,
                params.stakeRecordId,
                params.userStakerAddress
            );
        uint256 existingStakeRecordTimelock = stakeRecordBefore.timeLock;
        uint256 totalSupplyBefore = stakingNFTFacet.totalSupply();

        vm.prank(params.userStakerAddress);
        stakingFacet.increaseStakeRecordTimelock(
            params.operatorStakerAddress,
            params.stakeRecordId,
            1 days
        );

        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: params.userStakerAddress,
                tokenId: params.tokenId,
                tokenBalance: 1,
                operatorStakerAddress: params.operatorStakerAddress,
                tokenIndex: 0,
                totalSupply: totalSupplyBefore,
                ownershipChanged: params.isTransferredToken,
                stakeRecordId: params.stakeRecordId + 1
            })
        );
    }

    struct SplitStakeRecordAndAssertParams {
        address userStakerAddress;
        address operatorStakerAddress;
        uint256 stakeRecordId;
        uint256 tokenId;
        bool isTransferredToken;
    }

    function _splitStakeRecordAndAssert(
        SplitStakeRecordAndAssertParams memory params
    ) private {
        // Get the stake record before the split
        LibStakingStorage.StakeRecord
            memory stakeRecordBefore = stakingViewsFacet.getStakeRecord(
                params.operatorStakerAddress,
                params.stakeRecordId,
                params.userStakerAddress
            );
        uint256 existingStakeRecordAmount = stakeRecordBefore.amount;
        uint256 existingStakeRecordTimelock = stakeRecordBefore.timeLock;
        uint256 totalSupplyBefore = stakingNFTFacet.totalSupply();

        vm.prank(params.userStakerAddress);
        stakingFacet.splitStakeRecord(
            params.operatorStakerAddress,
            params.stakeRecordId,
            .5 ether // always split into 2 equal parts
        );

        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: params.userStakerAddress,
                tokenId: params.tokenId,
                tokenBalance: 2,
                operatorStakerAddress: params.operatorStakerAddress,
                tokenIndex: 0,
                totalSupply: totalSupplyBefore + 1,
                ownershipChanged: params.isTransferredToken,
                stakeRecordId: params.stakeRecordId + 1
            })
        );

        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: params.userStakerAddress,
                tokenId: params.tokenId + 1,
                tokenBalance: 2,
                operatorStakerAddress: params.operatorStakerAddress,
                tokenIndex: 1,
                totalSupply: totalSupplyBefore + 1,
                ownershipChanged: false,
                stakeRecordId: params.stakeRecordId + 2
            })
        );
    }

    struct MigrateStakeRecordAndAssertParams {
        address userStakerAddress;
        address operatorStakerAddress;
        address operatorStakerAddressToMigrateTo;
        uint256 stakeRecordId;
        uint256 tokenId;
        bool isTransferredToken;
    }

    function _migrateStakeRecordAndAssert(
        MigrateStakeRecordAndAssertParams memory params
    ) private {
        // Get the stake record before the migrate
        LibStakingStorage.StakeRecord
            memory stakeRecordBefore = stakingViewsFacet.getStakeRecord(
                params.operatorStakerAddress,
                params.stakeRecordId,
                params.userStakerAddress
            );
        uint256 existingStakeRecordAmount = stakeRecordBefore.amount;
        uint256 existingStakeRecordTimelock = stakeRecordBefore.timeLock;
        uint256 totalSupplyBefore = stakingNFTFacet.totalSupply();

        vm.prank(params.userStakerAddress);
        stakingFacet.migrateStakeRecord(
            params.operatorStakerAddress,
            params.stakeRecordId,
            params.operatorStakerAddressToMigrateTo
        );

        _assertNFTAssertions(
            NFTAssertions({
                tokenOwner: params.userStakerAddress,
                tokenId: params.tokenId,
                tokenBalance: 1,
                operatorStakerAddress: params.operatorStakerAddressToMigrateTo,
                tokenIndex: 0,
                totalSupply: totalSupplyBefore,
                ownershipChanged: params.isTransferredToken,
                stakeRecordId: 1
            })
        );
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

    function _assertNFTAssertions(NFTAssertions memory nftAssertions) private {
        assertEq(
            stakingNFTFacet.balanceOf(nftAssertions.tokenOwner),
            nftAssertions.tokenBalance,
            "Token balance mismatch"
        );
        assertEq(
            stakingNFTFacet.ownerOf(nftAssertions.tokenId),
            nftAssertions.tokenOwner,
            "Token owner mismatch"
        );
        assertEq(
            stakingNFTFacet.tokenOfOwnerByIndex(
                nftAssertions.tokenOwner,
                nftAssertions.tokenIndex
            ),
            nftAssertions.tokenId,
            "Token of owner by index mismatch"
        );
        assertEq(
            stakingNFTFacet.totalSupply(),
            nftAssertions.totalSupply,
            "Total supply mismatch"
        );
        if (nftAssertions.ownershipChanged) {
            assertGt(
                stakingNFTFacet.ownershipChange(nftAssertions.tokenId),
                0,
                "Ownership change mismatch"
            );
        } else {
            assertEq(
                stakingNFTFacet.ownershipChange(nftAssertions.tokenId),
                0,
                "Ownership change mismatch"
            );
        }
        assertEq(
            stakingNFTFacet
                .tokenToStakeRecord(nftAssertions.tokenId)
                .operatorStakerAddress,
            nftAssertions.operatorStakerAddress,
            "Operator staker address mismatch"
        );
        assertEq(
            stakingNFTFacet
                .tokenToStakeRecord(nftAssertions.tokenId)
                .stakeRecordId,
            nftAssertions.stakeRecordId,
            "Stake record id mismatch"
        );

        LibStakingStorage.StakeRecord memory stakeRecord = stakingViewsFacet
            .getStakeRecord(
                nftAssertions.operatorStakerAddress,
                nftAssertions.stakeRecordId,
                nftAssertions.tokenOwner
            );
        assertEq(
            stakeRecord.tokenId,
            nftAssertions.tokenId,
            "Token id mismatch"
        );
    }
}
