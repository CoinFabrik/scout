#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod vec_could_be_mapping {

    use ink::prelude::vec::Vec;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotFound,
    }
    #[ink(storage)]
    pub struct VecCouldBeMapping {
        balances: Vec<(AccountId, Balance)>,
    }

    impl VecCouldBeMapping {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Vec::new(),
            }
        }
        /// Returns the percentage difference between two values.
        #[ink(message)]
        pub fn get_balance(&mut self, acc: AccountId) -> Result<Balance, Error> {
            self.balances
                .iter()
                .find(|(a, _)| *a == acc)
                .map(|(_, b)| *b)
                .ok_or(Error::NotFound)
        }
    }
}
