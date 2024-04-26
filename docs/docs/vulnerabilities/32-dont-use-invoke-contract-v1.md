# Don't use invoke_contract_v1

## Description 
- Vulnerability Category: ` `
- Vulnerability Severity: ` `
- Detectors: [`dont-use-invoke-contract-v1`](https://github.com/CoinFabrik/scout/tree/main/detectors)
- Test Cases: [`dont-use-invoke-contract-v1`](https://github.com/CoinFabrik/scout/tree/main/test-cases)

Avoid using `invoke_contract_v1` as it is a low level way to evaluate another smart contract. 


## Exploit Scenario



## Remediation

```rust
// Dont use it
```

## References

- https://docs.rs/ink_env/5.0.0/ink_env/fn.invoke_contract_v1.html




