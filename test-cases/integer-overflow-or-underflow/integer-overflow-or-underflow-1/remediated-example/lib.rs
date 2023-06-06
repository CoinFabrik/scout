#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::integer_arithmetic)]

#[ink::contract]
mod integer_overflow_underflow {

    #[ink(storage)]
    pub struct IntegerOverflowUnderflow {
        /// Stored value.
        value: u8,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// An overflow was produced while adding
        OverflowError,
        /// An underflow was produced while substracting
        UnderflowError,
    }

    impl IntegerOverflowUnderflow {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new(value: u8) -> Self {
            Self { value }
        }

        /// Increments the stored value by the given amount.
        #[ink(message)]
        pub fn add(&mut self, value: u8) -> Result<(), Error> {
            match self.value.checked_add(value) {
                Some(v) => self.value = v,
                None => return Err(Error::OverflowError),
            };
            Ok(())
        }

        /// Decrements the stored value by the given amount.
        #[ink(message)]
        pub fn sub(&mut self, value: u8) -> Result<(), Error> {
            match self.value.checked_sub(value) {
                Some(v) => self.value = v,
                None => return Err(Error::UnderflowError),
            };
            Ok(())
        }

        /// Returns the stored value.
        #[ink(message)]
        pub fn get(&self) -> u8 {
            self.value
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
            let contract = IntegerOverflowUnderflow::new(value);

            // Assert
            assert_eq!(contract.get(), value);
        }

        #[ink::test]
        fn add_overflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MAX);
            let value_to_add = 1;

            // Act
            let result = contract.add(value_to_add);

            // Assert
            assert_eq!(result, Err(Error::OverflowError));
        }

        #[ink::test]
        fn sub_underflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MIN);
            let value_to_sub = 1;

            // Act
            let result = contract.sub(value_to_sub);

            // Assert
            assert_eq!(result, Err(Error::UnderflowError));
        }
    }
}
