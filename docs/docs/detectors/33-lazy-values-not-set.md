# Lazy values get and not set

### What it does

Check that if `get` is used on a `Lazy<T>` or `Mapping<K,V>` variable, `set` or `insert` is subsequently used to change its value.

### Why is this bad?

The `get` function of a `Mapping` or a `Lazy` returns a local copy of the value, so changes made to that variable aren't automatically saved in the storage. To save those changes, you must call the `insert` function in the case of a `Mapping`, or `set` in the case of a `Lazy`.

As indicated [here](https://use.ink/datastructures/mapping#updating-values), it's a common pitfall that we decided to warn about, even though there are cases like getter functions where using "get" is necessary and not modifying the value. For cases like this, you can ignore the lint by adding #[allow(lazy_values_not_set)] immediately before the function definition.

#### More info

- https://use.ink/datastructures/mapping#updating-values

### Example

```rust
    #[ink(message, payable)]
    pub fn transfer(&mut self) {
        let caller = self.env().caller();
        let mut balance = self.balances.get(caller).unwrap_or(0);
        let endowment = self.env().transferred_value();
        balance += endowment;
    }
```

Use instead:

```rust
    #[ink(message, payable)]
    pub fn transfer(&mut self) {
        let caller = self.env().caller();
        let mut balance = self.balances.get(caller).unwrap_or(0);
        let endowment = self.env().transferred_value();
        balance += endowment;
        self.balances.insert(caller, &balance);
    }
```

If you want to ignore the lint in getter functions do:

```rust
    #[ink(message)]
    #[allow(lazy_values_not_set)]
    pub fn get_balance(&self) -> Option<Balance> {
        let caller = self.env().caller();
        self.balances.get(caller)
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/lazy-values-not-set).
