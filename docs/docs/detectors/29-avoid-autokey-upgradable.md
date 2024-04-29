# Avoid AutoKey Upgradable

### What it does

Warns about the usage of `Lazy` storage (`Mapping`, `Lazy` and `StorageVec`) without a `ManualKey<...>` when the function `set_code_hash` is used in the contract.

### Why is this bad?

If the hash passed to `set_code_hash` corresponds to a contract that the compiler assigned `AutoKey<>` to the lazy values, the data in the old keys will be lost. This could lead not only to data loss, but to a locked contract too.

### More info

- https://use.ink/datastructures/storage-layout/#manual-vs-automatic-key-generation

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-autokey-upgradable).
