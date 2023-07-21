# Assert violation

## Description

- Vulnerability Category: `Validations and error handling`
- Vulnerability Severity: `Enhacement`
- Detectors: [`assert-violation`](https://github.com/CoinFabrik/scout/tree/main/detectors/assert-violation)
- Test Cases: [`assert-violation-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/assert-violation/assert-violation-1)

The `assert!` macro can cause the contract to panic. This is not a good practice.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
    #[ink(message)]
    pub fn assert_if_greater_than_10(&self, value: u128) -> bool {
        assert!(value <= 10, "value should be less than 10");
        true
    }
```

The problem arises from the use of the `assert!` macro, if the condition is not met, the contract panics.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/assert-violation/assert-violation-1/vulnerable-example).

## Remediation

Avoid the use of `assert!` macro. Instead, use a proper error and return it.

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/assert-violation/assert-violation-1/remediated-example).

## References

- [Assert violation](https://docs.alephzero.org/aleph-zero/security-course-by-kudelski-security/ink-developers-security-guideline#assert-violation)
