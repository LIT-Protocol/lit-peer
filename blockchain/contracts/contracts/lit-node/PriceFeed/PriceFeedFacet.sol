//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { StakingAcrossRealmsFacet } from "../Staking/StakingAcrossRealmsFacet.sol";
import { LibPriceFeedStorage } from "./LibPriceFeedStorage.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { ERC2771 } from "../../common/ERC2771.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { StakingUtilsLib } from "../Staking/StakingUtilsLib.sol";

// import { console } from "hardhat/console.sol";

contract PriceFeedFacet is ERC2771 {
    using EnumerableSet for EnumerableSet.AddressSet;

    /* ========== ERRORS ========== */
    error MustBeNonzero();

    /* ========== MODIFIERS ========== */

    modifier onlyOwner() override {
        if (LibERC2771._msgSender() != LibDiamond.contractOwner())
            revert CallerNotOwner();
        _;
    }

    /* ========== VIEWS ========== */
    function s()
        internal
        pure
        returns (LibPriceFeedStorage.PriceFeedStorage storage)
    {
        return LibPriceFeedStorage.getStorage();
    }

    function realms() internal view returns (StakingAcrossRealmsFacet) {
        return StakingAcrossRealmsFacet(getStakingAddress());
    }

    function stakingByRealm() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(getStakingAddress());
    }

    function getStakingAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.STAKING_CONTRACT(),
                s().env
            );
    }

    // get the stored price for a node for given product(s)
    function price(
        address node,
        uint256[] memory productIds
    ) external view returns (LibPriceFeedStorage.NodePriceData[] memory) {
        LibPriceFeedStorage.NodePriceData[]
            memory collectedPrices = new LibPriceFeedStorage.NodePriceData[](
                productIds.length
            );
        for (uint256 i = 0; i < productIds.length; i++) {
            collectedPrices[i] = s().nodePrices[node][productIds[i]];
        }
        return collectedPrices;
    }

    // gets the stored prices for all active nodes for given product(s)
    function prices(
        uint256 productId
    ) external view returns (LibPriceFeedStorage.NodePriceData[] memory) {
        // get all the staker addresses
        address[] memory stakers = realms().getAllUnkickedValidators();
        LibPriceFeedStorage.NodePriceData[]
            memory allPrices = new LibPriceFeedStorage.NodePriceData[](
                stakers.length
            );
        for (uint256 i = 0; i < stakers.length; i++) {
            address staker = stakers[i];
            allPrices[i] = s().nodePrices[staker][productId];
        }
        return allPrices;
    }

    // gets the base network prices for given product(s)
    function baseNetworkPrices(
        uint256[] memory productIds
    ) external view returns (uint256[] memory) {
        uint256[] memory networkPrices = new uint256[](productIds.length);
        for (uint256 i = 0; i < productIds.length; i++) {
            networkPrices[i] = s().baseNetworkPrices[productIds[i]];
        }
        return networkPrices;
    }

    // gets the max network prices for given product(s)
    function maxNetworkPrices(
        uint256[] memory productIds
    ) external view returns (uint256[] memory) {
        uint256[] memory networkPrices = new uint256[](productIds.length);
        for (uint256 i = 0; i < productIds.length; i++) {
            networkPrices[i] = s().maxNetworkPrices[productIds[i]];
        }
        return networkPrices;
    }

    // convert a usage percent to prices for given product(s)
    function usagePercentToPrices(
        uint256 usagePercent,
        uint256[] memory productIds
    ) public view returns (uint256[] memory) {
        // If the usage is greater than 100, we need to cap it at 100.
        if (usagePercent > 100) {
            usagePercent = 100;
        }
        uint256[] memory calculatedPrices = new uint256[](productIds.length);

        for (uint256 i = 0; i < productIds.length; i++) {
            uint256 scalingFactor = (s().maxNetworkPrices[productIds[i]] -
                s().baseNetworkPrices[productIds[i]]);
            // divide by 100 since the usagePercent is an integer where 100 represents 100%.
            calculatedPrices[i] =
                s().baseNetworkPrices[productIds[i]] +
                ((scalingFactor * usagePercent) / 100);
        }
        return calculatedPrices;
    }

    // convert a usage percent to price for the given product
    // the nodes will use this to calculate their prices
    function usagePercentToPrice(
        uint256 usagePercent,
        uint256 productId
    ) public view returns (uint256) {
        uint256[] memory productIds = new uint256[](1);
        productIds[0] = productId;
        return usagePercentToPrices(usagePercent, productIds)[0];
    }

    // get the node capacity config for all products
    function getNodeCapacityConfig()
        public
        view
        returns (LibPriceFeedStorage.NodeCapacityConfig memory)
    {
        return s().nodeCapacityConfigs[0];
    }

    // get all the nodes and data needed to make a request.
    // this will be called by the SDK when it connects, and may be called
    // again before making a request
    function getNodesForRequest(
        uint256 realmId,
        uint256[] memory productIds
    )
        public
        view
        returns (
            uint256,
            uint256,
            LibPriceFeedStorage.NodeInfoAndPrices[] memory
        )
    {
        // get active validators
        address[] memory activeValidators = stakingByRealm()
            .getActiveUnkickedValidators(realmId);
        LibStakingStorage.Validator[] memory validatorStructs = stakingByRealm()
            .getValidatorsStructs(activeValidators);
        LibPriceFeedStorage.NodeInfoAndPrices[]
            memory nodeInfoAndPrices = new LibPriceFeedStorage.NodeInfoAndPrices[](
                activeValidators.length
            );
        // get prices for those nodes
        for (uint256 i = 0; i < activeValidators.length; i++) {
            nodeInfoAndPrices[i].validator = validatorStructs[i];
            nodeInfoAndPrices[i].prices = new uint256[](productIds.length);
            for (uint256 j = 0; j < productIds.length; j++) {
                nodeInfoAndPrices[i].prices[j] = s()
                .nodePrices[activeValidators[i]][productIds[j]].price;
            }
        }
        return (
            stakingByRealm().epoch(realmId).number,
            stakingByRealm().currentValidatorCountForConsensus(realmId),
            nodeInfoAndPrices
        );
    }

    /* ========== MUTATIVE FUNCTIONS ========== */
    // used by a node to set their usage percent.  the corresponding prices will be stored on chain.
    // usagePercent is a 2 digit percentage of the current node usage.  For example, 50% would be represented as 50.
    function setUsage(
        uint256 usagePercent,
        uint256[] memory productIds
    ) external {
        address stakerAddress = realms().nodeAddressToStakerAddressAcrossRealms(
            LibERC2771._msgSender()
        );
        // calculate the prices according to the curve
        uint256[] memory newPrices = new uint256[](productIds.length);

        for (uint256 i = 0; i < productIds.length; i++) {
            newPrices[i] = usagePercentToPrice(usagePercent, productIds[i]);
            s().nodePrices[stakerAddress][productIds[i]] = LibPriceFeedStorage
                .NodePriceData({
                    stakerAddress: stakerAddress,
                    price: newPrices[i],
                    productId: productIds[i],
                    timestamp: block.timestamp
                });
        }
        emit UsageSet(stakerAddress, usagePercent, newPrices);
    }

    // admin function to set base network prices for given product(s)
    function setBaseNetworkPrices(
        uint256 newPrice,
        uint256[] memory productIds
    ) external onlyOwner {
        for (uint256 i = 0; i < productIds.length; i++) {
            s().baseNetworkPrices[productIds[i]] = newPrice;
        }
        emit BaseNetworkPriceSet(newPrice);
    }

    // admin function to set max network prices for given product(s)
    function setMaxNetworkPrices(
        uint256 newPrice,
        uint256[] memory productIds
    ) external onlyOwner {
        for (uint256 i = 0; i < productIds.length; i++) {
            s().maxNetworkPrices[productIds[i]] = newPrice;
        }
        emit MaxNetworkPriceSet(newPrice);
    }

    // admin function to set node capacity config for all products
    function setNodeCapacityConfig(
        LibPriceFeedStorage.NodeCapacityConfig memory config
    ) external onlyOwner {
        s().nodeCapacityConfigs[0] = config;
    }

    function getLitActionPriceConfigs()
        external
        view
        returns (LibPriceFeedStorage.LitActionPriceConfig[] memory)
    {
        uint256 length = uint(
            type(LibPriceFeedStorage.LitActionPriceComponent).max
        ) + 1;

        LibPriceFeedStorage.LitActionPriceConfig[]
            memory configs = new LibPriceFeedStorage.LitActionPriceConfig[](
                length
            );
        for (uint256 i = 0; i < length; i++) {
            configs[i] = s().litActionPriceConfigs[
                LibPriceFeedStorage.LitActionPriceComponent(i)
            ];
        }
        return configs;
    }

    function setLitActionPriceConfig(
        LibPriceFeedStorage.LitActionPriceComponent priceComponent,
        LibPriceFeedStorage.NodePriceMeasurement priceMeasurement,
        uint256 new_price
    ) external onlyOwner {
        s().litActionPriceConfigs[priceComponent] = LibPriceFeedStorage
            .LitActionPriceConfig({
                priceComponent: priceComponent,
                priceMeasurement: priceMeasurement,
                price: new_price
            });
    }

    function setLitActionPriceConfigs(
        LibPriceFeedStorage.LitActionPriceConfig[] memory configs
    ) external onlyOwner {
        for (uint256 i = 0; i < configs.length; i++) {
            s().litActionPriceConfigs[configs[i].priceComponent] = configs[i];
        }
    }

    /* ========== EVENTS ========== */

    event BaseNetworkPriceSet(uint256 newPrice);
    event MaxNetworkPriceSet(uint256 newPrice);
    event UsageSet(
        address indexed stakingAddress,
        uint256 usagePercent,
        uint256[] newPrices
    );
}
