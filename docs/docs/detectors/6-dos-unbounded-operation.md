# DoS unbounded operation

### What it does

This detector checks that when using for or while loops, their conditions limit the execution to a constant number of iterations.

### Why is this bad?

If the number of iterations is not limited to a specific range, it could potentially cause out of gas exceptions.

### Known problems

False positives are to be expected when using variables that can only be set using controlled flows that limit the values within acceptable ranges.

### Example

```rust
pub fn pay_out(&mut self) {
    for i in 0..self.next_payee_ix {
        let payee = self.payees.get(&i).unwrap();
        self.env().transfer(payee.address, payee.value).unwrap();
    }
}
```

Use instead:

```rust
pub fn pay_out(&mut self, payee: u128) {
    let payee = self.payees.get(&payee).unwrap();
    self.env().transfer(payee.address, payee.value).unwrap();
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/dos-unbounded-operation).
