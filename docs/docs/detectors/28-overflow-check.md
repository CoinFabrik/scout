# Overflow Check

### What it does

Checks that `overflow-checks` is enabled in the `[profile.release]` section of the `Cargo.toml`.

### Why is this bad?

Integer overflow will trigger a panic in debug builds or will wrap in
release mode. Division by zero will cause a panic in either mode. In some applications one
wants explicitly checked, wrapping or saturating arithmetic.

#### More info

Consider the following example.

```toml
[package]
name = "overflow-check-vulnerable-1"
version = "0.1.0"
edition = "2021"
authors = ["[your_name] <[your_email]>"]

[lib]
path = "src/lib.rs"

[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
]
ink-as-dependency = []
e2e-tests = []


[dependencies]
ink = { workspace = true }
scale = { workspace = true }
scale-info = { workspace = true }


[dev-dependencies]
ink_e2e = { workspace = true }

[profile.release]
overflow-checks = false
```
Use instead:

[package]
name = "overflow-check-vulnerable-1"
version = "0.1.0"
edition = "2021"
authors = ["[your_name] <[your_email]>"]

[lib]
path = "src/lib.rs"

[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
]
ink-as-dependency = []
e2e-tests = []


[dependencies]
ink = { workspace = true }
scale = { workspace = true }
scale-info = { workspace = true }


[dev-dependencies]
ink_e2e = { workspace = true }

[profile.release]
overflow-checks = true
```
### Example

### Implementation

TODO: Add link!
The detector's implementation can be found at [this link].
