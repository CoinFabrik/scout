# Assert violation

### What it does

Checks for `assert!` macro usage.

### Why is this bad?

The `assert!` macro can cause the contract to panic.

### Example

```rust
    #[ink(message)]
    pub fn assert_if_greater_than_10(&self, value: u128) -> bool {
        assert!(value <= 10, "value should be less than 10");
        true
    }
```

Use instead:

```rust
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        GreaterThan10,
    }

    #[ink(message)]
    pub fn revert_if_greater_than_10(&self, value: u128) -> Result<bool, Error> {
        if value <= 10 {
            Ok(true)
        } else {
            Err(Error::GreaterThan10)
        }
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/assert-violation).
