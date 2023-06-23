# Reentrancy

### What it does
This linting rule checks wether a cross-contract call to PSP22 transfer_from function with user supplied arguments.
### Why is this bad?
If the from argument is not restricted, the user could transfer tokens from any user who has approved the contract to spend PSP22 tokens.
### Known problems
In this early version of this lint we don't check the order of the pushed arguments.

### Example
```rust
let call_params = build_call::<DefaultEnvironment>()
    .exec_input(
        ExecutionInput::new(Selector::new(ink::selector_bytes!(
            "PSP22::transfer_from"
        )))
        .push_arg(from)
        .push_arg(self.env().account_id())
        .push_arg(self.amount)
        .push_arg([0u8])
    )
    .returns::<Result<(),PSP22Error>>()
    .call(self.psp22_address)
    .params();
```
Use instead:
```rust
let call_params = build_call::<DefaultEnvironment>()
    .exec_input(
        ExecutionInput::new(Selector::new(ink::selector_bytes!(
            "PSP22::transfer_from"
        )))
        .push_arg(self.env().caller())
        .push_arg(self.env().account_id())
        .push_arg(self.amount)
        .push_arg([0u8])
    )
    .returns::<Result<(),PSP22Error>>()
    .call(self.psp22_address)
    .params();
```
