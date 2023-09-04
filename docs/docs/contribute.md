---
sidebar_position: 4
---

# Contribute

Thank you for your interest in contributing to the development of new detectors.

### Getting Started

Create a new issue on our [repository](https://github.com/CoinFabrik/scout) with the name of the new detector or test case you wish to contribute. Then, link a new branch to that issue.

If your detector or test case doesn't belong to an existing [vulnerability class](https://coinfabrik.github.io/scout/docs/vulnerabilities#vulnerability-classes), please provide documentation for the new vulnerability class you're proposing.

> :exclamation: **Requirement**: All detectors and test cases should follow the **kebab-case** naming convention, using **lowercase and hyphens** only.

### Detectors

To contribute a new detector:

1. Choose an appropriate template. Browse our templates at [`templates/detectors`](https://github.com/CoinFabrik/scout/tree/main/templates/detectors). Decide on the `early-lint` or `late-lint` template, based on whether you want to lint before or after macro expansion.

2. Add your modified detector files to a new folder, naming it after your detector, inside the [`detectors`](https://github.com/CoinFabrik/scout/tree/main/detectors) directory.

### Test Cases

To contribute a new test case:

1. Determine the [vulnerability class](https://coinfabrik.github.io/scout/docs/vulnerabilities#vulnerability-classes) to which your test case belongs. Then, create a new sub-folder under that class in the [`test-cases`](https://github.com/CoinFabrik/scout/tree/main/test-cases) directory. Remember to append the detector number at the end, separated by a hyphen.

2. Within this sub-folder, create two directories: `vulnerable-example` and `remediated-example`. Fill each with the relevant files for their respective test cases. If possible, incorporate integration or e2e tests. For guidance, refer to the `flipper` template in [`templates/test-case`](https://github.com/CoinFabrik/scout/tree/main/templates/test-case).
