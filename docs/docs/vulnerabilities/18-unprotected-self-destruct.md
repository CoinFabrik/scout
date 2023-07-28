# Unprotected Self Destruct

## Description

- Vulnerability Category: `Authorization`
- Vulnerability Severity: `Critical`
- Detectors: [`unprotected-self-destruct`](https://github.com/CoinFabrik/scout/tree/main/detectors/unprotected-self-destruct)
- Test Cases: [`unprotected-self-destruct-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unprotected-self-destruct/unprotected-self-destruct-1)

Allowing users to call `terminate_contract` can be a significant vulnerability due to the following reasons:

- Permanent Deletion of Contract: The `terminate_contract` function in a smart contract is intended to allow the contract itself to be destroyed and remove it permanently from the blockchain. If users are allowed to call this function, they can intentionally or accidentally destroy the contract, leading to the loss of all associated data and functionalities.

- Loss of Funds: If the contract holds any funds or tokens, invoking `terminate_contract` would transfer the contract's remaining balance to the specified target address. If users can call this function, they may attempt to drain the contract's funds, leading to a loss of funds for the contract owner or other users interacting with the contract.

- Contract Dependency Issues: If other contracts or systems depend on the functionality provided by the contract being self-destructed, those dependent contracts or systems may become dysfunctional or throw errors, potentially causing further disruptions in the blockchain ecosystem.

## Exploit Scenario

Consider the following `ink!` contract:

### Example


```rust
    #[ink(message)]
    pub fn delete_contract(&mut self, beneficiary: AccountId) {
        self.env().terminate_contract(beneficiary)
    }
``` 

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unprotected-self-destruct/unprotected-self-destruct-1/vulnerable-example).

## Remediation

To prevent this, the function should be restricted to administrators or authorized users only.
```rust
pub fn delete_contract(&mut self, beneficiary: AccountId) {
        if self.admin == self.env().caller() {
            self.env().terminate_contract(beneficiary)
        }
    }
```


## References

- [Aleph Zero ink! Developer security guidelines](https://docs.alephzero.org/aleph-zero/security-course-by-kudelski-security/ink-developers-security-guideline#unprotected-self-destruction-or-burning-instruction-s)
- [Slither: Suicidal](https://github.com/crytic/slither/wiki/Detector-Documentation#suicidal)