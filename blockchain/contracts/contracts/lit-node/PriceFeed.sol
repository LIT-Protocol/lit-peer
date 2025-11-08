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
        // set default prices for the first 4 product IDs.  goes from 0.01 tokens min to 1 token max
        uint256 baseAmount = 10000000000000000;
        for (uint256 i = 0; i < 4; i++) {
            s.baseNetworkPrices[i] = baseAmount;
            s.maxNetworkPrices[i] = baseAmount * 100;
        }
        s.nodeCapacityConfig = LibPriceFeedStorage.NodeCapacityConfig({
            pkpSignMaxConcurrency: 75,
            encSignMaxConcurrency: 300,
            litActionMaxConcurrency: 50,
            signSessionKeyMaxConcurrency: 300,
            globalMaxCapacity: 300
        });

        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.baseAmount,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            baseAmount / 2
        );

        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.runtimeLength,
            LibPriceFeedStorage.NodePriceMeasurement.perSecond,
            baseAmount / 10
        );

        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.memoryUsage,
            LibPriceFeedStorage.NodePriceMeasurement.perMegabyte,
            baseAmount / 100
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.codeLength,
            LibPriceFeedStorage.NodePriceMeasurement.perMegabyte,
            baseAmount / 10
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.responseLength,
            LibPriceFeedStorage.NodePriceMeasurement.perMegabyte,
            baseAmount / 10
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.signatures,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            baseAmount
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.broadcasts,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            baseAmount / 10
        );

        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.contractCalls,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            baseAmount / 10
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.callDepth,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            baseAmount / 10
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.decrypts,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            baseAmount / 10
        );
        setLitActionPriceConfig(
            LibPriceFeedStorage.LitActionPriceComponent.fetches,
            LibPriceFeedStorage.NodePriceMeasurement.perCount,
            baseAmount / 10
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
