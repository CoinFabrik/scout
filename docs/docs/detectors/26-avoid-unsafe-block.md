# Avoid Unsafe Block

### What it does

It tells not to use `Unsafe rust`.

### Why is this bad?

`Unsafe Rust` does not enforce memory safety guarantees at compile time.


### Example

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

Use instead:

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

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/iterators-over-indexing).
