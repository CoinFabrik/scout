# Unused return enum

### What it does

It warns if a function that returns a Result type does not return the Result enum variant (Ok/Err).

### Why is this bad?

If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug.

### Known problems

If definitions of Err() and/or Ok() are in the code but do not flow to the return value due to the definition of a variable or because they are defined in a dead code block, the warning will not be shown. If the definitions are made in an auxiliary method, the warning will be shown, resulting in a false positive.

### Example

Instead of using:

```rust
// example code that does not raise a warning
    #![cfg_attr(not(feature = "std"), no_std)]
    pub enum TradingPairErrors {
        Overflow,
    }

    #[ink(message)]
    pub fn get_percentage_difference(&mut self, value1: Balance, value2: Balance) -> Result<Balance, TradingPairErrors>  {
        let absolute_difference = value1.abs_diff(value2);
        let sum = value1 + value2;
        let percentage_difference =
        match 100u128.checked_mul(absolute_difference / sum) {
           Some(result) => Ok(result),
           None => panic!("overflow!"),
        };
        return Err(TradingPairErrors::Overflow);
    }
```

Use this:

```rust
    #![cfg_attr(not(feature = "std"), no_std)]
    pub enum TradingPairErrors {
        Overflow,
    }

    #[ink(message)]
    pub fn get_percentage_difference(&mut self, value1: Balance, value2: Balance) -> Result<Balance, TradingPairErrors>  {
        let absolute_difference = value1.abs_diff(value2);
        let sum = value1 + value2;
        let percentage_difference =
        match 100u128.checked_mul(absolute_difference / sum) {
           Some(result) => result,
           None => Err(TradingPairErrors::Overflow),
        }
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/unused-return-enum).
