// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
*
* Implementation of a diamond.
/******************************************************************************/

import { LibDiamond } from "../libraries/LibDiamond.sol";
import { IDiamondCut } from "../interfaces/IDiamondCut.sol";
import { IDiamondLoupe } from "../interfaces/IDiamondLoupe.sol";

import { ContractResolver } from "../lit-core/ContractResolver.sol";
import { LibPriceFeedStorage } from "./PriceFeed/LibPriceFeedStorage.sol";

// When no function exists for function called
error FunctionNotFound(bytes4 _functionSelector);

// This is used in diamond constructor
// more arguments are added to this struct
// this avoids stack too deep errors
struct PriceFeedArgs {
    address owner;
    address init;
    bytes initCalldata;
    address contractResolver;
    ContractResolver.Env env;
}

contract PriceFeed {
    constructor(
        IDiamondCut.FacetCut[] memory _diamondCut,
        PriceFeedArgs memory _args
    ) payable {
        LibDiamond.setContractOwner(_args.owner);
        LibDiamond.diamondCut(_diamondCut, _args.init, _args.initCalldata);

        LibPriceFeedStorage.PriceFeedStorage storage s = LibPriceFeedStorage
            .getStorage();

        s.contractResolver = ContractResolver(_args.contractResolver);
        s.env = _args.env;
        // set default prices for the first 4 product IDs (see LibPriceFeedStorage.ProductId)
        //
        // NOTE (testnet defaults):
        // These values are appropriate for testnet deployments and were taken from the
        // `naga-test` network on 2026-01-01.
        //
        // Base Network Prices:
        // - Encryption Sign:      29751.916618468 gwei
        // - Lit Action:           5950.383323693 gwei
        // - PKP Sign:             29751.916618468 gwei
        // - Session Key Sign:     148759.583092342 gwei
        uint256 pkpSignBasePrice = 29751.916618468 gwei;
        uint256 encSignBasePrice = 29751.916618468 gwei;
        uint256 litActionBasePrice = 5950.383323693 gwei;
        uint256 signSessionKeyBasePrice = 148759.583092342 gwei;

        s.baseNetworkPrices[uint256(LibPriceFeedStorage.ProductId.PkpSign)] = pkpSignBasePrice;
        s.maxNetworkPrices[uint256(LibPriceFeedStorage.ProductId.PkpSign)] = pkpSignBasePrice * 100;

        s.baseNetworkPrices[uint256(LibPriceFeedStorage.ProductId.EncSign)] = encSignBasePrice;
        s.maxNetworkPrices[uint256(LibPriceFeedStorage.ProductId.EncSign)] = encSignBasePrice * 100;

        s.baseNetworkPrices[uint256(LibPriceFeedStorage.ProductId.LitAction)] = litActionBasePrice;
        s.maxNetworkPrices[uint256(LibPriceFeedStorage.ProductId.LitAction)] = litActionBasePrice * 100;

        s.baseNetworkPrices[uint256(LibPriceFeedStorage.ProductId.SignSessionKey)] = signSessionKeyBasePrice;
        s.maxNetworkPrices[uint256(LibPriceFeedStorage.ProductId.SignSessionKey)] = signSessionKeyBasePrice * 100;

        s.nodeCapacityConfigs[0] = LibPriceFeedStorage.NodeCapacityConfig({
            pkpSignMaxConcurrency: 75,
            encSignMaxConcurrency: 300,
            litActionMaxConcurrency: 50,
            signSessionKeyMaxConcurrency: 300,
            globalMaxCapacity: 300
        });

        // Lit Action Individual Prices
        //
        // NOTE (testnet defaults):
        // These values are appropriate for testnet deployments and were taken from the
        // `naga-test` network on 2026-01-01.
        // - baseAmount:      29751.916618468 gwei  perCount
        // - runtimeLength:   595.038332369 gwei    perSecond
        // - memoryUsage:     59.503833236 gwei     perMegabyte
        // - codeLength:      59.503833236 gwei     perMegabyte
        // - responseLength:  59.503833236 gwei     perMegabyte
        // - signatures:      29751.916618468 gwei  perCount
        // - broadcasts:      595.038332369 gwei    perCount
        // - contractCalls:   2975.191661846 gwei   perCount
        // - callDepth:       595.038332369 gwei    perCount
        // - decrypts:        5950.383323693 gwei   perCount
        // - fetches:         595.038332369 gwei    perCount

        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.baseAmount,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            29751.916618468 gwei
        );

        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.runtimeLength,
            LibPriceFeedStorage.NodePriceMeasurement.perSecond,
            595.038332369 gwei
        );

        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.memoryUsage,
            LibPriceFeedStorage.NodePriceMeasurement.perMegabyte,
            59.503833236 gwei
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.codeLength,
            LibPriceFeedStorage.NodePriceMeasurement.perMegabyte,
            59.503833236 gwei
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.responseLength,
            LibPriceFeedStorage.NodePriceMeasurement.perMegabyte,
            59.503833236 gwei
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.signatures,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            29751.916618468 gwei
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.broadcasts,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            595.038332369 gwei
        );

        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.contractCalls,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            2975.191661846 gwei
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.callDepth,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            595.038332369 gwei
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.decrypts,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            5950.383323693 gwei
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.fetches,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            595.038332369 gwei
        );
    }

    function setLitActionPriceConfig(
        LibPriceFeedStorage.LitActionPriceComponent component,
        LibPriceFeedStorage.NodePriceMeasurement priceMeasurement,
        uint256 price
    ) internal {
        LibPriceFeedStorage.PriceFeedStorage storage s = LibPriceFeedStorage
            .getStorage();
        s.litActionPriceConfigs[component] = LibPriceFeedStorage
            .LitActionPriceConfig({
                priceComponent: component,
                priceMeasurement: priceMeasurement,
                price: price
            });
    }

    // Find facet for function that is called and execute the
    // function if a facet is found and return any value.
    fallback() external {
        LibDiamond.DiamondStorage storage ds;
        bytes32 position = LibDiamond.DIAMOND_STORAGE_POSITION;
        // get diamond storage
        assembly {
            ds.slot := position
        }
        // get facet from function selector
        address facet = ds
            .facetAddressAndSelectorPosition[msg.sig]
            .facetAddress;
        if (facet == address(0)) {
            revert FunctionNotFound(msg.sig);
        }
        // Execute external function from facet using delegatecall and return any value.
        assembly {
            // copy function selector and any arguments
            calldatacopy(0, 0, calldatasize())
            // execute function call using the facet
            let result := delegatecall(gas(), facet, 0, calldatasize(), 0, 0)
            // get any return value
            returndatacopy(0, 0, returndatasize())
            // return any return value or error back to the caller
            switch result
            case 0 {
                revert(0, returndatasize())
            }
            default {
                return(0, returndatasize())
            }
        }
    }
}
