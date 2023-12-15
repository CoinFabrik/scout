# DoS unexpected revert with vector

### What it does

Checks for array pushes without access control.

### Why is this bad?

Arrays have a maximum size according to the storage cell. If the array is full, the push will revert. This can be used to prevent the execution of a function.

### Known problems

If the owner validation is performed in an auxiliary function, the warning will be shown, resulting in a false positive.

### Example

```rust
if self.votes.contains(candidate) {
    Err(Errors::CandidateAlreadyAdded)
} else {
    self.candidates.push(candidate); // Where candidates: Vec<AccountId>
    self.votes.insert(candidate, &0);
    Ok(())
}
```

Use instead:

```rust
if self.votes.contains(candidate) {
    Err(Errors::CandidateAlreadyAdded)
} else {
    self.candidates.insert(self.total_candidates, &candidate); // Where candidates: Mapping<u64, AccountId>
    self.total_candidates += 1;
    self.votes.insert(candidate, &0);
    Ok(())
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/dos-unexpected-revert-with-vector).
