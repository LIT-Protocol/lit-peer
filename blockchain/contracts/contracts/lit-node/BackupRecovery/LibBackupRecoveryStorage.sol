//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { IPubkeyRouter } from "../../lit-node/PubkeyRouter/LibPubkeyRouterStorage.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

library LibBackupRecoveryStorage {
    using EnumerableSet for EnumerableSet.AddressSet;
    bytes32 constant BACKUP_RECOVERY_POSITION =
        keccak256("backuprecovery.storage");

    /**
     * @dev
     * Simply maps a backup party member to the node that generated their backup share
     */
    struct RecoveryPair {
        address backupPartyAddress;
        address nodeAddress;
        bool isRegistered;
    }

    /**
     * @dev
     *  Holds the current backup party state
     */
    struct BackupRecoveryState {
        bytes sessionId;
        uint256 partyThreshold;
        address[] partyMembers;
        RecoveryKey[] registeredRecoveryKeys;
    }

    struct RecoveryKey {
        bytes pubkey;
        uint256 keyType; // see rust/lit-node/src/tss/common/curve_type.rs 1 = BLS, 2 = K256, etc.  Not doing this in an enum so we can add more keytypes in the future without redeploying.
        // NOTE: DO NOT ADD ANYTHING TO THIS STRUCT SINCE IT IS NOT CONTAINED IN A MAPPING IN THE ROOT LEVEL STORAGE STRUCT
        // AND MAY RESULT IN STORAGE POINTERS SHIFTING.
    }

    /**
     * @dev
     * Holds the votes per Recovery pubKey
     */
    struct VotesToRegisterRecoveryKey {
        uint256 votes;
        mapping(address => bool) voted;
    }

    /**
     * @dev
     * Holds the next state of the backup party
     */
    struct NextState {
        address[] partyMembers;
        mapping(address => address) peerToBackupMemberMapping;
        uint nodesAssignedCount;
        mapping(address => address) backupMemberPeerMapping;
        mapping(address => bool) keysReceived;
        mapping(address => K256Proof) proofsReceived;
        // pubkey -> votes
        mapping(bytes => VotesToRegisterRecoveryKey) votesToRegisterRecoveryKeys;
        RecoveryKey[] registeredRecoveryKeys;
        bytes sessionId;
        uint256 partyThreshold;
        mapping(string => bool) keySetIdExists;
        string[] keySetIds;
    }

    struct K256Proof {
        bytes1 version;
        uint256 timestamp;
        bytes1 participantId;
        bytes schnorrProof;
        bytes schnorrVerification;
        bytes ecdsaProof;
        bytes ecdsaVerificationKey;
    }

    struct NextStateDownloadable {
        address[] partyMembers;
        RecoveryKey[] registeredRecoveryKeys;
        bytes sessionId;
    }

    struct BackupRecoveryStorage {
        bytes1 verificationVersion;
        // current state of the backup recovery
        mapping(uint256 => BackupRecoveryState) recoveryState;
        // holds the next party state until promoted to active
        mapping(uint256 => NextState) nextState;
        // holds past states after a new backup party is registered
        mapping(bytes => BackupRecoveryState) pastBackupStates;
        // proof submission mapping
        mapping(address => uint256[]) submittedProofs;
        // instance of the deployed contract resolver
        ContractResolver resolver;
        // env context
        ContractResolver.Env env;
        // Status of the recovering nodes
        mapping(uint256 => NodeRecoveryStatusMap[]) nodeStatusMap;
        // A mapping from the node address and peer id of a recovering node to the peer id
        // of the node which generated the the private shares that it recovered.
        // Necessary for the first DKG after the recovery
        // Use recovered_peer_ids[0] for now.
        mapping(uint256 => RecoveredPeerId[]) recovered_peer_ids;
    }

    function getStorage()
        internal
        pure
        returns (BackupRecoveryStorage storage storageStruct)
    {
        bytes32 position = BACKUP_RECOVERY_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }

    enum NodeRecoveryStatus {
        Null,
        StartedInRestoreState,
        BackupsAreLoaded,
        AllKeysAreRestored,
        AbandonedRecoveryDueToNetworkState
    }

    struct NodeRecoveryStatusMap {
        address node_address;
        NodeRecoveryStatus status;
    }

    struct RecoveredPeerId {
        address node_address;
        uint256 old_peer_id;
        uint256 new_peer_id;
    }
}
