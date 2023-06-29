# Set contract storage

### What it does

Checks for calls to env::set_contract_storage.

### Why is this bad?

Functions using keys as variables without proper access control or input sanitation can allow users to perform changes in arbitrary memory locations.

### Known problems

Only check the function call, so false positives could result.

### Example

```rust
fn set_contract_storage(
    &mut self,
    user_input_key: [u8; 68],
    user_input_data: u128,
) -> Result<()> {
    env::set_contract_storage(&user_input_key, &user_input_data);
    Ok(())
}
```

Use instead:

```rust
fn set_contract_storage(
    &mut self,
    user_input_key: [u8; 68],
    user_input_data: u128,
) -> Result<()> {
    if self.env().caller() == self.owner {
        env::set_contract_storage(&user_input_key, &user_input_data);
        Ok(())
    } else {
        Err(Error::UserNotOwner)
    }
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/set-contract-storage).
