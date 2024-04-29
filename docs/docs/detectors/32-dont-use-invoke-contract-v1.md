# Don't use invoke_contract_v1

### What it does

Checks that method `invoke_contract_v1` is not used in the contract.

### Why is this bad?

This will call into the original version of the host function. It is recommended to use `invoke_contract` to use the latest version if the target runtime supports it.

#### More info

- https://docs.rs/ink_env/5.0.0/ink_env/fn.invoke_contract_v1.html

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/dont-use-invoke-contract-v1).
