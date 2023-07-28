# Unprotected set code hash

### What it does

It warns you if `set_code_hash` function is called without a previous check of the address of the caller.

### Why is this bad?

If users are allowed to call `terminate_contract`, they can intentionally modify the contract behaviour, leading to the loss of all associated data/tokens and functionalities given by this contract or by others that depend on it.

### Known problems

None.

### Example


```rust
    #[ink(message)]
    pub fn update_code(&self, value: [u8; 32]) -> Result<(), Error> {
        let res = set_code_hash(&value);

        if res.is_err() {
            return res.map_err(|_| Error::InvalidCodeHash);
        }

        Ok(())
    }
``` 

Use instead:

```rust
    pub fn update_code(&self, value: [u8; 32]) -> Result<(), Error> {
        if self.admin != Self::env().caller() {
            return Err(Error::NotAnAdmin);
        }

        let res = set_code_hash(&value);

        if res.is_err() {
            return res.map_err(|_| Error::InvalidCodeHash);
        }

        Ok(())
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/set-code-hash)