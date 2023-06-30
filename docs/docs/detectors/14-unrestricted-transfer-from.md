# Unrestricted transfer from

### What it does
Checks the call of `transfer_from` from a `psp22` contract where the used arguments are provided by a user.


### Why is this bad?
The user could provide as an argument the address of any actor with a token approval on the contract, and this actor could then withdraw funds from the contract.

### Example

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
Use instead

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

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout/tree/main/detectors/unrestricted-transfer-from).