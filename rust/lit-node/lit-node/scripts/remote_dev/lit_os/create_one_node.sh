#!/bin/bash

source ./scripts/remote_dev/vars.sh

# we need stoml below
which stoml >/dev/null 2>&1 || { echo "stoml required but not found, install from https://github.com/freshautomations/stoml/releases"; exit -1; }

node_index="$1"
toml_path="../../blockchain/contracts/node_configs"

if [ -z "$node_index" ]; then
    echo "Usage: $0 <node index>"
    echo "Error: You must pass node index"
    exit
fi

file="$toml_path/lit_config$node_index.toml"
if [ -f "$file" ]; then
    echo "Creating node $node_index with file $file"
    export host_ip=${HOSTS[$node_index]}
    export host_ip_no_cidr=${host_ip%/*}
    export ip_address=${GUESTS_WITH_SM[$node_index]}
    export gw=${GUEST_GATEWAYS[$node_index]}
    export subnet_id=`stoml "$file" subnet.id`
    export staker_address=`stoml "$file" node.staker_address`
    export wallet_key=`stoml "$file" blockchain.wallet.default.private_key`
    export coms_sender_key=`stoml "$file" node.coms_keys_sender_privkey`
    export coms_receiver_key=`stoml "$file" node.coms_keys_receiver_privkey`
    export admin_address=`stoml "$file" node.admin_address`
    ./scripts/remote_dev/lit_os/expect/destroy_then_create_node.exp "$host_ip_no_cidr" "$ip_address" "$gw" "$subnet_id" "$staker_address" "$wallet_key" "$coms_sender_key" "$coms_receiver_key" "$admin_address" &
fi