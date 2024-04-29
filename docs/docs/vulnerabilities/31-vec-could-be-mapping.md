# Vec could be mapping

## Description

- Vulnerability Category: `Gas usage`
- Vulnerability Severity: `Enhancement`
- Detectors: [`vec-could-be-mapping`](https://github.com/CoinFabrik/scout/tree/main/vec-could-be-mapping/)
- Test Cases: [`vec-could-be-mapping-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/vec-could-be-mapping/vec-could-be-mapping-1)

When using a `Vec` to store key-value pairs, it is possible to use a `Mapping` instead. This will reduce the gas usage of the contract, as the `Vec` will have to iterate over all elements to find the desired key-value pair.

## Exploit Scenario

Consider the following ink! contract, where you have a `balances` vec of tuples of `(AccountId, Balance)`. If you want to find the `Balance` from a specific `AccountId`, you will have to iterate over all elements of the `balances` vec to find the desired `AccountId`.

```rust
    pub struct Contract {
        balances: Vec<(AccountId, Balance)>,
    }

    pub enum Error {
        NotFound,
    }

    impl Contract {
        /* --- snip --- */
        #[ink(message)]
        pub fn get_balance(&mut self, acc: AccountId) -> Result<Balance, Error> {
            self.balances
                .iter()
                .find(|(a, _)| *a == acc)
                .map(|(_, b)| *b)
                .ok_or(Error::NotFound)
        }
        /* --- snip --- */
    }

```

Using `.find(...)` over an iterator of tuples consumes more gas than using a `Mapping` to store the key-value pairs.

## Remediation

```rust
    pub struct VecCouldBeMapping {
        balances: Mapping<AccountId, Balance>,
    }

    pub enum Error {
        NotFound,
    }

    impl Contract {
        /* --- snip --- */
        #[ink(message)]
        pub fn get_balance(&mut self, acc: AccountId) -> Result<Balance, Error> {
            self.balances.get(&acc).ok_or(Error::NotFound)
        }
        /* --- snip --- */
    }

```

