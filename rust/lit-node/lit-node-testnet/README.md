# Local Multi-Chain Compatibility

This crate supports interacting with a secondary local chain. Currently, the secondary chain is implied as `datil` related but it doesn't have to be (fix: rename variables to be more general than "datil"). This is possible because of the following reasons:
1. `ImportedDatilTestnet` accepts an Anvil state file (from `--dump-state` option, see instructions [here](https://github.com/LIT-Protocol/lit-assets/pull/2356)) which allows a secondary Anvil node to be spun up from recorded state.
2. The `lit-blockchain-datil` dependency allows this crate to use the correct Rust bindings that enable interacting with this Anvil node.

Due to the above, **it is important to ensure the compatibility of the state file and the Rust bindings that are used**. This means that the state file committed to source and used in CI needs to be obtained from the same `lit-assets` branch that is specified in `Cargo.toml`.