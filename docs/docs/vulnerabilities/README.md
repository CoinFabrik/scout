---
sidebar_position: 2
---

# Vulnerabilities

This section lists relevant security-related issues typically introduced during the development of smart contracts in Substrate Ink!. While many of these issues can be generalized to Substrate-based networks, that is not always the case. The list, though non-exhaustive, features highly relevant items. Each issue is assigned a severity label based on the taxonomy presented below.

## Vulnerability Severity

This severity classification, although arbitrary, has been used in hundreds
of security audits and helps to understand the vulnerabilities we introduce
and measure the utility of this proof of concept.

- **Critical**: These issues seriously compromise the system and must be addressed immediately.
- **Medium**: These are potentially exploitable issues which might represent
  a security risk in the near future. We suggest fixing them as soon as possible.
- **Minor**: These issues represent problems that are relatively small or difficult to exploit, but might be exploited in combination with other issues. These kinds of issues do not block deployments in production environments. They should be taken into account and fixed when possible.
- **Enhancement**: This class relates to issues stemming from deviations from best practices or stylistic conventions, which could escalate into higher-priority issues due to other changes. For instance, these issues may lead to development errors in future updates.

## Vulnerability Categories

We follow with a taxonomy of Vulnerabilities. Many "top vulnerability" lists
can be found covering Ethereum/Solidity smart contracts. This list below is
used by the Coinfabrik Audit Team, when source code (security) audits in
Ethereum/Solidity, Stacks/Clarity, Algorand/PyTEAL /TEAL, Solana/RUST, etc.
The team discusses the creation of the list in this
[blogpost](https://blog.coinfabrik.com/analysis-categories/).

| Category                       | Description                                                                                       |
| ------------------------------ | ------------------------------------------------------------------------------------------------- |
| Arithmetic                     | Proper usage of arithmetic and number representation.                                             |
| Assembly Usage                 | Detailed analysis of implementations using assembly.                                              |
| Authorization                  | Vulnerabilities related to insufficient access control or incorrect authorization implementation. |
| Best practices                 | Conventions and best practices for improved code quality and vulnerability prevention.            |
| Block attributes               | Appropriate usage of block attributes, especially when used as a source of randomness.            |
| Centralization                 | Analysis of centralization and single points of failure.                                          |
| Denial of Service              | Denial of service. attacks.                                                                       |
| Gas Usage                      | Performance issues, enhancements and vulnerabilities related to use of gas.                       |
| Known Bugs                     | Known issues that remain unresolved.                                                              |
| MEV                            | Patterns that could lead to the exploitation of Maximal Extractable Value.                        |
| Privacy                        | Patterns revealing sensible user or state data.                                                   |
| Reentrancy                     | Consistency of contract state under recursive calls.                                              |
| Unexpected transfers           | Contract behavior under unexpected or forced transfers of tokens.                                 |
| Upgradability                  | Proxy patterns and upgradable smart contracts.                                                    |
| Validations and error handling | Handling of errors, exceptions and parameters.                                                    |

We used the above Vulnerability Categories, along with common examples of vulnerabilities detected within each category in other blockchains, as a guideline for finding and developing vulnerable examples of Substrate Ink! smart contracts.

## Vulnerability Classes

As a result of our research, we have so far identified thirteen types of vulnerabilities.

What follows is a description of each vulnerability in the context of ink! smart contracts. In each case, we have produced at least one [test-case](https://github.com/CoinFabrik/scout/tree/main/test-cases) smart contract that exposes one of these vulnerabilities.

Check our
[test-cases](https://github.com/CoinFabrik/scout/tree/main/test-cases)
for code examples of these vulnerabilities and their respective remediations.

### 1 - Integer overflow or underflow

This type of vulnerability occurs when an arithmetic operation attempts to
create a numeric value that is outside the valid range in substrate, e.g,
a `u8` unsigned integer can be at most _M:=2^8-1=255_, hence the sum `M+1`
produces an overflow.

An overflow/underflow is typically caught and generates an error. When it
is not caught, the operation will result in an inexact result which could
lead to serious problems.

We classified this type of vulnerability under
the [Arithmetic](#vulnerability-categories) category and assigned it a
Critical severity.

In the context of Substrate, we found that this vulnerability could only be
realized if overflow and underflow checks are disabled during compilation.
Notwithstanding, there are contexts where developers do turn off checks for
valid reasons and hence the reason for including this vulnerability in the
list.

Check the following [documentation](1-integer-overflow-or-underflow.md) for a more detailed explanation of this vulnerability class.

### 2 - Set contract storage

Smart contracts can store important information in memory which changes through the contract's lifecycle. Changes happen via user interaction with the smart contract. An _unauthorized_ set contract storage vulnerability happens when a smart contract call allows a user to set or modify contract memory when they were not supposed to be authorized.

Common practice is to have functions with the ability to change
security-relevant values in memory to be only accessible to specific roles,
e.g, only an admin can call the function `reset()` which resets auction values.
When this does not happen, arbitrary users may alter memory which may impose
great damage to the smart contract users.

We classified this type of vulnerability under
the [Authorization](#vulnerability-categories) category and assigned it a
Critical severity.

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

We classified this type of vulnerability under
the [Reentrancy](#vulnerability-categories) category and assigned it a
Critical severity.

In the context of `ink!` Substrate smart contracts there are controls
preventing reentrancy which could be turned off (validly) using the flag
`set_allow_reentry(true)`.

Check the following [documentation](3-reentrancy.md) for a more detailed explanation of this vulnerability class.

### 4 - Panic error

The use of the `panic!` macro to stop execution when a condition is not met is
useful for testing and prototyping but should be avoided in production code.
Using `Result` as the return type for functions that can fail is the idiomatic
way to handle errors in Rust.

We classified this issue, a deviation from best practices which could have
security implications, under the [Validations and error handling](#vulnerability-categories) category and assigned it an Enhancement severity.

Check the following [documentation](4-panic-error.md) for a more detailed explanation of this vulnerability class.

### 5 - Unused return enum

`Ink!` messages can return a `Result` `enum` with a custom error type. This is
useful for the caller to know what went wrong when the message fails. The
definition of the `Result` type enum consists of two variants: Ok and Err. If
any of the variants is not used, the code could be simplified or it could imply
a bug.

We put this vulnerability under the [Validations and error handling category](#vulnerability-categories)
with a Minor Severity.

In our example, we see how lack of revision on the usage of both types (`Ok`
and `Err`) leads to code where its intended functionality is not realized.

Check the following [documentation](5-unused-return-enum.md) for a more detailed explanation of this vulnerability class.

### 6 - DoS unbounded operation

Each block in a Substrate Blockchain has an upper bound on the amount of gas
that can be spent, and thus the amount of computation that can be done. This
is the Block Gas Limit. If the gas spent by a function call on an `ink!` smart
contract exceeds this limit, the transaction will fail. Sometimes it is the
case that the contract logic allows a malicious user to modify conditions
so that other users are forced to exhaust gas on standard function calls.

In order to prevent a single transaction from consuming all the gas in a block,
unbounded operations must be avoided. This includes loops that do not have a
bounded number of iterations, and recursive calls.

We classified this type of vulnerability under
the [Denial of Service](#vulnerability-categories) category and assigned it a
Medium severity.

A denial of service vulnerability allows the exploiter to hamper the
availability of a service rendered by the smart contract. In the context
of `ink!` smart contracts, it can be caused by the exhaustion of gas,
storage space, or other failures in the contract's logic.

Needless to say, there are many different ways to cause a DoS vulnerability.
This case is relevant and introduced repeatedly by the developer untrained in
web3 environments.

Check the following [documentation](6-dos-unbounded-operation.md) for a more detailed explanation of this vulnerability class.

### 7 - DoS unexpected revert with vector

Another type of Denial of Service attack is called unexpected revert. It occurs
by preventing transactions by other users from being successfully executed
forcing the blockchain state to revert to its original state.

This vulnerability again falls under the [Denial of Service](#vulnerability-categories) category
and has a Medium severity.

In this particular example, a Denial of Service through unexpected revert is
accomplished by exploiting a smart contract that does not manage storage size
errors correctly. It can be prevented by using Mapping instead of Vec to avoid
storage limit problems.

Check the following [documentation](7-dos-unexpected-revert-with-vector.md) for a more detailed explanation of this vulnerability class.

### 8 - Unsafe expect

In Rust, the `expect` method is commonly used for error handling. It retrieves the value from a `Result` or `Option` and panics with a specified error message if an error occurs. However, using `expect` can lead to unexpected program crashes.

This vulnerability falls under the [Validations and error handling](#vulnerability-categories) category
and has a Medium severity.

In our example, we see an exploit scenario involving a contract using the `expect` method in a function that retrieves the balance of an account. If there is no entry for the account, the contract panics and halts execution, enabling malicious exploitation.

Check the following [documentation](8-unsafe-expect.md) for a more detailed explanation of this vulnerability class.

### 9 - Unsafe unrwap

This vulnerability class pertains to the inappropriate usage of the `unwrap` method in Rust, which is commonly employed for error handling. The `unwrap` method retrieves the inner value of an `Option` or `Result`, but if an error or `None` occurs, it triggers a panic and crashes the program.

This vulnerability again falls under the [Validations and error handling](#vulnerability-categories) category and has a Medium severity.

In our example, we consider an contract that utilizes the `unwrap` method to retrieve the balance of an account from a mapping. If there is no entry for the specified account, the contract will panic and abruptly halt execution, opening avenues for malicious exploitation.

Check the following [documentation](9-unsafe-unwrap.md) for a more detailed explanation of this vulnerability class.

### 10 - Divide before multiply

This vulnerability class relates to the order of operations in Rust, specifically in integer arithmetic. Performing a division operation before a multiplication can lead to a loss of precision. This issue becomes significant in programs like smart contracts where numerical precision is crucial.

This vulnerability falls under the [Arithmetic](#vulnerability-categories) category
and has a Medium Severity.

Check the following [documentation](10-divide-before-multiply.md) for a more detailed explanation of this vulnerability class.

### 11 - Delegate call

Delegate calls can introduce security vulnerabilities if not handled carefully. The main idea is that delegate calls to contracts passed as arguments can be used to change the expected behavior of the contract, leading to potential attacks. It is important to validate and restrict delegate calls to trusted contracts, implement proper access control mechanisms, and carefully review external contracts to prevent unauthorized modifications, unexpected behavior, and potential exploits. By following these best practices, developers can enhance the security of their smart contracts and mitigate the risks associated with delegate calls.

This vulnerability falls under the [Authorization](#vulnerability-categories) category
and has a Critical severity.

Check the following [documentation](11-delegate-call.md) for a more detailed explanation of this vulnerability class.

### 12 - Zero or test address

The assignment of the zero address to a variable in a smart contract represents a critical vulnerability because it can lead to loss of control over the contract. This stems from the fact that the zero address does not have an associated private key, which means it's impossible to claim ownership, rendering any contract assets or functions permanently inaccessible.

Assigning a test address can also have similar implications, including the loss of access or granting access to a malicious actor if its private keys are not handled with care.

This vulnerability falls under the [Validations and error handling](#vulnerability-categories) category
and has a Medium severity.

Check the following [documentation](12-zero-or-test-address.md) for a more detailed explanation of this vulnerability class.

### 13 - Insufficiently random values

Using block attributes like `block_timestamp` or `block_number` for random number generation in ink! Substrate smart contracts is not recommended due to the predictability of these values. Block attributes are publicly visible and deterministic, making it easy for malicious actors to anticipate their values and manipulate outcomes to their advantage. Furthermore, validators could potentially influence these attributes, further exacerbating the risk of manipulation. For truly random number generation, it's important to use a source that is both unpredictable and external to the blockchain environment, reducing the potential for malicious exploitation.

This vulnerability again falls under the [Block attributes](#vulnerability-categories) category
and has a Critical severity.

Check the following [documentation](13-insufficiently-random-values.md) for a more detailed explanation of this vulnerability class.

### 14 - Unrestricted transfer from

In an ink! Substrate smart contract, allowing unrestricted `transfer_from` operations poses a significant vulnerability. When `from` arguments for that function is provided directly by the user, this might enable the withdrawal of funds from any actor with token approval on the contract. This could result in unauthorized transfers and loss of funds. To mitigate this vulnerability, instead of allowing an arbitrary `from` address, the `from` address should be restricted, ideally to the address of the caller (`self.env().caller()`), ensuring that the sender can initiate a transfer only with their own tokens.

This vulnerability falls under the [Validations and error handling](#vulnerability-categories) category
and has a Critical severity.

Check the following [documentation](14-unrestricted-transfer-from.md) for a more detailed explanation of this vulnerability class.

### 15 - Assert violation

The `assert!` macro is used in Rust to ensure that a certain condition holds true at a certain point in your code. If the condition does not hold, then the assert! macro will cause the program to panic. This is a problem, as seen in [panic-error](#4-panic-error)

We classified this issue, a deviation from best practices which could have
security implications, under the [Validations and error handling](#vulnerability-categories) category and assigned it an Enhancement severity.

### 16 - Avoid core::mem::forget

The `core::mem::forget` function is used to forget about a value without running its destructor. This could lead to memory leaks and logic errors.

We classified this issue, a deviation from best practices which could have
security implications, under the [Best practices](#vulnerability-categories) category and assigned it an Enhancement severity.

### 17 - Avoid format! macro

The `format!` macro is used to create a String from a given set of arguments. This macro is not recommended, it is better to use a custom error type enum.

We classified this issue, a deviation from best practices which could have
security implications, under the [Validations and error handling](#vulnerability-categories) category and assigned it an Enhancement severity.

### 18 - Unprotected self destruct

If users are allowed to call `terminate_contract`, they can intentionally or accidentally destroy the contract, leading to the loss of all associated data and functionalities given by this contract or by others that depend on it. To prevent this, the function should be restricted to administrators or authorized users only.

This vulnerability falls under the [Authorization](#vulnerability-categories) category
and has a Critical severity.

Check the following [documentation](18-unprotected-self-destruct.md) for a more detailed explanation of this vulnerability class.

### 19 - Iterators over indexing

The use of iterators over indexing is a best practice that should be followed in Rust. This is because accessing a vector by index is slower than using an iterator. Also, if the index is out of bounds, it will panic.

We classified this issue, a deviation from best practices which could have
security implications, under the [Best practices](#vulnerability-categories) category and assigned it an Enhancement severity.

### 20 - Ink version

Using an old version of ink! can be dangerous, as it may have bugs or security issues. Use the latest version available.

We classified this issue, a deviation from best practices which could have
security implications, under the [Best practices](#vulnerability-categories) category and assigned it an Enhancement severity.

### 21 - Unprotected set code hash

If users are allowed to call `set_code_hash`, they can intentionally modify the contract behaviour, leading to the loss of all associated data/tokens and functionalities given by this contract or by others that depend on it. To prevent this, the function should be restricted to administrators or authorized users only.

This vulnerability falls under the [Authorization](#vulnerability-categories) category
and has a Critical severity.

Check the following [documentation](21-unprotected-set-code-hash.md) for a more detailed explanation of this vulnerability class.

### 22 - Unprotected mapping operation

Modifying mappings with an arbitrary key given by the user could lead to unintented modifications of critical data, modifying data belonging to other users, causing denial of service, unathorized access, and other potential issues.

This vulnerability falls under the [Validations and error handling](#vulnerability-categories) category
and has a Critical severity.

Check the following [documentation](22-unprotected-mapping-operation.md) for a more detailed explanation of this vulnerability class.

### 23 - Lazy storage on delegate

A bug in ink! causes delegated calls to not modify the caller's storage unless Lazy with ManualKey or Mapping is used.

This vulnerability falls under the [Known Bugs](#vulnerability-categories) category
and has a Critical severity.

Check the following [documentation](23-lazy-delegate.md) for a more detailed explanation of this vulnerability class.
