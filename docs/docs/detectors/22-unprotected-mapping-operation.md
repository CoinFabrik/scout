# Unprotected Mapping Operation

### What it does

It warns you if a mapping operation (`insert`, `take`, `remove`) function is called with a user-given `key` field of the type `AccountId`.

### Why is this bad?

Modifying mappings with an arbitrary key given by users can be a significant vulnerability for several reasons:

- Unintended Modifications: Allowing users to provide arbitrary keys can lead to unintended modifications of critical data within the smart contract. If the input validation and sanitation are not done properly, users may be able to manipulate the data in ways that were not intended by the contract's author.

- Data Corruption: Malicious users could intentionally provide keys that result in the corruption or manipulation of important data stored in the mapping. This could lead to incorrect calculations, unauthorized access, or other undesirable outcomes.

- Denial-of-Service (DoS) Attacks: If users can set arbitrary keys, they may be able to create mappings with a large number of entries, potentially causing the contract to exceed its gas limit. This could lead to denial-of-service attacks, making the contract unusable for other users.

### Known problems

### Example

```rust
    #[ink(message)]
    pub fn withdraw(&mut self, amount: Balance, from: AccountId) -> Result<(), Error> {
        let current_bal = self.balances.take(from).unwrap_or(0);
        if current_bal >= amount {
            self.balances.insert(from, &(current_bal - amount));
            self.env()
                .transfer(from, current_bal)
                .map_err(|_| Error::TransferError)
        } else {
            Err(Error::BalanceNotEnough)
        }
    }
```

Use instead:

```rust
    #[ink(message)]
    pub fn withdraw(&mut self, amount: Balance) -> Result<(), Error> {
        let caller = self.env().caller();
        let current_bal = self.balances.take(caller).unwrap_or(0);
        if current_bal >= amount {
            self.balances.insert(caller, &(current_bal - amount));
            self.env()
                .transfer(caller, current_bal)
                .map_err(|_| Error::TransferError)
        } else {
            Err(Error::BalanceNotEnough)
        }
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/unprotected-mapping-operation).
