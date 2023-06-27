# Unsafe unwrap

## Description

- Vulnerability Category: `Validations and error handling`
- Vulnerability Severity: `Minor`
- Detectors: [`unsafe-unwrap`](https://github.com/CoinFabrik/scout/tree/main/detectors/unsafe-unwrap)
- Test Cases: [`unsafe-unwrap-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unsafe-unwrap/unsafe-unwrap-1)

In Rust, the `unwrap` method is commonly used for error handling. It retrieves the inner value of an `Option` or `Result`. If an error or `None` occurs, it calls `panic!` without a custom error message.

The usage of `unwrap` can lead to a panic and crash the program, which is not desired behavior in most cases, particularly in smart contracts.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
#[ink::contract]
mod unsafe_unwrap {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct UnsafeUnwrap {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    // ...

    impl UnsafeUnwrap {
        /// Returns the balance of a given account.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap()
        }

        // ...
    }
}
```

In this contract, the `balance_of` function uses the `unwrap` method to retrieve the balance of an account. If there is no entry for this account in the balances mapping, the contract will panic and halt execution, potentially leading to malicious exploitation to disrupt the contract's operation.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unsafe-unwrap/unsafe-unwrap-1/vulnerable-example).

## Remediation

Instead of using `unwrap`, use a safer method for error handling. In this case, if there is no entry for an account in the `balances` mapping, return a default value (like `0`).

```rust
#[ink::contract]
mod unsafe_unwrap {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct UnsafeUnwrap {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    // ...

    impl UnsafeUnwrap {
        /// Returns the balance of a given account.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        // ...
    }
}
```

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unsafe-unwrap/unsafe-unwrap-1/remediated-example).

## References

[Rust documentation: `unwrap`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap)
