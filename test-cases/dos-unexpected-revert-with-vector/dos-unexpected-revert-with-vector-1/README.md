# DoS Unexpected Revert With Vector
## Description
- Vulnerability Category: `DoS`
- Severity: `High`
- Detector ID: `dos-unexpected-revert`

This vulnerability of DoS through unexpected revert arises when a smart 
contract does not handle storage size errors correctly, and a user can add an
excessive number of entries, leading to an unexpected revert of transactions
by other users and a Denial of Service. This vulnerability can be exploited by
an attacker to perform a DoS attack on the network and can result in lost 
funds, poor user experience, and even harm the network's overall security.

## Exploit Scenario
The vulnerable smart contract we developed for his example allows users to
vote for one of different candidates. 
The smart contract contains a struct named `UnexpectedRevert` that stores the
total number of votes, a list of candidates, their votes, and whether an 
account has voted. It also stores information about the most voted candidate
and when the vote will end.

```rust
#[ink(storage)]
pub struct UnexpectedRevert {
    /// Total votes performed.
    total_votes: u64,
    /// List of candidates.
    candidates: Vec<AccountId>,
    /// Votes for each candidate.
    votes: Mapping<AccountId, u64>,
    /// Accounts that already voted.
    already_voted: Mapping<AccountId, bool>,
    /// Account id of the most voted candidate.
    most_voted_candidate: AccountId,
    /// Votes of the most voted candidate.
    candidate_votes: u64,
    /// Timestamp when the vote ends.
    vote_timestamp_end: u64,
}
```

The smart contract has several functions that allow adding a candidate, getting
votes for a specific candidate, getting the account ID of the most voted 
candidate, getting the total votes, getting the total number of candidates,
getting a candidate by index, checking if an account has voted, and voting for
a candidate.

The #[cfg(test)] block contains a single test that adds 512 candidates to the
smart contract. It initializes the contract with the current timestamp + 10 
minutes and then uses a loop to add each candidate. The test verifies that 
the function to add a candidate fails with an error indicating that the vote
has ended. The purpose of this test is to trigger an unexpected revert due 
to the contract's storage size, but this does not occur since the deployment
is mocked and does not check the size of storage cells.

On the other hand, the end to end test instantiates the contract using 
`ink_e2e::alice()` as the deployer account and an `UnexpectedRevertRef` 
instance with a specified now value. It then uses a loop to add 512 candidates
to the contract by calling the add_candidate function for each candidate 
account.

The loop generates a unique `AccountId` for each candidate by creating a 
vector of 28 zeroes, appending the current index as a big-endian byte array,
and converting the resulting vector to a fixed-length array of 32 bytes.

The test expects the `add_candidate()` function to fail with a `CallDryRun` error,
which indicates that the transaction execution failed during a dry run. This 
is indicated by the `#[should_panic(expected = "add_candidate failed: CallDryRun")]` 
attribute on the test function. This test does trigger an unexpected revert 
due to the contract's storage size.

### Deployment (of the vulnerable contract)
In order to run the tests associated to this contract and view this 
vulnerability in action:
1. Save the `vulnerable-example` directory.
2. Run a substrate node and save its `FULL_PATH`.
3. Open a new terminal at the `vulnerable-example` directory and set the 
contract node environmental variable by running:
    `export CONTRACTS_NODE=[FULL_PATH]`
4. Finally, run the test with:
    `cargo test --features e2e-tests`

You should see that the vulnerability is not realized for the integration test
since the deployment is mocked and does not check the size of storage cells, 
but it is for the e2e-test.

```console
user:/mnt/c/user/docs/candidates/unexpected_revert/vulnerable-example$ cargo test --features e2e-tests
    Updating crates.io index
  Downloaded toml_edit v0.19.7

  ...

   Compiling metadata-gen v0.1.0 (/tmp/cargo-contract_5urX7T/.ink/metadata_gen)
    Finished release [optimized] target(s) in 1m 06s
     Running `target/ink/release/metadata-gen`
 [5/5] Generating bundle
    Finished test [unoptimized + debuginfo] target(s) in 15m 02s
     Running unittests lib.rs (target/debug/deps/unexpected_revert-feae385052f36b92)

running 2 tests
test unexpected_revert::tests::insert_512_candidates ... ok
test unexpected_revert::e2e_tests::insert_512_candidates - should panic ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 59.94s

   Doc-tests unexpected-revert

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## Remediation
In order to prevent this vulnerability we discourage the use of `Vec` and 
propose the use of `Mapping` in order to avoid storage limits.

```rust
#[ink(storage)]
pub struct UnexpectedRevert {
    /// Total votes performed.
    total_votes: u64,
    /// Total candidates.
    total_candidates: u64,
    /// List of candidates.
    candidates: Mapping<u64, AccountId>,
    /// Votes for each candidate.
    votes: Mapping<AccountId, u64>,
    /// Accounts that already voted.
    already_voted: Mapping<AccountId, bool>,
    /// Account id of the most voted candidate.
    most_voted_candidate: AccountId,
    /// Votes of the most voted candidate
    candidate_votes: u64,
    /// Timestamp when the vote ends.
    vote_timestamp_end: u64,
}
```

### Deployment (of the remediated contract)
In order to run the tests associated to this remediated contract in action:
1. Save the `remediated-example` directory.
2. Run a substrate node and save its `FULL_PATH`.
3. Open a new terminal at the `vulnerable-example` directory and set the contract node environmental variable by running:
    `export CONTRACTS_NODE=[FULL_PATH]`
4. Finally, run the test with:
    `cargo test --features e2e-tests`

You should see that the vulnerability is not realized for any of the tests.

```rust
user:/mnt/c/user/docs/candidates/unexpected_revert/remediated-example$ cargo test --features e2e-tests
    Updating crates.io index
    Compiling unicode-ident v1.0.8

    ...

 [5/5] Generating bundle
    Finished test [unoptimized + debuginfo] target(s) in 14m 24s
     Running unittests lib.rs (target/debug/deps/unexpected_revert-feae385052f36b92)

running 2 tests
test unexpected_revert::tests::insert_512_candidates ... ok
test unexpected_revert::e2e_tests::insert_512_candidates ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 46.06s

   Doc-tests unexpected-revert

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## References
- [SWC-113](https://swcregistry.io/docs/SWC-113)
- https://consensys.github.io/smart-contract-best-practices/attacks/denial-of-service/#dos-with-unexpected-revert
- [Ethernaut: King](https://ethernaut.openzeppelin.com/level/0x43BA674B4fbb8B157b7441C2187bCdD2cdF84FD5)
