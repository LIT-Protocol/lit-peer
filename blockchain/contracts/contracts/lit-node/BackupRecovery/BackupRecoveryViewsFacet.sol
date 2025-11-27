//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { LibBackupRecoveryStorage } from "./LibBackupRecoveryStorage.sol";
import { StakingViewsFacet } from "./../Staking/StakingViewsFacet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

contract BackupRecoveryViewsFacet {
    function s()
        internal
        pure
        returns (LibBackupRecoveryStorage.BackupRecoveryStorage storage)
    {
        return LibBackupRecoveryStorage.getStorage();
    }

    /**
     * @dev
     * Returns the current recovery party state
     * If not all backup party members register their state the struct will be null
     */
    function getBackupPartyState()
        public
        view
        returns (LibBackupRecoveryStorage.BackupRecoveryState memory)
    {
        return s().recoveryState[0];
    }

    /**
     * @dev
     * Get the next set of backup party members.
     * if looking for the current set the `getBackupPartyState`
     * contains the current backup party members
     */
    function getNextBackupPartyMembers()
        public
        view
        returns (address[] memory backupMembers)
    {
        return s().nextState[0].partyMembers;
    }

    /**
     * @dev
     * get the state of a backup from a previous set of backup party members
     * sessionId is used for looking up the historic state.
     * If called before the state is made the non active state it will return a null struct
     * to get the most recent state use `getBackupPartyState`
     */
    function getPastBackupState(
        bytes memory sessionId
    )
        public
        view
        returns (LibBackupRecoveryStorage.BackupRecoveryState memory partyState)
    {
        return s().pastBackupStates[sessionId];
    }

    function getNextBackupState()
        public
        view
        returns (
            LibBackupRecoveryStorage.NextStateDownloadable memory nextState
        )
    {
        return
            LibBackupRecoveryStorage.NextStateDownloadable(
                s().nextState[0].partyMembers,
                s().nextState[0].registeredRecoveryKeys,
                s().nextState[0].sessionId
            );
    }

    /**
     * @dev
     * returns the mapped backup party member to a member of the peer set
     * this peer will contain the backup party members key share from dkg
     * msg.sender is the nodeAddress
     */
    function getMemberForNodeDkg() public view returns (address bp) {
        return s().nextState[0].peerToBackupMemberMapping[msg.sender];
    }

    /**
     * @dev
     * @helper
     * Returns whether all the Backup members have been mapped to a node operator
     */
    function allBackupMembersMapped() public view returns (bool mapped) {
        return
            s().nextState[0].partyMembers.length ==
            s().nextState[0].nodesAssignedCount;
    }

    /**
     * @dev
     * @helper
     * Helper function to check whether the current Recovery DKG has completed successfully
     */
    function isRecoveryDkgCompleted() public view returns (bool) {
        return s().nextState[0].registeredRecoveryKeys.length > 0;
    }

    /**
     * @dev
     * Returns the count of party members needed to perform recovery
     */
    function getDecryptionThreshold() public view returns (uint256) {
        if (s().nextState[0].partyMembers.length <= 5) {
            return 3;
        }
        return (s().nextState[0].partyMembers.length * 2) / 3;
    }

    /**
     * @dev
     * @helper
     * returns a boolean value relating to if a node is part of the dkg set
     * msg.sender is the nodeAddress
     */
    function isNodeForDkg() public view returns (bool inSet) {
        if (
            s().nextState[0].peerToBackupMemberMapping[msg.sender] != address(0)
        ) {
            return true;
        }

        return false;
    }

    function getNonSubmitingBackupMembersInNextState()
        public
        view
        returns (address[] memory missingRecoveryMembers)
    {
        LibBackupRecoveryStorage.NextState
            storage nextState = LibBackupRecoveryStorage.getStorage().nextState[
                0
            ];

        // Alloc the size of the whole backup party set on the stack
        // this allows us to not use sotrage and keep the method view
        // the returned array will contained zeroized elements for the indexes in
        // the array which map to the parties position in their storage array.
        address[] memory nonRegisteredPartyMembers = new address[](
            nextState.partyMembers.length
        );
        for (uint256 i = 0; i < nextState.partyMembers.length; i++) {
            if (!nextState.keysReceived[nextState.partyMembers[i]]) {
                nonRegisteredPartyMembers[i] = nextState.partyMembers[i];
            }
        }

        return nonRegisteredPartyMembers;
    }
}
