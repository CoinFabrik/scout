---
sidebar_position: 2
---

# Vulnerabilities
This section lists relevant security-related issues typically introduced during the development of smart contracts in Substrate Ink!. While many of these issues can be generalized to Substrate-based networks, that is not always the case. The list, though non-exhaustive, features highly relevant items. Each issue is assigned a severity label based on the taxonomy presented below.

## Vulnerability Severity
This severity classification, although arbitrary, has been used in hundreds
of security audits and helps to understand the vulnerabilities we introduce
and measure the utility of this proof of concept.
* __Critical__: These issues seriously compromise the system and must be addressed immediately.
* __Medium__: These are potentially exploitable issues which might represent 
a security risk in the near future. We suggest fixing them as soon as possible.
* __Minor__: These issues represent problems that are relatively small or difficult to exploit, but might be exploited in combination with other issues. These kinds of issues do not block deployments in production environments. They should be taken into account and fixed when possible.
* __Enhancement__: This class relates to issues stemming from deviations from best practices or stylistic conventions, which could escalate into higher-priority issues due to other changes. For instance, these issues may lead to development errors in future updates.

## Vulnerability Categories
We follow with a taxonomy of Vulnerabilities. Many "top vulnerability" lists 
can be found covering Ethereum/Solidity smart contracts. This list below is 
used by the Coinfabrik Audit Team, when source code (security) audits in
Ethereum/Solidity, Stacks/Clarity, Algorand/PyTEAL /TEAL, Solana/RUST, etc.
The team discusses the creation of the list in this 
[blogpost](https://blog.coinfabrik.com/analysis-categories/).

| Category                  | Description                                                                                                      |
| -------------------------| -----------------------------------------------------------------------------------------------------------------|
| Arithmetic                | Proper usage of arithmetic and number representation.                                                              |
| Assembly Usage            | Detailed analysis of implementations using assembly.                                                             |
| Authorization             | Vulnerabilities related to insufficient access control or incorrect authorization implementation.                |
| Best practices            | Conventions and best practices for improved code quality and vulnerability prevention.                           |
| Block attributes          | Appropriate usage of block attributes, especially when used as a source of randomness.                      |
| Centralization            | Analysis of centralization and single points of failure.                                                         |
| DoS                       | Denial of service attacks.                                                                                        |
| Gas Usage                 | Performance issues, enhancements and vulnerabilities related to use of gas.                                      |
| MEV                       | Patterns that could lead to the exploitation of Maximal Extractable Value.                                        |
| Privacy                   | Patterns revealing sensible user or state data.                                                                   |
| Reentrancy                | Consistency of contract state under recursive calls.                                                              |
| Unexpected transfers      | Contract behavior under unexpected or forced transfers of tokens.                                               |
| Upgradability             | Proxy patterns and upgradable smart contracts.                                                                   |
| Validations and error handling | Handling of errors, exceptions and parameters.                                                               |

We used the above Vulnerability Categories, along with common examples of vulnerabilities detected within each category in other blockchains, as a guideline for finding and developing vulnerable examples of Substrate Ink! smart contracts.

## Vulnerability Classes
As a result of our research, we have so far identified seven types of vulnerabilities that fall under the six following different vulnerability categories (so two types fall under one category): Arithmetic, Authorization, Denial of Service, Reentrancy, and Validations and Error Handling.

What follows is a description of each vulnerability in the context of ink! smart contracts. In each case, we have produced a smart contract exposing one of these vulnerabilities.

Check our
[test-cases](https://github.com/CoinFabrik/scout/tree/main/test-cases)
for code examples of these vulnerabilities

### 1 - Integer Overflow and Integer Underflow
This type of vulnerability occurs when an arithmetic operation attempts to 
create a numeric value that is outside the valid range in substrate, e.g, 
a `u8` unsigned integer can be at most *M:=2^8-1=255*, hence the sum `M+1` 
produces an overflow. 

An overflow/underflow is typically caught and generates an error. When it 
is not caught, the operation will result in an inexact result which could 
lead to serious problems. We classified this type of vulnerability under 
the [Arithmetic Category](#vulnerability-categories) type and assinged it a
Critical Severity.

In the context of Substrate, we found that this vulnerability could only be 
realized if overflow and underflow checks are disabled during compilation. 
Notwithstanding, there are contexts where developers do turn off checks for 
valid reasons and hence the reason for including this vulnerability in the 
list. 

Check the following [documentation](1-integer-overflow-or-underflow.md) for a more detailed explanation of this vulnerability class.

### 2 - Unauthotized Set Contract Storage
Smart contracts can store important information in memory which changes through the contract's lifecycle. Changes happen via user interaction with the smart contract. An _unauthorized_ set contract storage vulnerability happens when a smart contract call allows a user to set or modify contract memory when they were not supposed to be authorized.

Common practice is to have functions with the ability to change
security-relevant values in memory to be only accessible to specific roles,
e.g, only an admin can call the function `reset()` which resets auction values.
When this does not happen, arbitrary users may alter memory which may impose 
great damage to the smart contract users. We classified this vulnerability 
under the [Authorization Category](#vulnerability-categories) and assigned it a 
Critical Severity.

In `ink!` the function `set_contract_storage(key: &K, value: &V)` can be used
to modify the contract storage under a given key. When a smart contract uses
this function, the contract needs to check if the caller should be able to
alter this storage. If this does not happen, an arbitary caller may modify
balances and other relevant contract storage.

Check the following [documentation](2-set-contract-storage.md) for a more detailed explanation of this vulnerability class.

### 3 - Reentrancy
An `ink!` smart contract can interact with other smart contracts. These
operations imply (external) calls where control flow is passed to the called
contract until the execution of the called code is over, then the control is
delivered back to the caller. A _reentrancy_ vulnerability may happen when a
user calls a function, this function calls a malicious contract which again
calls this same function, and this 'reentrancy' has unexpected reprecussions
to the contract.
This kind of attack was used in Ethereum for
[the infamous DAO Hack](https://www.economist.com/finance-and-economics/2016/05/19/the-dao-of-accrue).

This vulnerability may be prevented with the use of the Check-Effect-Interaction
pattern that dictates that we first evaluate (check) if the necessary conditions
are granted, next we record the effects of the interaction and finally we
execute the interaction (e.g., check if the user has funds, substract the funds
from the records, then transfer the funds). There's also so-called
_reentrancy guards_ which prevent the marked piece of code to be called twice
from the same contract call. When the vulnerability may be exercised, the
successive calls to the contract may allow the malicious contract to execute a
function partially many times, e.g., transfering funds many times but 
substracting the funds only once. 
This vulnerability is of the [Reentrancy Category](#vulnerability-categories) and 
assign it a Critical Severity.

In the context of `ink!` Substrate smart contracts there are controls
preventing reentrancy which could be turned off (validly) using the flag
`set_allow_reentry(true)`.

Check the following [documentation](3-reentrancy.md) for a more detailed explanation of this vulnerability class.

### 4 - Panic error
The use of the `panic!` macro to stop execution when a condition is not met is
useful for testing and prototyping but should be avoided in production code.
Using `Result` as the return type for functions that can fail is the idiomatic
way to handle errors in Rust.

We classified this issue, a deviation for best practices which could have 
security implications, under the [Validations and Error Handling Category](#vulnerability-categories)
with the severity of an Enhancement.

Check the following [documentation](4-panic-error.md) for a more detailed explanation of this vulnerability class.

### 5 - Unused Return enum
`Ink!` messages can return a `Result` `enum` with a custom error type. This is
useful for the caller to know what went wrong when the message fails. The 
definition of the `Result` type enum consists of two variants: Ok and Err. If 
any of the variants is not used, the code could be simplified or it could imply
a bug.

We put this vulnerability under the [Validations and Error Handling Category](#vulnerability-categories) 
with a Minor Severity.

In our example, we see how lack of revision on the usage of both types (`Ok`
and `Err`) leads to code where its intended functionality is not realized.

Check the following [documentation](5-unused-return-enum.md) for a more detailed explanation of this vulnerability class.

### 6 - DoS Unbounded Operation
Each block in a Substrate Blockchain has an upper bound on the amount of gas
that can be spent, and thus the amount of computation that can be done. This
is the Block Gas Limit. If the gas spent by a function call on an `ink!` smart
contract exceeds this limit, the transaction will fail. Sometimes it is the
case that the contract logic allows a malicious user to modify conditions
so that other users are forced to exhaust gas on standard function calls.

In order to prevent a single transaction from consuming all the gas in a block, 
unbounded operations must be avoided. This includes loops that do not have a 
bounded number of iterations, and recursive calls. This vulnerability falls
under the [Denial of Service Category](#vulnerability-categories) and has a Medium
Severity.
A denial of service vulnerability allows the exploiter to hamper the 
availability of a service rendered by the smart contract. In the context 
of `ink!` smart contracts, it can be caused by the exhaustion of gas,
storage space, or other failures in the contract's logic.

Needless to say, there are many different ways to cause a DOS vulnerability.
This case is relevant and introduced repeatedly by the developer untrained in
web3 environments. 

Check the following [documentation](6-dos-unbounded-operation.md) for a more detailed explanation of this vulnerability class.

### 7 - DoS Unexpected Revert With Vector
Another type of Denial of Service attack is called unexpected revert. It occurs
by preventing transactions by other users from being successfully executed
forcing the blockchain state to revert to its original state.

This vulnerability again falls under the [Denial of Service Category](#vulnerability-categories)
and similarly has a Medium Severity.

In this particular example, a Denial of Service through unexpected revert is
accomplished by exploiting a smart contract that does not manage storage size
errors correctly. It can be prevented by using Mapping instead of Vec to avoid
storage limit problems.

Check the following [documentation](7-dos-unexpected-revert-with-vector.md) for a more detailed explanation of this vulnerability class.