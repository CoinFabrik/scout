# Avoid autokey upgradable

## Description

- Vulnerability Category: `Upgradability`
- Vulnerability Severity:`Critical`
- Detectors: [`avoid-autokey-upgradable`](https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-autokey-upgradable)
- Test Cases: [`avoid-autokey-upgradable-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-autokey-upgradable/avoid-autokey-upgradable-1)

## Exploit Scenario

Consider the following contract:

```rust

    #[ink(storage)]
    pub struct AvoidAutoKeyUpgradable {
        balances: Mapping<AccountId, Balances>,
        total_supply: Lazy<Balance>,
    }

    pub enum Error {
        NotAnAdmin,
        FailedSetCodeHash,
    }

    impl Contract {
        /* --- snip --- */
        #[ink(message)]
        pub fn upgrade_contract(&self, value: [u8; 32]) -> Result<(), Error> {
            if self.admin != Self::env().caller() {
                return Err(Error::NotAnAdmin);
            }

            match self.env().set_code_hash(&value.into()) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::FailedSetCodeHash),
            }
        }
        /* --- snip --- */
    }

```

When you have a contract that has any kind of `Lazy` storage (`Lazy`, `Mapping` or `StorageVec`) and your contract is upgradable, you need to ensure that every `Lazy` storage has a manual key. If you don't do this, the compiler can assign a new key to the `Lazy` storage when you upgrade the contract, and you will lose all that data.

## Remediation

Use `ManualKey` to ensure that the `Lazy` storage has a fixed key. You can use either a literal value or an Enum variant.

```rust

    pub enum Keys {
        TotalSupply,
    }

    #[ink(storage)]
    pub struct AvoidAutoKeyUpgradable {
        balances: Mapping<AccountId, Balances, ManualKey<0xDEAD>>,
        total_supply: Lazy<Balance, ManualKey<TotalSupply>>,
    }

    pub enum Error {
        NotAnAdmin,
        FailedSetCodeHash,
    }

    impl Contract {
        /* --- snip --- */
        #[ink(message)]
        pub fn upgrade_contract(&self, value: [u8; 32]) -> Result<(), Error> {
            if self.admin != Self::env().caller() {
                return Err(Error::NotAnAdmin);
            }

            match self.env().set_code_hash(&value.into()) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::FailedSetCodeHash),
            }
        }
        /* --- snip --- */
    }


## References

- https://use.ink/datastructures/storage-layout
```
