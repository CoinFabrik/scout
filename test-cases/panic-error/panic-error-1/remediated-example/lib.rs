#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod panic_error {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// An overflow was produced while adding
        OverflowError,
    }

    #[ink(storage)]
    pub struct PanicError {
        /// Stored value.
        value: u32,
    }

    impl PanicError {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new(value: u32) -> Self {
            Self { value }
        }

        /// Increments the stored value by the given amount.
        #[ink(message)]
        pub fn add(&mut self, value: u32) -> Result<(), Error> {
            match self.value.checked_add(value) {
                Some(v) => self.value = v,
                None => return Err(Error::OverflowError),
            };
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            // Arrange
            let value = 42;

            // Act
            let contract = PanicError::new(42);

            // Assert
            assert_eq!(contract.value, value);
        }

        #[ink::test]
        fn add_returns_error() {
            // Arrange
            let mut contract = PanicError::new(u32::MAX);

            // Act
            let result = contract.add(1);

            // Assert
            assert_eq!(result, Err(Error::OverflowError));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn add_returns_error(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let constructor = PanicErrorRef::new(u32::MAX);
            let contract_acc_id = client
                .instantiate("panic-error", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Act
            let add = build_message::<PanicErrorRef>(contract_acc_id.clone())
                .call(|contract| contract.add(1));
            let result = client.call(&ink_e2e::alice(), add, 0, None).await;

            // Assert
            assert!(result.is_err());

            Ok(())
        }
    }
}
