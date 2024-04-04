#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod vec_considerations {

    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[derive(Default)]
    #[ink(storage)]
    pub struct MyContract {
        on_chain_log: Mapping<AccountId, String>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsertFailed,
    }


    impl MyContract {

        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn do_something2(&mut self, data: String) -> Result<(), Error> {
            let caller = self.env().caller();
    
            match self.on_chain_log.try_insert(caller, &data) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::InsertFailed)
            }
        }

    }
}
