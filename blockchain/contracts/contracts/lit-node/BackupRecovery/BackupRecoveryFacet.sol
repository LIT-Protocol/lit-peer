//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibBackupRecoveryStorage } from "./LibBackupRecoveryStorage.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";
import { BackupRecoveryViewsFacet } from "./BackupRecoveryViewsFacet.sol";

contract BackupRecoveryFacet {
    // hardcoded realm id 1 for now
    uint256 public constant REALM_ID = 1;
    address public constant BASE_EC_OP_ADDRESS =
        0x000000000000000000000000000000000000012D;
    bytes32 HASH_SHA2_256 = sha256("sha2-256");

    // errors
    error CallerNotOwner();
    error BackupStateAlreadyRegistered(bytes pubkey);
    error NodesAllMappedToBackupMembers(address addr);
    error BackupStateNotRegistered();
    error InvalidCaller(address addr);
    error BackupMemberNotMappedToNode(address peer);
    error BackupKeysMismatch(
        bytes sessionId,
        bytes senderSessionId,
        bytes32 pubkeysHash,
        bytes32 sendersPubkeyHash
    );
    error BackupSetIncomplete(address[] members);
    error WrongVerificationVersion();
    error ProofExpired();

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    function s()
        internal
        pure
        returns (LibBackupRecoveryStorage.BackupRecoveryStorage storage)
    {
        return LibBackupRecoveryStorage.getStorage();
    }

    function ss()
        internal
        pure
        returns (LibStakingStorage.GlobalStakingStorage storage)
    {
        return LibStakingStorage.getStakingStorage();
    }

    function _getStakingViewsFacet() public view returns (StakingViewsFacet) {
        address stakingAddress = s().resolver.getContract(
            s().resolver.STAKING_CONTRACT(),
            s().env
        );
        return StakingViewsFacet(stakingAddress);
    }

    function views() internal view returns (BackupRecoveryViewsFacet) {
        return BackupRecoveryViewsFacet(address(this));
    }

    function _checkValidatorSetForAddress(
        address sender
    ) public view returns (bool) {
        address[] memory validators = _getStakingViewsFacet()
            .getValidatorsInCurrentEpoch(REALM_ID);
        address stakerAddressOfSender = _getStakingViewsFacet()
            .nodeAddressToStakerAddress(sender);

        for (uint256 i = 0; i < validators.length; i++) {
            if (validators[i] == stakerAddressOfSender) {
                return true;
            }
        }

        return false;
    }

    function _checkBackupSetForSender(
        address sender
    ) internal view returns (bool) {
        for (uint256 i = 0; i < s().nextState[0].partyMembers.length; i++) {
            if (sender == s().nextState[0].partyMembers[i]) {
                return true;
            }
        }

        return false;
    }

    function _updateRecoveryState() internal {
        s().recoveryState[0].partyThreshold = views().getDecryptionThreshold();
        s().recoveryState[0].sessionId = s().nextState[0].sessionId;
        s().recoveryState[0].partyMembers = s().nextState[0].partyMembers;
        delete s().recoveryState[0].registeredRecoveryKeys;
        for (
            uint i = 0;
            i < s().nextState[0].registeredRecoveryKeys.length;
            i++
        ) {
            s().recoveryState[0].registeredRecoveryKeys.push(
                s().nextState[0].registeredRecoveryKeys[i]
            );
        }

        _deleteNextStateMappings();
        delete s().nextState[0];
    }

    function _schnorr_verify_1(
        bytes memory input
    ) internal view returns (bool, bytes memory) {
        (bool success, bytes memory res) = BASE_EC_OP_ADDRESS.staticcall(input);

        return (success, res);
    }

    /**
     * @dev
     * get a peer address for a given backup party members key share.
     * the returned address will contain it's backup share
     * you can look up the wallet address in the peer state.
     */
    function getNodeForBackupMember() public view returns (address peer) {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );

        return s().nextState[0].backupMemberPeerMapping[msg.sender];
    }

    function receiveProofsK256(
        bytes calldata proof
    ) public view returns (bool) {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );

        // TODO: should we be using the ecdsaRes too?
        //(bool ecdsaSuccess, bytes memory ecdsaRes) = _schnorr_verify_1(proof);
        (bool ecdsaSuccess, ) = _schnorr_verify_1(proof);

        return ecdsaSuccess;
    }

    /**
     * @dev
     * Verifies the encoded schnorr proof for BLS 12381G1 curve type
     * utilizes a precompile for bls schnorr verification
     * if the proof is valid we map the proofs received to the back up party member which submitted it
     */
    function receiveProofBls12381G1(
        bytes calldata proof
    ) public returns (bool) {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );

        bytes1 version = bytes1(proof[0]);
        if (version != s().verificationVersion) {
            revert WrongVerificationVersion();
        }

        // encoding format
        // Version || Timestamp || ParticipantId || Curve || SchnorrProof || SchnorrVerificationKey
        uint64 timestamp = uint64(bytes8(proof[1:9]));
        bytes32 crv = bytes32(proof[10:42]);
        bytes memory sProof = bytes(proof[42:170]);
        bytes memory sKey = bytes(proof[170:266]);
        bytes32 hName = HASH_SHA2_256;
        bytes32 y = sha256(proof[0:42]);
        bytes1 op = 0x51; // bls schnorr_verify_1

        bytes memory pcProof = abi.encodePacked(
            op,
            crv,
            hName,
            y,
            sKey,
            sProof
        );

        (bool blsSuccess, bytes memory blsRes) = _schnorr_verify_1(pcProof);

        // If the precompile status is true and the byte returned is 1 we have verification of the proof
        // If the byte returned is 0 then we know the proof verification has failed
        // if the success boolean is false then we know that the precompile had an error returned and was not successful
        if (blsSuccess && bytes1(blsRes) == 0x01) {
            if (s().submittedProofs[sender].length < 1) {
                s().submittedProofs[sender] = new uint256[](2);
                s().submittedProofs[sender][0] = timestamp;
            } else if (s().submittedProofs[sender].length < 3) {
                s().submittedProofs[sender][1] = timestamp;
            }

            return true;
        }

        return false;
    }

    function getProofSubmissionForBackupPartyMember()
        public
        view
        returns (uint256)
    {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );
        return s().submittedProofs[sender].length;
    }

    /**
     * @dev
     * Registers a set of decryption keys for a given party member based on the sender address
     * Creates a backup recovery snapshot with the respective keys
     * which is incomplete until all recovery signatures are registered to chain
     */
    function receiveNewKeySet(
        LibBackupRecoveryStorage.RecoveryKey[] memory recoveryKeys,
        bytes memory sessionId
    ) public {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );

        // condition checks if the current set of public keys matches what is held in
        // the next state for the backup. If they do not match we set the next state
        // public keys to what is submitted by the backup party member
        // byte comparison is not supported between memory and storage
        // so we encode and hash to compare 32 byte hashes
        if (
            keccak256(abi.encodePacked(sessionId)) ==
            keccak256(abi.encodePacked(s().nextState[0].sessionId)) &&
            keccak256(abi.encode(recoveryKeys)) ==
            keccak256(abi.encode(s().nextState[0].registeredRecoveryKeys))
        ) {
            s().nextState[0].keysReceived[msg.sender] = true;
            for (uint256 i = 0; i < s().nextState[0].partyMembers.length; i++) {
                if (
                    s().nextState[0].keysReceived[
                        s().nextState[0].partyMembers[i]
                    ] != true
                ) {
                    return;
                }
            }

            // make nextState backupState
            _updateRecoveryState();
            emit BackupKeysRegistered(s().recoveryState[0]);
        } else {
            // Check the length for null. If null, set the state.
            // Otherwise we reject the transaction as there is a lack of consensus.
            if (s().nextState[0].registeredRecoveryKeys.length == 0) {
                s().nextState[0].sessionId = sessionId;
                s().nextState[0].keysReceived[msg.sender] = true;
                for (uint i = 0; i < recoveryKeys.length; i++) {
                    LibBackupRecoveryStorage.RecoveryKey
                        memory recoveryKey = recoveryKeys[i];
                    s().nextState[0].registeredRecoveryKeys.push(recoveryKey);
                }
            } else {
                // revert as the there is a key mismatch and not the first sender
                revert BackupKeysMismatch(
                    s().nextState[0].sessionId,
                    sessionId,
                    keccak256(
                        abi.encode(s().nextState[0].registeredRecoveryKeys)
                    ),
                    keccak256(abi.encode(recoveryKeys))
                );
            }
        }
    }

    /**
     * @dev
     * Resets the contract state by registering a new set of backup members
     * emits `BackupPartyRegistered`
     */
    function registerNewBackupParty(
        address[] memory partyMembers
    ) public onlyOwner {
        require(
            partyMembers.length > 1,
            "BackupRecovery: cannot vote for empty party set"
        );

        // record the past state for history
        s().pastBackupStates[s().recoveryState[0].sessionId] = s()
            .recoveryState[0];

        // null the recoveryState and public key state
        delete s().recoveryState[0];

        // clear the next state to start over
        _deleteNextStateMappings();
        delete s().nextState[0];

        s().nextState[0].partyMembers = partyMembers;
        s().nextState[0].nodesAssignedCount = 0;

        emit BackupPartyRegistered(views().getDecryptionThreshold());
    }

    function _deleteNextStateMappings() internal {
        address[] memory oldPartyMembers = s().nextState[0].partyMembers;
        for (uint256 i = 0; i < oldPartyMembers.length; i++) {
            address oldPartyMember = oldPartyMembers[i];
            address oldPeer = s().nextState[0].backupMemberPeerMapping[
                oldPartyMember
            ];

            delete s().nextState[0].backupMemberPeerMapping[oldPartyMember];
            delete s().nextState[0].peerToBackupMemberMapping[oldPeer];
            delete s().nextState[0].keysReceived[oldPartyMember];
            delete s().submittedProofs[oldPartyMember];
            // No need to delete `votesToRegisterRecoveryKeys` as the pubKey will be different for all the DKGs
        }
        delete s().nextState[0].registeredRecoveryKeys;
        for (uint i = 0; i < s().nextState[0].keySetIds.length; i++) {
            string memory keySetId = s().nextState[0].keySetIds[i];
            delete s().nextState[0].keySetIdExists[keySetId];
        }
        delete s().nextState[0].keySetIds;
    }

    /**
     * @dev
     * Vote to register Recovery pubKeys
     * msg.sender is the nodeAddress
     */
    function registerRecoveryKeys(
        LibBackupRecoveryStorage.RecoveryKey[] memory recoveryKeys,
        bytes memory sessionId,
        string memory keySetId
    ) public {
        require(
            s().nextState[0].partyMembers.length > 1,
            "BackupRecovery: cannot do dkg for empty party set"
        );

        require(
            s().nextState[0].peerToBackupMemberMapping[msg.sender] !=
                address(0),
            "BackupRecovery: not a member of the Recovery DKG peer group"
        );

        require(
            s().nextState[0].registeredRecoveryKeys.length == 0,
            "BackupRecovery: recovery keys already set for this Recovery DKG"
        );

        require(
            s().nextState[0].sessionId.length == 0 ||
                keccak256(s().nextState[0].sessionId) == keccak256(sessionId),
            "BackupRecovery: sessionId mismatch"
        );

        // record the votes
        for (uint i = 0; i < recoveryKeys.length; i++) {
            LibBackupRecoveryStorage.RecoveryKey
                memory recoveryKey = recoveryKeys[i];

            require(
                s()
                    .nextState[0]
                    .votesToRegisterRecoveryKeys[recoveryKey.pubkey]
                    .voted[msg.sender] == false,
                "BackupRecovery: validator has already voted for this recovery key"
            );

            s()
                .nextState[0]
                .votesToRegisterRecoveryKeys[recoveryKey.pubkey]
                .votes += 1;
            s()
                .nextState[0]
                .votesToRegisterRecoveryKeys[recoveryKey.pubkey]
                .voted[msg.sender] = true;
            if (!s().nextState[0].keySetIdExists[keySetId]) {
                s().nextState[0].keySetIdExists[keySetId] = true;
                s().nextState[0].keySetIds.push(keySetId);
            }

            // If all the Recovery peers have voted, register it
            if (
                s()
                    .nextState[0]
                    .votesToRegisterRecoveryKeys[recoveryKey.pubkey]
                    .votes == s().nextState[0].partyMembers.length
            ) {
                s().nextState[0].registeredRecoveryKeys.push(recoveryKey);
                // once a recovery key is registered so is the sessionId
                s().nextState[0].sessionId = sessionId;

                for (uint i = 0; i < s().nextState[0].keySetIds.length; i++) {
                    string memory keySetId = s().nextState[0].keySetIds[i];
                    ss()
                        .keySetsConfigs[keccak256(abi.encodePacked(keySetId))]
                        .recoverySessionId = sessionId;
                }

                emit RecoveryKeySet(recoveryKey);
            }
        }
    }

    /**
     * @dev
     * Maps a backup party member to a validator node. returns the address of the mapped validator
     * Validator refers to a node operator in this contenxt.
     * msg.sender is the nodeAddress
     */
    function setMemberForDkg() public returns (address bp) {
        require(
            _checkValidatorSetForAddress(msg.sender),
            "BackupRecovery: not a member of the current peer group"
        );
        if (views().allBackupMembersMapped()) {
            // Also checks whether backup party exists
            revert NodesAllMappedToBackupMembers(msg.sender);
        }
        if (
            s().nextState[0].peerToBackupMemberMapping[msg.sender] != address(0)
        ) {
            return s().nextState[0].peerToBackupMemberMapping[msg.sender];
        }

        s().nextState[0].peerToBackupMemberMapping[msg.sender] = s()
            .nextState[0]
            .partyMembers[s().nextState[0].nodesAssignedCount];
        s().nextState[0].backupMemberPeerMapping[
            s().nextState[0].partyMembers[s().nextState[0].nodesAssignedCount]
        ] = msg.sender;
        s().nextState[0].nodesAssignedCount += 1;
        emit NodeAssignedToBackupMember(
            s().nextState[0].peerToBackupMemberMapping[msg.sender],
            msg.sender
        );

        return s().nextState[0].peerToBackupMemberMapping[msg.sender];
    }

    function setContractResolver(address newResolverAddress) public onlyOwner {
        s().resolver = ContractResolver(newResolverAddress);
        emit ContractResolverAddressSet(newResolverAddress);
    }

    /**
     * @dev
     * @helper
     * after mapping of nodes for dkg this can be used as a helper method to get all node addresses for the dkg operation
     * these then can be used to look up the `validatorStruct` in the staking contract
     * If called before all party members have been mapped to a node for dkg the set will be incomplete.
     * Correct set size can be validated by comparing with the backup recovery party state.
     */
    function getNodeAddressesForDkg()
        public
        view
        returns (address[] memory nodes)
    {
        require(
            _checkValidatorSetForAddress(msg.sender),
            "BackupRecovery: not a member of the current peer group"
        );

        address[] memory nodeAddresses = new address[](
            s().nextState[0].partyMembers.length
        );

        for (uint256 i = 0; i < s().nextState[0].partyMembers.length; i++) {
            nodeAddresses[i] = s().nextState[0].backupMemberPeerMapping[
                s().nextState[0].partyMembers[i]
            ];

            if (nodeAddresses[i] == address(0)) {
                revert BackupSetIncomplete(nodeAddresses);
            }
        }

        return nodeAddresses;
    }

    /**
     * @dev
     * Registers the peer_id of the node which generated the private shares which
     * are recovered by the node calling this function.
     */
    function setRecoveredPeerId(
        uint256 old_peer_id,
        uint256 new_peer_id
    ) public {
        require(
            _checkValidatorSetForAddress(msg.sender),
            "BackupRecovery: not a member of the current peer group"
        );

        LibBackupRecoveryStorage.RecoveredPeerId[]
            storage recovered_peer_ids = s().recovered_peer_ids;
        for (uint256 i = 0; i < recovered_peer_ids.length; i++) {
            if (recovered_peer_ids[i].node_address == msg.sender) {
                recovered_peer_ids[i].old_peer_id = old_peer_id;
                recovered_peer_ids[i].new_peer_id = new_peer_id;
                return;
            }
        }

        LibBackupRecoveryStorage.RecoveredPeerId
            memory recovered_peer_id = LibBackupRecoveryStorage.RecoveredPeerId(
                msg.sender,
                old_peer_id,
                new_peer_id
            );
        recovered_peer_ids.push(recovered_peer_id);
    }

    /**
     * @dev
     * Retrieves the peer_ids of the nodes which generated the private shares
     * which are recovered by the nodes.
     */
    function getRecoveredPeerIds()
        public
        view
        returns (LibBackupRecoveryStorage.RecoveredPeerId[] memory peer_ids)
    {
        return s().recovered_peer_ids;
    }

    /**
     * @dev
     * @helper
     * after mapping of nodes for dkg this can be used as a helper method to get all node addresses for the dkg operation
     * these then can be used to look up the `validatorStruct` in the staking contract
     * If called before all party members have been mapped to a node for dkg the set will be incomplete.
     * Correct set size can be validated by comparing with the backup recovery party state.
     */
    function getStakerAddressesForDkg()
        public
        view
        returns (address[] memory nodes)
    {
        address[] memory stakerAddresses = new address[](
            s().nextState[0].partyMembers.length
        );

        for (uint256 i = 0; i < s().nextState[0].partyMembers.length; i++) {
            address nodeAddress = s().nextState[0].backupMemberPeerMapping[
                s().nextState[0].partyMembers[i]
            ];

            if (nodeAddress == address(0)) {
                revert BackupSetIncomplete(stakerAddresses);
            }

            stakerAddresses[i] = _getStakingViewsFacet()
                .nodeAddressToStakerAddress(nodeAddress);
        }

        return stakerAddresses;
    }

    /* ========== EVENTS ========== */

    /**
     *
     * Emits when a new backup party is registered, after the inital
     */
    event BackupPartyRegistered(uint256 partyTheshold);

    /**
     * Emits when keys are reigstered from all parties in the backup set
     */
    event BackupKeysRegistered(
        LibBackupRecoveryStorage.BackupRecoveryState state
    );

    /**
     * Emits when a node is assigned to a backup member for dkg
     */
    event NodeAssignedToBackupMember(
        address backupMemberAddress,
        address NodeAddress
    );

    /**
     * Emits when a recovery key has been registered for the current Recovery DKG
     */
    event RecoveryKeySet(LibBackupRecoveryStorage.RecoveryKey recoveryKey);

    /**
     * Emits when the contract resolver address is set
     */
    event ContractResolverAddressSet(address newResolverAddress);
}
