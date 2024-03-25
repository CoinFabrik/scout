# Ink! version

## Description

- Vulnerability Category: `Best practices`
- Vulnerability Severity: `Enhacement`
- Detectors: [`ink-version`](https://github.com/CoinFabrik/scout/tree/main/detectors/ink-version)
- Test Cases: [`ink-version-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/ink-version/ink-version-1)

Using an old version of ink! can be dangerous, as it may have bugs or security issues. Use the latest version available.

## Exploit Scenario

Consider the following `ink!` contract:

```toml
[dependencies]
    ink = { version = "=4.2.0", default-features = false }
```

Problems can arise if the version is not updated to the latest available.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/ink-version/ink-version-1/vulnerable-example).

## Remediation

Use the latest stable version available.

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/ink-version/ink-version-1/remediated-example).

## References

- [Floating Pragma](https://swcregistry.io/docs/SWC-103/)
- [outdated Compiler Version](https://swcregistry.io/docs/SWC-102/)
