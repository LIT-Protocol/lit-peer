//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingAcrossRealmsFacet } from "./StakingAcrossRealmsFacet.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { console } from "hardhat/console.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";

contract StakingValidatorFacet {
    using EnumerableSet for EnumerableSet.AddressSet;

    // errors
    error ValidatorNotInNextEpoch(address staker);
    error ValidatorAlreadyInNextValidatorSet(address staker);
    error MustBeInActiveOrUnlockedState(LibStakingStorage.States state);
    error MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(
        LibStakingStorage.States state
    );
    error MustBeInNextValidatorSetLockedState(LibStakingStorage.States state);
    error MustBeInReadyForNextEpochState(LibStakingStorage.States state);
    error MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
        LibStakingStorage.States state
    );
    error NotEnoughValidatorsReadyForNextEpoch(
        uint256 currentReadyValidatorCount,
        uint256 nextReadyValidatorCount,
        uint256 minimumValidatorCountToBeReady
    );
    error CannotKickBelowCurrentValidatorThreshold();
    error CannotRejoinUntilNextEpochBecauseKicked(address stakingAddress);
    error CannotRejoinBecauseBanned(address stakingAddress);
    error ActiveValidatorsCannotLeave();
    error TryingToWithdrawMoreThanStaked(
        uint256 yourBalance,
        uint256 requestedWithdrawlAmount
    );
    error CouldNotMapNodeAddressToStakerAddress(address nodeAddress);
    error MustBeValidatorInNextEpochToKick(address stakerAddress);
    error CannotVoteTwice(address stakerAddress);
    error NotEnoughTimeElapsedSinceLastEpoch(
        uint256 currentTimestamp,
        uint256 epochEndTime
    );
    error NotEnoughTimeElapsedForTimeoutSinceLastEpoch(
        uint256 currentTimestamp,
        uint256 epochEndTime,
        uint256 timeout
    );
    error ValueMustBeNonzero(string valueName);
    error CannotWithdrawZero();
    error CannotReuseCommsKeys(uint256 senderPubKey, uint256 receiverPubKey);
    error ValidatorNotPermitted(address validatorAddress, uint256 realmId);
    error SignaledReadyForWrongEpochNumber(
        uint256 currentEpochNumber,
        uint256 receivedEpochNumber
    );
    error ValidatorAlreadyInRealm(address staker, uint256 existingRealmId);
    error StakerAddressMismatch(
        address senderAddress,
        address operatorAddress,
        address stakerAddress
    );
    error InvalidAttestedAddress();
    error ValidatorRegisterAttestedWalletDisabled();

    /* ========== VIEWS ========== */

    modifier onlyOwnerOrDevopsAdmin() {
        if (
            msg.sender != LibDiamond.contractOwner() &&
            msg.sender != s().devopsAdmin
        ) revert StakingUtilsLib.CallerNotOwnerOrDevopsAdmin();
        _;
    }

    function s()
        internal
        pure
        returns (LibStakingStorage.GlobalStakingStorage storage)
    {
        return LibStakingStorage.getStakingStorage();
    }

    function validators() internal view returns (StakingValidatorFacet) {
        return StakingValidatorFacet(getStakingContractAddress());
    }

    function getStakingContractAddress() internal view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.STAKING_CONTRACT(),
                s().env
            );
    }

    function mutableEpoch(
        uint256 realmId
    ) internal view returns (LibStakingStorage.Epoch storage) {
        return StakingUtilsLib.realm(realmId).epochs[0];
    }

    function mutableConfig(
        uint256 realmId
    ) internal view returns (LibStakingStorage.RealmConfig storage) {
        return StakingUtilsLib.realm(realmId).realm_configs[0];
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    /// Lock in the validators for the next epoch
    function lockValidatorsForNextEpoch(uint256 realmId) external {
        if (block.timestamp < mutableEpoch(realmId).endTime) {
            revert NotEnoughTimeElapsedSinceLastEpoch(
                block.timestamp,
                mutableEpoch(realmId).endTime
            );
        }
        if (
            !(StakingUtilsLib.realm(realmId).state ==
                LibStakingStorage.States.Active ||
                StakingUtilsLib.realm(realmId).state ==
                LibStakingStorage.States.Unlocked)
        ) {
            revert MustBeInActiveOrUnlockedState(
                StakingUtilsLib.realm(realmId).state
            );
        }

        StakingUtilsLib.checkNextSetAboveThreshold(realmId);

        StakingUtilsLib.realm(realmId).state = LibStakingStorage
            .States
            .NextValidatorSetLocked;
        emit StakingUtilsLib.StateChanged(StakingUtilsLib.realm(realmId).state);
    }

    /// After proactive secret sharing is complete, the nodes may signal that they are ready for the next epoch.  Note that this function is called by the node itself, and so LibERC2771._msgSender() is the nodeAddress and not the stakerAddress.
    function signalReadyForNextEpoch(
        uint256 realmId,
        uint256 epochNumber
    ) external {
        LibStakingStorage.Epoch storage epoch = mutableEpoch(realmId);
        if (epoch.number != epochNumber) {
            revert SignaledReadyForWrongEpochNumber(epoch.number, epochNumber);
        }

        LibStakingStorage.RealmStorage storage realm = StakingUtilsLib.realm(
            realmId
        );

        address stakerAddress = s().nodeAddressToStakerAddress[
            LibERC2771._msgSender()
        ];
        if (
            StakingUtilsLib.views().validatorSelfStakeWillExpire(
                realmId,
                stakerAddress,
                StakingUtilsLib
                    .realm(realmId)
                    .validatorsInCurrentEpoch
                    .contains(stakerAddress) //LibERC2771._msgSender())
            )
        ) {
            console.log(
                "validatorSelfStakeWillExpire.  Epoch number: %s",
                epochNumber
            );
            revert StakingFacet.InsufficientSelfStake(stakerAddress);
        }

        if (
            !(realm.state == LibStakingStorage.States.NextValidatorSetLocked ||
                realm.state == LibStakingStorage.States.ReadyForNextEpoch ||
                realm.state == LibStakingStorage.States.Restore)
        ) {
            revert MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(
                realm.state
            );
        }

        // at the first epoch, validatorsInCurrentEpoch is empty
        if (epoch.number != 1) {
            if (!realm.validatorsInNextEpoch.contains(stakerAddress)) {
                revert StakingUtilsLib.ValidatorIsNotInNextEpoch(
                    stakerAddress,
                    StakingUtilsLib.views().getValidatorsInNextEpoch(realmId)
                );
            }
        }

        realm.readyForNextEpoch[stakerAddress] = true;
        epoch.lastAdvanceVoteTime = block.timestamp;
        emit ReadyForNextEpoch(stakerAddress, epoch.number);

        if (StakingUtilsLib.views().isReadyForNextEpoch(realmId)) {
            realm.state = LibStakingStorage.States.ReadyForNextEpoch;
            emit StakingUtilsLib.StateChanged(realm.state);
        }
    }

    /// Advance to the next Epoch.  Rewards validators, adds the joiners, and removes the leavers
    function advanceEpoch(uint256 realmId) external {
        LibStakingStorage.Epoch storage mutableEpoch = mutableEpoch(realmId);
        LibStakingStorage.RealmStorage storage realmStorage = StakingUtilsLib
            .realm(realmId);

        if (block.timestamp < mutableEpoch.endTime) {
            revert NotEnoughTimeElapsedSinceLastEpoch(
                block.timestamp,
                mutableEpoch.endTime
            );
        }

        if (realmStorage.state != LibStakingStorage.States.ReadyForNextEpoch) {
            revert MustBeInReadyForNextEpochState(realmStorage.state);
        }
        // It's possible for an unusual timing event to occur where a one or more nodes have signaled ready
        // to advance the epoch, and a DKG is aborted by other nodes, and the further miss the signal that this occurred,
        // due to a network issue, or manual intervention.
        // If the network "splits" on the state ( some waiting for other nodes to signalReady, and others attempting to start
        // another round of DKG ), the network remains up, but proactive secret sharing is paused until we intervene manually.
        // This check creates a 60 second window ( post DKG completion by any node ), for everyone else to complete,
        // otherwise it resets the network state.
        // Internal DKG retries ( aka DKG LifeCycles ) are unaffected by this change.

        uint voteToAdvanceTimeOut = 60; // after completing DKG, the time required to vote to advance the epoch.
        // This comparions is <= to allow us to more accurately test the functionality in CI.
        if (
            mutableEpoch.lastAdvanceVoteTime > 0 &&
            mutableEpoch.lastAdvanceVoteTime +
                s().globalConfig[0].voteToAdvanceTimeOut <=
            block.timestamp
        ) {
            StakingUtilsLib.unlockEpoch(realmId);
            emit VoteToAdvanceTimeOutElapsed(realmId);
            return; // we can't advance the epoch yet, because the timeout has not elapsed.
        }

        if (realmStorage.state != LibStakingStorage.States.ReadyForNextEpoch) {
            revert MustBeInReadyForNextEpochState(realmStorage.state);
        }

        if (!StakingUtilsLib.views().isReadyForNextEpoch(realmId)) {
            revert NotEnoughValidatorsReadyForNextEpoch(
                StakingUtilsLib
                    .views()
                    .countOfCurrentValidatorsReadyForNextEpoch(realmId),
                StakingUtilsLib.views().countOfNextValidatorsReadyForNextEpoch(
                    realmId
                ),
                StakingUtilsLib.views().currentValidatorCountForConsensus(
                    realmId
                )
            );
        }

        // Check all pending rejoins
        uint256 i = 0;
        while (i < s().pendingRejoins.length) {
            address pending = s().pendingRejoins[i].addr;
            uint256 kickTimestamp = s().pendingRejoins[i].timestamp;

            // If a kicked node didn't rejoin in time, slash their stake and ban them
            if (block.timestamp >= kickTimestamp + s().pendingRejoinTimeout) {
                // Slash their stake
                uint256 kickPenaltyPercent = s()
                    .complaintReasonToConfig[uint256(1)]
                    .kickPenaltyPercent;
                StakingUtilsLib.slashValidator(kickPenaltyPercent, pending);

                // Ban them for good
                s().demeritsCounter[pending] = s().demeritRejoinThreshold;

                // shame them with an event
                emit StakingUtilsLib.ValidatorBanned(pending);

                // Remove this entry by swapping with last element and popping
                s().pendingRejoins[i] = s().pendingRejoins[
                    s().pendingRejoins.length - 1
                ];
                s().pendingRejoins.pop();
                s().isValidatorInPendingRejoin[pending] = false;
                // Don't increment i since we swapped in a new element
            } else {
                // If not past timeout, move to next one
                i++;
            }
        }

        // Update parameters for the validator who are either dealt out or kicked from the next epoch
        for (i = 0; i < realmStorage.validatorsInCurrentEpoch.length(); i++) {
            address validator = realmStorage.validatorsInCurrentEpoch.at(i);
            // If the validator is not in the next epoch, update their parameters
            if (!realmStorage.validatorsInNextEpoch.contains(validator)) {
                s().validatorToCurrentRealmId[validator] = 0;
                s().validatorToNextRealmId[validator] = 0;
                s().allActiveValidators.remove(validator);

                // We allow inactive validators to register attested wallets.
                s()
                    .validators[validator]
                    .registerAttestedWalletDisabled = false;
            }
        }

        // reward the validators
        // copy validators from next epoch to current epoch
        uint256 validatorLength = realmStorage.validatorsInNextEpoch.length();
        StakingUtilsLib.updateRewardEpoch(realmId);

        // clear out validators in current epoch
        StakingUtilsLib.clearEnumerableAddressSet(
            realmStorage.validatorsInCurrentEpoch
        );

        for (i = 0; i < validatorLength; i++) {
            address validator = realmStorage.validatorsInNextEpoch.at(i);
            realmStorage.validatorsInCurrentEpoch.add(validator);
            // clear out readyForNextEpoch
            realmStorage.readyForNextEpoch[validator] = false;

            s().validators[validator].lastActiveEpoch = mutableEpoch.number + 1;

            // We disallow active validators from registering attested wallets.
            s().validators[validator].registerAttestedWalletDisabled = true;

            // decrement demerits counter
            if (
                s().demeritsCounter[validator] > 0 &&
                s().demeritsCounter[validator] < s().demeritRejoinThreshold
            ) {
                s().demeritsCounter[validator]--;
            }

            // Now that the epoch actually succeeded advancing, we can properly set the realmId and the last
            // reward epoch number for each validator.
            s().validatorToCurrentRealmId[validator] = realmId;
            s().validatorToNextRealmId[validator] = realmId;
            s().validators[validator].lastRewardEpoch = mutableEpoch
                .rewardEpochNumber;
        }

        // clear out the validators kicked from next epoch
        StakingUtilsLib.clearEnumerableAddressSet(
            realmStorage.validatorsKickedFromNextEpoch
        );

        // clear out the current validators kicked from next epoch
        StakingUtilsLib.clearEnumerableAddressSet(
            realmStorage.currentValidatorsKickedFromNextEpoch
        );

        mutableEpoch.number++;
        emit AdvancedEpoch(realmId, mutableEpoch.number);

        mutableEpoch.endTime = block.timestamp + mutableEpoch.epochLength;
        mutableEpoch.startTime = block.timestamp;

        // Shift the nextRewardEpochNumber (already claimed by this epoch) as the current reward epoch number
        mutableEpoch.rewardEpochNumber = mutableEpoch.nextRewardEpochNumber;
        // Claim the next reward epoch number
        mutableEpoch.nextRewardEpochNumber = s().nextAvailableRewardEpochNumber;
        // Now increment the global next available reward epoch number that is up for grabs
        s().nextAvailableRewardEpochNumber++;

        realmStorage.state = LibStakingStorage.States.Active;

        // deal with shadow validators by adding in new nodes waiting to replace shadow nodes
        // this is done right at the end, because we need the state to be Active before we do this
        if (realmStorage.shadowValidators.length() > 0) {
            console.log("Adding shadow validators to the validator set");
            uint256 validatorsToAdd = realmStorage.targetValidatorCount -
                StakingUtilsLib.views().currentValidatorCountForConsensus(
                    realmId
                );

            for (i = 0; i < validatorsToAdd; i++) {
                // if there are still nodes waiting to replace shadow nodes, add them to the validator set
                // nodes can drop out in the middle of a Shadow splicing process, and we don't want to
                // cause the function to panic...
                // This check also catches the case where the number of nodes waiting to replace shadow nodes is less than the number of validators to add ( ie, we skip a check )
                if (
                    realmStorage
                        .validatorsWaitingToReplaceShadowNodes
                        .length() > 0
                ) {
                    address nextValidatorAddress = realmStorage
                        .validatorsWaitingToReplaceShadowNodes
                        .at(0);
                    executeRequestToJoin(realmId, nextValidatorAddress);
                    realmStorage.validatorsWaitingToReplaceShadowNodes.remove(
                        nextValidatorAddress
                    );
                }
            }

            // remove shadow validators from the validator set if the number of validators in the validator set is greater than the target validator count

            for (i = 0; i < realmStorage.shadowValidators.length(); i++) {
                if (
                    (realmStorage.validatorsInCurrentEpoch.length() +
                        realmStorage.shadowValidators.length()) >
                    realmStorage.targetValidatorCount
                ) {
                    realmStorage.shadowValidators.remove(
                        realmStorage.shadowValidators.at(0)
                    );
                }

                if (
                    realmStorage.validatorsInNextEpoch.length() +
                        realmStorage.shadowValidatorsInNextEpoch.length() >
                    realmStorage.targetValidatorCount
                ) {
                    realmStorage.shadowValidatorsInNextEpoch.remove(
                        realmStorage.shadowValidatorsInNextEpoch.at(0)
                    );
                }
            }

            // no shadow validators left, we can remove the shadow splicing state
            if (realmStorage.shadowValidators.length() == 0) {
                s().realmToShadowRealm[realmId] = 0;
                mutableEpoch.epochLength = s()
                    .globalConfig[0]
                    .rewardEpochDuration;
                emit EpochLengthSet(
                    realmId,
                    s().globalConfig[0].rewardEpochDuration
                );
            }
        }

        emit StakingUtilsLib.StateChanged(realmStorage.state);
    }

    function requestToJoin(uint256 realmId) public {
        executeRequestToJoin(realmId, LibERC2771._msgSender());
    }

    function requestToJoinAsNode(
        uint256 realmId,
        address stakerAddress
    ) public {
        // Check that the staker address is correctly mapped to.
        address resolvedAddress = StakingUtilsLib
            .views()
            .nodeAddressToStakerAddress(LibERC2771._msgSender());
        if (resolvedAddress != stakerAddress) {
            revert StakerAddressMismatch(
                LibERC2771._msgSender(),
                resolvedAddress,
                stakerAddress
            );
        }
        if (stakerAddress == address(0)) {
            revert CouldNotMapNodeAddressToStakerAddress(
                LibERC2771._msgSender()
            );
        }
        executeRequestToJoin(realmId, stakerAddress);
    }

    function requestToJoinAsAdmin(
        uint256 realmId,
        address stakerAddress
    ) external onlyOwnerOrDevopsAdmin {
        executeRequestToJoin(realmId, stakerAddress);
    }

    /// @notice Function to request to join the validator set as a shadow node for shadow splicing
    function requestToJoinAsForShadowSplicing(
        uint256 realmId,
        address stakerAddress
    ) external {
        if (msg.sender != address(this)) {
            revert("Only the Staking contract can call this function");
        }

        executeRequestToJoin(realmId, stakerAddress);
    }

    function checkActiveOrUnlockedOrPausedState(
        LibStakingStorage.States state
    ) public pure {
        if (
            !(state == LibStakingStorage.States.Active ||
                state == LibStakingStorage.States.Unlocked ||
                state == LibStakingStorage.States.Paused)
        ) {
            revert StakingUtilsLib.MustBeInActiveOrUnlockedOrPausedState(state);
        }
    }

    function executeRequestToJoin(
        uint256 realmId,
        address stakerAddress
    ) private {
        // If permittedValidatorsOn is set, we need to check that the staker address is permitted to join.
        if (
            StakingUtilsLib.views().realmConfig(realmId).permittedValidatorsOn
        ) {
            if (
                !StakingUtilsLib.realm(realmId).permittedValidators.contains(
                    stakerAddress
                )
            ) {
                revert ValidatorNotPermitted(stakerAddress, realmId);
            }
        }

        // check all realms to see if the staker is already here ...
        // if they are, then we need to remove them from that realm before adding them
        uint256 existingRealmId = s().validatorToCurrentRealmId[stakerAddress];
        if (existingRealmId != 0) {
            revert ValidatorAlreadyInRealm(
                LibERC2771._msgSender(),
                existingRealmId
            );
        }

        // Revert if the validator is already in the next valiator set for this realmId.
        // There is an attack vector where Mallory can call requestToJoinAsNode while specifying
        // another peer's stakerAddress which would override the nodeAddressToStakerAddress
        // (and the reciprocal stakerAddressToNodeAddress) mapping, and this revert would prevent
        // this from happening.
        LibStakingStorage.RealmStorage storage realmStorage = StakingUtilsLib
            .realm(realmId);
        if (realmStorage.validatorsInNextEpoch.contains(stakerAddress)) {
            revert ValidatorAlreadyInNextValidatorSet(stakerAddress);
        }

        // Now that we know which realm the operator staker is requesting to join, we can initialize the next
        // reward epoch. Since the staker is new to the validator set, the next reward epoch number is also set
        // with initial as true.
        StakingUtilsLib.setupNextRewardEpoch(realmId, stakerAddress, true);

        require(
            StakingUtilsLib.staking().checkStakingAmounts(stakerAddress),
            "Staking amount is not valid"
        );

        checkActiveOrUnlockedOrPausedState(realmStorage.state);

        // make sure they haven't been banned
        if (s().demeritsCounter[stakerAddress] >= s().demeritRejoinThreshold) {
            revert CannotRejoinBecauseBanned(stakerAddress);
        }

        // make sure they haven't been kicked
        if (
            realmStorage.validatorsKickedFromNextEpoch.contains(stakerAddress)
        ) {
            revert CannotRejoinUntilNextEpochBecauseKicked(stakerAddress);
        }

        // Remove any pending rejoin entry for this staker
        for (uint256 i = 0; i < s().pendingRejoins.length; i++) {
            if (s().pendingRejoins[i].addr == stakerAddress) {
                // Remove by swapping with last element and popping
                s().pendingRejoins[i] = s().pendingRejoins[
                    s().pendingRejoins.length - 1
                ];
                s().pendingRejoins.pop();
                s().isValidatorInPendingRejoin[stakerAddress] = false;
                break;
            }
        }

        LibStakingStorage.Validator storage validator = s().validators[
            stakerAddress
        ];

        // Set the last realm ID before _setValidatorInfo so that the used comms keys check
        // will work.
        validator.lastRealmId = realmId;

        if (!s().allValidators.contains(stakerAddress)) {
            s().allValidators.add(stakerAddress);
        }

        if (!s().allActiveValidators.contains(stakerAddress)) {
            s().allActiveValidators.add(stakerAddress);
        }

        // Add to validatorsInNextEpoch
        realmStorage.validatorsInNextEpoch.add(stakerAddress);

        s().validatorToNextRealmId[stakerAddress] = realmId;

        if (
            StakingUtilsLib.views().validatorSelfStakeWillExpire(
                realmId,
                stakerAddress,
                false
            )
        ) {
            revert StakingFacet.InsufficientSelfStake(stakerAddress);
        }

        emit RequestToJoin(stakerAddress);
    }

    /// @notice This will be called using the node operator wallet (unattested).
    function registerAttestedWallet(
        address stakerAddress,
        address attestedAddress,
        bytes calldata attestedPubKey,
        uint256 senderPubKey,
        uint256 receiverPubKey
    ) external {
        require(attestedPubKey.length == 65, "Invalid uncompressed key length");
        uint8 prefix = uint8(attestedPubKey[0]);
        require(prefix == 0x04, "Invalid uncompressed key prefix");

        // Check that the staker address is correct.
        address resolvedAddress = StakingUtilsLib
            .views()
            .operatorAddressToStakerAddress(msg.sender);
        if (resolvedAddress != stakerAddress) {
            revert StakerAddressMismatch(
                msg.sender,
                resolvedAddress,
                stakerAddress
            );
        } else if (
            StakingUtilsLib.views().nodeAddressToStakerAddress(
                attestedAddress
            ) !=
            address(0) &&
            attestedAddress != msg.sender
        ) {
            revert InvalidAttestedAddress();
        }

        if (senderPubKey == 0) {
            revert ValueMustBeNonzero("senderPubKey");
        }
        if (receiverPubKey == 0) {
            revert ValueMustBeNonzero("receiverPubKey");
        }

        LibStakingStorage.Validator storage validator = s().validators[
            stakerAddress
        ];

        if (validator.registerAttestedWalletDisabled) {
            revert ValidatorRegisterAttestedWalletDisabled();
        }
        if (validator.lastRealmId != 0) {
            // Skip check if the keys are the same
            if (
                !(senderPubKey == validator.senderPubKey &&
                    receiverPubKey == validator.receiverPubKey)
            ) {
                LibStakingStorage.RealmStorage
                    storage realmStorage = StakingUtilsLib.realm(
                        validator.lastRealmId
                    );
                bytes32 commsKeysHash = keccak256(
                    abi.encodePacked(senderPubKey, receiverPubKey)
                );
                if (realmStorage.usedCommsKeys[commsKeysHash]) {
                    revert CannotReuseCommsKeys(senderPubKey, receiverPubKey);
                }
                realmStorage.usedCommsKeys[commsKeysHash] = true;
            }
        }

        uint256 x;
        uint256 y;
        assembly {
            x := calldataload(add(attestedPubKey.offset, 1))
            y := calldataload(add(attestedPubKey.offset, 33))
        }

        validator.senderPubKey = senderPubKey;
        validator.receiverPubKey = receiverPubKey;
        validator.nodeAddress = attestedAddress;
        s().stakerAddressToNodeAddress[stakerAddress] = attestedAddress;
        s().nodeAddressToStakerAddress[attestedAddress] = stakerAddress;
        s().attestedAddressToPubKey[attestedAddress] = LibStakingStorage
            .UncompressedK256Key(x, y);
        emit AttestedWalletRegistered(
            stakerAddress,
            attestedAddress,
            LibStakingStorage.UncompressedK256Key(x, y)
        );
    }

    /// Exit staking and get any outstanding rewards
    function exit() external pure {
        //    "Not implemented - check the docs to validate a proper withdrawl process."
        revert ActiveValidatorsCannotLeave();
    }

    /// If more than the threshold of validators vote to kick someone, kick them.
    /// It's expected that this will be called by the node directly, so LibERC2771._msgSender() will be the nodeAddress
    function kickValidatorInNextEpoch(
        address validatorToKickStakerAddress,
        uint256 reason,
        bytes calldata data
    ) external {
        address stakerAddressOfSender = s().nodeAddressToStakerAddress[
            LibERC2771._msgSender()
        ];

        // Sanity check
        if (stakerAddressOfSender == address(0)) {
            revert CouldNotMapNodeAddressToStakerAddress(
                LibERC2771._msgSender()
            );
        }

        uint256 realmId = StakingUtilsLib.realms().getRealmIdForStakerAddress(
            stakerAddressOfSender
        );
        LibStakingStorage.RealmStorage storage realmStorage = StakingUtilsLib
            .realm(realmId);

        if (
            !realmStorage.validatorsInNextEpoch.contains(stakerAddressOfSender)
        ) {
            revert MustBeValidatorInNextEpochToKick(stakerAddressOfSender);
        }
        if (
            realmStorage
            .votesToKickValidatorsInNextEpoch[mutableEpoch(realmId).number][
                validatorToKickStakerAddress
            ].voted[stakerAddressOfSender]
        ) {
            revert CannotVoteTwice(stakerAddressOfSender);
        }

        // A threshold number of validators from the current validator set MUST NOT
        // be kicked in order for DKG resharing to be successful.
        // This is only valid for epoch 2+ since epoch 1 has no current validator set,
        // and if we enforce this in epoch 1, we are effectively prohibiting any votes
        // to kick.
        bool isValidatorInCurrentSet = realmStorage
            .validatorsInCurrentEpoch
            .contains(validatorToKickStakerAddress);
        if (
            StakingUtilsLib.views().epoch(realmId).number > 1 &&
            realmStorage.currentValidatorsKickedFromNextEpoch.length() >=
            (StakingUtilsLib
                .views()
                .getValidatorsInCurrentEpoch(realmId)
                .length -
                StakingUtilsLib.views().currentValidatorCountForConsensus(
                    realmId
                ))
        ) {
            revert CannotKickBelowCurrentValidatorThreshold();
        }

        LibStakingStorage.Epoch memory currentEpoch = mutableEpoch(realmId);
        // Vote to kick
        // Increment the number of votes
        realmStorage
        .votesToKickValidatorsInNextEpoch[currentEpoch.number][
            validatorToKickStakerAddress
        ].votes++;
        // Mark the staker as having voted
        realmStorage
        .votesToKickValidatorsInNextEpoch[currentEpoch.number][
            validatorToKickStakerAddress
        ].voted[stakerAddressOfSender] = true;

        if (
            realmStorage.validatorsInNextEpoch.contains(
                validatorToKickStakerAddress
            ) &&
            StakingUtilsLib.views().shouldKickValidator(
                realmId,
                validatorToKickStakerAddress
            )
        ) {
            // block them from rejoining the next epoch
            realmStorage.validatorsKickedFromNextEpoch.add(
                validatorToKickStakerAddress
            );
            // mark them if they are in the current validator set
            if (isValidatorInCurrentSet) {
                realmStorage.currentValidatorsKickedFromNextEpoch.add(
                    validatorToKickStakerAddress
                );
            }

            // shame them with an event
            emit StakingUtilsLib.ValidatorKickedFromNextEpoch(
                validatorToKickStakerAddress
            );

            // increment demerits counter
            uint256 kickPenaltyDemerits = s()
                .complaintReasonToConfig[reason]
                .kickPenaltyDemerits;
            s().demeritsCounter[
                validatorToKickStakerAddress
            ] += kickPenaltyDemerits;

            // If they accrue too many demerits, ban them for good and slash their stake
            if (
                s().demeritsCounter[validatorToKickStakerAddress] >=
                s().demeritRejoinThreshold
            ) {
                // slash the stake
                uint256 kickPenaltyPercent = s()
                    .complaintReasonToConfig[reason]
                    .kickPenaltyPercent;
                StakingUtilsLib.slashValidator(
                    kickPenaltyPercent,
                    validatorToKickStakerAddress
                );

                // shame them with an event
                emit StakingUtilsLib.ValidatorBanned(
                    validatorToKickStakerAddress
                );
            } else {
                // They haven't accrued enough demerits to be banned, so they can rejoin in the next epoch
                // Mark them as pending rejoin
                s().pendingRejoins.push(
                    LibStakingStorage.PendingRejoin({
                        addr: validatorToKickStakerAddress,
                        timestamp: block.timestamp
                    })
                );
                s().isValidatorInPendingRejoin[
                    validatorToKickStakerAddress
                ] = true;
            }

            // if we're in the locked state, then we need to unlock, because we kicked a node
            if (
                realmStorage.state ==
                LibStakingStorage.States.NextValidatorSetLocked ||
                realmStorage.state == LibStakingStorage.States.ReadyForNextEpoch
            ) {
                unlockEpoch(realmId);
            } else if (realmStorage.state == LibStakingStorage.States.Active) {
                // if we're in the active state, then we need to lock, because we kicked a node
                // we want to kick off the next epoch transition to remove this node from the set

                // check that it's safe to move to locked
                StakingUtilsLib.checkNextSetAboveThreshold(realmId);

                realmStorage.state = LibStakingStorage
                    .States
                    .NextValidatorSetLocked;
                emit StakingUtilsLib.StateChanged(
                    StakingUtilsLib.realm(realmId).state
                );
                // change the epoch end time to now
                StakingUtilsLib.mutableEpoch(realmId).endTime = block.timestamp;
            }

            // This must be the last thing we do, in order to check realms correctly in other functions
            // remove them from the validator set
            StakingUtilsLib.removeValidatorFromNextEpoch(
                realmId,
                validatorToKickStakerAddress
            );
        }

        emit VotedToKickValidatorInNextEpoch(
            stakerAddressOfSender,
            validatorToKickStakerAddress,
            reason,
            data
        );
    }

    /// Set the IP and port of your node
    /// @param ip The ip address of your node
    /// @param port The port of your node
    /// @param operatorAddress The address of the node operator's wallet
    function setIpPortNodeAddress(
        uint32 ip,
        uint128 ipv6,
        uint32 port,
        address operatorAddress
    ) external {
        _setValidatorInfo(
            s().validators[LibERC2771._msgSender()],
            ip,
            ipv6,
            port,
            operatorAddress,
            LibERC2771._msgSender()
        );

        if (!s().allValidators.contains(LibERC2771._msgSender())) {
            s().allValidators.add(LibERC2771._msgSender());
        }
    }

    function _setValidatorInfo(
        LibStakingStorage.Validator storage validator,
        uint32 ip,
        uint128 ipv6,
        uint32 port,
        address operatorAddress,
        address stakerAddress
    ) internal {
        // check for zeroes
        if (ip == 0) {
            revert ValueMustBeNonzero("ip");
        }
        if (port == 0) {
            revert ValueMustBeNonzero("port");
        }
        if (operatorAddress == address(0)) {
            revert ValueMustBeNonzero("operatorAddress");
        }

        validator.ip = ip;
        validator.ipv6 = ipv6;
        validator.port = port;
        // We set the nodeAddress to the operator address for now, since it will be overwritten
        // when the node boots and registers the attested wallet.
        validator.nodeAddress = operatorAddress;
        validator.operatorAddress = operatorAddress;

        // Don't let them overwrite an existing mapping
        // because it could belong to someone else. But
        // we let them create a new mapping.
        if (s().nodeAddressToStakerAddress[operatorAddress] == address(0)) {
            s().nodeAddressToStakerAddress[operatorAddress] = stakerAddress;
            s().stakerAddressToNodeAddress[stakerAddress] = operatorAddress;
        }
        if (s().operatorAddressToStakerAddress[operatorAddress] == address(0)) {
            s().operatorAddressToStakerAddress[operatorAddress] = stakerAddress;
        }
    }

    function unlockEpoch(uint256 realmId) internal {
        // this should only be callable from the ReadyForNextEpoch state or the NextValidatorSetLocked state
        if (
            !(StakingUtilsLib.realm(realmId).state ==
                LibStakingStorage.States.ReadyForNextEpoch ||
                StakingUtilsLib.realm(realmId).state ==
                LibStakingStorage.States.NextValidatorSetLocked)
        ) {
            revert MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
                StakingUtilsLib.realm(realmId).state
            );
        }
        // clear out readyForNextEpoch for current nodes
        uint256 validatorLength = StakingUtilsLib
            .realm(realmId)
            .validatorsInCurrentEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            StakingUtilsLib.realm(realmId).readyForNextEpoch[
                StakingUtilsLib.realm(realmId).validatorsInCurrentEpoch.at(i)
            ] = false;
        }

        // clear out readyForNextEpoch for next nodes
        validatorLength = StakingUtilsLib
            .realm(realmId)
            .validatorsInNextEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            StakingUtilsLib.realm(realmId).readyForNextEpoch[
                StakingUtilsLib.realm(realmId).validatorsInNextEpoch.at(i)
            ] = false;
        }

        StakingUtilsLib.realm(realmId).state = LibStakingStorage
            .States
            .Unlocked;
        StakingUtilsLib.realm(realmId).epochs[0].retries++;
        emit StakingUtilsLib.StateChanged(StakingUtilsLib.realm(realmId).state);
    }

    /**
     * @notice Request to leave the validator set
     * @dev This function is called by the staker itself, and so LibERC2771._msgSender() is the stakerAddress and not the nodeAddress.
     */
    function requestToLeave() external {
        address stakerAddress = LibERC2771._msgSender();
        uint256 realmId = StakingUtilsLib.realms().getRealmIdForStakerAddress(
            LibERC2771._msgSender()
        ); // reverts if staker address not found.
        stakerRequestToLeave(realmId, stakerAddress);
    }

    /**
     * @notice Request to leave the validator set
     * @dev This function is called by the node itself, and so LibERC2771._msgSender() is the nodeAddress and not the stakerAddress.
     */
    function requestToLeaveAsNode(uint256 realmId) external {
        address nodeAddress = LibERC2771._msgSender();
        address stakerAddress = StakingUtilsLib
            .realms()
            .nodeAddressToStakerAddressAcrossRealms(nodeAddress);
        stakerRequestToLeave(realmId, stakerAddress);
    }

    /**
     * @notice Request to leave the validator set
     * @param stakerAddress The address of the staker
     * @param realmId The ID of the realm
     */
    function stakerRequestToLeave(
        uint256 realmId,
        address stakerAddress
    ) internal {
        checkActiveOrUnlockedOrPausedState(
            StakingUtilsLib.realm(realmId).state
        );

        if (
            StakingUtilsLib.realm(realmId).validatorsInNextEpoch.length() - 1 <
            s().globalConfig[0].minimumValidatorCount
        ) {
            revert StakingUtilsLib.NotEnoughValidatorsInNextEpoch(
                StakingUtilsLib.realm(realmId).validatorsInNextEpoch.length(),
                s().globalConfig[0].minimumValidatorCount
            );
        }
        StakingUtilsLib.removeValidatorFromNextEpoch(realmId, stakerAddress);

        // ensure this won't drop us below the minimum validator count.
        // technically, if we would drop below the threshold in the next set due to this node leaving,
        // it should be okay, since this node is "gracefully" leaving and participating in the Reshare.
        // but we still need to prevent it from dropping below the threshold due to kicks.
        StakingUtilsLib.checkNextSetAboveThreshold(realmId);
        emit RequestToLeave(stakerAddress);
    }

    /* ========== INTERNAL FUNCTIONS ========== */

    /*
     * @notice Internal version of getRewardEpoch for returning a storage reference
     */
    function _getRewardEpoch(
        address stakerAddress,
        uint256 rewardEpochNumber
    ) internal returns (LibStakingStorage.RewardEpoch storage) {
        LibStakingStorage.RewardEpoch storage rewardEpoch = s().rewardEpochs[
            stakerAddress
        ][rewardEpochNumber];

        if (rewardEpoch.epochEnd == 0) {
            StakingUtilsLib.staking().initializeRewardEpoch(
                stakerAddress,
                rewardEpochNumber,
                false
            );
            rewardEpoch = s().rewardEpochs[stakerAddress][rewardEpochNumber];
        }

        return rewardEpoch;
    }

    /* ========== EVENTS ========== */

    event RewardsDurationUpdated(uint256 newDuration);
    event RequestToJoin(address indexed staker);
    event RequestToLeave(address indexed staker);
    event AttestedWalletRegistered(
        address indexed staker,
        address indexed attestedAddress,
        LibStakingStorage.UncompressedK256Key indexed attestedPubKey
    );
    event Recovered(address token, uint256 amount);
    event ReadyForNextEpoch(address indexed staker, uint256 epochNumber);
    event VotedToKickValidatorInNextEpoch(
        address indexed reporter,
        address indexed validatorToKickStakerAddress,
        uint256 indexed reason,
        bytes data
    );

    event EpochLengthSet(uint256 realmId, uint256 newEpochLength);
    event StakingTokenSet(address newStakingTokenAddress);
    event KickPenaltyPercentSet(uint256 reason, uint256 newKickPenaltyPercent);
    event ResolverContractAddressSet(address newResolverContractAddress);
    event ComplaintConfigSet(
        uint256 reason,
        LibStakingStorage.ComplaintConfig config
    );
    event ValidatorRejoinedNextEpoch(address staker);
    event AdvancedEpoch(uint256 realmId, uint256 epochNumber);
    event DebugEvent(string message, address sender, uint256 value);
    event VoteToAdvanceTimeOutElapsed(uint256 realmId);
}
