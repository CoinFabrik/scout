# DoS unbounded operation
## Description
- Vulnerability Category: `Denial of Service`
- Severity: `Medium`
- Detectors: [`dos-unbounded-operation`](https://github.com/CoinFabrik/scout/tree/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-1)
- Test Cases: [`dos-unbounded-operation-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-1)

Each block in a Substrate Blockchain has an upper bound on the amount of gas 
that can be spent, and thus the amount computation that can be done. This is 
the Block Gas Limit. If the gas spent exceeds this limit, the transaction 
will fail.

In this smart contract a malicious user may modify the smart contract's
conditions so that any transaction coming after will fail, thus imposing
a denial of service for other users.

## Exploit Scenario
In the following example, a contract has a function `add_payee` that allows 
adding a new element to a vector. The function `pay_out` iterates over the 
vector and transfers the value to the payee's address. The problem is that 
the `pay_out()` function does not have a fixed number of iterations, and thus 
it can consume all the gas in a block.

A malicious user could call `add_payee` a large number of times, thus 
populating the vector with a large number of elements. Then, the function 
`pay_out` when iterating over all the elements, will consume all the gas in 
a block, and the transaction will fail, successfully performing a DoS attack.

```rust
/// Adds a new payee to the operation.
#[ink(message, payable)]
pub fn add_payee(&mut self) -> u128 {
    let address = self.env().caller();
    let value = self.env().transferred_value();
    let new_payee = Payee { address, value };

    self.payees.insert(self.next_payee_ix, &new_payee);
    self.next_payee_ix = self.next_payee_ix.checked_add(1).unwrap();

    // Return the index of the new payee
    self.next_payee_ix.checked_sub(1).unwrap()
}
```
The vulnerable code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-1/vulnerable-example/lib.rs).

### Deployment
An example can be found under the directory 
[vulnerable-example](https://github.com/CoinFabrik/scout/blob/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-1/vulnerable-example). The exploit can be tested by
running the end-to-end test called `pay_out_runs_out_of_gas`.

## Remediation
The main recommendation is to change the form of payments to favor pull over 
push. This way, the contract does not need to iterate over a vector of payees,
and thus it does not need to consume all the gas in a block. The payee could 
instead call a function that will transfer the value to the payee's address.

If looping over an array of unknown size is absolutely necessary, then it 
should be planned to potentially take multiple blocks, and therefore require
multiple transactions.

The remediated code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-1/remediated-example/lib.rs).

## References
- https://consensys.github.io/smart-contract-best-practices/attacks/denial-of-service
- https://consensys.github.io/smart-contract-best-practices/development-recommendations/general/external-calls/
