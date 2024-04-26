#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod buffering_unsized_types {
    use ink::prelude::{format, string::String};
    use ink::storage::{Mapping, StorageVec};

    #[derive(Default)]
    #[ink(storage)]
    pub struct BufferingUnsizedTypes {
        on_chain_log: Mapping<AccountId, String>,
        donations: StorageVec<String>,
        test: Mapping<AccountId, Balance>,
    }

    impl BufferingUnsizedTypes {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn do_something(&mut self, data: String) {
            let caller = self.env().caller();
            let example = format!("{caller:?}: {data}");
            // Panics if data overgrows the static buffer size!
            self.on_chain_log.insert(caller, &example);
        }

        #[ink(message)]
        pub fn donate(&mut self) {
            let caller = self.env().caller();
            let endowment = self.env().transferred_value();

            let log_message = format!("{caller:?} donated {endowment}");
            self.donations.push(&log_message);
        }

        #[ink(message)]
        pub fn last_donation(&self) -> Option<String> {
            self.donations.peek()
        }

        #[ink(message)]
        pub fn shouldnt_turn_on(&mut self, person: AccountId, balance: Balance) {
            self.test.insert(person, &balance);
        }
    }
}
