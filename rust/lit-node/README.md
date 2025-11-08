# What

This directory is for code that is common across crates (test and binaries) in the `rust/lit-node` directory.

# Notes

- `lit-node-common` has some common code that is used by both the `shiva` binary and the `lit-node-testnet` testnet, effectively avoiding some circular dependencies.
- `lit-node-testnet` is a testnet for testing the lit-node binary, and is shared withe `tests` in `lit-node` and the `shiva` binary.
- `shiva` is a runner designed to handle multiple instances of a lit network (multiple validators) and the local `anvil` chain.

Most of this should be flattened into a generic `lit-node` project, rather than having multiple sub-directoriers.... just a todo for now.
