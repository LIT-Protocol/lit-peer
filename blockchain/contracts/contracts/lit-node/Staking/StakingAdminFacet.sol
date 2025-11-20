//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.21;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ERC20Burnable } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { StakingValidatorFacet } from "./StakingValidatorFacet.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingCommon } from "./StakingCommon.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "hardhat/console.sol";

contract StakingAdminFacet is StakingCommon {
    using EnumerableSet for EnumerableSet.AddressSet;

    /* ========== Modifier Equivalents ========== */

    function onlyOwner() internal view {
        if (msg.sender != LibDiamond.contractOwner()) {
            revert StakingUtilsLib.CallerNotOwner();
        }
    }

    function onlyOwnerOrDevopsAdmin() internal view {
        if (
            msg.sender != LibDiamond.contractOwner() &&
            msg.sender != s().devopsAdmin
        ) {
            revert StakingUtilsLib.CallerNotOwnerOrDevopsAdmin();
        }
    }

    /* ========== VIEWS ========== */

    function validators() internal view returns (StakingValidatorFacet) {
        return StakingValidatorFacet(address(this));
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function setEpochLength(uint256 realmId, uint256 newEpochLength) external {
        onlyOwner();
        mutableEpoch(realmId).epochLength = newEpochLength;
        s().globalConfig[0].rewardEpochDuration = newEpochLength;
        emit EpochLengthSet(realmId, newEpochLength);
    }

    function setEpochTimeout(
        uint256 realmId,
        uint256 newEpochTimeout
    ) external {
        onlyOwner();
        mutableEpoch(realmId).timeout = newEpochTimeout;
        emit EpochTimeoutSet(realmId, newEpochTimeout);
    }

    function setEpochEndTime(
        uint256 realmId,
        uint256 newEpochEndTime
    ) external {
        onlyOwnerOrDevopsAdmin();
        mutableEpoch(realmId).endTime = newEpochEndTime;
        emit EpochEndTimeSet(realmId, newEpochEndTime);
    }

    function setPendingRejoinTimeout(uint256 newTimeout) external {
        onlyOwner();
        s().pendingRejoinTimeout = newTimeout;
    }

    function setEpochState(
        uint256 realmId,
        LibStakingStorage.States newState
    ) external {
        onlyOwnerOrDevopsAdmin();
        if (
            (newState == LibStakingStorage.States.NextValidatorSetLocked) &&
            (mutableEpoch(realmId).endTime > block.timestamp)
        ) {
            revert StakingUtilsLib
                .CannotMoveToLockedValidatorStateBeforeEpochEnds();
        }

        realm(realmId).state = newState;
        emit StakingUtilsLib.StateChanged(newState);
    }

    function setDemeritRejoinThreshold(uint256 newThreshold) external {
        onlyOwner();
        s().demeritRejoinThreshold = newThreshold;
    }

    /*
     * @notice Sets the contract resolver for the staking contract
     * @param newResolverAddress The address of the new contract resolver
     */
    function setContractResolver(address newResolverAddress) external {
        onlyOwner();
        s().contractResolver = ContractResolver(newResolverAddress);
        emit ResolverContractAddressSet(newResolverAddress);
    }

    function setPermittedValidatorsOn(
        uint256 realmId,
        bool permittedValidatorsOn
    ) external {
        onlyOwner();
        realm(realmId)
            .realm_configs[0]
            .permittedValidatorsOn = permittedValidatorsOn;
    }

    function adminKickValidatorInNextEpoch(
        address validatorStakerAddress
    ) public {
        onlyOwnerOrDevopsAdmin();
        uint256 realmId = realms().getRealmIdForStakerAddress(
            validatorStakerAddress
        );

        LibStakingStorage.RealmStorage storage realmStorage = realm(realmId);
        // block them from rejoining the next epoch
        realmStorage.validatorsKickedFromNextEpoch.add(validatorStakerAddress);

        // remove from the validator set
        StakingUtilsLib.removeValidatorFromNextEpoch(
            realmId,
            validatorStakerAddress
        );

        // if they're in the current set, we need to mark them as kicked from the current set too
        bool isValidatorInCurrentSet = realmStorage
            .validatorsInCurrentEpoch
            .contains(validatorStakerAddress);
        if (isValidatorInCurrentSet) {
            realmStorage.currentValidatorsKickedFromNextEpoch.add(
                validatorStakerAddress
            );
        }

        emit StakingUtilsLib.ValidatorKickedFromNextEpoch(
            validatorStakerAddress
        );

        // admin kicked, ban for ten thousand lifetimes or until admin rejoins
        s().demeritsCounter[validatorStakerAddress] +=
            s().demeritRejoinThreshold *
            1000;
        emit StakingUtilsLib.ValidatorBanned(validatorStakerAddress);

        // // if we're in the locked state, then we need to unlock, because we kicked a node
        if (
            realmStorage.state ==
            LibStakingStorage.States.NextValidatorSetLocked ||
            realmStorage.state == LibStakingStorage.States.ReadyForNextEpoch
        ) {
            StakingUtilsLib.unlockEpoch(realmId);
        }
    }

    function adminRejoinValidator(
        uint256 realmId,
        address stakerAddress
    ) external {
        onlyOwnerOrDevopsAdmin();
        LibStakingStorage.RealmStorage storage realmStorage = realm(realmId);
        validators().checkActiveOrUnlockedOrPausedState(realmStorage.state);

        // Update the reward epoch parameters.
        if (!realmStorage.validatorsInNextEpoch.contains(stakerAddress)) {
            console.log("setupNextRewardEpoch");
            StakingUtilsLib.setupNextRewardEpoch(realmId, stakerAddress, false);
        }

        // remove from next validator kicked list
        realmStorage.validatorsKickedFromNextEpoch.remove(stakerAddress);
        // remove from current validator kicked list
        realmStorage.currentValidatorsKickedFromNextEpoch.remove(stakerAddress);
        // add to next validator set
        realmStorage.validatorsInNextEpoch.add(stakerAddress);
        //clear the demerits counter
        s().demeritsCounter[stakerAddress] = 0;

        if (!s().allActiveValidators.contains(stakerAddress)) {
            s().allActiveValidators.add(stakerAddress);
        }

        s().validatorToNextRealmId[stakerAddress] = realmId;
        // Loop through pendingRejoins and remove matching staker
        for (uint i = 0; i < s().pendingRejoins.length; i++) {
            if (s().pendingRejoins[i].addr == stakerAddress) {
                // Remove element by swapping with last element and then pop
                s().pendingRejoins[i] = s().pendingRejoins[
                    s().pendingRejoins.length - 1
                ];
                s().pendingRejoins.pop();
                s().isValidatorInPendingRejoin[stakerAddress] = false;
                break; // Exit loop after finding and removing the match
            }
        }

        emit ValidatorRejoinedNextEpoch(stakerAddress);
    }

    function adminSetValidatorsInCurrentEpoch(
        uint256 realmId,
        address[] memory validatorsForCurrentEpoch
    ) external {
        onlyOwner();
        // Clear the current validator set
        StakingUtilsLib.clearEnumerableAddressSet(
            realm(realmId).validatorsInCurrentEpoch
        );

        for (uint256 i = 0; i < validatorsForCurrentEpoch.length; i++) {
            realm(realmId).validatorsInCurrentEpoch.add(
                validatorsForCurrentEpoch[i]
            );
        }
    }

    function adminSetValidatorsInNextEpoch(
        uint256 realmId,
        address[] memory validatorsForNextEpoch
    ) external {
        onlyOwner();
        // Clear the next validator set
        LibStakingStorage.RealmStorage storage realmStorage = realm(realmId);

        StakingUtilsLib.clearEnumerableAddressSet(
            realmStorage.validatorsInNextEpoch
        );
        StakingUtilsLib.clearEnumerableAddressSet(
            realmStorage.validatorsKickedFromNextEpoch
        );
        StakingUtilsLib.clearEnumerableAddressSet(
            realmStorage.currentValidatorsKickedFromNextEpoch
        );

        for (uint256 i = 0; i < validatorsForNextEpoch.length; i++) {
            realmStorage.validatorsInNextEpoch.add(validatorsForNextEpoch[i]);
        }
    }

    function adminSlashValidator(
        uint256 percentage,
        address stakerAddress
    ) external {
        onlyOwner();
        StakingUtilsLib.slashValidator(percentage, stakerAddress);
    }

    function setTokenTotalSupplyStandIn(uint256 newTotalSupply) external {
        onlyOwner();
        s().tokenTotalSupplyStandIn = newTotalSupply;
    }

    function setPermittedValidators(
        uint256 realmId,
        address[] memory validatorsToSet
    ) external {
        onlyOwner();
        LibStakingStorage.RealmStorage storage realmStorage = realm(realmId);

        StakingUtilsLib.clearEnumerableAddressSet(
            realmStorage.permittedValidators
        );
        for (uint256 i = 0; i < validatorsToSet.length; i++) {
            realmStorage.permittedValidators.add(validatorsToSet[i]);
        }
    }

    function increaseRewardPool(uint256 realmId, uint256 amount) public {
        onlyOwner();
        _stake(amount, msg.sender);
        realm(realmId).rewardPool += amount;
    }

    function decreaseRewardPool(
        uint256 realmId,
        uint256 amount
    ) public nonReentrant {
        onlyOwner();
        SafeERC20.safeTransfer(
            IERC20(views().getTokenContractAddress()),
            msg.sender,
            amount
        );
        realm(realmId).rewardPool -= amount;
    }

    function setConfig(
        LibStakingStorage.GlobalConfig memory newConfig
    ) external {
        onlyOwner();
        LibStakingStorage.GlobalConfig storage config = LibStakingStorage
            .getStakingStorage()
            .globalConfig[0];
        config.tokenRewardPerTokenPerEpoch = newConfig
            .tokenRewardPerTokenPerEpoch;
        config.minimumValidatorCount = newConfig.minimumValidatorCount;

        // thunderhead
        config.rewardEpochDuration = newConfig.rewardEpochDuration;
        config.maxTimeLock = newConfig.maxTimeLock;
        config.minTimeLock = newConfig.minTimeLock;
        config.bmin = newConfig.bmin;
        config.bmax = newConfig.bmax;
        config.k = newConfig.k;
        config.p = newConfig.p;
        config.enableStakeAutolock = newConfig.enableStakeAutolock;
        config.tokenPrice = newConfig.tokenPrice;
        config.profitMultiplier = newConfig.profitMultiplier;
        config.usdCostPerMonth = newConfig.usdCostPerMonth;
        config.maxEmissionRate = newConfig.maxEmissionRate;
        config.minStakeAmount = newConfig.minStakeAmount;
        config.maxStakeAmount = newConfig.maxStakeAmount;
        config.minSelfStake = newConfig.minSelfStake;
        config.minSelfStakeTimelock = newConfig.minSelfStakeTimelock;
        config.minValidatorCountToClampMinimumThreshold = newConfig
            .minValidatorCountToClampMinimumThreshold;
        config.minThresholdToClampAt = newConfig.minThresholdToClampAt;
        config.voteToAdvanceTimeOut = newConfig.voteToAdvanceTimeOut;
    }

    function setComplaintConfig(
        uint256 reason,
        LibStakingStorage.ComplaintConfig memory config
    ) external {
        onlyOwnerOrDevopsAdmin();
        s().complaintReasonToConfig[reason] = config;
    }

    // this function adds a completely empty realm to the storage
    function addRealm() external returns (uint256 realmId) {
        onlyOwner();
        realmId = ++s().nextRealmId;

        EnumerableSet.add(s().realmIds, realmId);

        setupRealm(realmId);
    }

    function setupRealm(uint256 realmId) internal {
        LibStakingStorage.GlobalStakingStorage storage gs = s();
        LibStakingStorage.RealmStorage storage rs = realm(realmId);

        rs.realm_configs[0] = LibStakingStorage.RealmConfig({
            maxConcurrentRequests: 1000,
            maxPresignCount: 25,
            minPresignCount: 10,
            peerCheckingIntervalSecs: 7,
            maxPresignConcurrency: 2,
            rpcHealthcheckEnabled: true,
            minEpochForRewards: 3,
            permittedValidatorsOn: false,
            defaultKeySet: ""
        });
        uint256 epochLengthSeconds = 1 seconds;

        // this is the epoch for the realm
        rs.epochs[0] = LibStakingStorage.Epoch({
            epochLength: epochLengthSeconds,
            number: 1,
            rewardEpochNumber: gs.nextAvailableRewardEpochNumber + 1,
            nextRewardEpochNumber: gs.nextAvailableRewardEpochNumber + 2,
            endTime: block.timestamp + epochLengthSeconds,
            retries: 0,
            timeout: 60,
            startTime: block.timestamp,
            lastAdvanceVoteTime: 0
        });
        rs.state = LibStakingStorage.States.Paused;

        // After assigning reward epoch numbers, we increment the nextAvailableRewardEpochNumber to the next available number.
        gs.nextAvailableRewardEpochNumber =
            gs.nextAvailableRewardEpochNumber +
            3;

        // set sane defaults for min and max version
        rs.versionRequirements[0] = LibStakingStorage.Version({
            major: 0,
            minor: 0,
            patch: 0
        });
        rs.versionRequirements[1] = LibStakingStorage.Version({
            major: 10000,
            minor: 0,
            patch: 0
        });
        rs.litActionConfigs[0] = LibStakingStorage.LitActionConfig({
            timeoutMs: 30000,
            memoryLimitMb: 256,
            maxCodeLength: 16 * 1024 * 1024,
            maxResponseLength: 1024 * 100,
            maxConsoleLogLength: 1024 * 100,
            maxFetchCount: 75,
            maxSignCount: 10,
            maxContractCallCount: 30,
            maxBroadcastAndCollectCount: 30,
            maxCallDepth: 5,
            maxRetries: 3,
            asyncActionsEnabled: false
        });

        // set demerit threshold for rejoining
        gs.demeritRejoinThreshold = 10;

        rs.realmId = realmId;
    }

    function removeRealm(uint256 realmId) external {
        onlyOwner();
        // Immediately revert if the realm is the last one
        if (realms().numRealms() == 1) {
            revert("Cannot remove the last realm");
        }

        // Get current validators in realm
        EnumerableSet.AddressSet storage validatorsInRealm = realm(realmId)
            .validatorsInCurrentEpoch;

        // Revert if there are any validators in the realm
        if (validatorsInRealm.length() > 0) {
            revert("Realm has validators");
        }

        // Remove realm from storage
        EnumerableSet.remove(s().realmIds, realmId);
    }

    /// @notice Admin function to stake on behalf of a validator or delegating staker.
    function adminStakeForUser(
        address userStakerAddress,
        address operatorStakerAddress,
        uint256 timeLock,
        uint256 amount
    ) external nonReentrant {
        onlyOwner();
        bool isSelfStake = operatorStakerAddress == userStakerAddress;
        StakingUtilsLib.checkStakeParameters(isSelfStake, timeLock);
        StakingUtilsLib.checkStakeAmountMinMax(amount, isSelfStake);

        _createStakeRecord(
            amount,
            timeLock,
            operatorStakerAddress,
            userStakerAddress,
            CreateStakeRecordOpts({
                targetCurrentRewardEpoch: false,
                lastRewardEpochClaimedToSet: 0, // setting this to zero relies in the inner function to determine what this value should be
                unfreezeStartToSet: 0
            })
        );

        // if staking is done by a validator add them to the allValidators set
        if (isSelfStake) {
            if (!s().allValidators.contains(operatorStakerAddress)) {
                s().allValidators.add(operatorStakerAddress);
            }
        }

        // If this is the first stake this delegator has against this validator, then increment the unique delegating staker count.
        if (
            (userStakerAddress != operatorStakerAddress) &&
            (views().getStakeRecordCount(
                userStakerAddress,
                operatorStakerAddress
            ) == 1)
        ) {
            s().validators[operatorStakerAddress].uniqueDelegatingStakerCount++;
        }

        s().stakerToValidatorsTheyStakedTo[userStakerAddress].add(
            operatorStakerAddress
        );

        SafeERC20.safeTransferFrom(
            IERC20(views().getTokenContractAddress()),
            msg.sender, // Send from the admin's account
            address(this),
            amount
        );
    }

    /// @notice Admin function to unfreeze a stake on behalf of a delegating staker.
    function adminUnfreezeForUser(
        address userStakerAddress,
        address operatorStakerAddress,
        uint256 stakeId
    ) external {
        onlyOwner();
        // Do not allow unfreezing on behalf of validators.
        if (operatorStakerAddress == userStakerAddress) {
            revert("Cannot unfreeze on behalf of validators");
        }

        _unfreezeStake(userStakerAddress, operatorStakerAddress, stakeId);
    }

    function adminSetupShadowSplicing(
        uint256 source_realmId,
        uint256 target_realmId,
        address[] memory target_validators
    ) external {
        onlyOwner();
        // Example plan: src & target both have 10 validators, threshold = 6
        // 1. Epoch #1 Use 6 shadow nodes & 4 incoming validators for the first DKG
        // 2. Epoch #2 Replace 4 shadow nodes with 4 incoming validators ( preserving the threshold
        // 3. Epoch #3, replace remaining 2 shadow nodes with 2 incoming validators
        // Because only the 1st and 2nd threshold contains shadow nodes, we set up 2 structs to track the shadow nodes ->
        // validatorsForShadowCurrentEpoch & validatorsForShadowNextEpoch

        // get the validators in the source realm that will donate cryptographic material
        address[] memory sourceValidators = views().getValidatorsInCurrentEpoch(
            source_realmId
        );

        if (sourceValidators.length < 3) {
            revert("Minimum of 3 validators required in source realm.");
        }

        // check if the source and new validators counts are the same
        // strictly speaking, this isn't required, but it's simplifies the logic and maintains the threshold
        if (sourceValidators.length != target_validators.length) {
            revert("Source and new validators counts must be the same.");
        }

        // set the number validators that will be added to the target realm
        LibStakingStorage.RealmStorage storage targetRealm = realm(
            target_realmId
        );

        targetRealm.targetValidatorCount = sourceValidators.length;

        uint256 threshold = views().currentValidatorCountForConsensus(
            source_realmId
        );

        // check if there are enough new validators to replace at least the threshold number of shadow nodes
        if (threshold > target_validators.length) {
            revert("Not enough new validators to replace all shadow nodes");
        }

        StakingUtilsLib.checkValidatorCountAgainstKeySetsInRealm(
            target_realmId,
            threshold,
            1
        );

        // add the source validators to the target realm as shadow nodes
        for (uint256 i = 0; i < threshold; i++) {
            targetRealm.shadowValidators.add(sourceValidators[i]);
            targetRealm.shadowValidatorsInNextEpoch.add(sourceValidators[i]);
        }
        // add the validators that are to replace the shadow nodes into a set of waiting nodes
        for (uint256 i = 0; i < threshold; i++) {
            targetRealm.validatorsWaitingToReplaceShadowNodes.add(
                target_validators[i]
            );
        }

        // join in the new validators that are not shadow nodes, but will be filling in the validator set
        for (uint256 i = threshold; i < target_validators.length; i++) {
            addNodeForShadowSplicing(target_realmId, target_validators[i]);
        }

        // set a notifier that the current realm is being shadow spliced into the target realm
        s().realmToShadowRealm[source_realmId] = target_realmId;

        targetRealm.state = LibStakingStorage.States.NextValidatorSetLocked;
        emit StakingUtilsLib.StateChanged(realm(target_realmId).state);
    }

    function addNodeForShadowSplicing(
        uint256 target_realmId,
        address stakerAddress
    ) private {
        address[] memory stakerAddresses = new address[](1);
        stakerAddresses[0] = stakerAddress;
        validators().requestToJoinAsForShadowSplicing(
            target_realmId,
            stakerAddress
        );
    }

    function setDevopsAdmin(address newDevopsAdmin) external {
        onlyOwner();
        s().devopsAdmin = newDevopsAdmin;
        emit DevopsAdminSet(newDevopsAdmin);
    }

    function emitClearOfflinePhaseData(uint256 dataType) external {
        onlyOwnerOrDevopsAdmin();
        emit ClearOfflinePhaseData(dataType);
    }

    function emitCountOfflinePhaseData(uint256 dataType) external {
        onlyOwnerOrDevopsAdmin();
        emit CountOfflinePhaseData(dataType);
    }

    function adminSetValidatorRegisterAttestedWalletDisabled(
        address validatorAddress,
        bool disabled
    ) external {
        onlyOwner();
        s()
            .validators[validatorAddress]
            .registerAttestedWalletDisabled = disabled;
    }

    function setLitActionConfig(
        uint256 realmId,
        LibStakingStorage.LitActionConfig memory newConfig
    ) external {
        onlyOwnerOrDevopsAdmin();
        LibStakingStorage.RealmStorage storage realmStorage = realm(realmId);
        LibStakingStorage.LitActionConfig storage config = realmStorage
            .litActionConfigs[0];
        config.timeoutMs = newConfig.timeoutMs;
        config.memoryLimitMb = newConfig.memoryLimitMb;
        config.maxCodeLength = newConfig.maxCodeLength;
        config.maxResponseLength = newConfig.maxResponseLength;
        config.maxConsoleLogLength = newConfig.maxConsoleLogLength;
        config.maxFetchCount = newConfig.maxFetchCount;
        config.maxSignCount = newConfig.maxSignCount;
        config.maxContractCallCount = newConfig.maxContractCallCount;
        config.maxBroadcastAndCollectCount = newConfig
            .maxBroadcastAndCollectCount;
        config.maxCallDepth = newConfig.maxCallDepth;
        config.maxRetries = newConfig.maxRetries;
        config.asyncActionsEnabled = newConfig.asyncActionsEnabled;
    }

    /* ========== EVENTS ========== */

    event EpochLengthSet(uint256 realmId, uint256 newEpochLength);
    event EpochTimeoutSet(uint256 realmId, uint256 newEpochTimeout);
    event EpochEndTimeSet(uint256 realmId, uint256 newEpochEndTime);
    event KickPenaltyPercentSet(uint256 reason, uint256 newKickPenaltyPercent);
    event ValidatorRejoinedNextEpoch(address staker);
    event DevopsAdminSet(address newDevopsAdmin);
    event ClearOfflinePhaseData(uint256 dataType);
    event CountOfflinePhaseData(uint256 dataType);
    event ResolverContractAddressSet(address newResolverContractAddress);
}
