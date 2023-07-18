# Unrestricted Transfer From

### What it does

It warns you if a `transfer_from` function is called with a user-defined parameter in the `from` field.

### Why is this bad?

An user Alice can approve a contract to spend their tokens. An user Bob can call that contract, use that allowance to send themselves Alice's tokens. 

### Known problems

None.

### Example


```rust
// build_call example
    #[ink(message)]
    pub fn deposit(&mut self, from: AccountId) -> Result<(), Error> {
        let call_params = build_call::<DefaultEnvironment>()
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "PSP22::transfer_from"
                )))
                .push_arg(from)
                .push_arg(self.env().account_id())
                .push_arg(self.amount)
                .push_arg([0u8]),
            )
    }
// ContractRef example
    #[ink(message)]
    pub fn deposit(&mut self, from: AccountId) -> Result<(), Error> {
        let res = PSP22Ref::transfer_from(
            &self.psp22_address,
            from,
            self.env().account_id(),
            self.amount,
            vec![],
        );
    }
```


Use instead:

```rust
// build_call example
    pub fn deposit(&mut self) -> Result<(), Error> {
        let call_params = build_call::<DefaultEnvironment>()
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "PSP22::transfer_from"
                )))
                .push_arg(self.env().caller())
                .push_arg(self.env().account_id())
                .push_arg(self.amount)
                .push_arg([0u8]),
            )
    }

// ContractRef example
    pub fn deposit(&mut self) -> Result<(), Error> {
        let res = PSP22Ref::transfer_from(
            &self.psp22_address,
            self.env().caller(),
            self.env().account_id(),
            self.amount,
            vec![],
        );
    }

```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/unrestricted-transfer-from).