# Unprotected Set Code Hash

## Description

- Vulnerability Category: `Authorization`
- Vulnerability Severity: `Critical`
- Detectors: [`unprotected-set-code-hash`](https://github.com/CoinFabrik/scout/tree/main/detectors/set-code-hash)
- Test Cases: [`unprotected-set-code-hash-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/set-code-hash/set-code-hash-1)

Allowing users to call `set_code_hash` can be a significant vulnerability due to the following reasons:

- Unintended Modifications: `set_code_hash` allow for changes to the contract's logic or behavior after deployment. Without proper access restrictions, unauthorized users or malicious actors could upgrade functionality and modify the contract in unintended ways. This could lead to the introduction of bugs, security vulnerabilities, or undesirable changes to the contract's behavior.

- Unauthorized Upgrades: If access controls are not properly implemented, malicious users could upgrade the contract without authorization. Unauthorized upgrades can lead to the introduction of malicious code, exploitation of contract vulnerabilities, or even complete compromise of the contract, resulting in loss of funds or data.

- Dependency Risks: Upgrading a contract may introduce changes that affect other dependent contracts or systems. Without proper access restrictions, unauthorized upgrades may cause disruptions or compatibility issues with the rest of the blockchain ecosystem.

## Exploit Scenario

Consider the following `ink!` contract:

### Example


```rust
    #[ink(message)]
    pub fn update_code(&self, value: [u8; 32]) -> Result<(), Error> {
        let res = set_code_hash(&value);

        if res.is_err() {
            return res.map_err(|_| Error::InvalidCodeHash);
        }

        Ok(())
    }
``` 

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/set-code-hash/set-code-hash-1/vulnerable-example).

## Remediation

To prevent this, the function should be restricted to administrators or authorized users only.
```rust
    pub fn update_code(&self, value: [u8; 32]) -> Result<(), Error> {
        if self.admin != Self::env().caller() {
            return Err(Error::NotAnAdmin);
        }

        let res = set_code_hash(&value);

        if res.is_err() {
            return res.map_err(|_| Error::InvalidCodeHash);
        }

        Ok(())
    }
```


## References

- [Slither: Unprotected upgradeable contract](https://github.com/crytic/slither/wiki/Detector-Documentation#unprotected-upgradeable-contract)