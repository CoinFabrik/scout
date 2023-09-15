#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod divide_before_multiply {

    #[ink(storage)]
    pub struct DivideBeforeMultiply {}

    impl DivideBeforeMultiply {
        /// Creates a new DivideBeforeMultiply contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Calculates the profit for a given percentage of the total profit.
        #[ink(message)]
        pub fn split_profit(&self, percentage: u64, total_profit: u64) -> u64 {
            (percentage / 100) * total_profit
        }

        // same as split_profit but using checked functions
        #[ink(message)]
        pub fn checked_split_profit(&self, percentage: u64, total_profit: u64) -> Option<u64> {
            percentage.checked_div(100)?.checked_mul(total_profit)
        }

        // same as split_profit but using both normal operations and checked functions
        #[ink(message)]
        pub fn hybrid_split_profit(&self, percentage: u64, total_profit: u64) -> Option<u64> {
            Some(percentage.checked_div(100)? * total_profit)
        }
    }

    impl Default for DivideBeforeMultiply {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn split_profit_precision() {
            let contract = DivideBeforeMultiply::new();
            assert_eq!(contract.split_profit(33, 100), 0);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn split_profit_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = DivideBeforeMultiplyRef::new();

            // When
            let contract_acc_id = client
                .instantiate(
                    "divide-before-multiply",
                    &ink_e2e::bob(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let split_profit = build_message::<DivideBeforeMultiplyRef>(contract_acc_id.clone())
                .call(|contract| contract.split_profit(33, 100));
            let split_profit_res = client
                .call(&ink_e2e::bob(), split_profit, 0, None)
                .await
                .expect("split_profit failed");
            assert_eq!(split_profit_res.return_value(), 0);

            Ok(())
        }
    }
}
