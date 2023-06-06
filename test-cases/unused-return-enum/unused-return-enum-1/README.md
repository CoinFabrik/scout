# Unused return enum
## Description
- Vulnerability Category: `Validations and error handling`
- Vulnerability Severity: `Minor`
- Detector ID: `unused-return-enum`

Ink messages can return a `Result` enum with a custom error type. This is 
useful for the caller to know what went wrong when the message fails. The
definition in Rust of the `Result` enum is:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

If any of the variants is not used, the code could be simplified or it could 
imply a bug.

## Exploit Scenario
In order to perform this exploit we work through the following example:

```rust
#[ink(message)]
pub fn get_percentage_difference(
    &mut self,
    value1: Balance,
    value2: Balance
) -> Result<Balance, TradingPairErrors>  {
    let absolute_difference = value1.abs_diff(value2);
    let sum = value1 + value2;
    let percentage_difference =
        match 100u128.checked_mul(absolute_difference / sum) {
            Some(result) => result,
            None => panic!("overflow!"),
    };
    return Err(TradingPairErrors::Overflow);
}
```

This is an `ink!` message that returns the percentage difference between two values.

The function then returns an error enum variant `TradingPairErrors::Overflow`.
However, the function never returns a `Result` enum variant `Ok`, thus always 
failing.

The following code can be found [here](vulnerable-example/lib.rs)

## Remediation
This function could be easily fixed by returning a `Result` enum variant `Ok`
when the percentage difference is calculated successfully. By providing a check in 
the linter that ensures that all the variants of the `Result` enum are used, this 
bug could have been avoided.

First we define the `Error` enum:

````rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum TradingPairErrors {
    Overflow,
}
````

Then we change the function to:

```rust
#[ink(message)]
pub fn get_percentage_difference(
    &mut self,
    value1: Balance,
    value2: Balance
) -> Result<Balance, TradingPairErrors>  {
    let absolute_difference = value1.abs_diff(value2);
    let sum = value1 + value2;
    match 100u128.checked_mul(absolute_difference / sum) {
        Some(result) => Ok(result),
        None => Err(TradingPairErrors::Overflow)
    }
}
```

The full code can be found [here](remediated-example/lib.rs).


## References
- https://github.com/RottenKiwi/Panorama-Swap-INK-SC/blob/main/trading_pair_azero/lib.rs
