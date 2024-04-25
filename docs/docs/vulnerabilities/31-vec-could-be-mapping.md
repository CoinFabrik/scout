# Vec could be mapping

## Description 
- Vulnerability Category: `Gas usage`
- Vulnerability Severity: `Enhancement`
- Detectors: [`vec-could-be-mapping`](https://github.com/CoinFabrik/scout/tree/main/detectors)
- Test Cases: [`vec-could-be-mapping-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases)

When using a `Vec` to store key-value pairs, it is possible to use a `Mapping` instead. This will reduce the gas usage of the contract, as the `Vec` will have to iterate over all elements to find the desired key-value pair.

## Exploit Scenario



## Remediation





