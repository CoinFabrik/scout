# Avoid core::mem::forget usage

## Description

- Vulnerability Category: `Best practices`
- Vulnerability Severity: `Enhacement`
- Detectors: [`avoid-core-mem-forget`](https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-core-mem-forget)
- Test Cases: [`avoid-core-mem-forget-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-core-mem-forget/avoid-core-mem-forget-1)

The `core::mem::forget` function usage is a bad practice.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
   #[ink(message)]
   pub fn forget_value(&mut self) {
       let forgotten_value = self.value;
       self.value = false;
       core::mem::forget(forgotten_value);
   }
```

The problem arises from the use of the `core::mem::forget` function. This function is used to forget about a value without running its destructor. This is a bad practice because it can lead to memory leaks, resource leaks and logic errors.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-core-mem-forget/avoid-core-mem-forget-1/vulnerable-example).

## Remediation

Use the pattern `let _ = forgotten_value;` or the `.drop()` method instead of `core::mem::forget(forgotten_value);`.

## References

- [Memory Management](https://docs.alephzero.org/aleph-zero/security-course-by-kudelski-security/ink-developers-security-guideline#memory-management)
