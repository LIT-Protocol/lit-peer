/// NOTE TO DEVELOPERS OF INVARIANT TESTS: IF IT IS YOUR FIRST TIME WRITING FORGE INVARIANT TESTS,
/// PLEASE READ THE REFERENCE DOCUMENTATION FIRST AS WELL AS THE COMMENTS IN THIS PROJECT. THEY WILL
/// HELP YOU UNDERSTAND WHY WE MUST BE RATHER PARTICULAR ABOUT THE TEST IMPLEMENTATION IN ORDER TO
/// HAVE MEANINGFUL INVARIANT TESTS.
///
/// FOR EXAMPLE, IF THE NUMBER OF REVERTS == THE NUMBER OF CALLS MADE TO A FUNCTION, THEN YOU'RE
/// PROBABLY DOING SOMETHING WRONG.

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import { Test, console, console2 } from "lib/forge-std/src/Test.sol";
import { StakingFacet } from "../Staking/StakingFacet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { StakingAdminFacet } from "../Staking/StakingAdminFacet.sol";
import { StakingValidatorFacet } from "../Staking/StakingValidatorFacet.sol";
import { StakingAcrossRealmsFacet } from "../Staking/StakingAcrossRealmsFacet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { Staking, StakingArgs } from "../Staking.sol";
import { DiamondInit } from "../../upgradeInitializers/DiamondInit.sol";
import { IDiamond } from "../../interfaces/IDiamond.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { FunctionSelectorHelper } from "./FunctionSelectorHelper.t.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { LITToken } from "../LITToken.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { SetupAndUtils } from "./SetupAndUtils.t.sol";

