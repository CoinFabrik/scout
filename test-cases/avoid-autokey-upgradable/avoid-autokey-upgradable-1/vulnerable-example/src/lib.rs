#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod avoid_autokey_upgradable {
    use ink::storage::{Lazy, Mapping, StorageVec};

    #[ink(storage)]
    pub struct AvoidAutoKeyUpgradable {
        admin: AccountId,
        lazy_field: Lazy<[u8; 32]>,
        mapping: Mapping<AccountId, u32>,
        vec: StorageVec<u32>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotAnAdmin,
        FailedSetCodeHash,
    }

    impl AvoidAutoKeyUpgradable {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
                lazy_field: Lazy::new(),
                mapping: Mapping::new(),
                vec: StorageVec::new(),
            }
        }

        #[ink(message)]
        pub fn upgrade_contract(&self, value: [u8; 32]) -> Result<(), Error> {
            if self.admin != Self::env().caller() {
                return Err(Error::NotAnAdmin);
            }

            match self.env().set_code_hash(&value.into()) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::FailedSetCodeHash),
            }
        }
    }
}
