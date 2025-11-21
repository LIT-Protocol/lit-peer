// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
*
* Implementation of a diamond.
/******************************************************************************/

import { LibDiamond } from "../libraries/LibDiamond.sol";
import { IDiamondCut } from "../interfaces/IDiamondCut.sol";
import { IDiamondLoupe } from "../interfaces/IDiamondLoupe.sol";
import { IERC173 } from "../interfaces/IERC173.sol";
import { IERC165 } from "../interfaces/IERC165.sol";
import { ContractResolver } from "../lit-core/ContractResolver.sol";
import { LibStakingStorage } from "./Staking/LibStakingStorage.sol";

// When no function exists for function called
error FunctionNotFound(bytes4 _functionSelector);

// This is used in diamond constructor
// more arguments are added to this struct
// this avoids stack too deep errors
struct StakingArgs {
    address owner;
    address init;
    bytes initCalldata;
    address contractResolver;
    ContractResolver.Env env;
}

contract Staking {
    constructor(
        IDiamondCut.FacetCut[] memory _diamondCut,
        StakingArgs memory _args
    ) payable {
        LibDiamond.setContractOwner(_args.owner);
        LibDiamond.diamondCut(_diamondCut, _args.init, _args.initCalldata);

        LibStakingStorage.GlobalStakingStorage storage s = LibStakingStorage
            .getStakingStorage();
        s.contractResolver = ContractResolver(_args.contractResolver);
        s.env = _args.env;
        s.devopsAdmin = _args.owner;
        // this is a monotonic counter that is incremented every time a new reward epoch is created
        s.nextAvailableRewardEpochNumber = 0;

        uint256[] memory keyTypesTemp = new uint256[](2);
        keyTypesTemp[0] = 1;
        keyTypesTemp[1] = 2;

        // Hardcode the total supply of the token to 1 billion.
        s.tokenTotalSupplyStandIn = 1_000_000_000 ether;

        // Global Configuration values across all realms
        // Most of this is related to staking / delegation
        s.globalConfig[0] = LibStakingStorage.GlobalConfig({
            tokenRewardPerTokenPerEpoch: (10 ** 18) / 20, // 18 decimal places in token
            keyTypes: keyTypesTemp,
            rewardEpochDuration: 1 hours,
            maxTimeLock: 4 * 365 days,
            minTimeLock: 90 days,
            bmin: .004 ether, // Minimum reward budget (in basis points, i.e., 0.1%)
            bmax: .01 ether, // Maximum reward budget (in basis points, i.e., 0.5%)
            k: .5 ether, // Kink parameter for rewards (e.g., 0.5)
            p: .5 ether, // Power parameter (e.g., 0.5)
            enableStakeAutolock: true,
            tokenPrice: 0.2 ether,
            profitMultiplier: 6 ether,
            usdCostPerMonth: 1000 ether,
            maxEmissionRate: 0.12 ether,
            minStakeAmount: 0.01 ether,
            maxStakeAmount: 100_000_000 ether,
            minSelfStake: 0.1 ether,
            minSelfStakeTimelock: 15 days,
            minimumValidatorCount: 3,
            minValidatorCountToClampMinimumThreshold: 4,
            minThresholdToClampAt: 3,
            voteToAdvanceTimeOut: 60
        });

        // Set up complaint configs
        // set demerit threshold for rejoining
        s.demeritRejoinThreshold = 10;
        // set pending rejoin timeout (24h in seconds)
        s.pendingRejoinTimeout = 86400;
        // Unresponsive reasons - 1
        s.complaintReasonToConfig[1] = LibStakingStorage.ComplaintConfig({
            tolerance: 6,
            intervalSecs: 90,
            kickPenaltyPercent: 0.05 ether, // 5%
            kickPenaltyDemerits: 10
        });
        // Non-Participation reasons - 2
        s.complaintReasonToConfig[2] = LibStakingStorage.ComplaintConfig({
            tolerance: 10,
            intervalSecs: 90,
            kickPenaltyPercent: 0.05 ether, // 5%
            kickPenaltyDemerits: 10
        });
        // Incorrect info - 3
        s.complaintReasonToConfig[3] = LibStakingStorage.ComplaintConfig({
            tolerance: 5,
            intervalSecs: 900,
            kickPenaltyPercent: 0.05 ether, // 5%
            kickPenaltyDemerits: 10
        });
        // KeyShareValidationFailure - 4
        s.complaintReasonToConfig[4] = LibStakingStorage.ComplaintConfig({
            tolerance: 10,
            intervalSecs: 90,
            kickPenaltyPercent: 0.05 ether, // 5%
            kickPenaltyDemerits: 10
        });
    }

    // Find facet for function that is called and execute the
    // function if a facet is found and return any value.
    fallback() external {
        LibDiamond.DiamondStorage storage ds;
        bytes32 position = LibDiamond.DIAMOND_STORAGE_POSITION;
        // get diamond storage
        assembly {
            ds.slot := position
        }
        // get facet from function selector
        address facet = ds
            .facetAddressAndSelectorPosition[msg.sig]
            .facetAddress;
        if (facet == address(0)) {
            revert FunctionNotFound(msg.sig);
        }
        // Execute external function from facet using delegatecall and return any value.
        assembly {
            // copy function selector and any arguments
            calldatacopy(0, 0, calldatasize())
            // execute function call using the facet
            let result := delegatecall(gas(), facet, 0, calldatasize(), 0, 0)
            // get any return value
            returndatacopy(0, 0, returndatasize())
            // return any return value or error back to the caller
            switch result
            case 0 {
                revert(0, returndatasize())
            }
            default {
                return(0, returndatasize())
            }
        }
    }
}
