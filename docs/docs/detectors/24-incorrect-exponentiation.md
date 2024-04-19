# Incorrect Exponentiation

### What it does

Warns about `^` being a `bit XOR` operation instead of an exponentiation.  

### Why is this bad?

It can introduce unexpected behaviour in the smart contract.

#### More info

- https://doc.rust-lang.org/std/ops/trait.BitXor.html#tymethod.bitxor

### Example

```rust
    #[ink(message)]
    pub fn exp_data_by_3(&mut self) {
        self.data ^= 3
    }

Use instead:

```rust
    #[ink(message)]
    pub fn exp_data_by_3(&mut self) {
        self.data = self.data.pow(3)
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/incorrect-exponentiation).
