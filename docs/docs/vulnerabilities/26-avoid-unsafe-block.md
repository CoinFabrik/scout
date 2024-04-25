# Avoid Unsafe Block
## Description 
- Vulnerability Category: `Best practices`
- Vulnerability Severity: `Enhancement`
- Detectors: [`avoid-unsafe-block`](https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-unsafe-block)
- Test Cases: [`avoid-unsafe-block-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-unsafe-block/avoid-unsafe-block-1)


The use of `unsafe` blocks in Rust is discouraged because they bypass Rust's memory safety checks, potentially leading to issues like undefined behavior and security vulnerabilities.

## Exploit Scenario

Rust enforces memory safety at compile time. When using an `unsafe` block in Rust, it's up to the programmer to take this security measure into acount. However, this could lead to memory issues. For instance, disregarding the borrow checker, or dereferencing a null pointer. 

```rust
    #[ink(message)]
    pub fn unsafe_function(&mut self, n: u64) -> u64 {
        unsafe {
            let mut i = n as f64;
            let mut y = i.to_bits();
            y = 0x5fe6ec85e7de30da - (y >> 1);
            i = f64::from_bits(y);
            i *= 1.5 - 0.5 * n as f64 * i * i;
            i *= 1.5 - 0.5 * n as f64 * i * i;

            let result_ptr: *mut f64 = &mut i;

            (*result_ptr).to_bits()
        }
    }
```


## Remediation

To enforce memory safety, it's recommended not to use `unsafe`. 

```rust
    #[ink(message)]
    pub fn safe_function(&mut self, n: u64) -> u64 {
        let mut i = n as f64;
        let mut y = i.to_bits();
        y = 0x5fe6ec85e7de30da - (y >> 1);
        i = f64::from_bits(y);
        i *= 1.5 - 0.5 * n as f64 * i * i;
        i *= 1.5 - 0.5 * n as f64 * i * i;

        let result = &mut i;
        result.to_bits()    
    }
```

The remediated code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/avoid-unsafe-block/avoid-unsafe-block-1/remediated-example/src/lib.rs).

## References

- https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html