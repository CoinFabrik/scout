# Zero or test address

## Description

- Vulnerability Category: `Validations and error handling`
- Vulnerability Severity: `Medium`
- Detectors: [`zero-test-address`](https://github.com/CoinFabrik/scout/tree/main/detectors/zero-or-test-address)
- Test Cases: [`zero-test-address-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/zero-or-test-address/zero-or-test-address-1)

Verifying that the zero address is not assigned in a smart contract, including those built with ink! on the Substrate platform, is essential to avoid a potential vulnerability. The zero address has known private keys, and this poses a significant risk. If ownership is mistakenly transferred to the zero address, the contract becomes unmanageable as malicious actors can access and control it using the known private keys associated with the zero address. This would render any funds or functionality within the contract vulnerable and easily exploitable. Hence, always ensure the zero address is not set as the owner while coding or operating your ink! smart contracts to safeguard against this vulnerability.

Assigning a test address can also have similar implications, including the loss of access or granting access to a malicious actor if its private keys are not handled with care.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
#[ink(message)]
pub fn modify_admin(&mut self, admin: AccountId) -> Result<AccountId, Error> {
    if self.admin != self.env().caller() {
        return Err(Error::NotAuthorized);
    }

    self.admin = admin;
    Ok(self.admin)
}
```

The `modify_admin` function in this specific smart contract could be vulnerable due to an absence of validation for the incoming admin address. The function is intended to allow the existing admin to change the admin of the contract, but if the zero address is provided, it gets assigned as the admin. The private key for the zero address is known, which means anyone can claim ownership. Therefore, a validation check that rejects the zero address during the admin reassignment is crucial for the contract's security.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/zero-or-test-address/zero-or-test-address-1/vulnerable-example).

## Remediation

To remediate this problem, verify in your code whether the `admin` provided is the zero address and return an Error if this is the case.

```rust
#[ink(message)]
pub fn modify_admin(&mut self, admin: AccountId) -> Result<AccountId, Error> {
    if self.admin != self.env().caller() {
        return Err(Error::NotAuthorized);
    }

    if admin == AccountId::from([0x0; 32]) {
        return Err(Error::InvalidAddress);
    }

    self.admin = admin;
    Ok(self.admin)
}
```

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/zero-or-test-address/zero-or-test-address-1/remediated-example).

## References

* [Slither: Missing zero address validation](https://github.com/crytic/slither/wiki/Detector-Documentation#missing-zero-address-validation)
* https://blackadam.hashnode.dev/zero-address-check-the-danger
* https://substrate.stackexchange.com/questions/982/why-does-the-all-0-public-key-have-a-known-private-key-in-sr25519-and-ed25519
