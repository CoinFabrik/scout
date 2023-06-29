# Divide before multiply

## Description

- Vulnerability Category: `Arithmetic`
- Vulnerability Severity: `Medium`
- Detectors: [`divide-before-multiply`](https://github.com/CoinFabrik/scout/tree/main/detectors/divide-before-multiply)
- Test Cases: [`divide-before-multiply-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/divide-before-multiply/divide-before-multiply-1)

In Rust, the order of operations can influence the precision of the result, especially in integer arithmetic. Performing a division operation before a multiplication can lead to a loss of precision as division between integers might return zero. This issue can have serious consequences in programs such as smart contracts where numerical precision is critical.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
#[ink::contract]
mod divide_before_multiply {

    #[ink(storage)]
    pub struct FloatingPointAndNumericalPrecision {}

    impl FloatingPointAndNumericalPrecision {
        /// Creates a new FloatingPointAndNumericalPrecision contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Calculates the profit for a given percentage of the total profit.
        #[ink(message)]
        pub fn split_profit(&self, percentage: u64, total_profit: u64) -> u64 {
            (percentage / 100) * total_profit
        }
    }
}
```

In this contract, the `split_profit` function divides the `percentage` by `100` before multiplying it with `total_profit`. This could lead to a loss of precision if `percentage` is less than `100` as the division would return `0`. This could lead to incorrect calculations and potential financial loss in a real-world smart contract.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/divide-before-multiply/divide-before-multiply-1/vulnerable-example).

## Remediation

Reverse the order of operations to ensure multiplication occurs before division.

```rust
#[ink::contract]
mod divide_before_multiply {

    #[ink(storage)]
    pub struct FloatingPointAndNumericalPrecision {}

    impl FloatingPointAndNumericalPrecision {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn split_profit(&self, percentage: u64, total_profit: u64) -> u64 {
            (percentage * total_profit) / 100
        }
    }
}
```

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/divide-before-multiply/divide-before-multiply-1/remediated-example).

## References

[Rust documentation: `Integer Division`](https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators)
