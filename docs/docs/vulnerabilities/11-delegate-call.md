# Delegate call

## Description

- Vulnerability Category: `Authorization`
- Vulnerability Severity: `Critical`
- Detectors: [`delegate-call`](https://github.com/CoinFabrik/scout/tree/main/detectors/delegate-call)
- Test Cases: [`delegate-call-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1)

Delegate calls can introduce security vulnerabilities if not handled carefully. The main idea is that delegate calls to contracts passed as arguments can be used to change the expected behavior of the contract, leading to potential attacks. It is important to validate and restrict delegate calls to trusted contracts, implement proper access control mechanisms, and carefully review external contracts to prevent unauthorized modifications, unexpected behavior, and potential exploits. By following these best practices, developers can enhance the security of their smart contracts and mitigate the risks associated with delegate calls.


## Exploit Scenario

Consider the following `ink!` contract:

```rust
#[ink(message)]
pub fn delegate_call(&mut self, target: Hash, argument: Balance) {
    let selector_bytes = [0x0, 0x0, 0x0, 0x0];
    let result: T  = build_call::<DefaultEnvironment>()
        .delegate(target)
        .exec_input(
            ExecutionInput::new(Selector::new(selector_bytes))
                .push_arg(argument)
        )
        .returns::<T>()
        .invoke();
}
```

In this example, the `delegate_call` function allows for delegated calls to contracts passed as arguments without any validation or access control. This creates a vulnerability as it enables potential attackers to pass a malicious contract as the target, leading to unauthorized modifications or unexpected behavior in the smart contract.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1/vulnerable-example).

## Remediation

In the following remediated example, the vulnerability is addressed by removing the ability to pass the target contract as an argument in the `delegate_call` function. Instead, the target contract address is stored in a storage variable `self.target`, which can only be modified by calling the `set_target` function. The `set_target` function includes access control logic, allowing only the contract's administrator to update the target contract address. This remediation ensures that only trusted and authorized contracts can be delegated to, preventing the vulnerability associated with unvalidated and uncontrolled delegate calls.

```rust
    #[ink(message)]
    pub fn delegate_call(&mut self, argument: Balance) {
        let selector_bytes = [0x0, 0x0, 0x0, 0x0];
        let result: T  = build_call::<DefaultEnvironment>()
            .delegate(self.target)
            .exec_input(
                ExecutionInput::new(Selector::new(selector_bytes))
                    .push_arg(argument)
            )
            .returns::<T>()
            .invoke();
    }

    #[ink::message]
    pub fn set_target(&mut self, new_target: Hash) -> Result<(), Error> {
        if self.admin != self.env().caller() {
            Err(Error::Unauthorized)
        } else {
            self.target = new_target;
            Ok(())
        }
    }

```
The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1/remediated-example).

## References

- https://solidity-by-example.org/delegatecall/
- https://solidity-by-example.org/hacks/delegatecall/
- https://blog.sigmaprime.io/solidity-security.html#delegatecall
- [SWC-112](https://swcregistry.io/docs/SWC-112)
- [Ethernaut: Delegation](https://ethernaut.openzeppelin.com/level/0x9451961b7Aea1Df57bc20CC68D72f662241b5493)
- [Slither: Delegatecall](https://github.com/crytic/slither/wiki/Detector-Documentation#controlled-delegatecall)