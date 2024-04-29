# Vec could be mapping

### What it does

Warns about the usage of the `.find()` method in vector of tuples of the storage. This can be replaced by a `Mapping`

### Why is this bad?

Iterating over a vector is more expensive if you are trying to find something in a vector than if you are using a `Mapping`.

#### More info

- https://docs.rs/ink/latest/ink/storage/struct.Mapping.html

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/vec-could-be-mapping).
