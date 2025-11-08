//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { LibBackupRecoveryStorage } from "./LibBackupRecoveryStorage.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import "hardhat/console.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

contract BackupRecoveryTestFacet {
    function s()
        internal
        pure
        returns (LibBackupRecoveryStorage.BackupRecoveryStorage storage)
    {
        return LibBackupRecoveryStorage.getStorage();
    }

    /**
     * @dev
     * Sets the contract state by registering a new set of backup members
     */
    function setBackupPartyState(
        bytes[] calldata keys,
        address[] calldata partyMembers
    ) public {
        require(
            partyMembers.length > 1,
            "BackupRecovery: cannot vote for empty party set"
        );

        // Set a mock state
        delete s().recoveryState[0].registeredRecoveryKeys;
        for (uint i = 0; i < keys.length; i++) {
            s().recoveryState[0].registeredRecoveryKeys.push(
                LibBackupRecoveryStorage.RecoveryKey(keys[i], i + 1)
            );
        }

        if (partyMembers.length < 5) {
            s().recoveryState[0].partyThreshold = 3;
        } else {
            s().recoveryState[0].partyThreshold = (partyMembers.length * 2) / 3;
        }
        s().recoveryState[0].sessionId = "Test_session_id";
        s().recoveryState[0].partyMembers = partyMembers;
    }
}
