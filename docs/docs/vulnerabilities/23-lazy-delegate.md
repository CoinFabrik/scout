# Lazy storage on delegate

## Description

- Vulnerability Category: `Known Bugs`
- Vulnerability Severity: `Critical`
- Detectors: [`lazy-delegate`](https://github.com/CoinFabrik/scout/tree/main/detectors/lazy-delegate)
- Test Cases: [`lazy-delegate-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/lazy-delegate/lazy-delegate-1)

ink! has a bug that makes delegated calls not modify the storage of the caller, unless it's using `Lazy` with `ManualKey` or `Mapping`.

## Exploit Scenario

Consider the following `ink!` contract:

```rust

// With this storage
    #[ink(storage)]
    pub struct DelegateCall {
        admin: AccountId,
    }


    #[ink(message)]
    pub fn change_admin(
        &mut self,
        target: Hash,
        new_admin: AccountId,
    ) -> Result<AccountId, Error> {
        if self.admin != self.env().caller() {
            return Err(Error::NotAnAdmin);
        }

        let res = build_call::<DefaultEnvironment>()
            .delegate(target)
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!("change_admin")))
                    .push_arg(new_admin),
            )
            .returns::<AccountId>()
            .try_invoke()
            .map_err(|_| Error::DelegateCallFailed)?
            .map_err(|_| Error::DelegateCallFailed)?;

        Ok(res)
}
```

In this example, the function `change_admin` takes `new_admin` and sets it as the new admin. If this function is called, `self.admin` will be the same as before, even if it's setted to a new `AccountId`.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/lazy-delegate/lazy-delegate-1/vulnerable-example).

## Remediation

To remediate this, we can use `Lazy` to store things.

```rust
    #[ink(storage)]
    #[derive(Default)]
    pub struct DelegateCall {
        admin: Lazy<AccountId, ManualKey<123456>>,
    }

    #[ink(message, payable)]
    pub fn change_admin(
        &mut self,
        target: Hash,
        new_admin: AccountId,
    ) -> Result<AccountId, Error> {
        if self.admin.get().unwrap() != self.env().caller() {
            return Err(Error::NotAnAdmin);
        }

        let res = build_call::<DefaultEnvironment>()
            .delegate(target)
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!("change_admin")))
                    .push_arg(new_admin),
            )
            .returns::<AccountId>()
            .try_invoke()
            .map_err(|_| Error::DelegateCallFailed)?
            .map_err(|_| Error::DelegateCallFailed)?;

        Ok(res)
    }
```

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/lazy-delegate/lazy-delegate-1/remediated-example).

## References

- https://github.com/paritytech/ink/issues/1825
- https://github.com/paritytech/ink/issues/1826
