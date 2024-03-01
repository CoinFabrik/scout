#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[ink::contract]
pub mod psp22 {

    use ink::env::DefaultEnvironment;
    use ink::prelude::vec::Vec;
    use openbrush::contracts::psp22::*;
    use openbrush::traits::Storage;
    use PSP22Error;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
    }

    // Section contains default implementation without any modifications
    impl PSP22 for Contract {
        fn total_supply(&self) -> <DefaultEnvironment as ink::env::Environment>::Balance {
            todo!()
        }
        fn balance_of(
            &self,
            _: <DefaultEnvironment as ink::env::Environment>::AccountId,
        ) -> <DefaultEnvironment as ink::env::Environment>::Balance {
            todo!()
        }
        fn allowance(
            &self,
            _: <DefaultEnvironment as ink::env::Environment>::AccountId,
            _: <DefaultEnvironment as ink::env::Environment>::AccountId,
        ) -> <DefaultEnvironment as ink::env::Environment>::Balance {
            todo!()
        }
        fn transfer(
            &mut self,
            _: <DefaultEnvironment as ink::env::Environment>::AccountId,
            _: <DefaultEnvironment as ink::env::Environment>::Balance,
            _: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            todo!()
        }
        fn transfer_from(
            &mut self,
            _: <DefaultEnvironment as ink::env::Environment>::AccountId,
            _: <DefaultEnvironment as ink::env::Environment>::AccountId,
            _: <DefaultEnvironment as ink::env::Environment>::Balance,
            _: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            todo!()
        }
        fn increase_allowance(
            &mut self,
            _: <DefaultEnvironment as ink::env::Environment>::AccountId,
            _: <DefaultEnvironment as ink::env::Environment>::Balance,
        ) -> Result<(), PSP22Error> {
            todo!()
        }
        fn decrease_allowance(
            &mut self,
            _: <DefaultEnvironment as ink::env::Environment>::AccountId,
            _: <DefaultEnvironment as ink::env::Environment>::Balance,
        ) -> Result<(), PSP22Error> {
            todo!()
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
            _instance
        }

        #[ink(message)]
        pub fn test(&self) {
            ()
        }
    }
}
