# Unprotected mapping operation

## Description

- Vulnerability Category: `Authorization`
- Vulnerability Severity: `Critical`
- Detectors: [`unprotected-mapping-operation`](https://github.com/CoinFabrik/scout/tree/main/detectors/unprotected-mapping-operation)
- Test Cases: [`unprotected-mapping-operation-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unprotected-mapping-operation/unprotected-mapping-operation1)

Modifying mappings with an arbitrary key given by users can be a significant vulnerability for several reasons:

- Unintended Modifications: Allowing users to provide arbitrary keys can lead to unintended modifications of critical data within the smart contract. If the input validation and sanitation are not done properly, users may be able to manipulate the data in ways that were not intended by the contract's author.

- Data Corruption: Malicious users could intentionally provide keys that result in the corruption or manipulation of important data stored in the mapping. This could lead to incorrect calculations, unauthorized access, or other undesirable outcomes.

- Denial-of-Service (DoS) Attacks: If users can set arbitrary keys, they may be able to create mappings with a large number of entries, potentially causing the contract to exceed its gas limit. This could lead to denial-of-service attacks, making the contract unusable for other users.
 
## Exploit Scenario

Consider the following `ink!` contract:

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

The vulnerability in this `withdraw` function arises from the use of `from`, an user-defined parameter used as key in the mapping without prior sanitizing. Alice can withdraw tokens from any user to the user balance. 
 

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unprotected-mapping-operation/unprotected-mapping-operation1/vulnerable-example).
## Remediation

Avoid using user-given arguments as `key` parameter in mapping. Instead, use `self.env().caller()` or sanitize the values.

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

## References

- [Aleph Zero ink! Developer security guidelines](https://docs.alephzero.org/aleph-zero/security-course-by-kudelski-security/ink-developers-security-guideline#unprotected-self-destruction-or-burning-instruction-s)