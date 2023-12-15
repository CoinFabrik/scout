# Scout: Security Analysis Tool

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

<p align="center">
  <img src="/assets/scout.png" alt="Scout in a Dark Forest" width="300" center  />
</p>

Scout is an extensible open-source tool intended to assist ink! smart contract developers and auditors detect common security issues and deviations from best practices.

This tool will help developers write secure and more robust smart contracts.

Our interest in this project comes from our experience in manual auditing and our usage of comparable tools in other blockchains.
To improve coverage and precision, we´ll persist in research efforts on static and dynamic analysis techniques. Find more about our ongoing research at our associated repository.

## Quick Start

For a quick start, make sure that [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed on your computer. Then, install Scout dependencies by running the following command:

```bash
cargo install cargo-dylint dylint-link
```

Afterwards, install Scout with the following command:

```bash
cargo install cargo-scout-audit
```

To run Scout on your project, navigate to its root directory and execute the following command:

```bash
cargo scout-audit
```

For more information on installation and usage, please refer to the [Getting Started](https://coinfabrik.github.io/scout/docs/intro) section in our documentation below.

## Documentation

- [Getting Started](https://coinfabrik.github.io/scout/docs/intro)
- [Vulnerabilities](https://coinfabrik.github.io/scout/docs/vulnerabilities)
- [Detectors](https://coinfabrik.github.io/scout/docs/detectors)
- [Contribute](https://coinfabrik.github.io/scout/docs/contribute)
- [Architecture](https://coinfabrik.github.io/scout/docs/architecture)
- [Blog](https://blog.coinfabrik.com/)

Visit [Scout's website](https://coinfabrik.github.io/scout/) to view the full documentation.

## Detectors

| Detector ID                                                                                                              | What it Detects                                                                                                                                                                                           | Test Cases                                                                                                                                                                                                                                               | Severity    |
| ------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| [integer-overflow-or-underflow](https://coinfabrik.github.io/scout/docs/detectors/integer-overflow-or-underflow)         | [An arithmetic operation overflows or underflows the available memory allocated to the variable.](https://coinfabrik.github.io/scout/docs/vulnerabilities/integer-overflow-or-underflow)                  | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-1), [2](https://github.com/CoinFabrik/scout/tree/main/test-cases/integer-overflow-or-underflow/integer-overflow-or-underflow-2) | Critical    |
| [set-contract-storage](https://coinfabrik.github.io/scout/docs/detectors/set-contract-storage)                           | [Insufficient access control on set_contract_storage() function.](https://coinfabrik.github.io/scout/docs/vulnerabilities/set-contract-storage)                                                           | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/set-contract-storage/set-contract-storage-1)                                                                                                                                                | Critical    |
| [reentrancy](https://coinfabrik.github.io/scout/docs/detectors/reentrancy)                                               | [Consistency of contract state under recursive calls.](https://coinfabrik.github.io/scout/docs/vulnerabilities/reentrancy)                                                                                | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1), [2](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-2), [3](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-2/reentrancy-1)                                                                             | Critical    |
| [panic-error](https://coinfabrik.github.io/scout/docs/detectors/panic-error)                                          | [Code panics on error instead of using descriptive enum.](https://coinfabrik.github.io/scout/docs/vulnerabilities/panic-error)                                                                         | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/panic-error/panic-error-1)                                                                                                                                                                  | Enhancement |
| [unused-return-enum](https://coinfabrik.github.io/scout/docs/detectors/unused-return-enum)                               | [Return enum from a function is not completely used.](https://coinfabrik.github.io/scout/docs/vulnerabilities/unused-return-enum)                                                                         | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/unused-return-enum/unused-return-enum-1)                                                                                                                                                    | Minor       |
| [dos-unbounded-operation](https://coinfabrik.github.io/scout/docs/detectors/dos-unbounded-operation)                     | [DoS due to unbounded operation.](https://coinfabrik.github.io/scout/docs/vulnerabilities/dos-unbounded-operation)                                                                      | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-1)                                                                                                                                          | Medium      |
| [dos-unexpected-revert-with-vector](https://coinfabrik.github.io/scout/docs/detectors/dos-unexpected-revert-with-vector) | [DoS due to improper storage.](https://coinfabrik.github.io/scout/docs/vulnerabilities/dos-unexpected-revert-with-vector)                                                                                 | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/dos-unexpected-revert-with-vector/dos-unexpected-revert-with-vector-1)                                                                                                                      | Medium      |
| [unsafe-expect](https://coinfabrik.github.io/scout/docs/detectors/unsafe-expect)                                         | [Improper usage of the expect method, leading to unexpected program crashes.](https://coinfabrik.github.io/scout/docs/vulnerabilities/unsafe-expect)                                                      | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/unsafe-expect/unsafe-expect-1)                                                                                                                                                              | Medium      |
| [unsafe-unwrap](https://coinfabrik.github.io/scout/docs/detectors/unsafe-unwrap)                                         | [Inappropriate usage of the unwrap method, causing unexpected program crashes.](https://coinfabrik.github.io/scout/docs/vulnerabilities/unsafe-unwrap)                                                    | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/unsafe-unwrap/unsafe-unwrap-1)                                                                                                                                                              | Medium      |
| [divide-before-multiply](https://coinfabrik.github.io/scout/docs/detectors/divide-before-multiply)                       | [Performing a division operation before a multiplication, leading to loss of precision.](https://coinfabrik.github.io/scout/docs/vulnerabilities/divide-before-multiply)                                  | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/divide-before-multiply/divide-before-multiply-1)                                                                                                                                            | Medium      |
| [delegate-call](https://coinfabrik.github.io/scout/docs/detectors/delegate-call)                                         | [Invoking code in another contract keeping the first contract's context.](https://coinfabrik.github.io/scout/docs/vulnerabilities/delegate-call)                                                          | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1)                                                                                                                                                              | Critical    |
| [zero-or-test-address](https://coinfabrik.github.io/scout/docs/detectors/zero-or-test-address)                           | [Avoid zero or test address assignment to prevent contract control loss.](https://coinfabrik.github.io/scout/docs/vulnerabilities/zero-or-test-address)                                                   | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/zero-or-test-address/zero-or-test-address-1)                                                                                                                                                | Medium      |
| [insufficiently-random-values](https://coinfabrik.github.io/scout/docs/detectors/insufficiently-random-values)           | [Avoid using block attributes for random number generation to prevent manipulation.](https://coinfabrik.github.io/scout/docs/vulnerabilities/insufficiently-random-values)                                | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1)                                                                                                                                | Critical    |
| [unrestricted-transfer-from](https://coinfabrik.github.io/scout/docs/detectors/unrestricted-transfer-from)               | [Avoid passing an user-defined parameter as a `from` field in transfer-from ](https://coinfabrik.github.io/scout/docs/vulnerabilities/unrestricted-transfer-from)                                         | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1)                                                                                                                                    | Critical        |
| [assert-violation](https://coinfabrik.github.io/scout/docs/detectors/assert-violation)                                   | [Avoid the usage of the macro `assert!`, it can panic.](https://coinfabrik.github.io/scout/docs/vulnerabilities/assert-violation)                                                                         | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/assert-violation/assert-violation-1)                                                                                                                                                        | Enhacement  |
| [avoid-core-mem-forget](https://coinfabrik.github.io/scout/docs/detectors/avoid-core-mem-forget)                 | [The use of core::mem::forget could lead to memory leaks and logic errors](https://coinfabrik.github.io/scout/docs/vulnerabilities/avoid-core-mem-forget)                                                 | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-core-mem-forget/avoid-core-mem-forget-1)                                                                                                                                              | Enhacement  |
| [avoid-format-string](https://coinfabrik.github.io/scout/docs/detectors/avoid-format-string)                      | [The `format!` macro is not recommended. A custom error is recommended instead.](https://coinfabrik.github.io/scout/docs/vulnerabilities/avoid-format-string)                                            | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-format-string/avoid-format-string-1)                                                                                                                                                | Enhacement  |
| [unprotected-self-destruct](https://coinfabrik.github.io/scout/docs/detectors/unprotected-self-destruct)                  | [If users are allowed to call terminate_contract, they can intentionally or accidentally destroy the contract.](https://coinfabrik.github.io/scout/docs/vulnerabilities/unprotected-self-destruct) | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/unprotected-self-destruct/unprotected-self-destruct-1)                                                                                                                                          | Critical  |
| [iterators-over-indexing](https://coinfabrik.github.io/scout/docs/detectors/iterators-over-indexing)                  | [Iterating with hardcoded indexes is slower than using an iterator. Also, if the index is out of bounds, it will panic.](https://coinfabrik.github.io/scout/docs/vulnerabilities/iterators-over-indexing) | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/iterators-over-indexing/iterators-over-indexing-1)                                                                                                                                          | Enhacement  |
| [ink-version](https://coinfabrik.github.io/scout/docs/detectors/ink-version)                  | [Using a pinned version of ink! can be dangerous, as it may have bugs or security issues. Use the latest version available.](https://coinfabrik.github.io/scout/docs/vulnerabilities/ink-version) | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/ink-version/ink-version-1)                                                                                                                                          | Enhacement  |
| [unprotected-set-code-hash](https://coinfabrik.github.io/scout/docs/detectors/unprotected-set-code-hash)                  | [If users are allowed to call terminate_contract, they can intentionally modify the contract behaviour.](https://coinfabrik.github.io/scout/docs/vulnerabilities/unprotected-set-code-hash) | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/set-code-hash/set-code-hash-1)                                                                                                                                          | Critical  |
| [unprotected-mapping-operation](https://coinfabrik.github.io/scout/docs/detectors/unprotected-mapping-operation)                  | [Modifying mappings with an arbitrary key given by the user could lead to unintented modifications of critical data, modifying data belonging to other users, causing denial of service, unathorized access, and other potential issues.](https://coinfabrik.github.io/scout/docs/vulnerabilities/unprotected-mapping-operation) | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/unprotected-mapping-operation/unprotected-mapping-operation-1)                                                                                                                                          | Critical  |
| [lazy-delegate](https://coinfabrik.github.io/scout/docs/detectors/lazy-delegate)                  | [Delegated calls in ink! need lazy storage.](https://coinfabrik.github.io/scout/docs/vulnerabilities/lazy-delegate) | [1](https://github.com/CoinFabrik/scout/tree/main/test-cases/lazy-delegate/lazy-delegate-1)                                                                                                                                          | Critical  |
                                                                                                        


## Tests

To validate our tool, we provide a set of code examples located in the [test-cases](https://github.com/CoinFabrik/scout/tree/main/test-cases) folder.

In order to run the integration tests, navigate to `apps/cargo-scout-audit` and run:

```console
cargo test --all --all-features
```

In order to run the tests for a particular test-case, run the same command on that particular test-case folder (e.g: `test-cases/delegate-call/delegate-call-1/vulnerable-example`)

## Acknowledgements

Scout is an open source vulnerability analyzer developed by [CoinFabrik's](https://www.coinfabrik.com/) Research and Development team.

We received support through grants from both the [Web3 Foundation Grants Program](https://github.com/w3f/Grants-Program/tree/master) and the [Aleph Zero Ecosystem Funding Program](https://alephzero.org/ecosystem-funding-program).

| Grant Program | Description |
|---------------|-------------|
| ![Web3 Foundation](https://raw.githubusercontent.com/CoinFabrik/scout/main/assets/web3-foundation.png) | **Proof of Concept:** We collaborated with the [Laboratory on Foundations and Tools for Software Engineering (LaFHIS)](https://lafhis.dc.uba.ar/) at the [University of Buenos Aires](https://www.uba.ar/internacionales/index.php?lang=en) to establish analysis techniques and tools for our detectors, as well as to create an initial list of vulnerability classes and code examples. [View Grant](https://github.com/CoinFabrik/web3-grant) \| [Application Form](https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik.md).<br><br>**Prototype:** We built a functioning prototype using linting detectors built with [Dylint](https://github.com/trailofbits/dylint) and expanded the list of vulnerability classes, detectors, and test cases. [View Prototype](https://coinfabrik.github.io/scout/) \| [Application Form](https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik_2.md). |
| ![Aleph Zero](https://raw.githubusercontent.com/CoinFabrik/scout/main/assets/aleph-zero.png) | We improved the precision and number of detectors for the tool with a multi-phase approach. This included a manual vulnerability analysis of projects within the Aleph Zero ecosystem, comprehensive testing of the tool on leading projects, and refining its detection accuracy. |

## About CoinFabrik

We - [CoinFabrik](https://www.coinfabrik.com/) - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 180 blockchain-related projects, EVM based and also for Solana, Algorand, and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity professionals, currently working on code in Substrate, Solidity, Clarity, Rust, and TEAL.

Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.

## License

Scout is licensed and distributed under a MIT license. [Contact us](https://www.coinfabrik.com/) if you're looking for an exception to the terms.
