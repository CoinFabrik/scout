#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod set_code_hash {
    use ink::env::set_code_hash;

    #[ink(storage)]
    pub struct SetCodeHash {}

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidCodeHash,
    }

    impl SetCodeHash {
        #[ink(constructor)]
        pub fn constr() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn update_code(&self, value: [u8; 32]) -> Result<(), Error> {
            let res = set_code_hash(&value);

            if res.is_err() {
                return res.map_err(|_| Error::InvalidCodeHash);
            }

            Ok(())
        }
    }
}
