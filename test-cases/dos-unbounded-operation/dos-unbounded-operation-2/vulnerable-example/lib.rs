#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod dos_unbounded_operation_2 {

    /// A payment to be made to an account.
    #[derive(Debug, scale::Decode, scale::Encode, Clone, Copy)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Payee {
        /// The account to which the payment is to be made.
        pub address: AccountId,
        /// The amount to be paid.
        pub value: Balance,
    }

    #[ink(storage)]
    pub struct DosUnboundedOperation {
        /// The payees of the operation.
        payees: Vec<Payee>,
    }

    impl DosUnboundedOperation {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { payees: Vec::new() }
        }

        /// Adds a new payee to the operation.
        #[ink(message, payable)]
        pub fn add_payee(&mut self) -> u128 {
            let address = self.env().caller();
            let value = self.env().transferred_value();
            let new_payee = Payee { address, value };

            self.payees.push(new_payee);

            // Return the index of the new payee
            self.payees
                .len()
                .checked_sub(1)
                .unwrap()
                .try_into()
                .unwrap()
        }

        /// Add n payees to the operation, used only for testing.
        #[ink(message, payable)]
        pub fn add_n_payees(&mut self, n: u128) -> u128 {
            let address = self.env().caller();
            let value = self.env().transferred_value().checked_div(n).unwrap();
            let new_payee = Payee { address, value };

            for _ in 0..n {
                self.payees.push(new_payee);
            }

            // Return the index of the last added payee
            self.payees
                .len()
                .checked_sub(1)
                .unwrap()
                .try_into()
                .unwrap()
        }

        /// Returns the payee at the given index.
        #[ink(message)]
        pub fn get_payee(&self, id: u128) -> Option<Payee> {
            let payee = self.payees.get(usize::try_from(id).unwrap())?;
            return Some(*payee);
        }

        /// Pays out all payees.
        #[ink(message)]
        pub fn pay_out(&mut self) {
            for payee in &self.payees {
                self.env().transfer(payee.address, payee.value).unwrap();
            }
        }

        ///Same as pay_out but using a different approach to iterate self.payees
        #[ink(message)]
        pub fn pay_out2(&mut self) {
            for id in 0..self.payees.len() {
                self.env()
                    .transfer(self.payees[id].address, self.payees[id].value)
                    .unwrap();
            }
        }

        /// Pays out a range of payees.
        #[ink(message)]
        pub fn pay_out_range(&mut self, n: u64, m: u64) {
            for id in n..m {
                self.env()
                    .transfer(
                        self.payees[id as usize].address,
                        self.payees[id as usize].value,
                    )
                    .unwrap();
            }
        }
    }

    impl Default for DosUnboundedOperation {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            // Arrange
            let contract = DosUnboundedOperation::new();

            // Act
            let first_payee = contract.get_payee(0);

            // Assert
            assert!(first_payee.is_none());
        }

        #[ink::test]
        fn next_payee_advances() {
            // Arrange
            let mut contract = DosUnboundedOperation::new();

            // Act
            let first_payee_id = contract.add_payee();
            let second_payee_id = contract.add_payee();

            // Assert
            assert_eq!(first_payee_id, 0);
            assert_eq!(second_payee_id, 1);
        }

        #[ink::test]
        fn add_payee_works() {
            // Arrange
            let mut contract = DosUnboundedOperation::new();

            // Act
            let payee_id = contract.add_payee();
            let payee = contract.get_payee(payee_id).unwrap();

            // Assert
            assert_eq!(payee.address, AccountId::from([0x01; 32]));
            assert_eq!(payee.value, 0);
        }

        #[ink::test]
        fn add_n_payees_works() {
            // Arrange
            let mut contract = DosUnboundedOperation::new();

            // Act
            let payee_id = contract.add_n_payees(10);
            let payee = contract.get_payee(payee_id).unwrap();

            // Assert
            assert_eq!(payee.address, AccountId::from([0x01; 32]));
            assert_eq!(payee.value, 0);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn saves_payee_in_mapping(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let constructor = DosUnboundedOperationRef::new();
            let contract_acc_id = client
                .instantiate(
                    "dos-unbounded-operation",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            // Act
            let add_payee = build_message::<DosUnboundedOperationRef>(contract_acc_id.clone())
                .call(|contract| contract.add_payee());
            client
                .call(&ink_e2e::alice(), add_payee, 1000, None)
                .await
                .expect("add_payee failed");

            // Assert
            let get_payee = build_message::<DosUnboundedOperationRef>(contract_acc_id.clone())
                .call(|contract| contract.get_payee(0));
            let get_payee_res = client
                .call(&ink_e2e::alice(), get_payee, 0, None)
                .await
                .expect("get_payee failed");

            let payee = get_payee_res.return_value().expect("payee not found");
            let alice_account_id: ink::primitives::AccountId =
                ink_e2e::alice::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            assert_eq!(payee.address, alice_account_id);
            assert_eq!(payee.value, 1000);

            Ok(())
        }

        #[ink_e2e::test]
        async fn pay_out_runs_out_of_gas(mut client: ink_e2e::Client<C, E>) {
            // Arrange
            let constructor = DosUnboundedOperationRef::new();
            let contract_acc_id = client
                .instantiate(
                    "dos-unbounded-operation",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            for _ in 0..10 {
                let add_n_payees =
                    build_message::<DosUnboundedOperationRef>(contract_acc_id.clone())
                        .call(|contract| contract.add_n_payees(1000));
                client
                    .call(&ink_e2e::alice(), add_n_payees.clone(), 1000, None)
                    .await
                    .expect("add_n_payees failed");
            }

            // Act
            let pay_out = build_message::<DosUnboundedOperationRef>(contract_acc_id.clone())
                .call(|contract| contract.pay_out());
            let pay_out_result = client.call(&ink_e2e::alice(), pay_out, 0, None).await;

            // Assert
            assert!(pay_out_result.is_err());
        }
    }
}
