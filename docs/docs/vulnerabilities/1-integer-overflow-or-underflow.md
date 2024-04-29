# Integer overflow or underflow
## Description
- Vulnerability Category: `Arithmetic`
- Severity: `Critical`
- Detectors: [`integer-overflow-or-underflow`](https://github.com/CoinFabrik/scout/tree/main/detectors/integer-overflow-or-underflow)
- Test Cases: [`integer-overflow-or-underflow-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-1)

This type of vulnerability occurs when an arithmetic operation attempts to 
create a numeric value that is outside the valid range in substrate, e.g, 
an `u8` unsigned integer can be at most *M:=2^8-1=255*, hence the sum *M+1*
produces an overflow. 

## Exploit Scenario
There follows a snippet of a simple `ink!` smart contract that is vulnerable to
an integer overflow vulnerability.

```rust
#[ink(message)]
pub fn add(&mut self, value: u8) {
    self.value += value;
}

#[ink(message)]
pub fn sub(&mut self, value: u8) {
    self.value -= value;
}
```

The above contract stores a single value of type `u8` and provides three 
functions allowing interaction with the single value. 
The `add()` function allows users to add a specified amount to the stored value,
the `sub()` function allows users to subtract a specified amount, while the 
`get()` function allows users to retrieve the current value.

This contract is vulnerable to an integer overflow attack that may be exercised
if a user adds a value that exceeds the maximum value that can be stored in an 
`u8` variable, then the addition operation overflows the variable and the value
wraps to zero (ignoring the carry), potentially leading to unexpected behavior.

The vulnerable code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-1/vulnerable-example/src/lib.rs).

### Deployment
Before deployment, the contract must be built using the tool `cargo-contract`:

```shell
cargo contract build --release
```

Following that, the contract can be deployed either by using `cargo-contract`
or a GUI tool (e.g., [https://contracts-ui.substrate.io/](https://contracts-ui.substrate.io/)):

```shell
cargo contract instantiate --constructor new --args 0 --suri //Alice
```

## Remediation

<!-- Even though enabling the overflow/underflow checks in the `Cargo.toml` would eliminate the possibility of the
vulnerability being realized, a panic error would still be raised.
```toml
[profile.release]
overflow-checks = true
```

All in all, considering that this check might be disabled and that raising a panic error is not the best way to handle this issue,  -->
It is recommended that the code be changed to explicitly use checked, overflowing, or saturating arithmetic. For example:

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// An overflow was produced while adding
    OverflowError,
    /// An underflow was produced while substracting
    UnderflowError,
}
```

The problematic functions can be updated as follows:

```rust
#[ink(message)]
pub fn add(&mut self, value: u8) -> Result<(), Error> {
    match self.value.checked_add(value) {
        Some(v) => self.value = v,
        None => return Err(Error::OverflowError),
    };
    Ok(())
}

#[ink(message)]
pub fn sub(&mut self, value: u8) -> Result<(), Error> {
    match self.value.checked_sub(value) {
        Some(v) => self.value = v,
        None => return Err(Error::UnderflowError),
    };
    Ok(())
}
```

The remediated code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-1/remediated-example/src/lib.rs).

Other rules could be added to improve the checking. The set of rules can be found [here](https://rust-lang.github.io/rust-clippy/master/).


## References
- [SWC-101](https://swcregistry.io/docs/SWC-101)
- [Ethernaut: Token](https://github.com/OpenZeppelin/ethernaut/blob/master/contracts/src/levels/Token.sol)
- [20 cases of overflow/underflow](https://github.com/ethereum/solidity/issues/796#issuecomment-253578925)
- https://blog.sigmaprime.io/solidity-security.html#ouflow