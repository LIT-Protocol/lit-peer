#!/bin/bash

# prov leaseweb-staging-1.litgateway.com
declare -a PROV_HOST_IP="198.7.56.184"
declare -a PROV_GUEST_IP="207.244.86.72/27"
declare -a PROV_GUEST_GATEWAY="207.244.86.94"

# nodes
declare -a HOSTS=(
    "158.69.34.225/28"  # ovh-staging-1.litgateway.com
    "15.235.67.38/32"   # ovh-staging-2.litgateway.com
    "167.114.17.195/28" # ovh-staging-6.litgateway.com
    "167.114.17.196/28" # ovh-staging-7.litgateway.com
    "167.114.17.197/28" # ovh-staging-8.litgateway.com
    "192.96.205.46/27"  # leaseweb-staging-4.litgateway.com
    "108.62.0.103/26"   # leaseweb-staging-5.litgateway.com
)
declare -a GUESTS_WITH_SM=(
    "158.69.34.226/28"
    "158.69.34.228/28"
    "167.114.17.203/28"
    "167.114.17.204/28"
    "167.114.17.205/28"
    "199.115.117.115/26"
    "108.62.0.105/26"
)
declare -a GUEST_GATEWAYS=( 
    "158.69.34.238"
    "158.69.34.238"
    "167.114.17.206"
    "167.114.17.206"
    "167.114.17.206"
    "199.115.117.126"
    "108.62.0.126"
)
