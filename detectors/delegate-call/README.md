# Delegate Call

### What it does
Checks for delegated calls to contracts passed as arguments.

### Why is this bad?
Delegated calls to contracts passed as arguments can be used to change the expected behavior of the contract. If you need to change the target of a delegated call, you should use a storage variable, and make a function with proper access control to change it.

### Known problems


### Example

```rust
    pub fn delegateCall(&mut self, target: Hash, argument: Balance) {
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
    pub fn delegateCall(&mut self, argument: Balance) {
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

    pub fn set_target(&mut self, new_target: Hash) {
        assert_eq!(self.admin, self.env().caller(), "Only admin can set target");
        self.target = new_target;
    }
```