#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod avoid_format {

    #[ink(storage)]
    pub struct AvoidFormat {
        value: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        FormatError { msg: String },
    }

    impl AvoidFormat {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn crash(&self) -> Result<(), Error> {
            Err(Error::FormatError {
                msg: (format!("{}", self.value)),
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn crash_works() {
            let avoid_format = AvoidFormat::new(false);
            assert_eq!(
                avoid_format.crash(),
                Err(Error::FormatError {
                    msg: "false".to_string()
                })
            );
        }
    }
}
