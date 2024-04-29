# Buffering Unsized Types

### What it does

It checks for the correct use of fallible methods when reading/writing data into `StorageVec` type variables.

### Why is this bad?
 
`StorageVec` is a Lazy type. Hence the static buffer to store the encoded data is of limited size. Because of that, reading/writing methods can fail and trap the contract.

### Example

```rust
    #[ink(message)]
    pub fn do_something(&mut self, data: String) {
        let caller = self.env().caller();

        let example = format!("{caller:?}: {data}");

        // Panics if data overgrows the static buffer size!
        self.on_chain_log.insert(caller, &example);
    }
```

Use instead:

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

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/blob/main/detectors/buffering-unsized-types).
