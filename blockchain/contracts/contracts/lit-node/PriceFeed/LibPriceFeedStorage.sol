//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";

library LibPriceFeedStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    bytes32 constant PRICE_FEED_POSITION = keccak256("pricefeed.storage");

    enum ProductId {
        PkpSign,
        EncSign,
        LitAction,
        SignSessionKey
    }

    enum LitActionPriceComponent {
        baseAmount,
        runtimeLength,
        memoryUsage,
        codeLength,
        responseLength,
        signatures,
        broadcasts,
        contractCalls,
        callDepth,
        decrypts,
        fetches
    }

    enum NodePriceMeasurement {
        perSecond,
        perMegabyte,
        perCount
    }

    struct LitActionPriceConfig {
        LitActionPriceComponent priceComponent;
        NodePriceMeasurement priceMeasurement;
        uint256 price;
    }

    struct NodePriceData {
        address stakerAddress;
        uint256 price;
        uint256 productId;
        uint256 timestamp;
    }

    struct NodeInfoAndPrices {
        LibStakingStorage.Validator validator;
        uint256[] prices;
    }

    struct NodeCapacityConfig {
        uint256 pkpSignMaxConcurrency;
        uint256 encSignMaxConcurrency;
        uint256 litActionMaxConcurrency;
        uint256 signSessionKeyMaxConcurrency;
        uint256 globalMaxCapacity;
    }

    struct PriceFeedStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        // maps node staker address to product id to NodePriceData for that product id.
        mapping(address => mapping(uint256 => NodePriceData)) nodePrices;
        mapping(uint256 => uint256) baseNetworkPrices;
        mapping(uint256 => uint256) maxNetworkPrices;
        mapping(LitActionPriceComponent => LitActionPriceConfig) litActionPriceConfigs;
        // Use nodeCapacityConfigs[0] for now.
        mapping(uint256 => NodeCapacityConfig) nodeCapacityConfigs;
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (PriceFeedStorage storage storageStruct)
    {
        bytes32 position = PRICE_FEED_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
