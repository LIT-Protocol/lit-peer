#!/bin/bash

set -e

mkdir -p src/contracts

# Create rust bindings
pushd ../lit-contracts-minimal-generator
cargo run --bin generate_contracts
popd

# Format the generated contracts
directories=(
  "../../lit-core/lit-blockchain"
  "../../lit-core/lit-blockchain-lite"
  "../../lit-node"
)
for dir in "${directories[@]}"; do
  pushd "$dir"
  cargo fmt
  popd
done
