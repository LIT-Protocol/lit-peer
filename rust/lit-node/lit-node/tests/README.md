# Overview

Lit node testing comprises of a half dozen different categories of tests - 

## Component

Component test isolate a specific node functionality and mimic a variable network of nodes in a single threaded test, with little to no external dependencies. Typically component tests are run on cryptographic algorithms to ensure that they function correctly prior to adding in the complexities of chain interaction and MPC administration/management functions.   They often also remove checks and prerequisites, such as authorization checks.

## Integration

Integration tests include deployment of the network's contracts, and creation of configuration files for the nodes involved. They  involve a testnet (representing the underlying chain) and a node / validator collection.   Integration tests closely mimic a real deployment, and use production deployment scripts for setup purposes. 

Integration tests are further broken down into :

### Acceptance

A series of broad tests that provide a santity check of the network ensuring that basic cryptographic functions work properly, along with SDK prerequisites - like handshaking with the nodes. 

### Backup Tests

Tests the backup / restore functionality within the nodes.

### ECDSA

Tests related to using PKPs to sign ECDSA transactions.

### Epoch Change

Various tests to ensure that the nodes and they cryptographic material survive changing epochs, and continue to function during epoch changes and various edge cases ( like nodes dropping )

### Lit Action 

Tests various `lit action` functions - these tests use a folder titled `lit_action_scripts` which contain real `lit actions` to be executed by the nodes.   They cover most of the basic `op_code` functionality that the nodes support along with several edge and error cases.


### Session Sigs

Various tests to ensure that `session sig`s can be created by the SDK and used in various capacities, like delegation of permissions. 

### Integration Tests

A catch-all for other types of tests that rely upon a network being present.


### External

This is a single test that spins up a network for use with any other external tests.   The test also spins up a web server that provides secret keys and PKPs.   It will likely be removed in favor of using the Shiva Project in 2025. 

## Chaos (ToxiProxy) Tests

Various tests that ensure nodes work properly in unreliable environements - ToxiProxy is used to create slow connections, dropping of packets, etc, etc.

## Upgrade tests

Used to test upgrade a network which gradually replaces nodes with newer code.  This ensures that nodes can continue to interact with each other during upgrades abnd that breaking changes are handled gracefully. 

## Prerequisites:

### Supported chains

Tests that involve interaction with chains require `Anvil` to be installed an upgraded with Lit's precompiles.  

### Non-Component tests

Tests that are NOT component tests use `node` scripts to:

1. (Re)Start `Anvil` 
2. Deploy contracts
3. Fund nodes / stakers
4. Create node configuration files


# Running:

To run the chain tests, from the top level do (nocapture is optional).
```
cargo test --test integration_tests -- --nocapture
```


Run only a specific test within a specific integration test, saving the log:
```
rm addthenremove.log; cargo test chained_addthenremove --test integration_tests -- --nocapture > addthenremove.log
```


### Using logs
Example
```
RUST_LOG=lit_node=trace cargo test initial_only --test integration_tests -- --nocapture > the.log 2>&1
```

# Developer Notes

### Layout
Each test file in the `tests` folder generates a seperate crate.  Shared libraries are stored in `tests/common/` and are organized according to feature.    If the core of a test needs to be re-used, it is generally advisable to store shared functionality in the `common` folder.

With the advent of the `Shiva` project certain common files have been shifted to two new projects to avoid circular dependencies in the node code: 

1. `lit-node-common` contains a few common structures shared between the nodes and tests
2. `lit-node-testnet` contains 3 basic helper fucntions:

### Test helper structs/impls

- The `testnet` which is a coded representation of the chain.
- The `validator collection` which is a representation of the nodes themselves.  
- `Actions`, which are a function of the validators and use the `testnet` and `validator collection` to evaluate common actions seen in production - ie, `wait_for_epoch_change` or `hit_endpoints`.
 


