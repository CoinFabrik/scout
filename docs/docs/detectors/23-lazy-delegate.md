# Lazy storage on delegate

### What it does

Checks for non-lazy storage when using delegate calls.

### Why is this bad?

ink! has a bug that makes delegated calls not modify the storage of the caller.

#### More info

- https://github.com/paritytech/ink/issues/1825
- https://github.com/paritytech/ink/issues/1826

### Example

```rust
    #[ink(storage)]
    pub struct Contract {
       admin: AccountId,
    }
```

Use instead:

```rust
    #[ink(storage)]
    pub struct Contract {
        admin: Lazy<AccountId, ManualKey<12345>>,
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/lazy-delegate).
