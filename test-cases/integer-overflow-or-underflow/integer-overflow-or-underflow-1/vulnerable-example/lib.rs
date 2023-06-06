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

        /// Returns the stored value.
        #[ink(message)]
        pub fn get(&self) -> u8 {
            self.value
        }
    }

    #[cfg(feature = "std")]
    pub mod test_utils {
        use super::*;

        pub fn constructor_works(initial_value: u8) -> IntegerOverflowUnderflow {
            // Arrange
            // Act
            let contract = IntegerOverflowUnderflow::new(initial_value);

            // Assert
            assert_eq!(contract.get(), initial_value);

            contract
        }

        pub fn add_overflows(initial_value: u8, value_to_add: u8) -> IntegerOverflowUnderflow {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(initial_value);

            // Act
            contract.add(value_to_add);

            // Assert
            assert_eq!(contract.get(), initial_value.wrapping_add(value_to_add));

            contract
        }

        pub fn sub_underflows(initial_value: u8, value_to_sub: u8) -> IntegerOverflowUnderflow {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(initial_value);

            // Act
            contract.sub(value_to_sub);

            // Assert
            assert_eq!(contract.get(), initial_value.wrapping_sub(value_to_sub));

            contract
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            test_utils::constructor_works(42);
        }

        #[ink::test]
        fn add_overflows() {
            test_utils::add_overflows(u8::MAX, 1);
        }

        #[ink::test]
        fn sub_underflows() {
            test_utils::sub_underflows(u8::MIN, 1);
        }
    }
}
