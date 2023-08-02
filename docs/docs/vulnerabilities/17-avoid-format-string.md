# Avoid fromat! macro usage

## Description

- Vulnerability Category: `Validations and error handling`
- Vulnerability Severity: `Enhacement`
- Detectors: [`avoid-format!-string`](https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-format!-string)
- Test Cases: [`avoid-format!-string-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-format!-string/avoid-format!-string-1)

The `format!` macro is not recommended. A custom error is recommended instead.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
    #[ink(message)]
    pub fn crash(&self) -> Result<(), Error> {
        Err(Error::FormatError {
            msg: (format!("{:?}", "false")),
        })
    }
```

The problem arises from the use of the `format!` macro. This is used to format a string with the given arguments. Returning a custom error is desirable.


The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-format!-string/avoid-format!-string-1/vulnerable-example).

## Remediation

Create a custom error to avoid using the macro.

## References

- [Memory Management](https://docs.alephzero.org/aleph-zero/security-course-by-kudelski-security/ink-developers-security-guideline#be-careful-when-you-use-the-following-patterns-that-may-cause-panics.)
