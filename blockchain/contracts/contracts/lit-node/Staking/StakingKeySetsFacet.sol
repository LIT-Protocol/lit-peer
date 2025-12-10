pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { LibPubkeyRouterStorage } from "../PubkeyRouter/LibPubkeyRouterStorage.sol";
import { IPubkeyRouter } from "../PubkeyRouter/LibPubkeyRouterStorage.sol";
import "hardhat/console.sol";

contract StakingKeySetsFacet {
    using EnumerableSet for EnumerableSet.AddressSet;

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner())
            revert StakingUtilsLib.CallerNotOwner();
        _;
    }

    function s()
        internal
        pure
        returns (LibStakingStorage.GlobalStakingStorage storage)
    {
        return LibStakingStorage.getStakingStorage();
    }

    function pubkeyRouter()
        internal
        pure
        returns (LibPubkeyRouterStorage.PubkeyRouterStorage storage)
    {
        return LibPubkeyRouterStorage.getStorage();
    }
    /* ========== VIEWS ========== */

    function keySets()
        public
        view
        returns (LibStakingStorage.KeySetConfig[] memory)
    {
        LibStakingStorage.GlobalStakingStorage storage gs = s();
        bytes32[] memory ids = gs.keySetIds;
        LibStakingStorage.KeySetConfig[]
            memory configs = new LibStakingStorage.KeySetConfig[](ids.length);

        for (uint i = 0; i < ids.length; i++) {
            configs[i] = gs.keySetsConfigs[ids[i]];
        }

        return configs;
    }

    function getKeySet(
        string memory identifier
    ) public view returns (LibStakingStorage.KeySetConfig memory) {
        LibStakingStorage.GlobalStakingStorage storage gs = s();
        bytes32 keySetId = keccak256(abi.encodePacked(identifier));
        LibStakingStorage.KeySetConfig memory config = gs.keySetsConfigs[
            keySetId
        ];
        return config;
    }

    function verifyKeySetCounts(
        string memory identifier,
        IPubkeyRouter.RootKey[] memory newRootKeys
    ) public view returns (uint256, uint256) {
        bytes32 keySetId = keccak256(abi.encodePacked(identifier));
        for (uint i = 0; i < newRootKeys.length; i++) {
            IPubkeyRouter.RootKey memory rootKey = newRootKeys[i];
            uint256 count = s().keySetKeyCounts[keySetId][rootKey.keyType];
            for (uint j = 0; j < newRootKeys.length; j++) {
                IPubkeyRouter.RootKey memory rootKey2 = newRootKeys[j];
                if (rootKey2.keyType == rootKey.keyType) {
                    count--;
                }

                if (count == 0) {
                    break;
                }
            }
            if (count > 0) {
                return (rootKey.keyType, count);
            }
        }
        return (0, 0);
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function setKeySet(
        LibStakingStorage.KeySetConfig memory update
    ) external onlyOwner {
        require(
            update.minimumThreshold > 2,
            "minimum threshold cannot be less than 3"
        );

        LibStakingStorage.GlobalStakingStorage storage gs = s();
        LibPubkeyRouterStorage.PubkeyRouterStorage storage ps = pubkeyRouter();
        bytes32 keySetId = keccak256(abi.encodePacked(update.identifier));

        if (bytes(gs.keySetsConfigs[keySetId].identifier).length == 0) {
            gs.keySetIds.push(keySetId);
        }

        if (ps.rootKeys[address(this)][keySetId].length > 0) {
            // Only some things can be updated after keys exist
            LibStakingStorage.KeySetConfig storage config = gs.keySetsConfigs[
                keySetId
            ];

            require(
                update.minimumThreshold < config.minimumThreshold,
                "minimum threshold can only decrease once root keys have been created"
            );

            config.completeIsolation = update.completeIsolation;
            config.description = update.description;
            config.realms = update.realms;
            config.minimumThreshold = update.minimumThreshold;
            config.recoverySessionId = update.recoverySessionId;

            emit KeySetConfigUpdated(update.identifier);
        } else {
            require(update.counts.length > 0, "counts cannot be empty");
            require(update.curves.length > 0, "curves cannot be empty");
            require(
                update.curves.length == update.counts.length,
                "curves and counts must be equal"
            );

            // No keys exist yet so anything except the identifier can be updated
            gs.keySetsConfigs[keySetId].identifier = update.identifier;
            gs.keySetsConfigs[keySetId].description = update.description;
            gs.keySetsConfigs[keySetId].minimumThreshold = update
                .minimumThreshold;
            gs.keySetsConfigs[keySetId].monetaryValue = update.monetaryValue;
            gs.keySetsConfigs[keySetId].completeIsolation = update
                .completeIsolation;
            gs.keySetsConfigs[keySetId].realms = update.realms;
            gs.keySetsConfigs[keySetId].curves = update.curves;
            gs.keySetsConfigs[keySetId].counts = update.counts;
            gs.keySetsConfigs[keySetId].recoverySessionId = update
                .recoverySessionId;
            for (uint i = 0; i < update.curves.length; i++) {
                require(update.counts[i] > 0, "key counts cannot be set to 0");
                gs.keySetKeyCounts[keySetId][update.curves[i]] = update.counts[
                    i
                ];
            }

            LibStakingStorage.KeySetConfig memory config = gs.keySetsConfigs[
                keySetId
            ];

            emit KeySetConfigSet(
                bytes(config.identifier).length > 0,
                update.identifier,
                keySetId
            );
        }
    }

    function deleteKeySet(string memory identifier) external onlyOwner {
        LibStakingStorage.GlobalStakingStorage storage gs = s();
        bytes32 keySetId = keccak256(abi.encodePacked(identifier));
        require(
            bytes(gs.keySetsConfigs[keySetId].identifier).length > 0,
            "can't delete the specified key set"
        );
        LibStakingStorage.KeySetConfig memory config = gs.keySetsConfigs[
            keySetId
        ];
        delete gs.keySetsConfigs[keySetId];
        bool found = false;
        for (uint i = 0; i < gs.keySetIds.length; i++) {
            if (gs.keySetIds[i] == keySetId) {
                gs.keySetIds[i] = gs.keySetIds[gs.keySetIds.length - 1];
                gs.keySetIds.pop();
                found = true;
                break;
            }
        }

        require(found, "specified key set not found in list");

        for (uint i = 0; i < config.counts.length; i++) {
            delete gs.keySetKeyCounts[keySetId][config.curves[i]];
        }
        // Delete the root keys from the pub key router
        LibPubkeyRouterStorage.PubkeyRouterStorage storage ps = pubkeyRouter();
        delete ps.rootKeys[address(this)][keySetId];
    }

    event KeySetConfigSet(bool exists, string identifier, bytes32 hashed);
    event KeySetConfigUpdated(string identifier);
}
