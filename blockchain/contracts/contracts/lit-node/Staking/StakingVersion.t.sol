// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import { Test, console2 } from "lib/forge-std/src/Test.sol";
import { StakingFacet } from "../Staking/StakingFacet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { StakingAdminFacet } from "../Staking/StakingAdminFacet.sol";
import { StakingValidatorFacet } from "../Staking/StakingValidatorFacet.sol";
import { StakingAcrossRealmsFacet } from "../Staking/StakingAcrossRealmsFacet.sol";
import { StakingUtilsLib } from "../Staking/StakingUtilsLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { Staking, StakingArgs } from "../Staking.sol";
import { DiamondInit } from "../../upgradeInitializers/DiamondInit.sol";
import { IDiamond } from "../../interfaces/IDiamond.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";
import { LITToken } from "../LITToken.sol";
import { SetupAndUtils } from "./SetupAndUtils.t.sol";
import { console } from "lib/forge-std/src/console.sol";

contract StakingVersionTest is Test, SetupAndUtils {
    function setUp() public {
        baseSetup();
    }

    struct VestedTokens {
        uint256 vestedAmount;
        /// @notice Time taken to vest the amount
        uint256 vestingTime;
    }

    struct TestCase {
        LibStakingStorage.Version minVersion;
        LibStakingStorage.Version maxVersion;
        LibStakingStorage.Version actualVersion;
        bool expectedResult;
    }

    /// @notice This test checks the version checking logic is correct against
    /// the test vector.
    function test_checkVersion() public {
        TestCase[] memory testCases = new TestCase[](9);
        testCases[0] = TestCase({
            // Tests for major version
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 2,
                patch: 0
            }),
            actualVersion: LibStakingStorage.Version({
                major: 1,
                minor: 0,
                patch: 0
            }),
            expectedResult: false
        });
        testCases[1] = TestCase({
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 2,
                patch: 0
            }),
            actualVersion: LibStakingStorage.Version({
                major: 2,
                minor: 0,
                patch: 0
            }),
            expectedResult: false
        });
        // Tests for minor versions
        testCases[2] = TestCase({
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 2,
                patch: 0
            }),
            actualVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            expectedResult: true
        });
        testCases[3] = TestCase({
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 2,
                patch: 0
            }),
            actualVersion: LibStakingStorage.Version({
                major: 1,
                minor: 3,
                patch: 0
            }),
            expectedResult: false
        });
        // Tests for patch versions
        testCases[4] = TestCase({
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 2,
                patch: 0
            }),
            actualVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 1
            }),
            expectedResult: true
        });
        // Miscellaneous tests
        testCases[5] = TestCase({
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 2,
                patch: 0
            }),
            actualVersion: LibStakingStorage.Version({
                major: 1,
                minor: 3,
                patch: 3
            }),
            expectedResult: false
        });
        testCases[6] = TestCase({
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 2,
                patch: 0
            }),
            actualVersion: LibStakingStorage.Version({
                major: 2,
                minor: 0,
                patch: 0
            }),
            expectedResult: false
        });
        testCases[7] = TestCase({
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 1
            }),
            actualVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 2
            }),
            expectedResult: false
        });
        testCases[8] = TestCase({
            minVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 1
            }),
            maxVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 2
            }),
            actualVersion: LibStakingStorage.Version({
                major: 1,
                minor: 1,
                patch: 0
            }),
            expectedResult: false
        });

        for (uint256 i = 0; i < testCases.length; i++) {
            // Set the min max versions
            stakingVersionFacet.setMinVersion(1, testCases[i].minVersion);
            stakingVersionFacet.setMaxVersion(1, testCases[i].maxVersion);

            // Check the version
            bool result = stakingVersionFacet.checkVersion(
                1,
                testCases[i].actualVersion
            );
            assertEq(result, testCases[i].expectedResult);
        }
    }
}
