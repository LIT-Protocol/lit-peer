//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import { LibPubkeyRouterStorage, IPubkeyRouter } from "./LibPubkeyRouterStorage.sol";
import { IKeyDeriver } from "../HDKeyDeriver.sol";
import { StakingAcrossRealmsFacet } from "../Staking/StakingAcrossRealmsFacet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { ERC2771 } from "../../common/ERC2771.sol";

import "hardhat/console.sol";

contract PubkeyRouterViewsFacet {
    using BytesLib for bytes;

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

    function ethAddressToPkpId(
        address ethAddress
    ) public view returns (uint256) {
        return s().ethAddressToPkpId[ethAddress];
    }

    function realms() internal view returns (StakingAcrossRealmsFacet) {
        return StakingAcrossRealmsFacet(getStakingAddress());
    }

    function stakingViews() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(getStakingAddress());
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
            bytes(prd.keySetIdentifier).length != 0 &&
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
}
