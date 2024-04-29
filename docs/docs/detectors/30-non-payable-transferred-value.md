# Non payable transferred value

### What it does

Warns about the usage of `self.env().transferred_value()` in non-`payable` functions.

### Why is this bad?

`self.env().transferred_value()` will always return `0` in non-`payable` functions. If `transferred_value()` is needed, the function should have `#[ink(..., payable)]` 

#### More info

- https://docs.rs/ink/latest/ink/struct.EnvAccess.html#method.transferred_value


### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/non-payable-transferred-value).