contract StakingHandler is Test {
    using EnumerableSet for EnumerableSet.UintSet;
    using EnumerableSet for EnumerableSet.AddressSet;

    address private stakingDiamondAddress;
    uint256 public totalStaked;
    uint256 public unfreezeCount;
    address private owner;
    LITToken private token;

    /// @notice This is an array of wallets funded with the token. Initially, each wallet is at
    /// least one of the following and through each fuzzing campaign run each wallet will likely
    /// become a combination of the following (eg. a validator can also be delegating stakes to
    /// other validators):
    ///   - operator stakers who are staked and joined to the realm
    ///   - operator stakers who are staked and not joined to the realm
    ///   - operator stakers who are not staked
    ///   - user stakers who have delegated stakes to operator stakers
    ///   - user stakers who have not delegated stakes to operator stakers
    address[] wallets;
    mapping(address => uint256) private walletToCommsKeys;

    /// Ghost variables for assertions and tracking.
    /// https://book.getfoundry.sh/forge/invariant-testing#handler-ghost-variables

    EnumerableSet.UintSet private realmIds;
    uint256 private currentRealmId;
    uint256 private currentEpochNumber;
    address private currentWallet;
    uint256 private currentWalletCommsKeys;
    /// @dev This is helpful to track the operator stakers that a staker is currently staked against.
    mapping(address => EnumerableSet.AddressSet) private stakerToOperatorStaker;
    /// @dev This is helpful to track each successive reward epoch number that each realm uses.
    mapping(uint256 => uint256[]) private realmIdToRewardEpochNumberHistory;

    /// Testing configuration.
    TestingConfig private testingConfig;

    struct TestingConfig {
        uint256 maxRealms;
        uint256 initialStakeAndJoinedValidators;
        uint256 maxStakeTimeLock;
        uint256 epochLength;
    }

    constructor(
        address _stakingDiamondAddress,
        address _owner,
        LITToken _token,
        address[] memory _wallets,
        uint256[] memory _commsKeys,
        TestingConfig memory _testingConfig
    ) {
        stakingDiamondAddress = _stakingDiamondAddress;
        owner = _owner;
        token = _token;
        wallets = _wallets;

        for (uint256 i = 0; i < wallets.length; i++) {
            walletToCommsKeys[wallets[i]] = _commsKeys[i];
        }

        realmIds.add(1);
        realmIds.add(2);

        realmIdToRewardEpochNumberHistory[1] = new uint256[](0);
        realmIdToRewardEpochNumberHistory[1].push(1);
        realmIdToRewardEpochNumberHistory[2] = new uint256[](0);
        realmIdToRewardEpochNumberHistory[2].push(3);

        // Set up the internal state for tracking.
        for (
            uint256 i = 0;
            i < _testingConfig.initialStakeAndJoinedValidators;
            i++
        ) {
            address operatorStaker = wallets[i];
            stakerToOperatorStaker[operatorStaker].add(operatorStaker);
        }

        // Set up the testing config.
        testingConfig = _testingConfig;
    }

    function getRewardEpochNumberHistory(
        uint256 realmId
    ) public view returns (uint256[] memory) {
        return realmIdToRewardEpochNumberHistory[realmId];
    }

    function getAllRealmIds() public view returns (uint256[] memory) {
        return realmIds.values();
    }

    function getAllWallets() public view returns (address[] memory) {
        return wallets;
    }

    /// @notice Gets all wallets that have at least one stake record.
    function getAllWalletsWithStake() public view returns (address[] memory) {
        address[] memory walletsWithStake = new address[](wallets.length);
        uint256 index = 0;
        for (uint256 i = 0; i < wallets.length; i++) {
            if (stakerToOperatorStaker[wallets[i]].length() > 0) {
                walletsWithStake[index] = wallets[i];
                index++;
            }
        }

        // Remove empty slots - this should be everything after the index.
        uint256 actualPopulatedLength = index;
        address[] memory actualWalletsWithStake = new address[](
            actualPopulatedLength
        );
        for (uint256 i = 0; i < actualPopulatedLength; i++) {
            actualWalletsWithStake[i] = walletsWithStake[i];
        }
        return actualWalletsWithStake;
    }

    // =============== MODIFIERS ===============

    /// @notice Use a random, valid realm ID which will update the current realm ID and epoch number.
    modifier useRealmId(uint256 realmIdIndex) {
        realmIdIndex = bound(realmIdIndex, 0, realmIds.length() - 1);
        uint256 realmId = realmIds.at(realmIdIndex);
        currentRealmId = realmId;
        currentEpochNumber = StakingViewsFacet(stakingDiamondAddress)
            .epoch(currentRealmId)
            .number;
        _;
    }

    modifier useAndPersistRandomWallet(uint256 walletIndex) {
        walletIndex = bound(walletIndex, 0, wallets.length - 1);
        currentWallet = wallets[walletIndex];
        currentWalletCommsKeys = walletToCommsKeys[currentWallet];
        vm.startPrank(currentWallet);
        _;
        vm.stopPrank();
    }

    modifier useAndPersistWalletWithStake(uint256 walletIndex) {
        address[] memory walletsWithStake = getAllWalletsWithStake();
        walletIndex = bound(walletIndex, 0, walletsWithStake.length - 1);
        currentWallet = walletsWithStake[walletIndex];
        currentWalletCommsKeys = walletToCommsKeys[currentWallet];
        vm.startPrank(currentWallet);
        _;
        vm.stopPrank();
    }

    modifier useRandomCurrentValidator(uint256 validatorIndex) {
        // Fetch the number of active validators for the realm
        uint256 numActiveValidators = StakingViewsFacet(stakingDiamondAddress)
            .getValidatorsInCurrentEpoch(currentRealmId)
            .length;

        if (numActiveValidators == 0) {
            revert("No active validators");
        }

        // Use a random validator
        validatorIndex = bound(validatorIndex, 0, numActiveValidators - 1);
        address validatorAddress = StakingViewsFacet(stakingDiamondAddress)
            .getValidatorsInCurrentEpoch(currentRealmId)[validatorIndex];

        vm.startPrank(validatorAddress);
        _;
        vm.stopPrank();
    }

    modifier useRandomNextValidator(uint256 validatorIndex) {
        // Fetch the number of active validators for the realm
        uint256 numNextValidators = StakingViewsFacet(stakingDiamondAddress)
            .getValidatorsInNextEpoch(currentRealmId)
            .length;

        if (numNextValidators == 0) {
            revert("No upcoming validators");
        }

        // Use a random validator
        validatorIndex = bound(validatorIndex, 0, numNextValidators - 1);
        address validatorAddress = StakingViewsFacet(stakingDiamondAddress)
            .getValidatorsInNextEpoch(currentRealmId)[validatorIndex];

        vm.startPrank(validatorAddress);
        _;
        vm.stopPrank();
    }

    // =============== STAKING VALIDATOR HANDLERS ==============={}

    function handle_requestToJoin(
        uint256 realmIdIndex,
        uint256 walletIndex
    ) public useRealmId(realmIdIndex) useAndPersistRandomWallet(walletIndex) {
        StakingValidatorFacet(stakingDiamondAddress).requestToJoin(
            currentRealmId
        );
    }

    function handle_requestToLeave(
        uint256 validatorIndex
    ) public useRandomCurrentValidator(validatorIndex) {
        StakingValidatorFacet(stakingDiamondAddress).requestToLeave();
    }

    function handle_lockSignalAndAdvance(
        uint256 realmIdIndex,
        uint256 epochsToAdvance
    ) public useRealmId(realmIdIndex) {
        // We make sure that it is possible to advance so many epochs that stakes are completely unfrozen.
        epochsToAdvance = bound(
            epochsToAdvance,
            1,
            (testingConfig.maxStakeTimeLock / testingConfig.epochLength) +
                (5 * 24) // Add additional 5 days to balance likelihood of advancing many epochs.
        );

        _lockSignalAndAdvance(
            currentRealmId,
            epochsToAdvance,
            currentEpochNumber
        );
    }

    function _lockSignalAndAdvance(
        uint256 realmId,
        uint256 epochsToAdvance,
        uint256 startingEpochNumber
    ) internal {
        // Pause gas metering to avoid running out of gas as we advance a potentially large number of epochs.
        vm.pauseGasMetering();

        StakingValidatorFacet stakingValidatorFacet = StakingValidatorFacet(
            address(stakingDiamondAddress)
        );

        // Fetch the validators in the next epoch.
        address[] memory validatorsInNextEpoch = StakingViewsFacet(
            stakingDiamondAddress
        ).getValidatorsInNextEpoch(realmId);

        for (
            uint256 i = startingEpochNumber;
            i < startingEpochNumber + epochsToAdvance;
            i++
        ) {
            skip(testingConfig.epochLength);

            // We absorb possible reverts from lockValidatorsForNextEpoch since it's possible that the network is
            // already locked.
            try stakingValidatorFacet.lockValidatorsForNextEpoch(realmId) {
                console.log("Locked validators for next epoch");
            } catch (bytes memory reason) {
                console.logBytes(reason);
            }

            // Signal ready for the next epoch.
            for (
                uint256 valIdx = 0;
                valIdx < validatorsInNextEpoch.length;
                valIdx++
            ) {
                vm.prank(validatorsInNextEpoch[valIdx]);
                try
                    StakingValidatorFacet(stakingDiamondAddress)
                        .signalReadyForNextEpoch(realmId, i)
                {
                    console.log("Signal ready for next epoch");
                } catch (bytes memory reason) {
                    // If we are here, then it likely could be that at least one of the validators
                    // do not have sufficient self-stake to signal ready for next epoch. At this point,
                    // we should just return this function so that the test harness can find a way to
                    // continue advancing the epoch, whether it is by kicking a validator and/or joining another
                    // one in.
                    //
                    // We need this try-catch block in order to put the epoch in this intermediate state. Otherwise,
                    // if this whole function reverts every single time, then the epoch will never advance.
                    console.logBytes(reason);
                    return;
                }
            }

            // Advance to the next epoch.
            stakingValidatorFacet.advanceEpoch(realmId);

            // Now that the epoch has advanced, get the current reward epoch number and store it.
            uint256 currentRewardEpochNumber = StakingViewsFacet(
                stakingDiamondAddress
            ).epoch(realmId).rewardEpochNumber;
            realmIdToRewardEpochNumberHistory[realmId].push(
                currentRewardEpochNumber
            );
        }

        // Resume gas metering.
        vm.resumeGasMetering();
    }

    /// @dev This function uses a probability parameter to reduce the likelihood of kicking a validator.
    function handle_kickValidatorInNextEpoch(
        uint256 validatorIndex,
        uint256 probability
    ) public {
        probability = bound(probability, 0, 100);

        if (probability < 1) {
            return;
        }

        // Choose a random staker to kick from the next epoch in the first realm.
        uint256 realmId = realmIds.at(0);
        address[] memory validatorsInNextEpoch = StakingViewsFacet(
            stakingDiamondAddress
        ).getValidatorsInNextEpoch(realmId);
        validatorIndex = bound(
            validatorIndex,
            0,
            validatorsInNextEpoch.length - 1
        );
        address validatorAddress = validatorsInNextEpoch[validatorIndex];

        // All validators in the current epoch in that realm votes to kick that validator.
        address[] memory validatorsInCurrentEpoch = StakingViewsFacet(
            stakingDiamondAddress
        ).getValidatorsInCurrentEpoch(realmId);
        for (uint256 i = 0; i < validatorsInCurrentEpoch.length; i++) {
            if (validatorsInCurrentEpoch[i] == validatorAddress) {
                continue;
            }

            vm.prank(validatorsInCurrentEpoch[i]);
            StakingValidatorFacet(stakingDiamondAddress)
                .kickValidatorInNextEpoch(validatorAddress, 1, "");
        }
    }

    // =============== STAKING HANDLERS ===============

    /// @dev This can result in one of the following:
    ///   - The staker self-stakes and becomes a validator.
    ///   - The delegating staker stakes against an active validator.
    ///   - The delegating staker stakes against a validator who is inactive but included in the next epoch.
    function handle_stakeAndSetNodeInfo(
        uint256 stakeAmount,
        uint256 stakeTimeLock,
        uint256 walletIndex,
        uint256 stakeMode
    ) public useAndPersistRandomWallet(walletIndex) {
        stakeMode = bound(stakeMode, 0, 2);
        stakeAmount = bound(stakeAmount, 32 ether, 100_000 ether);
        stakeTimeLock = bound(
            stakeTimeLock,
            45 days,
            testingConfig.maxStakeTimeLock
        );

        if (stakeMode == 0) {
            // Self-stake as a validator.
            StakingFacet(stakingDiamondAddress).stake(
                stakeAmount,
                stakeTimeLock,
                currentWallet
            );

            stakerToOperatorStaker[currentWallet].add(currentWallet);
        } else if (stakeMode == 1) {
            // Stake against a validator who is inactive but included in the next epoch.
            address operatorStakerToStakeAgainst;
            for (
                uint256 realmIdx = 0;
                realmIdx < realmIds.length();
                realmIdx++
            ) {
                // For each realm, find the first validator that is in the next epoch and not in the current epoch.
                uint256 realmId = realmIds.at(realmIdx);
                address[] memory validatorsInNextEpoch = StakingViewsFacet(
                    stakingDiamondAddress
                ).getValidatorsInNextEpoch(realmId);
                for (uint256 i = 0; i < validatorsInNextEpoch.length; i++) {
                    if (
                        !StakingViewsFacet(stakingDiamondAddress)
                            .isActiveValidator(
                                realmId,
                                validatorsInNextEpoch[i]
                            )
                    ) {
                        operatorStakerToStakeAgainst = validatorsInNextEpoch[i];
                        break;
                    }
                }
            }

            StakingFacet(stakingDiamondAddress).stake(
                stakeAmount,
                stakeTimeLock,
                operatorStakerToStakeAgainst
            );

            stakerToOperatorStaker[currentWallet].add(
                operatorStakerToStakeAgainst
            );
        } else if (stakeMode == 2) {
            // Stake against an active validator.
            address operatorStakerToStakeAgainst;
            for (
                uint256 realmIdx = 0;
                realmIdx < realmIds.length();
                realmIdx++
            ) {
                // For each realm, find the first active validator.
                uint256 realmId = realmIds.at(realmIdx);
                address[] memory validatorsInCurrentEpoch = StakingViewsFacet(
                    stakingDiamondAddress
                ).getValidatorsInCurrentEpoch(realmId);
                for (uint256 i = 0; i < validatorsInCurrentEpoch.length; i++) {
                    operatorStakerToStakeAgainst = validatorsInCurrentEpoch[i];
                    break;
                }
            }

            StakingFacet(stakingDiamondAddress).stake(
                stakeAmount,
                stakeTimeLock,
                operatorStakerToStakeAgainst
            );

            stakerToOperatorStaker[currentWallet].add(
                operatorStakerToStakeAgainst
            );
        }

        // Set IP port etc.
        StakingValidatorFacet(stakingDiamondAddress).setIpPortNodeAddress(
            1,
            1,
            1,
            currentWallet
        );
    }

    /// @dev Here we constrain the function to use a wallet that actually has a stake record.
    function handle_unfreezeStake(
        uint256 operatorStakerIndex,
        uint256 stakeRecordIndex,
        uint256 walletIndex
    ) public useAndPersistWalletWithStake(walletIndex) {
        // Choose a random operator staker to get stake records to unfreeze.
        EnumerableSet.AddressSet
            storage currentOperatorStakers = stakerToOperatorStaker[
                currentWallet
            ];
        operatorStakerIndex = bound(
            operatorStakerIndex,
            0,
            currentOperatorStakers.length() - 1
        );
        address operatorStakerToChooseStakeRecordsFrom = currentOperatorStakers
            .at(operatorStakerIndex);

        // Choose a random stake record to unfreeze.
        LibStakingStorage.StakeRecord[] memory stakeRecords = StakingViewsFacet(
            stakingDiamondAddress
        ).getStakeRecordsForUser(
                currentWallet,
                operatorStakerToChooseStakeRecordsFrom
            );
        stakeRecordIndex = bound(stakeRecordIndex, 0, stakeRecords.length - 1);
        uint256 stakeRecordIdToUnfreeze = stakeRecords[stakeRecordIndex].id;

        // Unfreeze the stake record.
        StakingFacet(stakingDiamondAddress).unfreezeStake(
            operatorStakerToChooseStakeRecordsFrom,
            stakeRecordIdToUnfreeze
        );

        unfreezeCount++;
    }

    /// @dev Here we constrain the function to use a wallet that actually has a stake record.
    function handle_claimStakeRewards(
        uint256 realmIdIndex,
        uint256 operatorStakerIndexToClaimFrom,
        uint256 stakeRecordIndexToClaim,
        uint256 walletIndex
    )
        public
        useRealmId(realmIdIndex)
        useAndPersistWalletWithStake(walletIndex)
    {
        // Choose a random operator staker to get stake records to claim rewards from.
        EnumerableSet.AddressSet
            storage currentOperatorStakers = stakerToOperatorStaker[
                currentWallet
            ];
        operatorStakerIndexToClaimFrom = bound(
            operatorStakerIndexToClaimFrom,
            0,
            currentOperatorStakers.length() - 1
        );
        address operatorStakerToClaimFrom = currentOperatorStakers.at(
            operatorStakerIndexToClaimFrom
        );

        // Choose a random stake record to claim rewards from.
        LibStakingStorage.StakeRecord[] memory stakeRecords = StakingViewsFacet(
            stakingDiamondAddress
        ).getStakeRecordsForUser(currentWallet, operatorStakerToClaimFrom);
        stakeRecordIndexToClaim = bound(
            stakeRecordIndexToClaim,
            0,
            stakeRecords.length - 1
        );
        uint256 stakeRecordIdToClaim = stakeRecords[stakeRecordIndexToClaim].id;

        // Claim the rewards.
        vm.pauseGasMetering(); // TODO: Once reward claiming has been optimized, remove this.
        StakingFacet(stakingDiamondAddress).claimStakeRewards(
            currentRealmId,
            operatorStakerToClaimFrom,
            stakeRecordIdToClaim,
            type(uint256).max
        );
    }

    /// @dev Here we constrain the function to use a wallet that is a validator.
    function handle_claimValidatorCommission(
        uint256 operatorStakerIndex
    ) public {
        // Choose a random validator.
        address[] memory allValidators = StakingViewsFacet(
            stakingDiamondAddress
        ).getAllValidators();
        operatorStakerIndex = bound(
            operatorStakerIndex,
            0,
            allValidators.length - 1
        );
        address validatorToClaim = allValidators[operatorStakerIndex];

        // Determine the realm they are / were in.
        uint256 realmId = StakingAcrossRealmsFacet(stakingDiamondAddress)
            .getRealmIdForStakerAddress(validatorToClaim);
        // If it is still 0, I can use the .lastRealmId to get the last realmId they were in.
        if (realmId == 0) {
            address[] memory validatorsToQueryFor = new address[](1);
            validatorsToQueryFor[0] = validatorToClaim;
            LibStakingStorage.Validator[] memory validators = StakingViewsFacet(
                stakingDiamondAddress
            ).getValidatorsStructs(validatorsToQueryFor);
            realmId = validators[0].lastRealmId;
        }

        // Claim the rewards.
        vm.prank(validatorToClaim);
        StakingFacet(stakingDiamondAddress).claimValidatorCommission(
            realmId,
            type(uint256).max
        );
    }

    /// @dev Here we constrain the function to use a wallet that is a validator.
    function handle_claimFixedCostRewards(uint256 operatorStakerIndex) public {
        // Choose a random validator.
        address[] memory allValidators = StakingViewsFacet(
            stakingDiamondAddress
        ).getAllValidators();
        operatorStakerIndex = bound(
            operatorStakerIndex,
            0,
            allValidators.length - 1
        );
        address validatorToClaim = allValidators[operatorStakerIndex];

        // Determine the realm they are / were in.
        uint256 realmId = StakingAcrossRealmsFacet(stakingDiamondAddress)
            .getRealmIdForStakerAddress(validatorToClaim);
        // If it is still 0, I can use the .lastRealmId to get the last realmId they were in.
        if (realmId == 0) {
            address[] memory validatorsToQueryFor = new address[](1);
            validatorsToQueryFor[0] = validatorToClaim;
            LibStakingStorage.Validator[] memory validators = StakingViewsFacet(
                stakingDiamondAddress
            ).getValidatorsStructs(validatorsToQueryFor);
            realmId = validators[0].lastRealmId;
        }

        // Claim the rewards.
        vm.prank(validatorToClaim);
        StakingFacet(stakingDiamondAddress).claimFixedCostRewards(
            realmId,
            type(uint256).max
        );
    }

    /// @dev Here we constrain the function to use a wallet that actually has a stake record.
    function handle_migrateStakeRecord(
        uint256 operatorStakerIndexToMigrateFrom,
        uint256 stakeRecordIndexToMigrate,
        uint256 operatorStakerIndexToMigrateTo,
        uint256 walletIndex
    ) public useAndPersistWalletWithStake(walletIndex) {
        // Choose a random staker to migrate from.
        EnumerableSet.AddressSet
            storage currentOperatorStakers = stakerToOperatorStaker[
                currentWallet
            ];
        operatorStakerIndexToMigrateFrom = bound(
            operatorStakerIndexToMigrateFrom,
            0,
            currentOperatorStakers.length() - 1
        );
        address operatorStakerToMigrateFrom = currentOperatorStakers.at(
            operatorStakerIndexToMigrateFrom
        );

        // Choose a random stake record to migrate.
        LibStakingStorage.StakeRecord[] memory stakeRecords = StakingViewsFacet(
            stakingDiamondAddress
        ).getStakeRecordsForUser(currentWallet, operatorStakerToMigrateFrom);
        stakeRecordIndexToMigrate = bound(
            stakeRecordIndexToMigrate,
            0,
            stakeRecords.length - 1
        );
        uint256 stakeRecordIdToMigrate = stakeRecords[stakeRecordIndexToMigrate]
            .id;

        // Choose a random staker to migrate to.
        operatorStakerIndexToMigrateTo = bound(
            operatorStakerIndexToMigrateTo,
            0,
            wallets.length - 1
        );
        address operatorStakerToMigrateTo = wallets[
            operatorStakerIndexToMigrateTo
        ];

        // Migrate the stake record.
        StakingFacet(stakingDiamondAddress).migrateStakeRecord(
            operatorStakerToMigrateFrom,
            stakeRecordIdToMigrate,
            operatorStakerToMigrateTo
        );

        // Finally, update the stakerToOperatorStaker mapping appropriately. We will need to check if there are
        // any more stake records with the operator staker that the staker migrated from. If there are none,
        // we will need to remove the operator staker from the stakerToOperatorStaker mapping.
        if (stakeRecords.length == 1) {
            stakerToOperatorStaker[currentWallet].remove(
                operatorStakerToMigrateFrom
            );
        }
        stakerToOperatorStaker[currentWallet].add(operatorStakerToMigrateTo);
    }

    /// @dev Here we constrain the function to use a wallet that actually has a stake record.
    function handle_splitStakeRecord(
        uint256 operatorStakerIndex,
        uint256 stakeRecordIndex,
        uint256 walletIndex,
        uint256 ratio
    ) public useAndPersistWalletWithStake(walletIndex) {
        ratio = bound(ratio, 0.001 ether, .999 ether);

        // Choose a random operator staker to get stake records to split.
        EnumerableSet.AddressSet
            storage currentOperatorStakers = stakerToOperatorStaker[
                currentWallet
            ];
        operatorStakerIndex = bound(
            operatorStakerIndex,
            0,
            currentOperatorStakers.length() - 1
        );
        address operatorStakerToSplit = currentOperatorStakers.at(
            operatorStakerIndex
        );

        // Choose a random stake record to split.
        LibStakingStorage.StakeRecord[] memory stakeRecords = StakingViewsFacet(
            stakingDiamondAddress
        ).getStakeRecordsForUser(currentWallet, operatorStakerToSplit);
        stakeRecordIndex = bound(stakeRecordIndex, 0, stakeRecords.length - 1);
        uint256 stakeRecordIdToSplit = stakeRecords[stakeRecordIndex].id;

        // Split the stake record.
        StakingFacet(stakingDiamondAddress).splitStakeRecord(
            operatorStakerToSplit,
            stakeRecordIdToSplit,
            ratio
        );
    }

    /// @dev Here we constrain the function to use a wallet that actually has a stake record.
    function handle_withdraw(
        uint256 operatorStakerIndex,
        uint256 stakeRecordIndex,
        uint256 walletIndex
    ) public useAndPersistWalletWithStake(walletIndex) {
        // Choose a random operator staker to get stake records to withdraw.
        EnumerableSet.AddressSet
            storage currentOperatorStakers = stakerToOperatorStaker[
                currentWallet
            ];
        operatorStakerIndex = bound(
            operatorStakerIndex,
            0,
            currentOperatorStakers.length() - 1
        );
        address operatorStakerToWithdrawFrom = currentOperatorStakers.at(
            operatorStakerIndex
        );

        // Choose a random stake record to withdraw.
        LibStakingStorage.StakeRecord[] memory stakeRecords = StakingViewsFacet(
            stakingDiamondAddress
        ).getStakeRecordsForUser(currentWallet, operatorStakerToWithdrawFrom);
        stakeRecordIndex = bound(stakeRecordIndex, 0, stakeRecords.length - 1);
        LibStakingStorage.StakeRecord
            memory stakeRecordToWithdraw = stakeRecords[stakeRecordIndex];
        uint256 stakeRecordIdToWithdraw = stakeRecordToWithdraw.id;

        // FIXME: For now, this is the only way to prevent this function from reverting via the
        // _lockSignalAndAdvance function. It seems that, with the _lockSignalAndAdvance function,
        // some validator is going to have insufficient self-stake, and the function will not
        // advance as many epochs as we need it to.
        skip(400 days);

        // Withdraw the stake record.
        StakingFacet(stakingDiamondAddress).withdraw(
            operatorStakerToWithdrawFrom,
            stakeRecordIdToWithdraw
        );

        // Finally, update the stakerToOperatorStaker mapping appropriately. We will need to check if there are
        // any more stake records with the operator staker that the staker withdrew from. If there are none,
        // we will need to remove the operator staker from the stakerToOperatorStaker mapping.
        if (stakeRecords.length == 1) {
            stakerToOperatorStaker[currentWallet].remove(
                operatorStakerToWithdrawFrom
            );
        }
    }

    function handle_increaseStakeRecordAmount(
        uint256 operatorStakerIndex,
        uint256 stakeRecordIndex,
        uint256 walletIndex,
        uint256 additionalAmount
    ) public useAndPersistWalletWithStake(walletIndex) {
        additionalAmount = bound(additionalAmount, 1, 100_000 ether);

        // Choose a random operator staker to get stake records to increase the amount.
        EnumerableSet.AddressSet
            storage currentOperatorStakers = stakerToOperatorStaker[
                currentWallet
            ];
        operatorStakerIndex = bound(
            operatorStakerIndex,
            0,
            currentOperatorStakers.length() - 1
        );
        address operatorStakerToIncreaseAmount = currentOperatorStakers.at(
            operatorStakerIndex
        );

        // Choose a random stake record to increase the amount.
        LibStakingStorage.StakeRecord[] memory stakeRecords = StakingViewsFacet(
            stakingDiamondAddress
        ).getStakeRecordsForUser(currentWallet, operatorStakerToIncreaseAmount);
        stakeRecordIndex = bound(stakeRecordIndex, 0, stakeRecords.length - 1);
        uint256 stakeRecordIdToIncreaseAmount = stakeRecords[stakeRecordIndex]
            .id;

        // Increase the amount.
        StakingFacet(stakingDiamondAddress).increaseStakeRecordAmount(
            operatorStakerToIncreaseAmount,
            stakeRecordIdToIncreaseAmount,
            additionalAmount
        );
    }

    function handle_increaseStakeRecordTimelock(
        uint256 operatorStakerIndex,
        uint256 stakeRecordIndex,
        uint256 walletIndex,
        uint256 additionalTimeLock
    ) public useAndPersistWalletWithStake(walletIndex) {
        additionalTimeLock = bound(
            additionalTimeLock,
            1,
            testingConfig.maxStakeTimeLock
        );

        // Choose a random operator staker to get stake records to increase the timelock.
        EnumerableSet.AddressSet
            storage currentOperatorStakers = stakerToOperatorStaker[
                currentWallet
            ];
        operatorStakerIndex = bound(
            operatorStakerIndex,
            0,
            currentOperatorStakers.length() - 1
        );
        address operatorStakerToIncreaseTimelock = currentOperatorStakers.at(
            operatorStakerIndex
        );

        // Choose a random stake record to increase the timelock.
        LibStakingStorage.StakeRecord[] memory stakeRecords = StakingViewsFacet(
            stakingDiamondAddress
        ).getStakeRecordsForUser(
                currentWallet,
                operatorStakerToIncreaseTimelock
            );
        stakeRecordIndex = bound(stakeRecordIndex, 0, stakeRecords.length - 1);
        uint256 stakeRecordIdToIncreaseTimelock = stakeRecords[stakeRecordIndex]
            .id;

        // Increase the timelock.
        StakingFacet(stakingDiamondAddress).increaseStakeRecordTimelock(
            operatorStakerToIncreaseTimelock,
            stakeRecordIdToIncreaseTimelock,
            additionalTimeLock
        );
    }
}

