# Divide before multiply

### What it does

Checks the existence of a division before a multiplication.

### Why is this bad?

Division between two integers might return zero.

### Example

```rust
let x = 1;
let y = 2;
let z = x / y * 3;
```

Use instead:

```rust
let x = 1;
let y = 2;
let z = x * 3 / y;
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/divide-before-multiply).
