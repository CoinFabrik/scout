#![allow(clippy::new_without_default)]
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod floating_point_and_numerical_precision {

    #[ink(storage)]
    pub struct FloatingPointAndNumericalPrecision {}

    impl FloatingPointAndNumericalPrecision {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn split_profit(&self, percentage: u64, total_profit: u64) -> u64 {
            (percentage * total_profit) / 100
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn split_profit_precision() {
            let contract = FloatingPointAndNumericalPrecision::new();
            assert_eq!(contract.split_profit(33, 100), 33);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn split_profit_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FloatingPointAndNumericalPrecisionRef::new();

            // When
            let contract_acc_id = client
                .instantiate(
                    "floating_point_and_numerical_precision",
                    &ink_e2e::bob(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let split_profit =
                build_message::<FloatingPointAndNumericalPrecisionRef>(contract_acc_id.clone())
                    .call(|floating_point_and_numerical_precision| {
                        floating_point_and_numerical_precision.split_profit(33, 100)
                    });
            let split_profit_res = client
                .call(&ink_e2e::bob(), split_profit, 0, None)
                .await
                .expect("split_profit failed");
            assert_eq!(split_profit_res.return_value(), 33);

            Ok(())
        }
    }
}
