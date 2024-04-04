#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod vec_considerations {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[derive(Default)]
    #[ink(storage)]
    pub struct VecConsiderations {
        on_chain_log: Mapping<AccountId, String>,
    }
    
    impl VecConsiderations {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn do_something(&mut self, data: String) {
            let caller = self.env().caller();

            let log_message = format!("{caller:?}: {data}");

            // Panics if data overgrows the static buffer size!
            self.on_chain_log.insert(caller, &data);
        }
    }
}
