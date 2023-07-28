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
        CrashError,
    }

    impl AvoidFormat {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn crash(&self) -> Result<(), Error> {
            Err(Error::FormatError {
                msg: self.value.to_string(),
            })
        }

        #[ink(message)]
        pub fn crash2(&self) -> Result<(), Error> {
            Err(Error::CrashError)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn crash_works() {
            let avoid_format = AvoidFormat::new(false);
            let result = avoid_format.crash();
            assert_eq!(
                result,
                Err(Error::FormatError {
                    msg: "false".to_string()
                })
            );
        }

        #[test]
        fn crash2_works() {
            let avoid_format = AvoidFormat::new(false);
            let result = avoid_format.crash2();
            assert_eq!(result, Err(Error::CrashError));
        }
    }
}
