//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { PKPNFT } from "../PKPNFT.sol";
import { Staking } from "../Staking.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { IKeyDeriver } from "../HDKeyDeriver.sol";
import { StakingAcrossRealmsFacet } from "../Staking/StakingAcrossRealmsFacet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { PKPNFTFacet } from "../PKPNFT/PKPNFTFacet.sol";
import { LibPubkeyRouterStorage, IPubkeyRouter } from "./LibPubkeyRouterStorage.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { ERC2771 } from "../../common/ERC2771.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { StakingUtilsLib } from "../Staking/StakingUtilsLib.sol";
import { StakingKeySetsFacet } from "../Staking/StakingKeySetsFacet.sol";
import "hardhat/console.sol";

// TODO: make the tests send PKPNFT into the constructor
// TODO: test interaction between PKPNFT and this contract, like mint a keypair and see if you can access it
// TODO: setRoutingData() for a batch of keys

contract PubkeyRouterFacet is ERC2771 {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using EnumerableSet for EnumerableSet.UintSet;
    using BytesLib for bytes;

    error KeySetNotFound(string identifier, bytes32 hash);
    error RootKeyMiscount(uint256 curveType, uint256 count);

    /* ========== MODIFIERS ========== */

    modifier onlyOwner() override {
        if (LibERC2771._msgSender() != LibDiamond.contractOwner())
            revert CallerNotOwner();
        _;
    }

    /* ========== VIEWS ========== */

    function s()
        internal
        pure
        returns (LibPubkeyRouterStorage.PubkeyRouterStorage storage)
    {
        return LibPubkeyRouterStorage.getStorage();
    }

    /// get the staking address from the resolver
    function getStakingAddress() internal view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.STAKING_CONTRACT(),
                s().env
            );
    }

    function realms() internal view returns (StakingAcrossRealmsFacet) {
        return StakingAcrossRealmsFacet(getStakingAddress());
    }

    function stakingViews() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(getStakingAddress());
    }

    function stakingKeySets() internal view returns (StakingKeySetsFacet) {
        return StakingKeySetsFacet(getStakingAddress());
    }

    function ethAddressToPkpId(
        address ethAddress
    ) public view returns (uint256) {
        return s().ethAddressToPkpId[ethAddress];
    }

    function pubkeys(
        uint256 tokenId
    ) public view returns (LibPubkeyRouterStorage.PubkeyRoutingData memory) {
        return s().pubkeys[tokenId];
    }

    function getPkpNftAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.PKP_NFT_CONTRACT(),
                s().env
            );
    }

    /// get root keys for a given staking contract
    function getRootKeys(
        address stakingContract,
        string memory keySetId
    ) public view returns (IPubkeyRouter.RootKey[] memory) {
        return
            s().rootKeys[stakingContract][
                keccak256(abi.encodePacked(keySetId))
            ];
    }

    /// get the routing data for a given key hash
    function getRoutingData(
        uint256 tokenId
    ) external view returns (LibPubkeyRouterStorage.PubkeyRoutingData memory) {
        return s().pubkeys[tokenId];
    }

    /// get if a given pubkey has routing data associated with it or not
    function isRouted(uint256 tokenId) public view returns (bool) {
        LibPubkeyRouterStorage.PubkeyRoutingData memory prd = s().pubkeys[
            tokenId
        ];
        return
            prd.pubkey.length != 0 &&
            prd.keyType != 0 &&
            prd.derivedKeyId != bytes32(0);
    }

    /// get the eth address for the keypair, as long as it's an ecdsa keypair
    function getEthAddress(uint256 tokenId) public view returns (address) {
        return deriveEthAddressFromPubkey(s().pubkeys[tokenId].pubkey);
    }

    function getPkpInfoFromTokenIds(
        uint256[] memory tokenIds
    ) public view returns (LibPubkeyRouterStorage.PkpInfo[] memory) {
        if (tokenIds.length == 0) {
            return new LibPubkeyRouterStorage.PkpInfo[](0);
        }

        uint256 count = 0;
        for (uint256 i = 0; i < tokenIds.length; i++) {
            if (s().pubkeys[tokenIds[i]].pubkey.length > 0) {
                count++;
            }
        }

        LibPubkeyRouterStorage.PkpInfo[]
            memory pkpInfos = new LibPubkeyRouterStorage.PkpInfo[](count);
        uint256 pkpIndex = 0;
        for (uint256 i = 0; i < tokenIds.length; i++) {
            if (s().pubkeys[tokenIds[i]].pubkey.length > 0) {
                pkpInfos[pkpIndex].tokenId = tokenIds[i];
                pkpInfos[pkpIndex].pubkey = s().pubkeys[tokenIds[i]].pubkey;
                pkpInfos[pkpIndex].ethAddress = deriveEthAddressFromPubkey(
                    s().pubkeys[tokenIds[i]].pubkey
                );
                pkpIndex++;
            }
        }
        return pkpInfos;
    }

    function getPkpInfoFromEthAddresses(
        address[] memory ethAddresses
    ) public view returns (LibPubkeyRouterStorage.PkpInfo[] memory) {
        if (ethAddresses.length == 0) {
            return new LibPubkeyRouterStorage.PkpInfo[](0);
        }

        uint256 count = 0;
        for (uint256 i = 0; i < ethAddresses.length; i++) {
            if (s().ethAddressToPkpId[ethAddresses[i]] != 0) {
                count++;
            }
        }

        LibPubkeyRouterStorage.PkpInfo[]
            memory pkpInfos = new LibPubkeyRouterStorage.PkpInfo[](count);
        uint256 pkpIndex = 0;
        for (uint256 i = 0; i < ethAddresses.length; i++) {
            if (s().ethAddressToPkpId[ethAddresses[i]] != 0) {
                pkpInfos[pkpIndex].tokenId = s().ethAddressToPkpId[
                    ethAddresses[i]
                ];
                pkpInfos[pkpIndex].pubkey = s()
                    .pubkeys[pkpInfos[pkpIndex].tokenId]
                    .pubkey;
                pkpInfos[pkpIndex].ethAddress = ethAddresses[i];
                pkpIndex++;
            }
        }
        return pkpInfos;
    }

    /// includes the 0x04 prefix so you can pass this directly to ethers.utils.computeAddress
    function getPubkey(uint256 tokenId) public view returns (bytes memory) {
        return s().pubkeys[tokenId].pubkey;
    }

    function deriveEthAddressFromPubkey(
        bytes memory pubkey
    ) public pure returns (address) {
        // remove 0x04 prefix
        bytes32 hashed = keccak256(pubkey.slice(1, 64));
        return address(uint160(uint256(hashed)));
    }

    function checkNodeSignatures(
        uint256 realmId,
        IPubkeyRouter.Signature[] memory signatures,
        bytes memory signedMessage
    ) public view returns (bool) {
        require(
            signatures.length >=
                stakingViews().currentValidatorCountForConsensus(realmId),
            "PubkeyRouter: incorrect number of signatures on a given root key"
        );
        for (uint256 i = 0; i < signatures.length; i++) {
            IPubkeyRouter.Signature memory sig = signatures[i];
            address signer = ECDSA.recover(
                ECDSA.toEthSignedMessageHash(signedMessage),
                sig.v,
                sig.r,
                sig.s
            );
            require(
                stakingViews().isActiveValidatorByNodeAddress(realmId, signer),
                "PubkeyRouter: signer is not active validator"
            );
        }
        return true;
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    /// register a pubkey and routing data for a given key hash
    function setRoutingData(
        uint256 tokenId,
        bytes memory pubkey,
        address stakingContractAddress,
        uint256 keyType,
        bytes32 derivedKeyId
    ) public {
        require(
            LibERC2771._msgSender() == address(getPkpNftAddress()),
            "setRoutingData must be called by PKPNFT contract"
        );

        require(
            tokenId == uint256(keccak256(pubkey)),
            "tokenId does not match hashed pubkey"
        );
        require(
            !isRouted(tokenId),
            "PubkeyRouter: pubkey already has routing data"
        );

        s().pubkeys[tokenId].pubkey = pubkey;
        s().pubkeys[tokenId].keyType = keyType;
        s().pubkeys[tokenId].derivedKeyId = derivedKeyId;

        address pkpAddress = deriveEthAddressFromPubkey(pubkey);
        s().ethAddressToPkpId[pkpAddress] = tokenId;

        emit PubkeyRoutingDataSet(
            tokenId,
            pubkey,
            stakingContractAddress,
            keyType,
            derivedKeyId
        );
    }

    /// Set the pubkey and routing data for a given key hash
    // this is only used by an admin in case of emergency.  can prob be removed.
    function setRoutingDataAsAdmin(
        uint256 tokenId,
        bytes memory pubkey,
        address stakingContract,
        uint256 keyType,
        bytes32 derivedKeyId
    ) public onlyOwner {
        s().pubkeys[tokenId].pubkey = pubkey;
        s().pubkeys[tokenId].keyType = keyType;
        s().pubkeys[tokenId].derivedKeyId = derivedKeyId;

        address pkpAddress = deriveEthAddressFromPubkey(pubkey);
        s().ethAddressToPkpId[pkpAddress] = tokenId;

        emit PubkeyRoutingDataSet(
            tokenId,
            pubkey,
            stakingContract,
            keyType,
            derivedKeyId
        );
    }

    function setContractResolver(address newResolverAddress) public onlyOwner {
        s().contractResolver = ContractResolver(newResolverAddress);
        emit ContractResolverAddressSet(newResolverAddress);
    }

    function voteForRootKeys(
        address stakingContractAddress,
        string memory identifier,
        IPubkeyRouter.RootKey[] memory newRootKeys
    ) public {
        address stakerAddress = realms().nodeAddressToStakerAddressAcrossRealms(
            LibERC2771._msgSender()
        );
        uint256 realmId = realms().getRealmIdForStakerAddress(stakerAddress);

        if (realmId == 0) {
            revert(
                "PubkeyRouter: msg sender is not active validator - realmId is 0."
            );
        }

        require(
            stakingViews().isActiveValidatorByNodeAddressForNextEpoch(
                realmId,
                LibERC2771._msgSender()
            ),
            "PubkeyRouter: txn sender is not active validator"
        );

        bytes32 keySetId = keccak256(abi.encodePacked(identifier));
        require(
            s().rootKeys[stakingContractAddress][keySetId].length == 0,
            "PubkeyRouter: root keys already set for this staking contract"
        );

        require(
            stakingViews().state(realmId) ==
                LibStakingStorage.States.NextValidatorSetLocked,
            "PubkeyRouter: staking contract is not in NextValidatorSetLocked state"
        );

        LibStakingStorage.KeySetConfig memory config = stakingKeySets()
            .getKeySet(identifier);

        require(bytes(config.identifier).length > 0, "no key set config found");
        (uint256 curveType, uint256 counts) = stakingKeySets()
            .verifyKeySetCounts(identifier, newRootKeys);
        if (curveType != 0 || counts != 0) {
            revert RootKeyMiscount(curveType, counts);
        }

        // record the votes
        for (uint i = 0; i < newRootKeys.length; i++) {
            IPubkeyRouter.RootKey memory rootKey = newRootKeys[i];
            require(
                s()
                .votesToRegisterRootKeys[stakingContractAddress][rootKey.pubkey]
                    .voted[LibERC2771._msgSender()] == false,
                "PubkeyRouter: validator has already voted for this root key"
            );
            s()
            .votesToRegisterRootKeys[stakingContractAddress][rootKey.pubkey]
                .votes += 1;
            s()
            .votesToRegisterRootKeys[stakingContractAddress][rootKey.pubkey]
                .voted[LibERC2771._msgSender()] = true;

            // FIXME: which one is correct ?  Will depend on the implementation a bit.
            // uint256 votesNeeded = stakingViews().getNonShadowValidatorsInCurrentEpochLength(realmId);
            uint256 votesNeeded = stakingViews()
                .getValidatorsInNextEpoch(realmId)
                .length;

            // if it has enough votes, register it
            if (
                s()
                .votesToRegisterRootKeys[stakingContractAddress][rootKey.pubkey]
                    .votes == votesNeeded
            ) {
                s().rootKeys[stakingContractAddress][keySetId].push(rootKey);
                emit RootKeySet(stakingContractAddress, rootKey);
            }
        }
    }

    function getDerivedPubkey(
        address stakingContract,
        string memory keySetId,
        bytes32 derivedKeyId
    ) public view returns (bytes memory) {
        IPubkeyRouter.RootKey[] memory rootPubkeys = getRootKeys(
            stakingContract,
            keySetId
        );

        bytes memory pubkey = _computeHDPubkey(derivedKeyId, rootPubkeys, 2);

        return pubkey;
    }

    function adminResetRootKeys(
        address stakingContract,
        string memory keySetId
    ) public onlyOwner {
        require(s().env == ContractResolver.Env.Dev, "only for dev env");
        bytes32 keySetIdentifier = keccak256(abi.encodePacked(keySetId));

        delete s().rootKeys[stakingContract][keySetIdentifier];
    }

    function adminSetRootKeys(
        address stakingContract,
        string memory keySetId,
        IPubkeyRouter.RootKey[] memory rootKeys
    ) public onlyOwner {
        require(s().env == ContractResolver.Env.Dev, "only for dev env");

        bytes32 keySetIdentifier = keccak256(abi.encodePacked(keySetId));
        for (uint256 i = 0; i < rootKeys.length; i++) {
            s().rootKeys[stakingContract][keySetIdentifier].push(rootKeys[i]);
            emit RootKeySet(stakingContract, rootKeys[i]);
        }
    }

    function _computeHDPubkey(
        bytes32 derivedKeyId,
        IPubkeyRouter.RootKey[] memory rootHDKeys,
        uint256 keyType
    ) internal view returns (bytes memory) {
        address deriverAddr = s().contractResolver.getContract(
            s().contractResolver.HD_KEY_DERIVER_CONTRACT(),
            s().env
        );
        (bool success, bytes memory pubkey) = IKeyDeriver(deriverAddr)
            .computeHDPubKey(derivedKeyId, rootHDKeys, keyType);

        require(success, "PubkeyRouter: Failed public key calculation");
        return pubkey;
    }

    /* ========== EVENTS ========== */

    event PubkeyRoutingDataSet(
        uint256 indexed tokenId,
        bytes pubkey,
        address stakingContract,
        uint256 keyType,
        bytes32 derivedKeyId
    );
    event ContractResolverAddressSet(address newResolverAddress);
    event RootKeySet(address stakingContract, IPubkeyRouter.RootKey rootKey);
    event ToggleEvent(uint256 value, address sender);
    event DebugEvent(string message, address sender, uint256 value);
}
