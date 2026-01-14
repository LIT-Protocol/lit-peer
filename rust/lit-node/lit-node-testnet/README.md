# Local Multi-Chain Compatibility

This crate supports interacting with a secondary local chain. Currently, the secondary chain is implied as `datil` related but it doesn't have to be (fix: rename variables to be more general than "datil"). This is possible because of the following reasons:
1. `ImportedDatilTestnet` accepts an Anvil state file (from `--dump-state` option, see instructions [here](https://github.com/LIT-Protocol/lit-assets/pull/2356)) which allows a secondary Anvil node to be spun up from recorded state.
2. The `lit-blockchain-datil` dependency allows this crate to use the correct Rust bindings that enable interacting with this Anvil node.

Due to the above, **it is important to ensure the compatibility of the state file and the Rust bindings that are used**. This means that the state file committed to source and used in CI needs to be obtained from the same `lit-assets` branch that is specified in `Cargo.toml`.

## Obtaining Anvil State File

1. Head over to `lit-assets`.
2. Make sure you are checked out on a release branch instead of the `datil` mainline. Run test with command `RUST_LOG=test=trace,lit_node=trace,integration_tests=trace,ecdsa=trace cargo nextest run --final-status-level pass -E 'test(/integration/)' --run-ignored=only --nocapture -- spin_up_network_for_state_dump > integ_tests.log 2>&1`. Make note of the staking and contract resolver addresses. For testing purposes, the state file just needs to be obtained from the same branch as is specified in `Cargo.toml`.

**NOTE: The `spin_up_network_for_state_dump` test uses `adminResetRootKeys` and `adminSetRootKeys` to hack in the root keys so this secondary Anvil chain will support interactions such as minting PKPs but not necessarily for everything else, especially anything that concerns Staking contract (because the validators will likely be different, and there actually are no nodes spun up to drive the Staking-related interactions for this secondary Anvil chain.)**