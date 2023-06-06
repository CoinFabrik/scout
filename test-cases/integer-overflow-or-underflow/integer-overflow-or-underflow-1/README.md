# Integer overflow and integer underflow
## Description
- Vulnerability Category: `Arithmetic`
- Severity: `Critical`
- Detector ID: `integer-overflow-or-underflow`

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

This vulnerability is **only** present if overflow and underflow checks are 
disabled at the time of compilation. We can disable it by adding to the 
`Cargo.toml` file the following configuration:

```toml
[profile.release]
overflow-checks = false
```

This way, the overflow checks will be disabled whenever the contract is built 
using the `release` profile. More info can be found 
[here](https://doc.rust-lang.org/cargo/reference/profiles.html).

To deploy this smart contract, you would need to compile it using the `ink!`
compiler and deploy it to a Polkadot Substrate network using a suitable 
deployment tool such as Polkadot JS. Once deployed, users could interact with
the contract by calling its functions using a compatible wallet or blockchain
explorer.

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
Of course, enabling the overflow/underflow checks would eliminate the 
vulnerability. 
```toml
[profile.release]
overflow-checks = true
```

But sometimes this is not possible. Thence, code should then be changed to 
explicitly use checked, overflowing or saturating arithmetics, e.g.:

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

And the problematic functions can be changed to:

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

Other rules could be added to improve the checking. The set of rules can be found [here](https://rust-lang.github.io/rust-clippy/master/).
