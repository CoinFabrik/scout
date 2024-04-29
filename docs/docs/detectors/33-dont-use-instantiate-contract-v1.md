# Don't use instantiate_contract_v1

### What it does

Checks that method `instantiate_contract_v1` is not used in the contract.

### Why is this bad?

This is a low level way to instantiate another smart contract, calling the legacy `instantiate_v1` host function.

Prefer to use methods on a `ContractRef` or the `CreateBuilder` through `build_create` instead.

#### More info

- https://docs.rs/ink_env/5.0.0/ink_env/fn.instantiate_contract_v1.html

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/dont-use-instantiate-contract-v1).
