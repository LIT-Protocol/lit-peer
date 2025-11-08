// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import { Test, console2 } from "lib/forge-std/src/Test.sol";
import { StakingFacet } from "../Staking/StakingFacet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { StakingAdminFacet } from "../Staking/StakingAdminFacet.sol";
import { StakingValidatorFacet } from "../Staking/StakingValidatorFacet.sol";
import { StakingAcrossRealmsFacet } from "../Staking/StakingAcrossRealmsFacet.sol";
import { StakingVersionFacet } from "../Staking/StakingVersionFacet.sol";
import { StakingUtilsLib } from "../Staking/StakingUtilsLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { Staking, StakingArgs } from "../Staking.sol";
import { DiamondInit } from "../../upgradeInitializers/DiamondInit.sol";
import { IDiamond } from "../../interfaces/IDiamond.sol";
import { FunctionSelectorHelper, PriceFeedFunctionSelectorHelper } from "./FunctionSelectorHelper.t.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { LITToken } from "../LITToken.sol";
import { FixedPointMathLib } from "solady/src/utils/FixedPointMathLib.sol";
import { PriceFeedFacet } from "../PriceFeed/PriceFeedFacet.sol";
import { PriceFeed, PriceFeedArgs } from "../PriceFeed.sol";
import { console } from "lib/forge-std/src/console.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

