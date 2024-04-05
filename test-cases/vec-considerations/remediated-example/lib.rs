#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod vec_considerations {

    use ink::prelude::{string::String, format};
    use ink::storage::{StorageVec, Mapping};

    #[derive(Default)]
    #[ink(storage)]
    pub struct MyContract {
        on_chain_log: Mapping<AccountId, String>,
        donations: StorageVec<String>,

    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsertFailed,
        PeekFailed,
        PushFailed,
        ErrNone,
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

        #[ink(message)]
        pub fn donate(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            let endowment = self.env().transferred_value();

            let log_message = format!("{caller:?} donated {endowment}");
            
            match self.donations.try_push(&log_message) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::PushFailed)
            }
        }

        #[ink(message)]
        pub fn last_donation(&self) -> Result<(), Error>{
            
            match self.donations.try_peek() {
                Some(Ok(_)) => Ok(()),
                Some(Err(_)) => Err(Error::PeekFailed),
                None => Err(Error::ErrNone),
            }
        }

    }
}
