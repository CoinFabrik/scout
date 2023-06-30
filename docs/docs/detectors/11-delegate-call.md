# Delegate call

### What it does
Checks for delegated calls to contracts passed as arguments.

### Why is this bad?
Delegated calls to contracts passed as arguments can be used to change the expected behavior of the contract. If you need to change the target of a delegated call, you should use a storage variable, and make a function with proper access control to change it.

### Example

```rust
    #[ink(message)]
    pub fn delegate_call(&mut self, target: Hash, argument: Balance) {
        let selector_bytes = [0x0, 0x0, 0x0, 0x0];
        let result: T  = build_call::<DefaultEnvironment>()
            .delegate(target)
            .exec_input(
                ExecutionInput::new(Selector::new(selector_bytes))
                    .push_arg(argument)
            )
            .returns::<T>()
            .invoke();
    }
```


Use instead:
```rust
    #[ink(message)]
    pub fn delegate_call(&mut self, argument: Balance) {
        let selector_bytes = [0x0, 0x0, 0x0, 0x0];
        let result: T  = build_call::<DefaultEnvironment>()
            .delegate(self.target)
            .exec_input(
                ExecutionInput::new(Selector::new(selector_bytes))
                    .push_arg(argument)
            )
            .returns::<T>()
            .invoke();
    }

    #[ink::message]
    pub fn set_target(&mut self, new_target: Hash) -> Result<(), Error> {
        if self.admin != self.env().caller() {
            Err(Error::Unauthorized)
        } else {
            self.target = new_target;
            Ok(())
        }
    }

```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/delegate-call).