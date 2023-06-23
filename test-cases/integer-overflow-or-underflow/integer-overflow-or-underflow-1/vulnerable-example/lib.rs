#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod integer_overflow_underflow {

    #[ink(storage)]
    pub struct IntegerOverflowUnderflow {
        /// Stored value.
        value: u8,
    }

    impl IntegerOverflowUnderflow {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new(value: u8) -> Self {
            Self { value }
        }

        /// Increments the stored value by the given amount.
        #[ink(message)]
        pub fn add(&mut self, value: u8) {
            self.value += value;
        }

        /// Decrements the stored value by the given amount.
        #[ink(message)]
        pub fn sub(&mut self, value: u8) {
            self.value -= value;
        }

        // Multiply the stored value by the given amount.
        #[ink(message)]
        pub fn mul(&mut self, value: u8) {
            self.value *= value;
        }

        // Raise the stored value to the power of the given amount.
        #[ink(message)]
        pub fn pow(&mut self, value: u8) {
            self.value = self.value.pow(value.into());
        }

        // Negate the stored value.
        #[ink(message)]
        pub fn neg(&mut self) {
            self.value = self.value.wrapping_neg();
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
            contract.add(value_to_add);

            // Assert
            assert_eq!(contract.get(), u8::MAX.wrapping_add(value_to_add));
        }

        #[ink::test]
        fn sub_underflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MIN);
            let value_to_sub = 1;

            // Act
            contract.sub(value_to_sub);

            // Assert
            assert_eq!(contract.get(), u8::MIN.wrapping_sub(value_to_sub));
        }

        #[ink::test]
        fn mul_overflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MAX);
            let value_to_mul = 2;

            // Act
            contract.mul(value_to_mul);

            // Assert
            assert_eq!(contract.get(), u8::MAX.wrapping_mul(value_to_mul));
        }

        #[ink::test]
        fn pow_overflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MAX);
            let value_to_pow = 2;

            // Act
            contract.pow(value_to_pow);

            // Assert
            assert_eq!(contract.get(), u8::MAX.wrapping_pow(value_to_pow.into()));
        }

        #[ink::test]
        fn neg_overflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MIN);

            // Act
            contract.neg();

            // Assert
            assert_eq!(contract.get(), u8::MIN.wrapping_neg());
        }
    }
}
