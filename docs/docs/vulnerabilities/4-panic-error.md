# Panic Error
## Description
- Detector ID: `panic-error`
- Vulnerability Category: `Validations and error handling`
- Severity: `Enhancement`
- Detectors: [`panic-error`](https://github.com/CoinFabrik/scout/tree/main/detectors/panic-error)
- Test Cases: [`panic-error-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/panic-error/panic-error-1)

This detector checks for the use of the `panic!` macro in the code. The 
`panic!` macro is used to stop execution when a condition is not met. 
This is useful for testing and prototyping, but should be avoided in 
production code.

Using `Result` as return type for functions that can fail is the idiomatic
way to handle errors in Rust. The `Result` type is an enum that can be either
`Ok` or `Err`. The `Err` variant can contain an error message. The `?` 
operator can be used to propagate the error message to the caller.

This way, the caller can decide how to handle the error, although the state of
the contract is always reverted on the callee.

## Exploit Scenario
In the following example, the `panic!` command is being used to handle errors,
disallowing the caller to handle the error in a different way, and completely 
stopping execution of the caller contract.

```rust
#[ink(message)]
pub fn add(&mut self, value: u32)   {
    match self.value.checked_add(value) {
        Some(v) => self.value = v,
        None => panic!("Overflow error"),
    };
}
```

The `add` function takes a value as an argument and adds it to the value stored
in the contract's storage. The function first checks if the addition will cause
an overflow. If the addition will cause an overflow, the function will panic. 
If the addition will not cause an overflow, the function will add the value to 
the contract's storage.

The usage of `panic!` in this example, is not recommended because it will stop
the execution of the caller contract. If the method was called by the user, 
then he will receive `ContractTrapped` as the only error message.

The vulnerable code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/panic-error/panic-error-1/vulnerable-example/lib.rs).

## Remediation
A possible remediation goes as follows:

```rust
#[ink(message)]
pub fn add(&mut self, value: u32) -> Result<(), Error>  {
    match self.value.checked_add(value) {
        Some(v) => self.value = v,
        None => return Err(Error::OverflowError),
    };
    Ok(())
}
```

And adding the following `Error` enum:

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// An overflow was produced while adding
    OverflowError,
}
```

By first defining the `Error` enum and then returning a `Result<(), Error>`, 
more information is added to the caller and, e.g. the caller contract could 
decide to revert the transaction or to continue execution.

The remediated code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/panic-error/panic-error-1/remediated-example/lib.rs).

## References

- https://substrate.stackexchange.com/questions/2391/panic-in-ink-smart-contracts
- https://github.com/paritytech/ink/issues/641
