# Unrestricted Transfer From

## Description

- Vulnerability Category: `Validations and error handling`
- Vulnerability Severity: `High`
- Detectors: [`unrestricted-transfer-from`](https://github.com/CoinFabrik/scout/tree/main/detectors/unrestricted-transfer-from)
- Test Cases: [`unrestricted-transfer-from-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1)

Using an user-defined argument as a `transfer_from`'s `from` parameter could lead to transfer funds from a third party account without proper authorization.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
// build_call example
    #[ink(message)]
    pub fn deposit(&mut self, from: AccountId) -> Result<(), Error> {
        let call_params = build_call::<DefaultEnvironment>()
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "PSP22::transfer_from"
                )))
                .push_arg(from)
                .push_arg(self.env().account_id())
                .push_arg(self.amount)
                .push_arg([0u8]),
            )
    }

// ContractRef example
    #[ink(message)]
    pub fn deposit(&mut self, from: AccountId) -> Result<(), Error> {
        let res = PSP22Ref::transfer_from(
            &self.psp22_address,
            from,
            self.env().account_id(),
            self.amount,
            vec![],
        );
    }
```

The vulnerability in this `deposit` function arises from the use of `from`, an user-defined parameter as an argument in the `from` field of the `transfer_from` function. Alice can approve a contract to spend their tokens, then Bob can call that contract, use that allowance to send as themselves Alice's tokens.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/vulnerable-example).

## Remediation

Avoid using user-defined arguments as `from` parameter in `transfer_from`. Instead, use `self.env().caller()`.

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/remediated-example).

## References

- [Slither: Arbitrary from in transferFrom](https://github.com/crytic/slither/wiki/Detector-Documentation#arbitrary-from-in-transferfrom)