abstract contract SetupAndUtils is Test {
    Staking staking;
    StakingFacet stakingFacet;
    StakingViewsFacet stakingViewsFacet;
    StakingAdminFacet stakingAdminFacet;
    StakingValidatorFacet stakingValidatorFacet;
    StakingAcrossRealmsFacet stakingAcrossRealmsFacet;
    StakingVersionFacet stakingVersionFacet;
    LITToken token;
    FunctionSelectorHelper functionSelectorHelper;

    PriceFeed priceFeed;
    PriceFeedFacet priceFeedFacet;

    PriceFeedFunctionSelectorHelper priceFeedFunctionSelectorHelper;

    uint256 rewardsBudget = 200_000_000 ether;
    ContractResolver contractResolver;
    event StakeRecordCreated(
        address stakerAddress,
        uint256 recordId,
        uint256 amount,
        address stakerAddressClient
    );
    event RequestToJoin(uint256 indexed validatorId);
    event StakeRewardsClaimed(
        uint256 validatorId,
        uint256 recordId,
        uint256 rewards
    );

    // Dynamic arrays to store reward epoch numbers for various assertions.
    // Each index corresponds to the reward epoch number that is previous to
    // the next index number. We need to do this since, with multiple realms,
    // the reward epoch numbers may not always be sequential.
    uint[] rewardEpochAssertionsRealm1;
    uint[] rewardEpochTotalRewardAssertionsRealm1;
    uint[] rewardEpochGlobalStatsAssertionsRealm1;
    uint[] rewardEpochAssertionsRealm2;
    uint[] rewardEpochTotalRewardAssertionsRealm2;
    uint[] rewardEpochGlobalStatsAssertionsRealm2;

    function baseSetup() internal {
        skip(1 days);

        DiamondInit diamondInit = new DiamondInit();
        contractResolver = new ContractResolver(ContractResolver.Env.Dev);
        stakingFacet = new StakingFacet();
        stakingViewsFacet = new StakingViewsFacet();
        stakingAdminFacet = new StakingAdminFacet();
        stakingValidatorFacet = new StakingValidatorFacet();
        stakingAcrossRealmsFacet = new StakingAcrossRealmsFacet();
        stakingVersionFacet = new StakingVersionFacet();

        priceFeedFacet = new PriceFeedFacet();

        functionSelectorHelper = new FunctionSelectorHelper();
        priceFeedFunctionSelectorHelper = new PriceFeedFunctionSelectorHelper();
        token = new LITToken(1_000_000_000 ether);
        // Mint 1_000_000_000 ether to this deployer to initialize totalSupply to that value.
        token.mint(address(this), 1_000_000_000 ether);

        // Cut the staking diamond.
        IDiamond.FacetCut[] memory facetCuts = new IDiamond.FacetCut[](6);
        facetCuts[0] = IDiamond.FacetCut({
            facetAddress: address(stakingFacet),
            action: IDiamond.FacetCutAction.Add,
            functionSelectors: functionSelectorHelper.getSelectorsStakingFacet()
        });
        facetCuts[1] = IDiamond.FacetCut({
            facetAddress: address(stakingViewsFacet),
            action: IDiamond.FacetCutAction.Add,
            functionSelectors: functionSelectorHelper
                .getSelectorsStakingViewsFacet()
        });
        facetCuts[2] = IDiamond.FacetCut({
            facetAddress: address(stakingAdminFacet),
            action: IDiamond.FacetCutAction.Add,
            functionSelectors: functionSelectorHelper
                .getSelectorsStakingAdminFacet()
        });
        facetCuts[3] = IDiamond.FacetCut({
            facetAddress: address(stakingValidatorFacet),
            action: IDiamond.FacetCutAction.Add,
            functionSelectors: functionSelectorHelper
                .getSelectorsStakingValidatorFacet()
        });
        facetCuts[4] = IDiamond.FacetCut({
            facetAddress: address(stakingAcrossRealmsFacet),
            action: IDiamond.FacetCutAction.Add,
            functionSelectors: functionSelectorHelper
                .getSelectorsStakingAcrossRealmsFacet()
        });
        facetCuts[5] = IDiamond.FacetCut({
            facetAddress: address(stakingVersionFacet),
            action: IDiamond.FacetCutAction.Add,
            functionSelectors: functionSelectorHelper
                .getSelectorsStakingVersionFacet()
        });

        StakingArgs memory stakingArgs = StakingArgs({
            owner: address(this),
            init: address(diamondInit),
            initCalldata: abi.encodeWithSignature("init()"),
            contractResolver: address(contractResolver),
            env: ContractResolver.Env.Dev
        });

        staking = new Staking(facetCuts, stakingArgs);
        stakingFacet = StakingFacet(address(staking));
        stakingValidatorFacet = StakingValidatorFacet(address(staking));
        stakingAcrossRealmsFacet = StakingAcrossRealmsFacet(address(staking));
        stakingViewsFacet = StakingViewsFacet(address(staking));
        stakingAdminFacet = StakingAdminFacet(address(staking));
        stakingVersionFacet = StakingVersionFacet(address(staking));

        // Cut the price feed diamond.
        IDiamond.FacetCut[] memory priceFeedFacetCuts = new IDiamond.FacetCut[](
            1
        );
        priceFeedFacetCuts[0] = IDiamond.FacetCut({
            facetAddress: address(priceFeedFacet),
            action: IDiamond.FacetCutAction.Add,
            functionSelectors: priceFeedFunctionSelectorHelper
                .getSelectorsPriceFeedFacet()
        });

        PriceFeedArgs memory priceFeedArgs = PriceFeedArgs({
            owner: address(this),
            init: address(diamondInit),
            initCalldata: abi.encodeWithSignature("init()"),
            contractResolver: address(contractResolver),
            env: ContractResolver.Env.Dev
        });

        priceFeed = new PriceFeed(priceFeedFacetCuts, priceFeedArgs);
        priceFeedFacet = PriceFeedFacet(address(priceFeed));

        // Set contract resolver
        contractResolver.setContract(
            contractResolver.LIT_TOKEN_CONTRACT(),
            ContractResolver.Env.Dev,
            address(token)
        );
        contractResolver.setContract(
            contractResolver.STAKING_CONTRACT(),
            ContractResolver.Env.Dev,
            address(staking)
        );
        contractResolver.setContract(
            contractResolver.PRICE_FEED_CONTRACT(),
            ContractResolver.Env.Dev,
            address(priceFeed)
        );

        // Add the first realm
        stakingAdminFacet.addRealm();

        // Set the epoch length to 1 hour
        stakingAdminFacet.setEpochLength(1, 1 hours);

        // Transfer rewards budget to the owner (and then to the staking contract)
        token.transfer(address(this), rewardsBudget);
        token.approve(address(staking), type(uint).max);
        stakingAdminFacet.increaseRewardPool(1, rewardsBudget);
    }

    /* ======================== UTILITY FUNCTIONS ======================== */

    struct TestValidator {
        address stakerAddress;
        uint256 commsKey;
    }

    function _generateValidators(
        uint256 numValidators
    ) public pure returns (TestValidator[] memory) {
        TestValidator[] memory validators = new TestValidator[](numValidators);

        address[] memory addresses = _generateAddresses(numValidators);
        uint256[] memory commsKeys = _generateUint256s(numValidators);

        for (uint256 i = 0; i < numValidators; i++) {
            validators[i] = TestValidator({
                stakerAddress: addresses[i],
                commsKey: commsKeys[i]
            });
        }
        return validators;
    }

    function _generatePubKeys(
        uint256 stakers
    ) public pure returns (bytes[] memory) {
        bytes[] memory pubKeys = new bytes[](stakers);
        for (uint256 i = 0; i < stakers; i++) {
            bytes memory pubKey = new bytes(65);

            pubKey[0] = 0x04;
            for (uint256 j = 1; j <= 32; j++) {
                pubKey[j] = bytes1(uint8((i * j) % 256));
            }

            for (uint256 j = 33; j < 65; j++) {
                pubKey[j] = bytes1(uint8((i * j + 128) % 256));
            }

            pubKeys[i] = pubKey;
        }
        return pubKeys;
    }

    function _generateAddresses(
        uint256 numAddresses
    ) public pure returns (address[] memory) {
        address[] memory addresses = new address[](numAddresses);
        for (uint256 i = 0; i < numAddresses; i++) {
            addresses[i] = address(uint160(i + 1)); // skip the 0 address
        }
        return addresses;
    }

    function _generateAddressesWithOffset(
        uint256 numAddresses,
        uint256 offset
    ) public pure returns (address[] memory) {
        address[] memory addresses = _generateAddresses(numAddresses);
        for (uint256 i = 0; i < numAddresses; i++) {
            addresses[i] = address(uint160(i + offset + 1));
        }
        return addresses;
    }

    function _generateUint256s(
        uint256 numUint256s
    ) public pure returns (uint256[] memory) {
        uint256[] memory uint256s = new uint256[](numUint256s);
        for (uint256 i = 0; i < numUint256s; i++) {
            uint256s[i] = i + 1;
        }
        return uint256s;
    }

    function _generateUint256sWithOffset(
        uint256 numUint256s,
        uint256 offset
    ) public pure returns (uint256[] memory) {
        uint256[] memory uint256s = _generateUint256s(numUint256s);
        for (uint256 i = 0; i < numUint256s; i++) {
            uint256s[i] = i + offset + 1;
        }
        return uint256s;
    }

    function _callPythonCalcRewards(
        uint256 p,
        uint256 k,
        uint256 b_min,
        uint256 b_max,
        uint256 circ,
        uint256 stake_weight,
        uint256 stake_amount
    ) internal returns (uint256) {
        // Ensure dependencies are installed
        string[] memory installCmd = new string[](3);
        installCmd[0] = "pip3";
        installCmd[1] = "install";
        installCmd[2] = "eth_abi";
        vm.ffi(installCmd);

        // Call python script to calculate rewards
        string[] memory calcRewardCmd = new string[](9);
        calcRewardCmd[0] = "python3";
        calcRewardCmd[1] = "contracts/lit-node/Staking/rewards.py";
        calcRewardCmd[2] = Strings.toString(p);
        calcRewardCmd[3] = Strings.toString(k);
        calcRewardCmd[4] = Strings.toString(b_min);
        calcRewardCmd[5] = Strings.toString(b_max);
        calcRewardCmd[6] = Strings.toString(circ);
        calcRewardCmd[7] = Strings.toString(stake_weight);
        calcRewardCmd[8] = Strings.toString(stake_amount);

        console.log("---");
        for (uint i = 0; i < calcRewardCmd.length; i++) {
            console.log(calcRewardCmd[i], "\\");
        }
        console.log("---");

        return abi.decode(vm.ffi(calcRewardCmd), (uint256));
    }

    function _fundAddressesWithTokensAndApprove(
        address[] memory addresses,
        uint256 perAddressAmount
    ) internal {
        for (uint256 i = 0; i < addresses.length; i++) {
            _fundAddressWithTokensAndApprove(addresses[i], perAddressAmount);
        }
    }

    function _fundAddressWithTokensAndApprove(
        address addressToFund,
        uint256 amount
    ) internal {
        token.transfer(addressToFund, amount);
        vm.prank(addressToFund);
        token.approve(address(staking), amount);
    }

    function _setupValidatorsStakeOnly(
        address[] memory operatorStakers,
        uint256 perOperatorAmountToTransferAndApprove,
        uint256 perOperatorAmountToStake,
        uint256 timeLock
    ) internal {
        // Have all validators stake
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            _fundAddressWithTokensAndApprove(
                operatorStakers[i],
                perOperatorAmountToTransferAndApprove
            );

            // Emit event check
            vm.expectEmit(true, true, true, true);
            emit StakeRecordCreated(
                operatorStakers[i],
                1,
                perOperatorAmountToStake,
                operatorStakers[i]
            );

            // Stake
            vm.prank(operatorStakers[i]);
            stakingFacet.stake(
                perOperatorAmountToStake,
                timeLock,
                operatorStakers[i]
            );
        }
    }

    function _setupValidators(
        uint256 realmId,
        address[] memory operatorStakers,
        uint256 perOperatorAmountToTransferAndApprove,
        uint256 perOperatorAmountToStake,
        uint256 timeLock,
        uint256[] memory operatorCommsKeys
    ) internal {
        // Have all validators stake and join
        for (uint256 i = 0; i < operatorStakers.length; i++) {
            _fundAddressWithTokensAndApprove(
                operatorStakers[i],
                perOperatorAmountToTransferAndApprove
            );

            // Set the IP and port of the validator
            vm.prank(operatorStakers[i]);
            stakingValidatorFacet.setIpPortNodeAddress(
                1,
                1,
                1,
                operatorStakers[i]
            );

            // Emit event check
            vm.expectEmit(true, true, true, true);
            emit StakeRecordCreated(
                operatorStakers[i],
                1,
                perOperatorAmountToStake,
                operatorStakers[i]
            );

            // Stake and join
            vm.prank(operatorStakers[i]);
            stakingFacet.stake(
                perOperatorAmountToStake,
                timeLock,
                operatorStakers[i]
            );
            vm.prank(operatorStakers[i]);
            stakingValidatorFacet.requestToJoin(realmId);
        }

        stakingAdminFacet.setEpochState(
            realmId,
            LibStakingStorage.States.Active
        );
    }

    function _setupUserStakers(
        address[] memory userStakers,
        uint256 perUserAmountToTransferAndApprove
    ) internal {
        for (uint256 i = 0; i < userStakers.length; i++) {
            token.transfer(userStakers[i], perUserAmountToTransferAndApprove);
            vm.prank(userStakers[i]);
            token.approve(address(staking), perUserAmountToTransferAndApprove);
        }
    }

    function _advanceEpochs(
        uint256 realmId,
        uint256 numEpochsToAdvance,
        address[] memory validators,
        uint256 startingEpochNumber
    ) internal returns (uint256) {
        for (
            uint256 i = startingEpochNumber;
            i < startingEpochNumber + numEpochsToAdvance;
            i++
        ) {
            skip(1 hours);
            stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);
            for (uint256 j = 0; j < validators.length; j++) {
                if (validators[j] == address(0)) {
                    continue;
                }

                vm.prank(validators[j]);
                stakingValidatorFacet.signalReadyForNextEpoch(realmId, i);
            }

            stakingValidatorFacet.advanceEpoch(realmId);
        }

        return startingEpochNumber + numEpochsToAdvance;
    }

    function _advanceEpochsCustomEpochLength(
        uint256 realmId,
        uint256 numEpochsToAdvance,
        address[] memory validators,
        uint256 startingEpochNumber,
        uint256 epochLength
    ) internal returns (uint256) {
        for (
            uint256 i = startingEpochNumber;
            i < startingEpochNumber + numEpochsToAdvance;
            i++
        ) {
            skip(epochLength);
            stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);
            for (uint256 j = 0; j < validators.length; j++) {
                if (validators[j] == address(0)) {
                    continue;
                }

                vm.prank(validators[j]);
                stakingValidatorFacet.signalReadyForNextEpoch(realmId, i);
            }

            stakingValidatorFacet.advanceEpoch(realmId);
        }

        return startingEpochNumber + numEpochsToAdvance;
    }

    function _advanceEpochsAndAssertConstantStakeWeight(
        uint256 realmId,
        uint256 numEpochsToAdvance,
        address[] memory validators,
        uint256 startingEpochNumber
    ) internal returns (uint256) {
        LibStakingStorage.RewardEpoch[]
            memory oldRewardEpochs = new LibStakingStorage.RewardEpoch[](
                validators.length
            );
        LibStakingStorage.RewardEpochGlobalStats
            memory oldRewardEpochGlobalStats;

        for (
            uint256 i = startingEpochNumber;
            i < startingEpochNumber + numEpochsToAdvance;
            i++
        ) {
            // Get references to the current reward epoch and global stats.
            uint256 currentRewardEpochNumber = stakingViewsFacet
                .getRewardEpochNumber(realmId);

            for (uint256 j = 0; j < validators.length; j++) {
                oldRewardEpochs[j] = stakingFacet.getRewardEpoch(
                    validators[j],
                    currentRewardEpochNumber
                );
            }
            oldRewardEpochGlobalStats = stakingViewsFacet
                .getRewardEpochGlobalStats(currentRewardEpochNumber);

            skip(1 hours);
            stakingValidatorFacet.lockValidatorsForNextEpoch(realmId);
            for (uint256 j = 0; j < validators.length; j++) {
                if (validators[j] == address(0)) {
                    continue;
                }

                vm.prank(validators[j]);
                stakingValidatorFacet.signalReadyForNextEpoch(realmId, i);
            }

            stakingValidatorFacet.advanceEpoch(realmId);

            // After each epoch advancement, assert that the stake weight is constant.
            currentRewardEpochNumber = stakingViewsFacet.getRewardEpochNumber(
                realmId
            );
            for (uint256 j = 0; j < validators.length; j++) {
                LibStakingStorage.RewardEpoch
                    memory newRewardEpochs = stakingFacet.getRewardEpoch(
                        validators[j],
                        currentRewardEpochNumber
                    );

                assertEq(
                    newRewardEpochs.totalStakeWeight,
                    oldRewardEpochs[j].totalStakeWeight,
                    "Reward epoch stake weight should remain constant"
                );
            }

            LibStakingStorage.RewardEpochGlobalStats
                memory newRewardEpochGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(currentRewardEpochNumber);

            assertEq(
                newRewardEpochGlobalStats.stakeWeight,
                oldRewardEpochGlobalStats.stakeWeight,
                "Global stats stake weight should remain constant"
            );
        }

        return startingEpochNumber + numEpochsToAdvance;
    }

    function _assertNewRewardEpoch(
        LibStakingStorage.RewardEpoch memory rewardEpoch,
        uint256 amount
    ) internal pure {
        assertEq(rewardEpoch.validatorSharePrice, 1 ether);
        assertEq(rewardEpoch.validatorSharePriceAtLastUpdate, 1 ether);
        assertEq(rewardEpoch.stakeAmount, amount);
        assertEq(rewardEpoch.initial, true);
    }

    function _assertRewardEpochIsZero(
        LibStakingStorage.RewardEpoch memory rewardEpoch
    ) internal pure {
        assertEq(rewardEpoch.totalStakeWeight, 0);
        assertEq(rewardEpoch.totalStakeRewards, 0);
        assertEq(rewardEpoch.slope, 0);
        assertEq(rewardEpoch.validatorSharePrice, 1 ether);
        assertEq(rewardEpoch.validatorSharePriceAtLastUpdate, 1 ether);
        assertEq(rewardEpoch.stakeAmount, 0);
        assertEq(rewardEpoch.initial, false);
    }

    function _assertStakeRecordIsZero(
        LibStakingStorage.StakeRecord memory stakeRecord
    ) internal pure {
        assertEq(stakeRecord.amount, 0);
        assertEq(stakeRecord.unfreezeStart, 0);
        assertEq(stakeRecord.timeLock, 0);
        assertEq(stakeRecord.lastUpdateTimestamp, 0);
        assertEq(stakeRecord.lastRewardEpochClaimed, 0);
        assertEq(stakeRecord.loaded, false);
        assertEq(stakeRecord.frozen, false);
        assertEq(stakeRecord.initialSharePrice, 0);
    }

    function _assertOldNewRewardEpochsConstant(
        uint[] memory rewardEpochNumbersToAssert,
        address[] memory stakers
    ) internal {
        for (uint i = 0; i < stakers.length; i++) {
            for (uint j = 0; j < rewardEpochNumbersToAssert.length - 1; j++) {
                vm.prank(address(staking));
                LibStakingStorage.RewardEpoch
                    memory oldRewardEpoch = stakingFacet.getRewardEpoch(
                        stakers[i],
                        rewardEpochNumbersToAssert[j]
                    );
                vm.prank(address(staking));
                LibStakingStorage.RewardEpoch
                    memory newRewardEpoch = stakingFacet.getRewardEpoch(
                        stakers[i],
                        rewardEpochNumbersToAssert[j + 1]
                    );
                assertEq(
                    newRewardEpoch.totalStakeWeight,
                    oldRewardEpoch.totalStakeWeight,
                    "Stake weight in epoch should remain constant"
                );
                assertEq(
                    newRewardEpoch.slope,
                    oldRewardEpoch.slope,
                    "Slope should remain constant"
                );
                assertEq(
                    newRewardEpoch.stakeAmount,
                    oldRewardEpoch.stakeAmount,
                    "Stake amount should remain constant"
                );
                assertNotEq(
                    newRewardEpoch.stakeAmount,
                    0,
                    "Stake amount should not be 0"
                );
            }
        }
    }

    function _assertOldNewRewardEpochsConstantForStaker(
        LibStakingStorage.RewardEpochGlobalStats memory oldGlobalStats,
        LibStakingStorage.RewardEpoch memory oldRewardEpoch,
        uint256 rewardEpochNumber,
        address stakerAddress,
        bool approximateEquality
    ) internal {
        LibStakingStorage.RewardEpoch memory newRewardEpoch = stakingFacet
            .getRewardEpoch(stakerAddress, rewardEpochNumber);
        LibStakingStorage.RewardEpochGlobalStats
            memory newGlobalStats = stakingViewsFacet.getRewardEpochGlobalStats(
                rewardEpochNumber
            );
        if (approximateEquality) {
            assertApproxEqAbs(
                newGlobalStats.stakeAmount,
                oldGlobalStats.stakeAmount,
                5,
                "Total stake amount in epoch should remain constant"
            );
            assertApproxEqAbs(
                newGlobalStats.stakeWeight,
                oldGlobalStats.stakeWeight,
                5,
                "Stake weight in epoch should remain constant"
            );
            assertApproxEqAbs(
                newRewardEpoch.stakeAmount,
                oldRewardEpoch.stakeAmount,
                5,
                "Stake amount in epoch should remain constant"
            );
            assertApproxEqAbs(
                newRewardEpoch.totalStakeWeight,
                oldRewardEpoch.totalStakeWeight,
                5,
                "Total stake weight in epoch should remain constant"
            );
        } else {
            assertEq(
                newGlobalStats.stakeAmount,
                oldGlobalStats.stakeAmount,
                "Total stake amount in epoch should remain constant"
            );
            assertEq(
                newGlobalStats.stakeWeight,
                oldGlobalStats.stakeWeight,
                "Stake weight in epoch should remain constant"
            );
            assertEq(
                newRewardEpoch.stakeAmount,
                oldRewardEpoch.stakeAmount,
                "Stake amount in epoch should remain constant"
            );
            assertEq(
                newRewardEpoch.totalStakeWeight,
                oldRewardEpoch.totalStakeWeight,
                "Total stake weight in epoch should remain constant"
            );
        }
    }

    function _assertOldNewRewardEpochTotalRewardsConstant(
        uint[] memory rewardEpochNumbersToAssert,
        address[] memory stakers
    ) internal {
        for (uint i = 0; i < stakers.length; i++) {
            for (uint j = 0; j < rewardEpochNumbersToAssert.length - 1; j++) {
                vm.prank(address(staking));
                LibStakingStorage.RewardEpoch
                    memory oldRewardEpoch = stakingFacet.getRewardEpoch(
                        stakers[i],
                        rewardEpochNumbersToAssert[j]
                    );
                vm.prank(address(staking));
                LibStakingStorage.RewardEpoch
                    memory newRewardEpoch = stakingFacet.getRewardEpoch(
                        stakers[i],
                        rewardEpochNumbersToAssert[j + 1]
                    );
                assertEq(
                    newRewardEpoch.totalStakeRewards,
                    oldRewardEpoch.totalStakeRewards,
                    "Total rewards should remain constant"
                );
                assertNotEq(
                    newRewardEpoch.totalStakeRewards,
                    0,
                    "Total rewards should not be 0"
                );
            }
        }
    }

    function _assertOldNewRewardEpochGlobalStatsConstant(
        uint[] memory rewardEpochNumbersToAssert
    ) internal {
        for (uint i = 0; i < rewardEpochNumbersToAssert.length - 1; i++) {
            vm.prank(address(staking));
            LibStakingStorage.RewardEpochGlobalStats
                memory oldGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(rewardEpochNumbersToAssert[i]);
            vm.prank(address(staking));
            LibStakingStorage.RewardEpochGlobalStats
                memory newGlobalStats = stakingViewsFacet
                    .getRewardEpochGlobalStats(
                        rewardEpochNumbersToAssert[i + 1]
                    );
            assertEq(
                newGlobalStats.stakeWeight,
                oldGlobalStats.stakeWeight,
                "Stake weight in epoch should remain constant"
            );
            assertEq(
                newGlobalStats.stakeAmount,
                oldGlobalStats.stakeAmount,
                "Stake amount in epoch should remain constant"
            );
        }
    }

    struct OldNewForRewardEpoch {
        LibStakingStorage.RewardEpochGlobalStats oldGlobalStats;
        LibStakingStorage.RewardEpoch oldRewardEpoch;
        address stakerAddress;
        uint256 newRewardEpochNumber;
    }

    function _assertOldNewRewardEpochsAndGlobalStatsIncreased(
        OldNewForRewardEpoch memory oldNewForRewardEpoch,
        uint256 additionalStakeAmount,
        uint256 additionalStakeWeight,
        bool approximateStakeWeightEquality
    ) internal {
        if (approximateStakeWeightEquality) {
            assertApproxEqAbs(
                stakingFacet
                    .getRewardEpoch(
                        oldNewForRewardEpoch.stakerAddress,
                        oldNewForRewardEpoch.newRewardEpochNumber
                    )
                    .totalStakeWeight,
                oldNewForRewardEpoch.oldRewardEpoch.totalStakeWeight +
                    additionalStakeWeight,
                100,
                "Reward epoch total stake weight should have increased"
            );
        } else {
            assertEq(
                stakingFacet
                    .getRewardEpoch(
                        oldNewForRewardEpoch.stakerAddress,
                        oldNewForRewardEpoch.newRewardEpochNumber
                    )
                    .totalStakeWeight,
                oldNewForRewardEpoch.oldRewardEpoch.totalStakeWeight +
                    additionalStakeWeight,
                "Reward epoch total stake weight should have increased"
            );
        }
        assertEq(
            stakingFacet
                .getRewardEpoch(
                    oldNewForRewardEpoch.stakerAddress,
                    oldNewForRewardEpoch.newRewardEpochNumber
                )
                .stakeAmount,
            oldNewForRewardEpoch.oldRewardEpoch.stakeAmount +
                additionalStakeAmount,
            "Reward epoch stake amount should have increased"
        );
        if (approximateStakeWeightEquality) {
            assertApproxEqAbs(
                stakingViewsFacet
                    .getRewardEpochGlobalStats(
                        oldNewForRewardEpoch.newRewardEpochNumber
                    )
                    .stakeWeight,
                oldNewForRewardEpoch.oldGlobalStats.stakeWeight +
                    additionalStakeWeight,
                100,
                "Global stats stake weight should have increased"
            );
        } else {
            assertEq(
                stakingViewsFacet
                    .getRewardEpochGlobalStats(
                        oldNewForRewardEpoch.newRewardEpochNumber
                    )
                    .stakeWeight,
                oldNewForRewardEpoch.oldGlobalStats.stakeWeight +
                    additionalStakeWeight,
                "Global stats stake weight should have increased"
            );
        }
        assertEq(
            stakingViewsFacet
                .getRewardEpochGlobalStats(
                    oldNewForRewardEpoch.newRewardEpochNumber
                )
                .stakeAmount,
            oldNewForRewardEpoch.oldGlobalStats.stakeAmount +
                additionalStakeAmount,
            "Global stats stake amount should have increased"
        );
    }

    function _logRewardEpoch(
        uint256 rewardEpochNumber,
        address[] memory stakers
    ) internal {
        _logRewardEpochGlobalStats(rewardEpochNumber);

        for (uint256 j = 0; j < stakers.length; j++) {
            _logRewardEpoch(stakers[j], rewardEpochNumber);
        }
    }

    function _logRewardEpoch(
        address stakerAddress,
        uint256 rewardEpochNumber
    ) internal {
        LibStakingStorage.RewardEpoch memory rewardEpoch = stakingFacet
            .getRewardEpoch(stakerAddress, rewardEpochNumber);

        console.log(
            "\n==== REWARD EPOCH (%s) FOR STAKER %s ====",
            rewardEpochNumber,
            stakerAddress
        );
        console.log("Epoch end: %s", rewardEpoch.epochEnd);
        console.log("Total stake weight: %s", rewardEpoch.totalStakeWeight);
        console.log("Total rewards: %s", rewardEpoch.totalStakeRewards);
        console.log("Slope: %s", rewardEpoch.slope);
        console.log("\n================================\n");
    }

    function _logRewardEpochGlobalStats(
        uint256 rewardEpochNumber
    ) internal view {
        LibStakingStorage.RewardEpochGlobalStats
            memory globalStats = stakingViewsFacet.getRewardEpochGlobalStats(
                rewardEpochNumber
            );

        console.log(
            "\n==== REWARD EPOCH GLOBAL STATS (%s) ====",
            rewardEpochNumber
        );

        console.log("Total stake weight: %s", globalStats.stakeWeight);
        console.log("Total stake amount: %s", globalStats.stakeAmount);
        console.log("\n================================\n");
    }
}
