#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod set_code_hash {
    #[ink(storage)]
    pub struct SetCodeHash {
        admin: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidCodeHash,
        NotAnAdmin,
    }

     impl Default for SetCodeHash {
         fn default() -> Self {
             Self::new()
         }
     }

    impl SetCodeHash {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn update_code(&self, value: Hash) -> Result<(), Error> {
            if self.admin != self.env().caller() {
                return Err(Error::NotAnAdmin);
            }

            let res = Self::env().set_code_hash(&value);

            if res.is_err() {
                return res.map_err(|_| Error::InvalidCodeHash);
            }

            Ok(())
        }
    }
}
