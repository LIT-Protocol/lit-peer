#!/bin/bash

source ./scripts/remote_dev/vars.sh

# we need stoml below
which stoml >/dev/null 2>&1 || { echo "stoml required but not found, install from https://github.com/freshautomations/stoml/releases"; exit -1; }

toml_path="$1"
default_toml_path="../../blockchain/contracts/node_configs"

if [ -z "$toml_path" ]; then
    echo "Usage: $0 <path to toml files>"
    echo "Using default TOML path since you didn't pass one: $default_toml_path"
    toml_path="$default_toml_path"
fi

if [ ! -d "$toml_path" ]; then
    echo "Error: $toml_path is not a directory"
    exit 1
fi
counter=0
for i in "${HOSTS[@]}"; do
    file="$toml_path/lit_config$counter.toml"
    if [ -f "$file" ]; then
        echo "Creating node $counter with file $file"
        export host_ip="$i"
        export host_ip_no_cidr=${host_ip%/*}
        export ip_address=${GUESTS_WITH_SM[$counter]}
        export gw=${GUEST_GATEWAYS[$counter]}
        export subnet_id=`stoml "$file" subnet.id`
        export staker_address=`stoml "$file" node.staker_address`
        export wallet_key=`stoml "$file" blockchain.wallet.default.private_key`
        export coms_sender_key=`stoml "$file" node.coms_keys_sender_privkey`
        export coms_receiver_key=`stoml "$file" node.coms_keys_receiver_privkey`
        export admin_address=`stoml "$file" node.admin_address`
        sleep $((counter * 120)) && ./scripts/remote_dev/lit_os/expect/destroy_then_create_node.exp "$host_ip_no_cidr" "$ip_address" "$gw" "$subnet_id" "$staker_address" "$wallet_key" "$coms_sender_key" "$coms_receiver_key" "$admin_address" &
        counter=$((counter+1))
        sleep 30
    fi
done