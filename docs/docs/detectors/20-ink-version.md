# Ink! version

### What it does

Warns you if you are using an old version of ink!.

### Why is this bad?

Using an old version of ink! can be dangerous, as it may have bugs or security issues.

### Example

```toml
[dependencies]
    ink = { version = "=4.2.0", default-features = false }
```

Instead, use the latest available version.

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/ink-version).
