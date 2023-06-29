# Unsafe unwrap

### What it does

Checks for usage of `.unwrap()`

### Why is this bad?

`.unwrap()` might panic if the result value is an error or `None`.

### Example

```rust
// example code where a warning is issued
fn main() {
    let result = result_fn().unwrap("error");
}
fn result_fn() -> Result<u8, Error> {
    Err(Error::new(ErrorKind::Other, "error"))
}
```

Use instead:

```rust
// example code that does not raise a warning
fn main() {
   let result = if let Ok(result) = result_fn() {
      result
  }
}
fn result_fn() -> Result<u8, Error> {
    Err(Error::new(ErrorKind::Other, "error"))
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/unsafe-unwrap).
