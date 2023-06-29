# Delegate Call

## Description

- Vulnerability Category: `Unsecure delegate calls`
- Vulnerability Severity: `Major`
- Detectors: [`delegate-call`](https://github.com/CoinFabrik/scout/tree/main/detectors/delegate-call)
- Test Cases: [`delegate-call-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1)

In Rust, the `delegate call` is used to invoke a method from another contract. If the target of the delegate call is passed as an argument, it can be used to change the expected behavior of the contract. This can be exploited maliciously to disrupt the contract's operation.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
#[ink::contract]
mod delegate_call {

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };

    #[ink(storage)]
    pub struct DelegateCall {
        admin: AccountId,
        addresses: [AccountId; 3],
        percent1: u128,
        percent2: u128,
        percent3: u128,
    }

    impl DelegateCall {

        // ...

        /// Delegates the fee calculation and pays the results to the corresponding addresses
        #[ink(message, payable)]
        pub fn ask_payouts(&mut self, target: Hash) -> Result<(Balance, Balance, Balance), Error> {
            let amount = self.env().transferred_value();

            let result: (Balance, Balance, Balance) = build_call::<DefaultEnvironment>()
                .delegate(target)
                // ...
        }
    }
}
```

In this contract, the `ask_payouts` function takes a `Hash` as a target and delegates a call to that target. A malicious user could potentially manipulate the function to their advantage by providing a malicious `Hash` as the target.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1/vulnerable-example).

## Remediation

Instead of passing the target of a delegate call as an argument, use a storage variable (like `self.target`). Also, provide a function with proper access control to change the target.

```rust
#[ink::contract]
mod delegate_call {

    #[ink(storage)]
    pub struct DelegateCall {
        admin: AccountId,
        addresses: [AccountId; 3],
        target: Hash,
    }

    impl DelegateCall {

        // ...

        /// Delegates the fee calculation and pays the results to the corresponding addresses
        #[ink(message, payable)]
        pub fn ask_payouts(&mut self, amount: Balance) -> Result<(), Error> {
            let result = ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .delegate(self.target)
                // ...
        }

        /// Sets the target codehash for the delegated call
        #[ink(message)]
        pub fn set_target(&mut self, new_target: Hash) -> Result<(), &'static str> {
           if self.admin != self.env().caller() {
                Err("Only admin can set target")
            } else {
                self.target = new_target;
                Ok(())
            }
        }

    }
}
```

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1/remediated-example).

## Reference

[ink! documentation: DelegateCall](https://paritytech.github.io/ink/ink_env/call/struct.DelegateCall.html)
