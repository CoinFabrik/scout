# DoS unexpected revert with vector

## Description

- Vulnerability Category: `DoS`
- Severity: `Critical`
- Detectors: [`dos-unexpected-revert-with-vector`](https://github.com/CoinFabrik/scout/tree/main/detectors/dos-unexpected-revert-with-vector)
- Test Cases: [`dos-unexpected-revert-with-vector-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/dos-unexpected-revert-with-vector/dos-unexpected-revert-with-vector-1)

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
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod unexpected_revert {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

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

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Errors {
        /// Account already voted.
        AccountAlreadyVoted,
        /// Candidate already added.
        CandidateAlreadyAdded,
        /// Candidate doesn't exist.
        CandidateDoesntExist,
        /// Overflow was detected.
        Overflow,
        /// Timestamp before current block.
        TimestampBeforeCurrentBlock,
        /// Vote ended.
        VoteEnded,
    }

    impl UnexpectedRevert {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(end_timestamp: u64) -> Result<Self, Errors> {
            if end_timestamp <= Self::env().block_timestamp() {
                return Err(Errors::TimestampBeforeCurrentBlock);
            }

            let zero_arr: [u8; 32] = [0; 32];
            let zero_addr = AccountId::from(zero_arr);
            Ok(Self {
                total_votes: 0,
                most_voted_candidate: zero_addr,
                candidate_votes: 0,
                candidates: Vec::default(),
                already_voted: Mapping::default(),
                votes: Mapping::default(),
                vote_timestamp_end: end_timestamp,
            })
        }

        /// Adds a candidate to this vote.
        #[ink(message)]
        pub fn add_candidate(&mut self, candidate: AccountId) -> Result<(), Errors> {
            if self.vote_ended() {
                return Err(Errors::VoteEnded);
            }
            if self.votes.contains(candidate) {
                Err(Errors::CandidateAlreadyAdded)
            } else {
                self.candidates.push(candidate);
                self.votes.insert(candidate, &0);
                Ok(())
            }
        }

        /// Returns votes for a given candidate.
        #[ink(message)]
        pub fn get_votes_for_a_candidate(&self, candidate: AccountId) -> Result<u64, Errors> {
            let votes_opt = self.votes.get(candidate);
            if votes_opt.is_none() {
                Err(Errors::CandidateDoesntExist)
            } else {
                Ok(votes_opt.unwrap())
            }
        }

        /// Returns votes for the most voted candidate.
        #[ink(message)]
        pub fn most_voted_candidate_votes(&self) -> u64 {
            self.candidate_votes
        }

        /// Returns account id for the most voted candidate.
        #[ink(message)]
        pub fn most_voted_candidate(&self) -> AccountId {
            self.most_voted_candidate
        }

        /// Returns total votes performed.
        #[ink(message)]
        pub fn get_total_votes(&self) -> u64 {
            self.total_votes
        }

        /// Returns total candidates.
        #[ink(message)]
        pub fn get_total_candidates(&self) -> u64 {
            self.candidates.len() as u64
        }

        /// Returns candidate at index.
        #[ink(message)]
        pub fn get_candidate(&self, index: u64) -> Result<AccountId, Errors> {
            if (index as usize) < self.candidates.len() {
                Ok(self.candidates[index as usize])
            } else {
                Err(Errors::CandidateDoesntExist)
            }
        }

        /// Returns true if the account has already voted.
        #[ink(message)]
        pub fn account_has_voted(&self, account: AccountId) -> bool {
            self.already_voted.get(account).unwrap_or(false)
        }

        /// Vote for one of the candidates.
        #[ink(message)]
        pub fn vote(&mut self, candidate: AccountId) -> Result<(), Errors> {
            if self.vote_ended() {
                return Err(Errors::VoteEnded);
            }
            let caller = self.env().caller();
            if self.already_voted.contains(caller) {
                Err(Errors::AccountAlreadyVoted)
            } else {
                self.already_voted.insert(caller, &true);
                let votes = self
                    .votes
                    .get(candidate)
                    .ok_or(Errors::CandidateDoesntExist)?
                    .checked_add(1)
                    .ok_or(Errors::Overflow)?;
                self.votes.insert(candidate, &votes);
                self.total_votes.checked_add(1).ok_or(Errors::Overflow)?;
                if self.candidate_votes < votes {
                    self.candidate_votes = votes;
                    self.most_voted_candidate = candidate;
                }
                Ok(())
            }
        }

        /// Returns true if the vote has ended.
        #[ink(message)]
        pub fn vote_ended(&self) -> bool {
            self.vote_timestamp_end <= self.env().block_timestamp()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::time::SystemTime;

        #[ink::test]
        fn insert_512_candidates() {
            let now: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => (n.as_secs() + 10 * 60) * 1000,
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            };
            let mut contract = UnexpectedRevert::new(now).unwrap();

            let mut candidate: Result<(), Errors> = Err(Errors::VoteEnded);
            for i in 0u32..512 {
                let mut zero_vec = vec![0u8; 28];
                zero_vec.extend(i.to_be_bytes().iter().cloned());
                let arr: [u8; 32] = match zero_vec.as_slice().try_into() {
                    Ok(arr) => arr,
                    Err(_) => panic!(),
                };
                let addr = AccountId::from(arr);
                candidate = contract.add_candidate(addr);
                assert_eq!(contract.get_total_candidates(), (i + 1) as u64);
            }

            assert_eq!(contract.get_total_candidates(), 512u64);
            assert_eq!(candidate.is_ok(), true);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;
        use std::time::SystemTime;

        #[ink_e2e::test]
        #[should_panic(expected = "add_candidate failed: CallDryRun")]
        async fn insert_512_candidates(mut client: ink_e2e::Client<C, E>) {
            let now: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => (n.as_secs() + 10 * 60) * 1000,
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            };
            let constructor = UnexpectedRevertRef::new(now);
            let contract_acc_id = client
                .instantiate("unexpected-revert", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            for i in 0u32..512 {
                let mut zero_vec = vec![0u8; 28];
                zero_vec.extend(i.to_be_bytes().iter().cloned());
                let arr: [u8; 32] = match zero_vec.as_slice().try_into() {
                    Ok(arr) => arr,
                    Err(_) => panic!(),
                };
                let addr = AccountId::from(arr);

                let add_candidate = build_message::<UnexpectedRevertRef>(contract_acc_id.clone())
                    .call(|contract| contract.add_candidate(addr));
                client
                    .call(&ink_e2e::bob(), add_candidate.clone(), 0, None)
                    .await
                    .expect("add_candidate failed");
            }
        }
    }
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
attribute on the test function. This test _does_ trigger an unexpected revert
due to the contract's storage size.

The vulnerable code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/dos-unexpected-revert-with-vector/dos-unexpected-revert-with-vector-1/vulnerable-example/lib.rs).

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

```bash
$ cargo test --features e2e-tests

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
propose the use of `Mapping` in order to avoid storage limits in the list of candidates.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod unexpected_revert {
    use ink::storage::Mapping;
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

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Errors {
        /// Account already voted.
        AccountAlreadyVoted,
        /// Candidate already added.
        CandidateAlreadyAdded,
        /// Candidate doesn't exist.
        CandidateDoesntExist,
        /// Overflow was detected.
        Overflow,
        /// Timestamp before current block.
        TimestampBeforeCurrentBlock,
        /// Vote ended.
        VoteEnded,
    }

    impl UnexpectedRevert {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(end_timestamp: u64) -> Result<Self, Errors> {
            if end_timestamp <= Self::env().block_timestamp() {
                return Err(Errors::TimestampBeforeCurrentBlock);
            }
            let zero_arr: [u8; 32] = [0; 32];
            let zero_addr = AccountId::from(zero_arr);
            Ok(Self {
                total_votes: 0,
                total_candidates: 0,
                most_voted_candidate: zero_addr,
                candidate_votes: 0,
                candidates: Mapping::default(),
                already_voted: Mapping::default(),
                votes: Mapping::default(),
                vote_timestamp_end: end_timestamp,
            })
        }

        /// Add a candidate to this vote
        #[ink(message)]
        pub fn add_candidate(&mut self, candidate: AccountId) -> Result<(), Errors> {
            if self.vote_ended() {
                return Err(Errors::VoteEnded);
            }
            if self.votes.contains(candidate) {
                Err(Errors::CandidateAlreadyAdded)
            } else {
                self.candidates.insert(self.total_candidates, &candidate);
                self.total_candidates += 1;
                self.votes.insert(candidate, &0);
                Ok(())
            }
        }

        /// Returns votes for a given candidate.
        #[ink(message)]
        pub fn get_votes_for_a_candidate(&self, candidate: AccountId) -> Result<u64, Errors> {
            let votes_opt = self.votes.get(candidate);
            if votes_opt.is_none() {
                Err(Errors::CandidateDoesntExist)
            } else {
                Ok(votes_opt.unwrap())
            }
        }

        /// Returns votes for the most voted candidate.
        #[ink(message)]
        pub fn most_voted_candidate_votes(&self) -> u64 {
            self.candidate_votes
        }

        /// Returns account id for the most voted candidate.
        #[ink(message)]
        pub fn most_voted_candidate(&self) -> AccountId {
            self.most_voted_candidate
        }

        /// Returns total votes performed.
        #[ink(message)]
        pub fn get_total_votes(&self) -> u64 {
            self.total_votes
        }

        /// Returns total candidates.
        #[ink(message)]
        pub fn get_total_candidates(&self) -> u64 {
            self.total_candidates
        }

        /// Returns candidate at index.
        #[ink(message)]
        pub fn get_candidate(&self, index: u64) -> Result<AccountId, Errors> {
            match self.candidates.get(index) {
                Some(candidate) => Ok(candidate),
                None => Err(Errors::CandidateDoesntExist),
            }
        }

        /// Returns true if the account has already voted.
        #[ink(message)]
        pub fn account_has_voted(&self, account: AccountId) -> bool {
            self.already_voted.get(account).unwrap_or(false)
        }

        /// Vote for one of the candidates.
        #[ink(message)]
        pub fn vote(&mut self, candidate: AccountId) -> Result<(), Errors> {
            if self.vote_ended() {
                return Err(Errors::VoteEnded);
            }
            let caller = self.env().caller();
            if self.already_voted.contains(caller) {
                Err(Errors::AccountAlreadyVoted)
            } else {
                self.already_voted.insert(caller, &true);
                let votes = self
                    .votes
                    .get(candidate)
                    .ok_or(Errors::CandidateDoesntExist)?
                    .checked_add(1)
                    .ok_or(Errors::Overflow)?;
                self.votes.insert(candidate, &votes);
                self.total_votes.checked_add(1).ok_or(Errors::Overflow)?;
                if self.candidate_votes < votes {
                    self.candidate_votes = votes;
                    self.most_voted_candidate = candidate;
                }
                Ok(())
            }
        }

        /// Returns true if the vote has ended.
        #[ink(message)]
        pub fn vote_ended(&self) -> bool {
            self.vote_timestamp_end <= self.env().block_timestamp()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::time::SystemTime;

        #[ink::test]
        fn insert_512_candidates() {
            let now: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => (n.as_secs() + 10 * 60) * 1000,
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            };
            let mut contract = UnexpectedRevert::new(now).unwrap();

            let mut candidate: Result<(), Errors> = Err(Errors::VoteEnded);
            for i in 0u32..512 {
                let mut zero_vec = vec![0u8; 28];
                zero_vec.extend(i.to_be_bytes().iter().cloned());
                let arr: [u8; 32] = match zero_vec.as_slice().try_into() {
                    Ok(arr) => arr,
                    Err(_) => panic!(),
                };
                let addr = AccountId::from(arr);
                candidate = contract.add_candidate(addr);
                assert_eq!(contract.get_total_candidates(), (i + 1) as u64);
            }

            assert_eq!(contract.get_total_candidates(), 512u64);
            assert_eq!(candidate.is_ok(), true);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;
        use std::time::SystemTime;

        #[ink_e2e::test]
        async fn insert_512_candidates(mut client: ink_e2e::Client<C, E>) {
            let now: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => (n.as_secs() + 10 * 60) * 1000,
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            };
            let constructor = UnexpectedRevertRef::new(now);
            let contract_acc_id = client
                .instantiate("unexpected-revert", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            for i in 0u32..512 {
                let mut zero_vec = vec![0u8; 28];
                zero_vec.extend(i.to_be_bytes().iter().cloned());
                let arr: [u8; 32] = match zero_vec.as_slice().try_into() {
                    Ok(arr) => arr,
                    Err(_) => panic!(),
                };
                let addr = AccountId::from(arr);

                let add_candidate = build_message::<UnexpectedRevertRef>(contract_acc_id.clone())
                    .call(|contract| contract.add_candidate(addr));
                client
                    .call(&ink_e2e::bob(), add_candidate.clone(), 0, None)
                    .await
                    .expect("add_candidate failed");
            }
            let get_total_candidates =
                build_message::<UnexpectedRevertRef>(contract_acc_id.clone())
                    .call(|contract| contract.get_total_candidates());
            let candidates_count = client
                .call(&ink_e2e::bob(), get_total_candidates.clone(), 0, None)
                .await
                .expect("candidates_count failed");
            assert_eq!(candidates_count.return_value(), 512);
        }
    }
}
```

The remediated code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/dos-unexpected-revert-with-vector/dos-unexpected-revert-with-vector-1/remediated-example/lib.rs).

### Deployment (of the remediated contract)

In order to run the tests associated to this remediated contract in action:

1. Save the `remediated-example` directory.
2. Run a substrate node and save its `FULL_PATH`.
3. Open a new terminal at the `vulnerable-example` directory and set the contract node environmental variable by running:
   `export CONTRACTS_NODE=[FULL_PATH]`
4. Finally, run the test with:
   `cargo test --features e2e-tests`

You should see that the vulnerability is not realized for any of the tests.

```bash
$ cargo test --features e2e-tests

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
