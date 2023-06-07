#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod unused_return_enum {

    #[ink(storage)]
    pub struct UnusedReturnEnum {}

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// An overflow was produced.
        Overflow,
    }

    impl UnusedReturnEnum {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Returns the percentage difference between two values.
        #[ink(message)]
        pub fn get_percentage_difference(
            &mut self,
            value1: Balance,
            value2: Balance,
        ) -> Result<Balance, Error> {
            let absolute_difference = value1.abs_diff(value2);
            let sum = value1 + value2;
            let _percentage_difference = match 100u128.checked_mul(absolute_difference / sum) {
                Some(result) => result,
                None => panic!("overflow!"),
            };
            return Err(Error::Overflow);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn get_percentage_difference_panics() {
            // Arrange
            let mut contract = UnusedReturnEnum::new();
            let value1 = 100;
            let value2 = 150;

            // Act
            let result = contract.get_percentage_difference(value1, value2);

            // Assert
            assert_eq!(result, Err(Error::Overflow));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn add_panics(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let constructor = UnusedReturnEnumRef::new();
            let contract_acc_id = client
                .instantiate(
                    "unused-return-enum",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            // Act
            let value1 = 100;
            let value2 = 150;
            let get_percentage_difference =
                build_message::<UnusedReturnEnumRef>(contract_acc_id.clone())
                    .call(|contract| contract.get_percentage_difference(value1, value2));
            let result = client
                .call(&ink_e2e::alice(), get_percentage_difference, 0, None)
                .await;

            // Assert
            assert!(result.is_err());

            Ok(())
        }
    }
}
