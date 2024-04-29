# Non payable transferred value

## Description

- Vulnerability Category: `Best practices`
- Vulnerability Severity: `Enhancement`
- Detectors: [`non-payable-transferred-value`](https://github.com/CoinFabrik/scout/tree/main/detectors/non-payable-transferred-value)
- Test Cases: [`non-payable-transferred-value-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/non-payable-transferred-value/non-payable-transferred-value-1)

## Exploit Scenario

Consider the following function.

```rust
    #[ink(message)]
    pub fn something(&self) -> bool {
        self.env().transferred_value() > 0
    }
```

This function is not payable as it does not have the `#[ink(payable)]` attribute, but it checks for `self.env().transferred_value()` and it will always evaluate to `0` if the function is not payable.

## Remediation

Make the function `payable` if you want to check the transferred value.

```rust
    #[ink(message, payable)]
    pub fn something(&self) -> bool {
        self.env().transferred_value() > 0
    }
```
