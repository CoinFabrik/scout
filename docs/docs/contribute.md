---
sidebar_position: 4
---

# Contribute

Thank you for your interest in contributing to the development of new detectors, test cases, or vulnerability classes for the Scout project. This document outlines the guidelines for contributing to these areas of the project.

### Getting Starting

Create a new issue explaining your contribution and link a new branch to your issue. 

If your detector or test-case does not belong to an existing vulnerability class, please include documentation of the new vulnerability class as specified in the respective section below. 

You may also contribute a new detector or test-case for an existing vulnerability class. In this case only pay attention to the contribution guidelines for new detectors and test-cases.

Once you are finished with the sections below, please remember to update the Detectors table in the main `README.md` file by adding a new row with information about the new detector or test-case. Please do this before performing your pull request.


### Detectors

To contribute a new detector, please follow these steps:

1. Create a new readme file in the [`docs/docs/detectors`](https://github.com/CoinFabrik/scout/tree/main/docs/docs/detectors) folder with the name `<NUMBER>-<VULNERABILITY_NAME>.md`. Replace `<NUMBER>` with the appropriate number for the new detector and `<VULNERABILITY_NAME>` with a descriptive name for the vulnerability class it detects. Provide detailed documentation in the new readme file. Use as a template any of the existing detector documentations and keep the same sections and titles (e.g: [Detector documentation for integer-overflow-or-underflow](https://github.com/CoinFabrik/scout/blob/main/docs/docs/detectors/1-integer-overflow-or-underflow.md)).

2. Add a new folder to [`detectors`](https://github.com/CoinFabrik/scout/tree/main/detectors) using the same `<VULNERABILITY_NAME>` and include all relevant files for the detector in that folder.

### Test Cases

To contribute new test cases for existing vulnerabilities, please follow these steps:

1. Create a new folder in the `test-cases` directory with a descriptive name for the vulnerability and the test case number appended at the end after a hyphen. If the vulnerability already has test cases, add the new test case to the existing folder (e.g: [Reentrancy test-cases](https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy)).

2. Create two sub-folders, one for the `vulnerable-example` and another one for the `remediated-example`. Include the necessary files for the test case.

3. If the test-case belongs to a new vulnerability class, follow first the instructions below. 


### Vulnerability Classes

To contribute a new vulnerability class documentation, please follow these steps:

1. Create a new numbered section at the [bottom of the Vulnerability Classes documentation](https://github.com/CoinFabrik/scout/blob/main/docs/docs/vulnerabilities/README.md#vulnerability-classes) with the name `<NUMBER>-<VULNERABILITY__CLASS_NAME>`. Replace `<NUMBER>` with the appropriate number for the new vulnerability and `<VULNERABILITY_CLASS_NAME>` with a descriptive name.

2. Create a new readme file in the `docs/vulnerabilities` folder with the name `<NUMBER>-<VULNERABILITY_CLASS_NAME>.md`. Replace `<NUMBER>` with the appropriate number for the new vulnerability class and `<VULNERABILITY_CLASS_NAME>` with a descriptive name. Provide detailed documentation in the new readme file. Take as a reference the titles and sections of any of the existing [vulnerability class documetations](https://github.com/CoinFabrik/scout/tree/main/docs/docs/vulnerabilities).

3. Update the number of identified vulnerabilities at the [beginning of the Vulnerability Classes documentation](https://github.com/CoinFabrik/scout/blob/main/docs/docs/vulnerabilities/README.md#vulnerability-classes) to reflect the addition of the new vulnerability class.



