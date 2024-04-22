# Vec Considerations

## Description

- Vulnerability Category: `Best practices`
- Vulnerability Severity: `Enhancement`
- Detectors: [`vec-considerations`](https://github.com/CoinFabrik/scout/tree/main/detectors/vec-considerations)
- Test Cases: [`vec-considerations-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/vec-considerations/vec-considerations-1)


## Exploit Scenario

Consider the following `ink!` contract:

```rust
    #[ink(message)]
    pub fn do_something(&mut self, data: String) {
        let caller = self.env().caller();
        let example = format!("{caller:?}: {data}");

        // Panics if data overgrows the static buffer size!
        self.on_chain_log.insert(caller, &example);
    }
```

The problem arises from the use of `.insert()` since Ink!'s static buffer defaults to 16KB in size. If data overgrows this size, the contract will panic!.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/vec-considerations/vec-considerations-1/vulnerable-example).

## Remediation

Instead, when working with dynamically sized values, use faillible storage methods. For instance:

```rust
    #[ink(message)]
        pub fn do_something2(&mut self, data: String) -> Result<(), Error> {
            let caller = self.env().caller();

            match self.on_chain_log.try_insert(caller, &data) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::InsertFailed),
            }
        }
```


The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/vec-considerations/vec-considerations-1/remediated-example).

## References
