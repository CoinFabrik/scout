#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::delegate_call_delegated::{DelegateCallDelegated, DelegateCallDelegatedRef};

#[ink::contract]
mod delegate_call_delegated {
    use ink::storage::{traits::*, Lazy};

    #[ink(storage)]
    pub struct DelegateCallDelegated {
        //var1: Lazy<u128, ManualKey<0xcafebabe> >
        var1: u128
    }

    impl DelegateCallDelegated {
        /// Creates a new instance of the contract
        #[ink(constructor)]
        pub fn new(
            var1: u128,
        ) -> Self {
            Self { var1: var1 }
            /*let mut instance = Self::default();
            instance.var1.set(&var1);
            instance*/
        }

        #[ink(message)]
        pub fn delegate(&mut self, value: u128) -> u128 {
            //self.var1.set(&value);
            //self.var1.get().unwrap()
            ink::env::set_contract_storage(&0u32, &value);
            ink::env::get_contract_storage(&0u32).unwrap().unwrap()
        }

        #[ink(message)]
        pub fn codehash(&mut self) -> Hash {
            self.env().own_code_hash().unwrap()
        }
    }
}
