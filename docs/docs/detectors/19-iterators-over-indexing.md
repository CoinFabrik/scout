# Iterators over indexing

### What it does

It warns if for loop uses indexing instead of iterator. If the indexing goes to `.len()` it will not warn.

### Why is this bad?

Accessing a vector by index is slower than using an iterator. Also, if the index is out of bounds, it will panic.

### Example

```rust
    #[ink(message)]
    pub fn bad_indexing(&self){
        for i in 0..3 {
            foo(self.value[i]);
        }
    }
```

Use instead:

```rust
   #[ink(message)]
   pub fn iterator(&self) {
       for item in self.value.iter() {
            foo(self.value[i]);
       }
   }

// or if its not iterable (with `in`, `iter` or `to_iter()`)

   #[ink(message)]
   pub fn index_to_len(&self){
       for i in 0..self.value.len() {
            foo(self.value[i]);
       }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/iterators-over-indexing).
