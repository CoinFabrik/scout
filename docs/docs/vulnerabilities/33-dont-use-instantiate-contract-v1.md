# Don't use instantiate_contract_v1

## Description 
- Vulnerability Category: `Best Practices`
- Vulnerability Severity: `Enhancement`
- Detectors: [`dont-use-instantiate-contract-v1`](https://github.com/CoinFabrik/scout/tree/main/detectors)
- Test Cases: [`dont-use-instantiate-contract-v1`](https://github.com/CoinFabrik/scout/tree/main/test-cases)

Avoid using `instantiate_contract_v1` as it is a low level way to evaluate another smart contract. If needed, use `instantiate_contract` instead. Also, use methods on a `ContractRef` or the `CreateBuilder` through `build_create` if possible.


## Exploit Scenario

Consider the following example

```rust
    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn instantiate_contract(&self) -> MyContractRef {
            let create_params = build_create::<OtherContractRef>()
                .instantiate_v1()
                .code_hash(Hash::from([0x42; 32]))
                .gas_limit(500_000_000)
                .endowment(25)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("new")))
                        .push_arg(42)
                        .push_arg(true)
                        .push_arg(&[0x10u8; 32]),
                )
                .salt_bytes(&[0xCA, 0xFE, 0xBA, 0xBE])
                .returns::<OtherContractRef>()
                .params();
            self.env()
                .instantiate_contract_v1(&create_params)
                .unwrap_or_else(|error| {
                    panic!(
                        "Received an error from the Contracts pallet while instantiating: {:?}",
                        error
                    )
                })
                .unwrap_or_else(|error| {
                    panic!("Received a `LangError` while instatiating: {:?}", error)
                })
        }
    }
```

## Remediation

```rust
    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn instantiate_contract(&self) -> MyContractRef {
            let create_params = build_create::<OtherContractRef>()
                .code_hash(Hash::from([0x42; 32]))
                .ref_time_limit(500_000_000)
                .proof_size_limit(100_000)
                .storage_deposit_limit(500_000_000_000)
                .endowment(25)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("new")))
                        .push_arg(42)
                        .push_arg(true)
                        .push_arg(&[0x10u8; 32]),
                )
                .salt_bytes(&[0xCA, 0xFE, 0xBA, 0xBE])
                .returns::<OtherContractRef>()
                .params();
            self.env()
                .instantiate_contract(&create_params)
                .unwrap_or_else(|error| {
                    panic!(
                        "Received an error from the Contracts pallet while instantiating: {:?}",
                        error
                    )
                })
                .unwrap_or_else(|error| {
                    panic!("Received a `LangError` while instatiating: {:?}", error)
                })
        }
    }
```


## References

- https://docs.rs/ink_env/5.0.0/ink_env/fn.instantiate_contract_v1.html
