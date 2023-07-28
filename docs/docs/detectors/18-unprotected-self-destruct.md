# Unprotected self destruct

### What it does

It warns you if `terminate_contract` function is called without a previous check of the address of the caller.

### Why is this bad?

If users are allowed to call `terminate_contract`, they can intentionally or accidentally destroy the contract, leading to the loss of all associated data and functionalities given by this contract or by others that depend on it.

### Known problems

None.

### Example


```rust
    #[ink(message)]
    pub fn delete_contract(&mut self, beneficiary: AccountId) {
        self.env().terminate_contract(beneficiary)
    }
```


Use instead:

```rust
pub fn delete_contract(&mut self, beneficiary: AccountId) {
        if self.admin == self.env().caller() {
            self.env().terminate_contract(beneficiary)
        }
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/unprotected-self-destruct).