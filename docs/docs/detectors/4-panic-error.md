# Panic error

### What it does

The panic! macro is used to stop execution when a condition is not met.
This is useful for testing and prototyping, but should be avoided in production code

### Why is this bad?

The usage of panic! is not recommended because it will stop the execution of the caller contract.

### Known problems

While this linter detects explicit calls to panic!, there are some ways to raise a panic such as unwrap() or expect().

### Example

```rust
// example code where a warning is issued
pub fn add(&mut self, value: u32)   {
   match self.value.checked_add(value) {
       Some(v) => self.value = v,
       None => panic!("Overflow error"),
   };
}
```

// example code that does not raise a warning

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
  /// An overflow was produced while adding
  OverflowError,
}

pub fn add(&mut self, value: u32) -> Result<(), Error>  {
    match self.value.checked_add(value) {
        Some(v) => self.value = v,
        None => return Err(Error::OverflowError),
    };
    Ok(())
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/panic-error).
