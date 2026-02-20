// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract FunctionSelectorHelper {
    function getSignaturesStakingFacet() public pure returns (string[] memory) {
        // Define the function signatures
        string[] memory functionSignatures = new string[](19);

        // Mutative functions that are either public or external.
        functionSignatures[0] = "stake(uint256,uint256,address)";
        functionSignatures[1] = "unfreezeStake(address,uint256)";
        functionSignatures[2] = "getRewardEpoch(address,uint256)";
        functionSignatures[3] = "initializeRewardEpoch(address,uint256,bool)";
        functionSignatures[
            4
        ] = "increaseStakeRecordTimelock(address,uint256,uint256)";
        functionSignatures[
            5
        ] = "increaseStakeRecordAmount(address,uint256,uint256)";
        functionSignatures[
            6
        ] = "claimStakeRewards(uint256,address,uint256,uint256)";
        functionSignatures[7] = "claimValidatorCommission(uint256,uint256)";
        functionSignatures[8] = "claimFixedCostRewards(uint256,uint256)";
        functionSignatures[9] = "migrateStakeRecord(address,uint256,address)";
        functionSignatures[10] = "splitStakeRecord(address,uint256,uint256)";
        functionSignatures[11] = "withdraw(address,uint256)";
        functionSignatures[12] = "slashValidator(uint256,address)";
        functionSignatures[13] = "checkStakingAmounts(address)";

        // View functions that are either public or external. These are included here not
        // for invariant testing, but for interacting via the Handler-based invariant tests.
        functionSignatures[14] = "balanceOf(address)";
        functionSignatures[15] = "getMinimumStake()";
        functionSignatures[16] = "getMinimumSelfStake()";
        functionSignatures[17] = "getMaximumStake()";
        functionSignatures[18] = "setValidatorCommissionRate(uint256)";

        return functionSignatures;
    }

    function getSelectorsStakingFacet() public pure returns (bytes4[] memory) {
        string[] memory functionSignatures = getSignaturesStakingFacet();
        bytes4[] memory selectors = new bytes4[](functionSignatures.length);

        for (uint256 i = 0; i < functionSignatures.length; i++) {
            selectors[i] = bytes4(keccak256(bytes(functionSignatures[i])));
        }

        return selectors;
    }

    function getSignaturesStakingViewsFacet()
        public
        pure
        returns (string[] memory)
    {
        // Define the function signatures
        string[] memory functionSignatures = new string[](71);

        // View functions that are either public or external.
        functionSignatures[0] = "epoch(uint256)";
        functionSignatures[1] = "realmConfig(uint256)";
        functionSignatures[2] = "globalConfig()";
        functionSignatures[3] = "complaintConfig(uint256)";
        functionSignatures[4] = "contractResolver()";
        functionSignatures[5] = "kickPenaltyPercentByReason(uint256)";
        functionSignatures[6] = "getNodeDemerits(address)";
        functionSignatures[7] = "nodeAddressToStakerAddress(address)";
        functionSignatures[8] = "readyForNextEpoch(uint256,address)";
        functionSignatures[9] = "state(uint256)";
        functionSignatures[10] = "getTokenContractAddress()";
        functionSignatures[11] = "validators(address)";
        functionSignatures[12] = "isActiveValidator(uint256,address)";
        functionSignatures[
            13
        ] = "isActiveValidatorForNextEpoch(uint256,address)";
        functionSignatures[
            14
        ] = "isActiveValidatorByNodeAddress(uint256,address)";
        functionSignatures[
            15
        ] = "isActiveValidatorByNodeAddressForNextEpoch(uint256,address)";
        functionSignatures[
            16
        ] = "getVotingStatusToKickValidator(uint256,uint256,address,address)";
        functionSignatures[17] = "getValidatorsInCurrentEpoch(uint256)";
        functionSignatures[
            18
        ] = "getNonShadowValidatorsInCurrentEpochLength(uint256)";
        functionSignatures[19] = "getValidatorsInNextEpoch(uint256)";
        functionSignatures[20] = "getValidatorsStructs(address[])";
        functionSignatures[21] = "getValidatorsStructsInCurrentEpoch(uint256)";
        functionSignatures[22] = "getValidatorsStructsInNextEpoch(uint256)";
        functionSignatures[23] = "getTotalStake(address)";
        functionSignatures[24] = "getTotalStakeByUser(address,address)";
        functionSignatures[25] = "getNodeStakerAddressMappings(address[])";
        functionSignatures[26] = "getNodeAttestedPubKeyMappings(address[])";
        functionSignatures[
            27
        ] = "countOfCurrentValidatorsReadyForNextEpoch(uint256)";
        functionSignatures[
            28
        ] = "countOfNextValidatorsReadyForNextEpoch(uint256)";
        functionSignatures[29] = "isReadyForNextEpoch(uint256)";
        functionSignatures[30] = "shouldKickValidator(uint256,address)";
        functionSignatures[31] = "currentValidatorCountForConsensus(uint256)";
        functionSignatures[32] = "isRecentValidator(uint256,address)";
        functionSignatures[33] = "nextValidatorCountForConsensus(uint256)";
        functionSignatures[34] = "getKickedValidators(uint256)";
        functionSignatures[35] = "getActiveUnkickedValidators(uint256)";
        functionSignatures[36] = "getStakeRecordCount(address,address)";
        functionSignatures[37] = "getValidatorsDelegated(address)";
        functionSignatures[38] = "getStakeRecordsForUser(address,address)";
        functionSignatures[39] = "getActiveUnkickedValidatorCount(uint256)";
        functionSignatures[40] = "getActiveUnkickedValidatorStructs(uint256)";
        functionSignatures[
            41
        ] = "getActiveUnkickedValidatorStructsAndCounts(uint256)";
        functionSignatures[
            42
        ] = "getTimelockInEpoch(address,(uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool,address),uint256)";
        functionSignatures[
            43
        ] = "getStakeWeightInEpoch(address,uint256,address,uint256)";
        functionSignatures[44] = "calculateStakeWeight(uint256,uint256)";
        functionSignatures[
            45
        ] = "getTokensStaked(address,(uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool,address),uint256)";
        functionSignatures[46] = "getRewardEpochNumber(uint256)";
        functionSignatures[47] = "pow(uint256,uint256)";
        functionSignatures[
            48
        ] = "calculateRewardsPerDay((uint256,uint256,address[],uint256))";
        functionSignatures[49] = "getLitCirc()";
        functionSignatures[50] = "getStakeRecord(address,uint256,address)";
        functionSignatures[
            51
        ] = "validatorSelfStakeWillExpire(uint256,address,bool)";
        functionSignatures[52] = "getRewardEpochGlobalStats(uint256)";
        functionSignatures[53] = "getTokenPrice()";
        functionSignatures[54] = "minSelfStake()";
        functionSignatures[55] = "minStake()";
        functionSignatures[56] = "maxStake()";
        functionSignatures[57] = "minTimeLock()";
        functionSignatures[58] = "maxTimeLock()";
        functionSignatures[59] = "getLowestRewardEpochNumber()";
        functionSignatures[60] = "getAllReserveValidators()";
        functionSignatures[61] = "getAllValidators()";
        functionSignatures[62] = "getSelfStakeRecordCount(address)";
        functionSignatures[63] = "permittedValidators(uint256)";
        functionSignatures[64] = "permittedRealmsForValidator(address)";
        functionSignatures[65] = "stakerToValidatorsTheyStakedTo(address)";
        functionSignatures[66] = "operatorAddressToStakerAddress(address)";
        functionSignatures[
            67
        ] = "getDelegatedStakersWithUnfreezingStakes(address,uint256,uint256)";
        functionSignatures[
            68
        ] = "getDelegatedStakersWithUnfreezingStakesCount(address)";
        functionSignatures[
            69
        ] = "getUnfrozenStakeCountForUser(address,address)";
        return functionSignatures;
    }

    function getSelectorsStakingViewsFacet()
        public
        pure
        returns (bytes4[] memory)
    {
        string[] memory functionSignatures = getSignaturesStakingViewsFacet();
        bytes4[] memory selectors = new bytes4[](functionSignatures.length);

        for (uint256 i = 0; i < functionSignatures.length; i++) {
            selectors[i] = bytes4(keccak256(bytes(functionSignatures[i])));
        }

        return selectors;
    }

    function getSignaturesStakingAdminFacet()
        public
        pure
        returns (string[] memory)
    {
        string[] memory functionSignatures = new string[](23);

        // Mutative functions that are either public or external.
        functionSignatures[0] = "setEpochLength(uint256,uint256)";
        functionSignatures[1] = "setEpochTimeout(uint256,uint256)";
        functionSignatures[2] = "setEpochEndTime(uint256,uint256)";
        functionSignatures[3] = "setEpochState(uint256,uint8)"; // uint8 is the enum value of the States enum
        functionSignatures[4] = "setDemeritRejoinThreshold(uint256,uint256)";
        functionSignatures[5] = "setContractResolver(address)";
        functionSignatures[6] = "setPermittedValidatorsOn(uint256,bool)";
        functionSignatures[7] = "adminKickValidatorInNextEpoch(address)";
        functionSignatures[8] = "adminSlashValidator(address,uint256)";
        functionSignatures[9] = "adminRejoinValidator(uint256,address)";
        functionSignatures[
            10
        ] = "adminSetValidatorsInCurrentEpoch(uint256,address[])";
        functionSignatures[
            11
        ] = "setRealmConfig(uint256,(uint256,uint256,uint256,uint256,uint256,bool,uint256))";
        functionSignatures[12] = "adminSlashValidator(uint256,address)";
        functionSignatures[13] = "setPermittedValidators(uint256,address[])";
        functionSignatures[14] = "increaseRewardPool(uint256,uint256)";
        functionSignatures[15] = "decreaseRewardPool(uint256,uint256)";
        functionSignatures[
            16
        ] = "setConfig((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))";
        functionSignatures[
            17
        ] = "setComplaintConfig(uint256,(uint256,uint256,uint256,uint256))";
        functionSignatures[
            18
        ] = "adminStakeForUser(address,address,uint256,uint256)";
        functionSignatures[
            19
        ] = "adminUnfreezeForUser(address,address,uint256)";
        functionSignatures[20] = "setDevopsAdmin(address)";
        functionSignatures[21] = "addRealm()";
        functionSignatures[22] = "removeRealm(uint256)";

        return functionSignatures;
    }

    function getSelectorsStakingAdminFacet()
        public
        pure
        returns (bytes4[] memory)
    {
        string[] memory functionSignatures = getSignaturesStakingAdminFacet();
        bytes4[] memory selectors = new bytes4[](functionSignatures.length);

        for (uint256 i = 0; i < functionSignatures.length; i++) {
            selectors[i] = bytes4(keccak256(bytes(functionSignatures[i])));
        }

        return selectors;
    }

    function getSignaturesStakingValidatorFacet()
        public
        pure
        returns (string[] memory)
    {
        string[] memory functionSignatures = new string[](14);

        // Mutative functions that are either public or external.
        functionSignatures[0] = "lockValidatorsForNextEpoch(uint256)";
        functionSignatures[1] = "signalReadyForNextEpoch(uint256,uint256)";
        functionSignatures[2] = "advanceEpoch(uint256)";
        functionSignatures[3] = "requestToJoin(uint256)";
        functionSignatures[4] = "requestToJoinAsNode(uint256,address)";
        functionSignatures[5] = "registerAttestedWallet(address,address,bytes)";
        functionSignatures[6] = "exit()";
        functionSignatures[
            7
        ] = "kickValidatorInNextEpoch(address,uint256,bytes)";
        functionSignatures[
            8
        ] = "setIpPortNodeAddress(uint32,uint128,uint32,address)";
        functionSignatures[9] = "requestToLeave()";
        functionSignatures[10] = "requestToLeaveAsNode(uint256)";
        functionSignatures[
            11
        ] = "removeValidatorFromNextEpoch(uint256,address)";
        functionSignatures[12] = "checkActiveOrUnlockedOrPausedState(uint8)"; // Enums are encoded as uint8.
        functionSignatures[
            13
        ] = "registerAttestedWallet(address,address,bytes,uint256,uint256)";

        return functionSignatures;
    }

    function getSelectorsStakingValidatorFacet()
        public
        pure
        returns (bytes4[] memory)
    {
        string[]
            memory functionSignatures = getSignaturesStakingValidatorFacet();
        bytes4[] memory selectors = new bytes4[](functionSignatures.length);

        for (uint256 i = 0; i < functionSignatures.length; i++) {
            selectors[i] = bytes4(keccak256(bytes(functionSignatures[i])));
        }

        return selectors;
    }

    function getSignaturesStakingAcrossRealmsFacet()
        public
        pure
        returns (string[] memory)
    {
        string[] memory functionSignatures = new string[](10);

        // View functions that are either public or external.
        functionSignatures[0] = "isValidatorInCurrentEpoch(address)";
        functionSignatures[1] = "isValidatorInNextEpoch(address)";
        functionSignatures[2] = "isValidatorInCurrentOrNextEpoch(address)";
        functionSignatures[3] = "getCurrentRealmIdForStakerAddress(address)";
        functionSignatures[4] = "getRealmIdForStakerAddress(address)";
        functionSignatures[
            5
        ] = "nodeAddressToStakerAddressAcrossRealms(address)";
        functionSignatures[6] = "isRecentValidator(address)";
        functionSignatures[7] = "getAllUnkickedValidators()";
        functionSignatures[8] = "validator_by_staker_address(address)";
        functionSignatures[9] = "numRealms()";

        return functionSignatures;
    }

    function getSelectorsStakingAcrossRealmsFacet()
        public
        pure
        returns (bytes4[] memory)
    {
        string[]
            memory functionSignatures = getSignaturesStakingAcrossRealmsFacet();
        bytes4[] memory selectors = new bytes4[](functionSignatures.length);

        for (uint256 i = 0; i < functionSignatures.length; i++) {
            selectors[i] = bytes4(keccak256(bytes(functionSignatures[i])));
        }

        return selectors;
    }

    function getSignaturesStakingVersionFacet()
        public
        pure
        returns (string[] memory)
    {
        string[] memory functionSignatures = new string[](7);

        // Mutative functions that are either public or external.
        functionSignatures[
            0
        ] = "setMinVersion(uint256,(uint256,uint256,uint256))";
        functionSignatures[
            1
        ] = "setMaxVersion(uint256,(uint256,uint256,uint256))";
        functionSignatures[
            2
        ] = "checkVersion(uint256,(uint256,uint256,uint256))";
        functionSignatures[3] = "getMinVersion(uint256)";
        functionSignatures[4] = "getMaxVersion(uint256)";
        functionSignatures[5] = "getMinVersionString(uint256)";
        functionSignatures[6] = "getMaxVersionString(uint256)";

        return functionSignatures;
    }

    function getSelectorsStakingVersionFacet()
        public
        pure
        returns (bytes4[] memory)
    {
        string[] memory functionSignatures = getSignaturesStakingVersionFacet();
        bytes4[] memory selectors = new bytes4[](functionSignatures.length);

        for (uint256 i = 0; i < functionSignatures.length; i++) {
            selectors[i] = bytes4(keccak256(bytes(functionSignatures[i])));
        }

        return selectors;
    }
}

