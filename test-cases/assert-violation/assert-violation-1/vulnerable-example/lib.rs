#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod assert_violation {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct AssertViolation {
        value: u128,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Provide a detailed comment on the error
        GreaterThan10,
    }


    impl AssertViolation {
        #[ink(constructor)]
        pub fn new(init_value: u128) -> Self {
            Self { value: init_value }
        }


        #[ink(message)]
        pub fn assert_if_greater_than_10(&self, value: u128) -> bool {
            assert!(value <= 10, "value should be less than 10");
            true
        }

        #[ink(message)]
        pub fn revert_if_greater_than_10(&self, value: u128) -> Result<(), Error> {

            if value <= 10 {
                return Ok(())
            } else {
                return Err(Error::GreaterThan10)
            }

        }


    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {

        }

        #[ink::test]
        fn it_works() {

        }
    }
}
