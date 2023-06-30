# Unrestricted transfer from

## Description

- Vulnerability Category: `Validations and error handling`
- Vulnerability Severity: `Critical`
- Detectors: [`unrestricted-transfer-from`](https://github.com/CoinFabrik/scout/tree/main/detectors/unrestricted-transfer-from)
- Test Cases: [`unrestricted-transfer-from-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1)

In an ink! Substrate smart contract, allowing unrestricted `transfer_from` operations poses a significant vulnerability. When arguments for such functions are provided directly by the user, this might enable the withdrawal of funds from any actor with token approval on the contract. Specifically, a user could pass the address of an actor with approval as an argument to `transfer_from`, allowing the user to transfer tokens from that actor's balance. This could result in unauthorized transfers and loss of funds. To mitigate this vulnerability, instead of allowing an arbitrary from address, the from address should be restricted, ideally to the address of the caller (`self.env().caller()`), ensuring that only the sender can initiate a transfer.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
#[ink(message)]
pub fn deposit(&mut self, from: AccountId) -> Result<(), Error>{
    if self.env().caller() != self.buyer {
        return Err(Error::CallerMustBeBuyer)
    } else if self.status != Status::Created {
        return Err(Error::StatusMustBeCreated)
    } else {
        // 0x54b3c76e selector comes from https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md
        let call_params = build_call::<DefaultEnvironment>()
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "PSP22::transfer_from"
                )))
                .push_arg(from)
                .push_arg(self.env().account_id())
                .push_arg(self.amount)
                .push_arg([0u8])
            )
            .returns::<Result<(),PSP22Error>>()
            .call(self.psp22_address)
            .params();
        let res = self.env().invoke_contract(&call_params)
            .unwrap_or_else(|err| panic!("Err {:?}", err))
            .unwrap_or_else(|err| panic!("LangErr {:?}", err))
            .map_err(|err| Error::PSP22Error(err));
        if res.is_ok() {
            self.status = Status::Locked;
        }
        return res;
    }
}
```

The deposit function in this example exhibits a vulnerability due to the unrestricted use of the from argument in the `PSP22::transfer_from` call. This argument is provided by the user and could refer to any account that has token approval on the contract. Consequently, a user can perform an unauthorized withdrawal from an account that didn't intend to make a deposit, resulting in potential loss of funds. Therefore, unrestricted `transfer_from` operations can be a serious security risk, as they allow manipulation of the `from` parameter, enabling undesired transfers.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/vulnerable-example).

## Remediation

In order to remediate this issue, avoid using function parameters as input for the `transfer_from` function.

```rust
#[ink(message)]
pub fn deposit(&mut self) -> Result<(), Error>{
    if self.env().caller() != self.buyer {
        return Err(Error::CallerMustBeBuyer)
    } else if self.status != Status::Created {
        return Err(Error::StatusMustBeCreated)
    } else {
        // 0x54b3c76e selector comes from https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md
        let call_params = build_call::<DefaultEnvironment>()
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "PSP22::transfer_from"
                )))
                .push_arg(self.env().caller())
                .push_arg(self.env().account_id())
                .push_arg(self.amount)
                .push_arg([0u8])
            )
            .returns::<Result<(),PSP22Error>>()
            .call(self.psp22_address)
            .params();
        let res = self.env().invoke_contract(&call_params)
            .unwrap_or_else(|err| panic!("Err {:?}", err))
            .unwrap_or_else(|err| panic!("LangErr {:?}", err))
            .map_err(|err| Error::PSP22Error(err));
        if res.is_ok() {
            self.status = Status::Locked;
        }
        return res;
    }

}
```

The vulnerability was addressed in the remediated deposit function by removing the arbitrary from argument and replacing it with `self.env().caller()`. This change ensures that the `transfer_from` function is called only with the account of the caller, thereby eliminating the risk of unauthorized token transfers. The remediation restricts `transfer_from` operations to the caller, significantly reducing the potential for malicious manipulation by confining token transfers to the initiating user's account. This enforces a safeguard where only the user invoking the function can transfer tokens from their account to the contract, thus ensuring the secure execution of the contract.

## References

- [Slither: Arbitrary from in transfer from](https://github.com/crytic/slither/wiki/Detector-Documentation#arbitrary-from-in-transferfrom)