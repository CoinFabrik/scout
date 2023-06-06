#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod panic_error {

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
        pub fn add(&mut self, value: u32) {
            match self.value.checked_add(value) {
                Some(v) => self.value = v,
                None => panic!("Overflow error"),
            };
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
        #[should_panic(expected = "Overflow error")]
        fn add_panics() {
            // Arrange
            let mut contract = PanicError::new(u32::MAX);

            // Act
            contract.add(1);

            // Assert - handled by the `should_panic` attribute
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