contract StakingInvariantsTest is SetupAndUtils {
    StakingHandler stakingHandler;

    // Keep this number relatively low to keep the chances of actually mutating state high enough.
    uint256 constant TOTAL_WALLETS = 10;
    uint256 constant MAX_REALMS = 5;

    /// @notice The setup for each invariant test is as follows:
    /// - Set up using baseSetup().
    /// - Create array of wallets and fund each address with tokens and approve them.
    /// - Create the initial state of 1 realm with 6 staked and joined validators.
    /// - Set up the StakingHandler.
    /// - Add the target contract and selectors to the fuzzer.
    /// - Set up the complaint config for reason 1.
    function setUp() public {
        baseSetup();

        // Set up wallets and fund each address with tokens and approve them.
        address[] memory wallets = _generateAddresses(TOTAL_WALLETS);
        uint256[] memory commsKeys = _generateUint256s(TOTAL_WALLETS);
        for (uint256 i = 0; i < wallets.length; i++) {
            _fundAddressWithTokensAndApprove(wallets[i], 10_000_000 ether);
        }

        // Create the initial state of 2 realms, each with 3 staked and joined validators.
        stakingAdminFacet.addRealm();
        stakingAdminFacet.increaseRewardPool(2, rewardsBudget);
        _setupValidators(
            1,
            _generateAddresses(3),
            100 ether,
            10 ether,
            365 days,
            _generateUint256s(3)
        );
        _setupValidators(
            2,
            _generateAddressesWithOffset(3, 3),
            100 ether,
            10 ether,
            365 days,
            _generateUint256sWithOffset(3, 3)
        );

        // Set up StakingHandler.
        StakingHandler.TestingConfig memory testingConfig = StakingHandler
            .TestingConfig({
                maxRealms: MAX_REALMS,
                initialStakeAndJoinedValidators: 6,
                maxStakeTimeLock: 365 days,
                epochLength: 1 hours
            });
        stakingHandler = new StakingHandler(
            address(staking),
            address(this),
            token,
            wallets,
            commsKeys,
            testingConfig
        );

        // Add the target contract and selectors to the fuzzer.
        // We are ONLY testing invariants against the following functions, all directed towards the single
        // Staking diamond contract, proxied through the StakingHandler.
        targetContract(address(stakingHandler));
        bytes4[] memory selectors = new bytes4[](14);
        selectors[0] = stakingHandler.handle_requestToJoin.selector;
        selectors[1] = stakingHandler.handle_requestToLeave.selector;
        selectors[2] = stakingHandler.handle_lockSignalAndAdvance.selector;
        selectors[3] = stakingHandler.handle_kickValidatorInNextEpoch.selector;
        selectors[4] = stakingHandler.handle_stakeAndSetNodeInfo.selector;
        selectors[5] = stakingHandler.handle_unfreezeStake.selector;
        selectors[6] = stakingHandler.handle_claimStakeRewards.selector;
        selectors[7] = stakingHandler.handle_claimValidatorCommission.selector;
        selectors[8] = stakingHandler.handle_claimFixedCostRewards.selector;
        selectors[9] = stakingHandler.handle_migrateStakeRecord.selector;
        selectors[10] = stakingHandler.handle_splitStakeRecord.selector;
        selectors[11] = stakingHandler.handle_withdraw.selector;
        selectors[12] = stakingHandler
            .handle_increaseStakeRecordTimelock
            .selector;
        selectors[13] = stakingHandler
            .handle_increaseStakeRecordAmount
            .selector;

        targetSelector(
            FuzzSelector({
                addr: address(stakingHandler),
                selectors: selectors
            })
        );

        // Set the complaint config for reason 1.
        stakingAdminFacet.setComplaintConfig(
            1,
            LibStakingStorage.ComplaintConfig({
                tolerance: 6,
                intervalSecs: 90,
                kickPenaltyPercent: 0.1 ether, // 10%
                kickPenaltyDemerits: 1
            })
        );
    }

    /// Invariant: The total amount of LIT staked in the Staking contract is equal or greater than the sum of the
    /// total amount of LIT staked in each validator's reward epoch.
    function invariant_REStakeAmountLEQTotalStakedAmount() external {
        // Loop through all the wallets and get the staked amount according to their lastRewardEpoch number to fetch all RE stake amounts.
        address[] memory wallets = stakingHandler.getAllWallets();
        LibStakingStorage.Validator[] memory maybeValidators = stakingViewsFacet
            .getValidatorsStructs(wallets);

        uint256 totalREStakedAmount = 0;
        for (
            uint256 validatorIdx = 0;
            validatorIdx < maybeValidators.length;
            validatorIdx++
        ) {
            LibStakingStorage.Validator memory validator = maybeValidators[
                validatorIdx
            ];

            LibStakingStorage.RewardEpoch memory lastRewardEpoch = stakingFacet
                .getRewardEpoch(
                    wallets[validatorIdx],
                    validator.lastRewardEpoch
                );
            totalREStakedAmount += lastRewardEpoch.stakeAmount;
        }

        assertLe(totalREStakedAmount, token.balanceOf(address(staking)));
    }

    /// Invariant: The total amount of LIT staked in the Staking contract is equal or greater than the sum of the
    /// total amount of LIT staked in each staker's individual stake records.
    function invariant_StakeRecordsStakeAmountEQTotalStakedAmount() public {
        address[] memory wallets = stakingHandler.getAllWallets();
        uint256 totalStakedAmount = 0;
        // TODO: See if there is a way to optimize this without relying on any internal data structures from
        // StakingHandler (which could track things incorrectly).
        // For each wallet, find all the stake records it may have against every other wallet, and sum the stake amounts.
        for (uint256 i = 0; i < wallets.length; i++) {
            address wallet = wallets[i];
            for (uint256 j = 0; j < wallets.length; j++) {
                address otherWallet = wallets[j];
                LibStakingStorage.StakeRecord[]
                    memory stakeRecords = stakingViewsFacet
                        .getStakeRecordsForUser(wallet, otherWallet);
                for (uint256 k = 0; k < stakeRecords.length; k++) {
                    totalStakedAmount += stakeRecords[k].amount;
                }
            }
        }

        assertLe(totalStakedAmount, token.balanceOf(address(staking)));
    }

    /// Invariant: If the total slope in a reward epoch is greater than 0, then at least one staker has unfrozen
    /// in the past, though not necessarily still in the process of unfreezing.
    function invariant_UnfreezeCount() public {
        for (
            uint256 realmIdx = 0;
            realmIdx < stakingHandler.getAllRealmIds().length;
            realmIdx++
        ) {
            uint256 realmId = stakingHandler.getAllRealmIds()[realmIdx];
            uint256 epochNumber = stakingViewsFacet.getRewardEpochNumber(
                realmId
            );
            address[] memory validators = stakingViewsFacet
                .getValidatorsInCurrentEpoch(realmId);

            for (uint256 i = 0; i < validators.length; i++) {
                uint256 slope = stakingFacet
                    .getRewardEpoch(validators[i], epochNumber)
                    .slope;
                if (slope > 0) {
                    assertGt(stakingHandler.unfreezeCount(), 0);
                }
            }
        }
    }

    /// Invariant: The number of validators in each realm never drops below the configured minimum amount.
    function invariant_ValidatorCountAboveMinimum() public {
        // Get the global config.
        LibStakingStorage.GlobalConfig memory globalConfig = stakingViewsFacet
            .globalConfig();
        uint256 minimumValidatorCount = globalConfig.minimumValidatorCount;

        for (
            uint256 realmIdx = 0;
            realmIdx < stakingHandler.getAllRealmIds().length;
            realmIdx++
        ) {
            uint256 realmId = stakingHandler.getAllRealmIds()[realmIdx];

            // Only do the assertion if the epoch number is greater than 1.
            if (stakingViewsFacet.epoch(realmId).number < 2) {
                continue;
            }

            assertGe(
                stakingViewsFacet.getValidatorsInCurrentEpoch(realmId).length,
                minimumValidatorCount
            );
        }
    }

    /// Invariant: The node address to staker address mappings are all intact (and don't point to the zero address), across all realms.
    function invariant_NodeAddressToStakerAddressMappings() public {
        for (
            uint256 realmIdx = 0;
            realmIdx < stakingHandler.getAllRealmIds().length;
            realmIdx++
        ) {
            uint256 realmId = stakingHandler.getAllRealmIds()[realmIdx];

            // Check validators in the current epoch.
            LibStakingStorage.Validator[] memory validators = stakingViewsFacet
                .getValidatorsStructsInCurrentEpoch(realmId);
            for (uint256 i = 0; i < validators.length; i++) {
                /// NOTE: We can only do this because we set the node address to be equal to the staker address.
                assertNotEq(
                    stakingViewsFacet.nodeAddressToStakerAddress(
                        validators[i].nodeAddress
                    ),
                    address(0)
                );
            }

            // Check validators in the next epoch.
            LibStakingStorage.Validator[]
                memory nextEpochValidators = stakingViewsFacet
                    .getValidatorsStructsInNextEpoch(realmId);
            for (uint256 i = 0; i < nextEpochValidators.length; i++) {
                /// NOTE: We can only do this because we set the node address to be equal to the staker address.
                assertNotEq(
                    stakingViewsFacet.nodeAddressToStakerAddress(
                        nextEpochValidators[i].nodeAddress
                    ),
                    address(0)
                );
            }
        }
    }
}
