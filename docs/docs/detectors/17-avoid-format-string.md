# Avoid fromat! macro usage

### What it does

Checks for `format!` macro usage.

### Why is this bad?

The usage of format! is not recommended.

### Example

```rust
    #[ink(message)]
    pub fn crash(&self) -> Result<(), Error> {
        Err(Error::FormatError {
            msg: (format!("{}", self.value)),
        })
    }
```

Use instead:

```rust
    pub enum Error {
        FormatError { msg: String },
        CrashError
    }

    #[ink(message)]
    pub fn crash(&self) -> Result<(), Error> {
        Err(Error::FormatError { msg: self.value.to_string() })
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-format!-string).
