# Iterators over indexing

## Description

- Vulnerability Category: `Best practices`
- Vulnerability Severity: `Enhacement`
- Detectors: [`iterators-over-indexing`](https://github.com/CoinFabrik/scout/tree/main/detectors/iterators-over-indexing)
- Test Cases: [`iterators-over-indexing-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/iterators-over-indexing/iterators-over-indexing-1)

Iterating with hardcoded indexes is slower than using an iterator. Also, if the index is out of bounds, it will panic.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
    #[ink(message)]
    pub fn bad_indexing(&self){
        for i in 0..3 {
            foo(self.value[i]);
        }
    }
```

The problem arises from the use of hardcoded indexes. If `self.value` has less than 4 elements, the contract will panic.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/iterators-over-indexing/iterators-over-indexing-1/vulnerable-example).

## Remediation

Avoid the use of hardcoded indexes. Instead, use `iter()`, `to_iter()`, `for ... in ...` or range over `0..value.len()`

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/iterators-over-indexing/iterators-over-indexing-1/remediated-example).

## References

- [Memory management](https://docs.alephzero.org/aleph-zero/security-course-by-kudelski-security/ink-developers-security-guideline#memory-management)
