# Incorrect Exponentiation

## Description

- Vulnerability Category: `Arithmetic`
- Vulnerability Severity: `Critical`
- Detectors: [`incorrect-exponentiation`](https://github.com/CoinFabrik/scout/tree/main/detectors/incorrect-exponentiation)
- Test Cases: [`incorrect-exponentiation-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/incorrect-exponentiation/incorrect-exponentiation-1)


The operator `^` is not an exponential operator, it is a bitwise XOR. Make sure to use `pow()` instead for exponentiation. In case of performing a XOR operation, use `.bitxor()` for clarity.

## Exploit Scenario

In the following example, the `^` operand is being used for exponentiation. But in Rust, `^` is the operand for an XOR operation. If misused,
this could lead to unexpected behaviour in our contract.

```rust
    #[ink(message)]
    pub fn exp_data_by_3(&mut self) {
        self.data ^= 3
    }
```

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/incorrect-exponentiation/incorrect-exponentiation-1/vulnerable-example).

## Remediation

A possible solution is to use the method `pow()`. But, if a XOR operation is wanted, `.bitxor()` method is recommended.

```rust
    #[ink(message)]
    pub fn exp_data_by_3(&mut self) {
        self.data = self.data.pow(3)
    }
```

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/incorrect-exponentiation/incorrect-exponentiation-1/remediated-example).

## References

- https://doc.rust-lang.org/std/ops/trait.BitXor.html
