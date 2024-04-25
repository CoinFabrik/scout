# Oveflow check

## Description 
- Vulnerability Category: `Arithmetic`
- Vulnerability Severity: `Critical`
- Detectors: [`oveflow-check`](https://github.com/CoinFabrik/scout/tree/main/detectors)
- Test Cases: [`overflow-check`](https://github.com/CoinFabrik/scout/tree/main/test-cases)

Relying on integer overflow rust’s wrapping behavior is considered an error. The program won't panic, but the result of the arithmetic operation will be incorrect.

## Exploit Scenario
Take for example the following section of a `Cargo.toml`:

```toml
[profile.release]
overflow-checks = false
```

## Remediation

Use instead:

```toml
[profile.release]
overflow-checks = true
```

The remediated code example can be found [here]().

## References

- https://doc.rust-lang.org/book/ch03-02-data-types.html

