#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod vec_could_be_mapping {

    use ink::storage::Mapping;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotFound,
    }
    #[ink(storage)]
    pub struct VecCouldBeMapping {
        balances: Mapping<AccountId, Balance>,
    }

    impl VecCouldBeMapping {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::new(),
            }
        }
        /// Returns the percentage difference between two values.
        #[ink(message)]
        pub fn get_balance(&mut self, acc: AccountId) -> Result<Balance, Error> {
            self.balances.get(&acc).ok_or(Error::NotFound)
        }
    }
}
