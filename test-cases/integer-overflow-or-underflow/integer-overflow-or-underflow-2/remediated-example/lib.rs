#![cfg_attr(not(feature = "std"), no_std)]

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

        // Multiply the stored value by the given amount.
        #[ink(message)]
        pub fn mul(&mut self, value: u8) -> Result<(), Error> {
            match self.value.checked_mul(value) {
                Some(v) => self.value = v,
                None => return Err(Error::OverflowError),
            };
            Ok(())
        }

        // Raise the stored value to the power of the given amount.
        #[ink(message)]
        pub fn pow(&mut self, value: u8) -> Result<(), Error> {
            match self.value.checked_pow(value.into()) {
                Some(v) => self.value = v,
                None => return Err(Error::OverflowError),
            };
            Ok(())
        }

        // Negate the stored value.
        #[ink(message)]
        pub fn neg(&mut self) -> Result<(), Error> {
            match self.value.checked_neg() {
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
        fn mul_overflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MAX);
            let value_to_mul = 2;

            // Act
            let result = contract.mul(value_to_mul);

            // Assert
            assert_eq!(result, Err(Error::OverflowError));
        }

        #[ink::test]
        fn pow_overflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MAX);
            let value_to_pow = 2;

            // Act
            let result = contract.pow(value_to_pow);

            // Assert
            assert_eq!(result, Err(Error::OverflowError));
        }

        #[ink::test]
        fn neg_underflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MAX);

            // Act
            let result = contract.neg();

            // Assert
            assert_eq!(result, Err(Error::UnderflowError));
        }
    }
}
