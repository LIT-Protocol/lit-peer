//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";

library LibLedgerStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    bytes32 constant LEDGER_POSITION = keccak256("ledger.storage");

    struct WithdrawRequest {
        uint256 timestamp;
        uint256 amount;
    }

    struct LedgerStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        uint256 userWithdrawDelay; // delay in seconds for a user to withdraw their balance
        uint256 rewardWithdrawDelay; // delay in seconds for a node to withdraw their reward
        mapping(address => int256) balances; // shadow ledger of all balances - can go negative
        mapping(address => uint256) rewards; // rewards for each node
        mapping(address => WithdrawRequest) userWithdrawRequests; // withdraw requests for a user
        mapping(address => WithdrawRequest) rewardWithdrawRequests; // withdraw requests for a node
        uint256 litFoundationRewards; // total rewards for the lit foundation
        uint256 litFoundationSplitPercentage; // percentage of rewards for the lit foundation.  must be less than 100.
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (LedgerStorage storage storageStruct)
    {
        bytes32 position = LEDGER_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
