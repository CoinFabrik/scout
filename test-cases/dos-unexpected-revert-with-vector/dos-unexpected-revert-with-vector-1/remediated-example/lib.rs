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
