#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod assert_violation {

    #[ink(storage)]
    pub struct AssertViolation {
        value: u128,
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
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn doesnt_revert_if_greater() {
            let contract = AssertViolation::new(0);
            assert_eq!(contract.assert_if_greater_than_10(5), true);
        }

        #[ink::test]
        #[should_panic(expected = "value should be less than 10")]
        fn reverts_if_greater() {
            let contract = AssertViolation::new(0);
            contract.assert_if_greater_than_10(11);
        }
    }
}
