# Acceptance Criteria for New Test Cases

This document outlines the acceptance criteria for any new test cases added to the "test-cases" directory. These criteria focus on the structure of the test cases.

## 1. Folder Structure

Each new test case should be stored in its own unique folder under the "test-cases" directory. The folder should be named in a way that clearly indicates the vulnerability it is demonstrating and testing.

## 2. Sub-Folders

Within each test case folder, there should be two sub-folders: "remediated-example" and "vulnerable-example". The names of these sub-folders should be consistent across all test cases.

## 3. File Content

Each "remediated-example" and "vulnerable-example" sub-folder should contain at least one file. The file(s) in the "remediated-example" sub-folder should contain code that has been fixed to address the vulnerability. The file(s) in the "vulnerable-example" sub-folder should contain code that demonstrates the vulnerability.

## 4. Documentation

Each test case folder should contain a README file. This file should provide a clear and concise description of the vulnerability, how the vulnerable code demonstrates this vulnerability, and how the remediated code fixes this vulnerability.

## 5. Uniqueness

Each test case should be unique and not duplicate any existing test cases in the "test-cases" directory. The uniqueness should be maintained at the vulnerability level, meaning each test case should demonstrate a different vulnerability.

Please ensure that all new test cases adhere to these criteria to maintain the organization and clarity of the "test-cases" directory.
