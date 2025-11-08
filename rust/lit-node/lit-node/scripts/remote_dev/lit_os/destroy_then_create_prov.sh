#!/bin/bash

source ./scripts/remote_dev/vars.sh

[[ -z $PROV_HOST_IP ]] && exit -1
[[ -z $PROV_GUEST_IP ]] && exit -1
[[ -z $PROV_GUEST_GATEWAY ]] && exit -1
[[ -z $LIT_DEV_PROV_WALLET_PRIVATE_KEY ]] && exit -1
[[ -z $LIT_DEV_PROV_ADMIN_PRIVATE_KEY ]] && exit -1

# destroy
./scripts/remote_dev/lit_os/expect/destroy_prov.exp "$PROV_HOST_IP" "$old_staking_contract_address"

# create template
./scripts/remote_dev/lit_os/expect/create_prov_template.exp "$PROV_HOST_IP"

# create prov instance
./scripts/remote_dev/lit_os/expect/create_prov.exp "$PROV_HOST_IP" "$new_staking_contract_address" "$LIT_DEV_PROV_WALLET_PRIVATE_KEY" "$LIT_DEV_PROV_ADMIN_PRIVATE_KEY" "$PROV_GUEST_IP" "$PROV_GUEST_GATEWAY"

echo "Please commit changes in the networks directory"
