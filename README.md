# stateless

Stateless Ethereum block validation using execution witnesses.

Provides types and functions for validating Ethereum blocks without access to a full node's persistent database. Instead, it relies on pre-generated witness data that proves the specific state accessed during block execution.

## Overview

- **Witness verification** against a trusted pre-state root
- **In-memory execution** using a witness-backed database
- **Post-execution validation** including state root computation
- **EF test support** for running Ethereum Foundation tests with stateless validation

## Usage

```rust
use stateless::{stateless_validation, ExecutionWitness};

let (block_hash, output) = stateless_validation(
    block,
    public_keys,
    witness,
    chain_spec,
    evm_config,
)?;
```

## EF Tests

To run the Ethereum Foundation blockchain tests with stateless validation:

```bash
# Clone the test fixtures
cd testing/ef-tests
git clone https://github.com/ethereum/tests ethereum-tests

# Run tests
cargo test --features ef-tests
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
