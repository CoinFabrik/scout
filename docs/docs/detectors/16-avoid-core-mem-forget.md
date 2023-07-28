# Avoid core::mem::forget usage

### What it does

Checks for `core::mem::forget` usage.

### Why is this bad?

This is a bad practice because it can lead to memory leaks, resource leaks and logic errors.

### Example

```rust
   #[ink(message)]
   pub fn forget_value(&mut self) {
       let forgotten_value = self.value;
       self.value = false;
       core::mem::forget(forgotten_value);
   }
```

Use instead:

```rust
   #[ink(message)]
   pub fn forget_value(&mut self) {
       let forgotten_value = self.value;
       self.value = false;
       let _ = forgotten_value;
   }

// or if droppable

    #[ink(message)]
    pub fn drop_value(&mut self) {
        let forgotten_value = self.value;
        self.value = false;
        forget_value.drop();
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-core-mem-forget).
