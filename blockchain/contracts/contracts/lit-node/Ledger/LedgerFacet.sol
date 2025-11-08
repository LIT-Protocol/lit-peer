//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { ERC2771 } from "../../common/ERC2771.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { StakingAcrossRealmsFacet } from "../Staking/StakingAcrossRealmsFacet.sol";
import { StakingUtilsLib } from "../Staking/StakingUtilsLib.sol";

import { LibLedgerStorage } from "./LibLedgerStorage.sol";

// import "hardhat/console.sol";

contract LedgerFacet is ERC2771 {
    using EnumerableSet for EnumerableSet.AddressSet;

    /* ========== MODIFIERS ========== */

    modifier onlyOwner() override {
        if (LibERC2771._msgSender() != LibDiamond.contractOwner())
            revert CallerNotOwner();
        _;
    }

    /* ========== ERRORS ========== */
    error AmountMustBePositive();
    error SessionAlreadyUsed();
    error NodeNotStakingNode();
    error InsufficientFunds();
    error WithdrawalDelayNotPassed();
    error InsufficientWithdrawAmount();
    error ValueExceedsUint128MaxLimit();
    error MustBeNonzero();
    error ArrayLengthsMustMatch();
    error PercentageMustBeLessThan100();

    /* ========== VIEWS ========== */
    function s()
        internal
        pure
        returns (LibLedgerStorage.LedgerStorage storage)
    {
        return LibLedgerStorage.getStorage();
    }

    function realms() internal view returns (StakingAcrossRealmsFacet) {
        return StakingAcrossRealmsFacet(getStakingAddress());
    }

    function getStakingAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.STAKING_CONTRACT(),
                s().env
            );
    }

    function balance(address user) public view returns (int256) {
        return s().balances[user];
    }

    function stableBalance(address user) public view returns (int256) {
        return
            s().balances[user] - int256(s().userWithdrawRequests[user].amount);
    }

    function rewardBalance(address user) public view returns (uint256) {
        return s().rewards[user];
    }

    function latestWithdrawRequest(
        address user
    ) public view returns (LibLedgerStorage.WithdrawRequest memory) {
        return s().userWithdrawRequests[user];
    }

    function latestRewardWithdrawRequest(
        address user
    ) public view returns (LibLedgerStorage.WithdrawRequest memory) {
        return s().rewardWithdrawRequests[user];
    }

    function userWithdrawDelay() public view returns (uint256) {
        return s().userWithdrawDelay;
    }

    function rewardWithdrawDelay() public view returns (uint256) {
        return s().rewardWithdrawDelay;
    }

    function litFoundationSplitPercentage() public view returns (uint256) {
        return s().litFoundationSplitPercentage;
    }

    function litFoundationRewards() public view returns (uint256) {
        return s().litFoundationRewards;
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    // charge a single user for a single request
    function chargeUser(address user, int256 amount) public {
        if (amount <= 0) revert AmountMustBePositive();
        StakingViewsFacet staking = StakingViewsFacet(getStakingAddress());
        address stakerAddress = realms().nodeAddressToStakerAddressAcrossRealms(
            LibERC2771._msgSender()
        );
        uint256 realmId = realms().getRealmIdForStakerAddress(stakerAddress);
        if (!staking.isRecentValidator(realmId, stakerAddress))
            revert NodeNotStakingNode();
        s().balances[user] -= amount;

        // split rewards between the staker and the lit foundation
        uint256 litFoundationRewardAmount = (uint256(amount) *
            s().litFoundationSplitPercentage) / 100;
        s().rewards[stakerAddress] +=
            uint256(amount) -
            litFoundationRewardAmount;
        s().litFoundationRewards += litFoundationRewardAmount;

        emit UserCharged(user, amount);
    }

    // charge multiple users for a single request each
    function chargeUsers(
        address[] memory users,
        int256[] memory amounts,
        uint64 batchId
    ) public {
        if (users.length != amounts.length) revert ArrayLengthsMustMatch();
        for (uint256 i = 0; i < users.length; i++) {
            chargeUser(users[i], amounts[i]);
        }
        address stakerAddress = realms().nodeAddressToStakerAddressAcrossRealms(
            LibERC2771._msgSender()
        );
        emit BatchCharged(stakerAddress, batchId);
    }

    // users can deposit funds with this, to be used for payment
    function deposit() public payable {
        if (msg.value > type(uint128).max) revert ValueExceedsUint128MaxLimit();
        s().balances[LibERC2771._msgSender()] += int256(msg.value);
        emit Deposit(LibERC2771._msgSender(), msg.value);
    }

    // users can deposit funds with this, to be used for payment
    function depositForUser(address user) public payable {
        if (msg.value > type(uint128).max) revert ValueExceedsUint128MaxLimit();
        s().balances[user] += int256(msg.value);
        emit DepositForUser(LibERC2771._msgSender(), user, msg.value);
    }

    // users can request a withdraw
    function requestWithdraw(int256 amount) public {
        if (amount <= 0) revert AmountMustBePositive();
        if (s().balances[LibERC2771._msgSender()] < amount)
            revert InsufficientFunds();
        s().userWithdrawRequests[LibERC2771._msgSender()] = LibLedgerStorage
            .WithdrawRequest(block.timestamp, uint256(amount));
        emit WithdrawRequest(LibERC2771._msgSender(), amount);
    }

    // users can withdraw once the delay has passed
    function withdraw(int256 amount) public {
        if (amount <= 0) revert AmountMustBePositive();
        if (s().balances[LibERC2771._msgSender()] < amount)
            revert InsufficientFunds();
        if (
            s().userWithdrawRequests[LibERC2771._msgSender()].timestamp == 0 ||
            s().userWithdrawRequests[LibERC2771._msgSender()].amount == 0
        ) revert MustBeNonzero();
        if (
            block.timestamp -
                s().userWithdrawRequests[LibERC2771._msgSender()].timestamp <
            s().userWithdrawDelay
        ) revert WithdrawalDelayNotPassed();
        if (
            s().userWithdrawRequests[LibERC2771._msgSender()].amount <
            uint256(amount)
        ) revert InsufficientWithdrawAmount();
        // clear out the request
        s().userWithdrawRequests[LibERC2771._msgSender()] = LibLedgerStorage
            .WithdrawRequest(0, 0);
        s().balances[LibERC2771._msgSender()] -= amount;
        payable(LibERC2771._msgSender()).transfer(uint256(amount));
        emit Withdraw(LibERC2771._msgSender(), amount);
    }

    // node operators can request to withdraw their rewards
    function requestRewardWithdraw(uint256 amount) public {
        if (amount <= 0) revert AmountMustBePositive();
        if (s().rewards[LibERC2771._msgSender()] < amount)
            revert InsufficientFunds();
        s().rewardWithdrawRequests[LibERC2771._msgSender()] = LibLedgerStorage
            .WithdrawRequest(block.timestamp, uint256(amount));
        emit RewardWithdrawRequest(LibERC2771._msgSender(), amount);
    }

    // node operators can withdraw their rewards once the delay has passed
    function withdrawRewards(uint256 amount) public {
        if (amount <= 0) revert AmountMustBePositive();
        if (s().rewards[LibERC2771._msgSender()] < amount)
            revert InsufficientFunds();
        if (
            s().rewardWithdrawRequests[LibERC2771._msgSender()].timestamp ==
            0 ||
            s().rewardWithdrawRequests[LibERC2771._msgSender()].amount == 0
        ) revert MustBeNonzero();
        if (
            block.timestamp -
                s().rewardWithdrawRequests[LibERC2771._msgSender()].timestamp <
            s().rewardWithdrawDelay
        ) revert WithdrawalDelayNotPassed();
        if (s().rewardWithdrawRequests[LibERC2771._msgSender()].amount < amount)
            revert InsufficientWithdrawAmount();
        // clear out the request
        s().rewardWithdrawRequests[LibERC2771._msgSender()] = LibLedgerStorage
            .WithdrawRequest(0, 0);
        s().rewards[LibERC2771._msgSender()] -= amount;
        payable(LibERC2771._msgSender()).transfer(amount);
        emit RewardWithdraw(LibERC2771._msgSender(), amount);
    }

    // admin function to set withdraw delay for users
    function setUserWithdrawDelay(uint256 delay) public onlyOwner {
        s().userWithdrawDelay = delay;
        emit UserWithdrawDelaySet(delay);
    }

    // admin function to set reward withdraw delay for node operators
    function setRewardWithdrawDelay(uint256 delay) public onlyOwner {
        s().rewardWithdrawDelay = delay;
        emit RewardWithdrawDelaySet(delay);
    }

    // admin function to set the percentage of rewards for the lit foundation
    function setLitFoundationSplitPercentage(
        uint256 percentage
    ) public onlyOwner {
        if (percentage > 100) revert PercentageMustBeLessThan100();
        s().litFoundationSplitPercentage = percentage;
        emit LitFoundationSplitPercentageSet(percentage);
    }

    // admin function to withdraw foundation rewards
    function withdrawFoundationRewards(uint256 amount) public onlyOwner {
        if (amount <= 0) revert AmountMustBePositive();
        if (s().litFoundationRewards < amount) revert InsufficientFunds();
        // subtract the amount from storage before the transfer to prevent reentrancy
        s().litFoundationRewards -= amount;
        payable(LibERC2771._msgSender()).transfer(amount);
        emit FoundationRewardsWithdrawn(amount);
    }

    /* ========== EVENTS ========== */

    event Deposit(address indexed user, uint256 amount);
    event DepositForUser(
        address indexed depositor,
        address indexed user,
        uint256 amount
    );
    event Withdraw(address indexed user, int256 amount);
    event WithdrawRequest(address indexed user, int256 amount);
    event RewardWithdraw(address indexed user, uint256 amount);
    event RewardWithdrawRequest(address indexed user, uint256 amount);
    event RewardWithdrawDelaySet(uint256 delay);
    event UserWithdrawDelaySet(uint256 delay);
    event UserCharged(address indexed user, int256 amount);
    event BatchCharged(address indexed node_address, uint256 batch_id);
    event LitFoundationSplitPercentageSet(uint256 percentage);
    event FoundationRewardsWithdrawn(uint256 amount);
}
