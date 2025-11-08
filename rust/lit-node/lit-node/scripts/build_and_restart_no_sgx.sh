#!/bin/bash

if ! command -v yq &> /dev/null; then
    echo "Error: yq is not installed. Please install yq to proceed."
    exit 1
fi

cargo build

./scripts/multi-stop.sh

sleep 2

# Update the rpc-config.yaml file
if [ -f "./rpc-config.yaml" ]; then
    echo "rpc-config.yaml file has $(yq '.chains | length' ./rpc-config.yaml) network definitions (pre-update)"
else
    echo "rpc-config.yaml file does not exist, creating it from rpc-config.example.yaml"
fi

# Check if there's an overlay file and merge on top of the example to create the final rpc-config.yaml file
if [ -f "./rpc-config.overlay.yaml" ]; then
    echo "Found rpc-config.overlay.yaml, merging with base configuration..."
    yq eval-all 'select(fileIndex == 0) * select(fileIndex == 1)' ./rpc-config.example.yaml ./rpc-config.overlay.yaml > ./rpc-config.yaml
    echo "Configuration overlay applied successfully."
else
    echo "No overlay file found, using base configuration."
    cp ./rpc-config.example.yaml ./rpc-config.yaml
fi
echo "rpc-config.yaml file has $(yq '.chains | length' ./rpc-config.yaml) network definitions (post-update)"

# Dump the binary to `lit-node/lit-node` so it's in the same folder as its config files
cp ../target/debug/lit_node ./

./scripts/multi-start.sh


# also build lit-actions
cd ../../lit-actions

cargo build

sudo systemctl stop lit-actions@{0..2}

sleep 2

cp target/debug/lit_actions .

sudo systemctl start lit-actions@{0..2}
