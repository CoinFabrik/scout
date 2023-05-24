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

Scout is currently available only for Linux/Mac. For a quick start, install Scout by running the following commands:

```bash
cargo install cargo-scout
cargo-dylint dylint-link
```

To run Scout on your project, navigate to its root directory and execute the following command:

```bash
cargo scout
```

If you're using Windows, you can still run Scout by installing the [Windows Subsistem for Linux](https://learn.microsoft.com/en-us/windows/wsl/install) and using Bash.


For more information on installation and usage, please refer to the [Getting Started](http://localhost:3000/docs/intro) section in our documentation below.

## Documentation

* [Getting Started](http://localhost:3000/docs/intro)
* [Vulnerabilities](http://localhost:3000/docs/vulnerabilities)
* [Detectors](http://localhost:3000/docs/detectors)
* [Learn](http://localhost:3000/docs/Learn)
* [Tutorials](http://localhost:3000/docs/tutorials)
* [Contribute](http://localhost:3000/docs/contribute)
* [FAQs](http://localhost:3000/docs/faqs)
* [Blog](http://localhost:3000/blog)


Visit [Scout's website](http://localhost:3000/) to view the full documentation.


## Detectors
| Detector ID                   | Category                       | Description                                                                                                                                                                                        | Severity      | 
| ----------------------------- | ------------------------------ | ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | 
| integer-overflow-or-underflow | Arithmetic                     | [An arithmetic operation overflows or underflows the available memory allocated to the variable.](./vulnerabilities/examples/integer-overflow-or-underflow/README.md) | Critical          |
| set-contract-storage          | Authorization                  |  [Insufficient access control on set_contract_storage() function.](./vulnerabilities/examples/set-contract-storage/README.md)                                          | Critical          |
| reentrancy                    | Reentrancy                     | [Consistency of contract state under recursive calls.](./vulnerabilities/examples/reentrancy/README.md)                                                               | Critical          |
| panic-error                   | Validations and error handling |  [Code panics on error instead of using descriptive enum.](./vulnerabilities/examples/panic-error/README.md)                                                            | Enhancement |
| unused-return-enum            | Validations and error handling |  [Return enum from a function is not completely used.](./vulnerabilities/examples/unused-return-enum/README.md)                                                         | Minor           |
| dos-unbounded-operation       | Denial of Service               | [DoS due to unbounded operation.](./vulnerabilities/examples/dos-unbounded-operation/README.md)                                                    | Medium          |
| dos-unexpected-revert-with-vector         | Denial of Service              |  [DoS due to improper storage.](./vulnerabilities/examples/dos-unexpected-revert-with-vector/README.md)                                                                                | Medium   

## About CoinFabrik

We - [CoinFabrik](https://www.coinfabrik.com/) - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 180 blockchain-related projects, EVM based and also for Solana, Algorand, and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity, currently working on code in Substrate, Solidity, Clarity, Rust, and TEAL.

Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.

## License

Scout is licensed and distributed under a MIT license. [Contact us](https://www.coinfabrik.com/) if you're looking for an exception to the terms.