contract PriceFeedFunctionSelectorHelper {
    function getSignaturesPriceFeedFacet()
        public
        pure
        returns (string[] memory)
    {
        string[] memory functionSignatures = new string[](15);
        functionSignatures[0] = "price(address,uint256[])";
        functionSignatures[1] = "prices(uint256)";
        functionSignatures[2] = "baseNetworkPrices(uint256[])";
        functionSignatures[3] = "maxNetworkPrices(uint256[])";
        functionSignatures[4] = "usagePercentToPrices(uint256,uint256[])";
        functionSignatures[5] = "usagePercentToPrice(uint256,uint256)";
        functionSignatures[6] = "getNodeCapacityConfig()";
        functionSignatures[7] = "getNodesForRequest(uint256,uint256[])";
        functionSignatures[8] = "setUsage(uint256,uint256[])";
        functionSignatures[9] = "setBaseNetworkPrices(uint256,uint256[])";
        functionSignatures[10] = "setMaxNetworkPrices(uint256,uint256[])";
        functionSignatures[
            11
        ] = "setNodeCapacityConfig(LibPriceFeedStorage.NodeCapacityConfig)";
        functionSignatures[12] = "getLitActionPriceConfigs()";
        functionSignatures[
            13
        ] = "setLitActionPriceConfig(LibPriceFeedStorage.LitActionPriceComponent,LibPriceFeedStorage.NodePriceMeasurement,uint256)";
        functionSignatures[
            14
        ] = "setLitActionPriceConfigs(LibPriceFeedStorage.LitActionPriceConfig[])";
        return functionSignatures;
    }

    function getSelectorsPriceFeedFacet()
        public
        pure
        returns (bytes4[] memory)
    {
        string[] memory functionSignatures = getSignaturesPriceFeedFacet();
        bytes4[] memory selectors = new bytes4[](functionSignatures.length);

        for (uint256 i = 0; i < functionSignatures.length; i++) {
            selectors[i] = bytes4(keccak256(bytes(functionSignatures[i])));
        }
        return selectors;
    }
}
