// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import { Test, console2 } from "lib/forge-std/src/Test.sol";
import { SetupAndUtils } from "../Staking/SetupAndUtils.t.sol";
import { LibPriceFeedStorage } from "../PriceFeed/LibPriceFeedStorage.sol";

contract PriceFeedTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    /// Test that the usage can be set correctly
    function testFuzz_setUsage(uint256 usagePercent) public {
        // Setup validators
        uint256 amount = 32 ether;
        uint256 timeLock = 90 days;
        address[] memory randomOperatorStakers = _generateAddresses(4);
        _setupValidators(
            1,
            randomOperatorStakers,
            amount * 10,
            amount,
            timeLock,
            _generateUint256s(4)
        );

        // Advance to epoch 3 to make active.
        _advanceEpochs(1, 2, randomOperatorStakers, 1);

        usagePercent = bound(usagePercent, 1, 200);

        uint256[] memory productIds = new uint256[](1);
        productIds[0] = 0;

        // Set the initial usage as 0.
        for (uint256 i = 0; i < randomOperatorStakers.length; i++) {
            vm.prank(randomOperatorStakers[i]);
            priceFeedFacet.setUsage(0, productIds);
        }

        // Get the prices.
        LibPriceFeedStorage.NodePriceData[] memory prices = priceFeedFacet
            .prices(productIds[0]);

        // Set the usage to be higher only for the first product, by the first node.
        vm.prank(randomOperatorStakers[0]);
        priceFeedFacet.setUsage(usagePercent, productIds);

        // Get the new prices.
        LibPriceFeedStorage.NodePriceData[] memory newPrices = priceFeedFacet
            .prices(productIds[0]);

        // Compare prices - for the first product, the price should be higher.
        assertGt(newPrices[0].price, prices[0].price);
        assertEq(newPrices[1].price, prices[1].price);
        assertEq(newPrices[2].price, prices[2].price);
    }
}
