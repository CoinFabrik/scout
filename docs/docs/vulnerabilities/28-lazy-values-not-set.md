# Lazy values not set

## Description

- Vulnerability Category: `Best Practices`
- Vulnerability Severity: `Critical`
- Detectors: [`lazy-values-not-set`](https://github.com/CoinFabrik/scout/tree/main/lazy-values-not-set/)
- Test Cases: [`lazy-values-not-set-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/lazy-values-not-set/lazy-values-not-set-1)

## Exploit Scenario

Consider the following contract:

```rust
    #[ink(storage)]
    pub struct Contract {
        mapping: Mapping<AccountId, u64>,
    }
    impl Contract {
        /* --- snip --- */
        #[ink(message)]
        pub fn sum(&mut self, value: u64) -> Result<(), Error> {
            let key = self.env().caller();
            let mut _val = self.mapping.get(key).unwrap_or_default();
            _val += value;
            Ok(())
        }
        /* --- snip --- */
    }
```

In this case, when you `.get(...)` a value from a `Lazy` storage field, you _probably_ want to mutate it. The values are not automatically flushed to storage, so you need to `.set(...)` it.

## Remediation

Use the `.set(...)` or `.insert(...)` method after using `.get(...)` to flush the new value to storage.

```rust
    #[ink(storage)]
    pub struct Contract {
        mapping: Mapping<AccountId, u64>,
    }
    impl Contract {
        /* --- snip --- */
        #[ink(message)]
        pub fn sum(&mut self, value: u64) -> Result<(), Error> {
            let key = self.env().caller();
            let mut _val = self.mapping.get(key).unwrap_or_default();
            _val += value;
            self.mapping.insert(key, value);
            Ok(())
        }
        /* --- snip --- */
    }
```

## Known Issues

If you have a `.get(...)` function that you don't mutate (e.g., used as a const value), this detector triggers, if you want to ignore the lint you could add #[allow(lazy_values_not_set)] immediately before the function definition.
