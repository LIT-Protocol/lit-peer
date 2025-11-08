//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { ERC2771 } from "../../common/ERC2771.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { StakingUtilsLib } from "../Staking/StakingUtilsLib.sol";

import { LibStakingStorage } from "./LibStakingStorage.sol";

// import "hardhat/console.sol";

contract StakingVersionFacet {
    /* ========== MODIFIERS ========== */

    modifier onlyOwner() {
        if (LibERC2771._msgSender() != LibDiamond.contractOwner())
            revert StakingUtilsLib.CallerNotOwner();
        _;
    }

    /* ========== VIEWS ========== */

    function getMinVersion(
        uint256 realmId
    ) external view returns (LibStakingStorage.Version memory) {
        return
            LibStakingStorage.getRealmStorage(realmId).versionRequirements[0];
    }

    function getMaxVersion(
        uint256 realmId
    ) external view returns (LibStakingStorage.Version memory) {
        return
            LibStakingStorage.getRealmStorage(realmId).versionRequirements[1];
    }

    function getMinVersionString(
        uint256 realmId
    ) external view returns (string memory) {
        LibStakingStorage.Version storage minVersion = LibStakingStorage
            .getRealmStorage(realmId)
            .versionRequirements[0];

        return
            string(
                abi.encodePacked(
                    Strings.toString(minVersion.major),
                    ".",
                    Strings.toString(minVersion.minor),
                    ".",
                    Strings.toString(minVersion.patch)
                )
            );
    }

    function getMaxVersionString(
        uint256 realmId
    ) external view returns (string memory) {
        LibStakingStorage.Version storage maxVersion = LibStakingStorage
            .getRealmStorage(realmId)
            .versionRequirements[1];

        return
            string(
                abi.encodePacked(
                    Strings.toString(maxVersion.major),
                    ".",
                    Strings.toString(maxVersion.minor),
                    ".",
                    Strings.toString(maxVersion.patch)
                )
            );
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function setMinVersion(
        uint256 realmId,
        LibStakingStorage.Version memory version
    ) external onlyOwner {
        LibStakingStorage.getRealmStorage(realmId).versionRequirements[
            0
        ] = version;
        emit VersionRequirementsUpdated(0, realmId, version);
    }

    function setMaxVersion(
        uint256 realmId,
        LibStakingStorage.Version memory version
    ) external onlyOwner {
        LibStakingStorage.getRealmStorage(realmId).versionRequirements[
            1
        ] = version;
        emit VersionRequirementsUpdated(1, realmId, version);
    }

    function checkVersion(
        uint256 realmId,
        LibStakingStorage.Version memory version
    ) public view returns (bool) {
        LibStakingStorage.Version storage minVersion = LibStakingStorage
            .getRealmStorage(realmId)
            .versionRequirements[0];
        LibStakingStorage.Version storage maxVersion = LibStakingStorage
            .getRealmStorage(realmId)
            .versionRequirements[1];

        bool gteMin = (version.major > minVersion.major ||
            (version.major == minVersion.major &&
                version.minor > minVersion.minor) ||
            (version.major == minVersion.major &&
                version.minor == minVersion.minor &&
                version.patch >= minVersion.patch));
        bool lteMax = (version.major < maxVersion.major ||
            (version.major == maxVersion.major &&
                version.minor < maxVersion.minor) ||
            (version.major == maxVersion.major &&
                version.minor == maxVersion.minor &&
                version.patch <= maxVersion.patch));
        return gteMin && lteMax;
    }

    /* ========== EVENTS ========== */

    event VersionRequirementsUpdated(
        uint256 index,
        uint256 realmId,
        LibStakingStorage.Version version
    );
}
