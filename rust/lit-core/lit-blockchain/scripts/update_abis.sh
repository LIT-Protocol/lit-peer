#!/bin/bash

set -e

export SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export BLOCKCHAIN_DIR="$SCRIPT_DIR/../../../../blockchain/contracts"

if [ ! -e "${BLOCKCHAIN_DIR}" ]; then
    echo "Error: blockchain contracts dir is required"
    echo "       (dir: $(readlink -f $BLOCKCHAIN_DIR)"
    exit 2
  exit 2
fi

cd $BLOCKCHAIN_DIR
npm install
npx hardhat clean
npx hardhat compile

cd $SCRIPT_DIR/..
mkdir -p abis

declare -a NORMAL_CONTRACTS=("LITToken" "Allowlist" "HDKeyDeriver" "ContractResolver" "ReleaseRegister"  "Multisender" "PKPNFTMetadata" "PKPHelper" "WLIT" "HostCommands" "Forwarder")

copy_just_abi_to_contracts_dir(){
  local contract_artifacts_path="$1"
  local blockchain_dir="$2"
  local filename="$3"
  local filename_no_suffix="$4"
  mkdir -p "$blockchain_dir/abis"
  jq '.abi' "$contract_path.sol/$filename" > "$blockchain_dir/abis/$filename_no_suffix.abi"
}


sync_abis() {
  local path="$1"

  for contract in $(ls -d $path/artifacts/contracts/**/*.sol); do
    local contract_path=${contract%.*}
    local contract_name=$(basename $contract_path)
    if [[ ! " ${NORMAL_CONTRACTS[@]} " =~ " ${contract_name} " ]]; then
      continue
    fi
    if [ -e "${contract_path}.sol/${contract_name}.json" ]; then
      # uncomment this if you just want the ABIs.  We now copy over the whole file though
      # jq '.abi' "${contract_path}.sol/${contract_name}.json" > ./abis/${contract_name}.json
      cp "${contract_path}.sol/${contract_name}.json" ./abis/${contract_name}.json
      copy_just_abi_to_contracts_dir "$contract_path" "$path" "$contract_name.json" "$contract_name"
    else # if a .sol file contains contract named differently than the file name
      for mismatch_contracts in $(ls "$contract_path.sol"); do
        local mismatch_contract_path=${mismatch_contracts%.*}
        local mismatch_contract_name=$(basename $mismatch_contract_path)
        if [[ $mismatch_contract_name != *".dbg"* ]]; then # filters debug files
          cp "${contract_path}.sol/${mismatch_contract_name}.json" ./abis/${mismatch_contract_name}.json
          copy_just_abi_to_contracts_dir "$contract_path" "$path" "$mismatch_contract_name.json" "$mismatch_contract_name"
        fi
      done
    fi
  done

  ## copy over the diamond ABIs and rename
  for contract in $(ls -d $path/artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI.sol/*.json); do
    local contract_path="$path/artifacts/@lit-protocol/hardhat-diamond-abi/HardhatDiamondABI"
    local contract_name=$(basename $contract)
    if [ -e "$contract" ]; then
      # uncomment this if you just want the ABIs.  We now copy over the whole file though
      # jq '.abi' "${contract_path}.sol/${contract_name}.json" > ./abis/${contract_name}.json
      # remove the word "Diamond" from the contract name
      contract_name_no_suffix=${contract_name/Diamond/}
      contract_name_no_extension=${contract_name_no_suffix%.*}

      cp "$contract" ./abis/${contract_name_no_suffix}
      copy_just_abi_to_contracts_dir "$contract_path" "$path" "$contract_name" "$contract_name_no_extension"
    else # if a .sol file contains contract named differently than the file name
      echo "Error!  $contract does not exist"
      exit 1
      # for mismatch_contracts in $(ls "$contract_path.sol"); do
      #   local mismatch_contract_path=${mismatch_contracts%.*}
      #   local mismatch_contract_name=$(basename $mismatch_contract_path)
      #   if [[ $mismatch_contract_name != *".dbg"* ]]; then # filters debug files
      #     cp "${contract_path}.sol/${mismatch_contract_name}.json" ./abis/${mismatch_contract_name}.json
      #     copy_just_abi_to_contracts_dir "$contract_path" "$path" "$mismatch_contract_name"
      #   fi
      # done
    fi
  done
}

sync_abis "$BLOCKCHAIN_DIR"
