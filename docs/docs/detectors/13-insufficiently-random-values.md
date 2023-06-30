# Insuficciently random values

### What it does
Checks the usage of `block_timestamp` or `block_number` for generation of random numbers.


### Why is this bad?
Using `block_timestamp` is not recommended because it could be potentially manipulated by validator. On the other hand, `block_number` is publicly available, an attacker could predict the random number to be generated.

### Example

```rust
#[ink(message, payable)]
pub fn bet_single(&mut self, number: u8) -> Result<bool> {
    let inputs = self.check_inputs(36, 0, 36, number);
    if inputs.is_err() {
        return Err(inputs.unwrap_err());
    }

    let pseudo_random: u8 = (self.env().block_number() % 37).try_into().unwrap();
    if pseudo_random == number {
        return self
            .env()
            .transfer(self.env().caller(), self.env().transferred_value() * 36)
            .map(|_| true)
            .map_err(|_e| Error::TransferFailed);
    }
    return Ok(false);
}
```

Avoid using block attributes like `block_timestamp` or `block_number` for randomness generation, and consider using oracles instead.

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/insufficiently-random-values).