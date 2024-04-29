# Don't use invoke_contract_v1

## Description 
- Vulnerability Category: `Best Practices`
- Vulnerability Severity: `Enhancement`
- Detectors: [`dont-use-invoke-contract-v1`](https://github.com/CoinFabrik/scout/tree/main/detectors/dont-use-invoke-contract-v1)
- Test Cases: [`dont-use-invoke-contract-v1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/dont-use-invoke-contract-v1)

Avoid using `invoke_contract_v1` as it is a low level way to evaluate another smart contract. If needed, use `invoke_contract` instead.


## Exploit Scenario

Consider the following example

```rust
        /// Calls the given address with the given amount and selector.
        #[ink(message)]
        pub fn call_with_value(
            &mut self,
            address: AccountId,
            amount: Balance,
            selector: u32,
        ) -> Balance {
            ink::env::debug_println!(
                "call_with_value function called from {:?}",
                self.env().caller()
            );
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            if amount <= caller_balance {
                //The call is built without allowing reentrancy calls
                let call = build_call::<ink::env::DefaultEnvironment>()
                    .call_v1(address)
                    .transferred_value(amount)
                    .exec_input(ink::env::call::ExecutionInput::new(Selector::new(
                        selector.to_be_bytes(),
                    )))
                    .returns::<()>()
                    .params();
                self.env()
                    .invoke_contract_v1(&call)
                    .unwrap_or_else(|err| panic!("Err {:?}", err))
                    .unwrap_or_else(|err| panic!("LangErr {:?}", err));
                self.balances
                    .insert(caller_addr, &(caller_balance - amount));

            Ok(())
                caller_balance - amount
            } else {
                caller_balance
            }
        }
```

## Remediation

```rust
    // Dont use it altogether or use invoke_contract but beware it can have several Errors.
    #[ink(message)]
    pub fn call_with_value(
            &mut self,
            address: AccountId,
            amount: Balance,
            selector: u32,
        ) -> Balance {
            ink::env::debug_println!(
                "call_with_value function called from {:?}",
                self.env().caller()
            );
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            if amount <= caller_balance {
                //The call is built without allowing reentrancy calls
                let call = build_call::<ink::env::DefaultEnvironment>()
                    .call(address)
                    .transferred_value(amount)
                    .exec_input(ink::env::call::ExecutionInput::new(Selector::new(
                        selector.to_be_bytes(),
                    )))
                    .returns::<()>()
                    .params();
                self.env()
                    .invoke_contract(&call)
                    .unwrap_or_else(|err| panic!("Err {:?}", err))
                    .unwrap_or_else(|err| panic!("LangErr {:?}", err));
                self.balances
                    .insert(caller_addr, &(caller_balance - amount));

                caller_balance - amount
            } else {
                caller_balance
            }
        }
```

## References

- https://docs.rs/ink_env/5.0.0/ink_env/fn.invoke_contract_v1.html




