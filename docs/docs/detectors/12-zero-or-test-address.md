# Zero or test address

### What it does
Checks whether the zero address is being inputed to a function without validation.

### Why is this bad?
Because if the zero address is assigned ownership of a contract, the control of the contract can be lost and not recovered.

### Example

```rust
#[ink(message)]
pub fn modify_admin(&mut self, admin: AccountId) -> Result<AccountId, Error> {
    if self.admin != self.env().caller() {
        return Err(Error::NotAuthorized);
    }

    self.admin = admin;
    Ok(self.admin)
}
```


Use instead:
```rust
#[ink(message)]
pub fn modify_admin(&mut self, admin: AccountId) -> Result<AccountId, Error> {
    if self.admin != self.env().caller() {
        return Err(Error::NotAuthorized);
    }

    if admin == AccountId::from([0x0; 32]) {
        return Err(Error::InvalidAddress);
    }

    self.admin = admin;
    Ok(self.admin)
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/zero-or-test-address).