# Reentrancy

### What it does

This linting rule checks whether the 'check-effect' interaction pattern has been properly followed by code that invokes a contract that may call back the original one.

### Why is this bad?

If state modifications are made after a contract call, reentrant calls may not detect these modifications, potentially leading to unexpected behaviors such as double spending.

### Known problems

If called method does not perform a malicious reentrancy (i.e. known method from known contract) false positives will arise.
If the usage of set_allow_reentry(true) or later state changes are performed in an auxiliary function, this detector will not detect the reentrancy.

### Example

```rust
let caller_addr = self.env().caller();
let caller_balance = self.balance(caller_addr);

if amount > caller_balance {
    return Ok(caller_balance);
}

let call = build_call::<ink::env::DefaultEnvironment>()
    .call(address)
    .transferred_value(amount)
    .exec_input(ink::env::call::ExecutionInput::new(Selector::new(
        selector.to_be_bytes(),
    )))
    .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
    .returns::<()>()
    .params();
self.env()
    .invoke_contract(&call)
    .map_err(|_| Error::ContractInvokeFailed)?
    .map_err(|_| Error::ContractInvokeFailed)?;

let new_balance = caller_balance.checked_sub(amount).ok_or(Error::Underflow)?;
self.balances.insert(caller_addr, &new_balance);
```

Use instead:

```rust
let caller_addr = self.env().caller();
let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
if amount <= caller_balance {
    //The balance is updated before the contract call
    self.balances
        .insert(caller_addr, &(caller_balance - amount));
    let call = build_call::<ink::env::DefaultEnvironment>()
        .call(address)
        .transferred_value(amount)
        .exec_input(ink::env::call::ExecutionInput::new(Selector::new(
            selector.to_be_bytes(),
        )))
        .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
        .returns::<()>()
        .params();
    self.env()
        .invoke_contract(&call)
        .unwrap_or_else(|err| panic!("Err {:?}", err))
        .unwrap_or_else(|err| panic!("LangErr {:?}", err));

    return caller_balance - amount;
} else {
    return caller_balance;
}
```

### Implementation

The detector's implementation can be found at these links [link1](https://github.com/CoinFabrik/scout/tree/main/detectors/reentrancy-1), [link2](https://github.com/CoinFabrik/scout/tree/main/detectors/reentrancy-2).
