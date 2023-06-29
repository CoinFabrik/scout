#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod delegate_call {

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };

    #[ink(storage)]
    pub struct DelegateCall {
        admin: AccountId,
        addresses: [AccountId; 3],
        percent1: u128,
        percent2: u128,
        percent3: u128,
    }

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        NotEnoughMoney,
        DelegateCallFailed,
        TransferFailed,
    }

    impl DelegateCall {
        /// Creates a new instance of the contract
        #[ink(constructor)]
        pub fn new(
            address1: AccountId,
            address2: AccountId,
            address3: AccountId,
            percent1: u128,
            percent2: u128,
            percent3: u128,
        ) -> Self {
            Self {
                admin: Self::env().caller(),
                addresses: [address1, address2, address3],
                percent1,
                percent2,
                percent3,
            }
        }

        /// Returns the addresses of the payees
        #[ink(message)]
        pub fn get_addresses(&self) -> [AccountId; 3] {
            self.addresses
        }

        /// Returns the percentages of the payees
        #[ink(message)]
        pub fn get_percentages(&self) -> (u128, u128, u128) {
            (self.percent1, self.percent2, self.percent3)
        }

        /// Delegates the fee calculation and pays the results to the corresponding addresses
        #[ink(message, payable)]
        pub fn ask_payouts(&mut self, target: Hash) -> Result<(), Error> {
            let amount = self.env().transferred_value();

            let result = build_call::<DefaultEnvironment>()
                .delegate(target)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("payouts")))
                        .push_arg(amount),
                )
                .returns::<(Balance, Balance, Balance)>()
                .try_invoke()
                .map_err(|_| Error::DelegateCallFailed)?
                .map_err(|_| Error::DelegateCallFailed)?;

            let total = result.0 + result.1 + result.2;
            if total > amount {
                return Err(Error::NotEnoughMoney);
            }

            self.env()
                .transfer(self.addresses[0], result.0)
                .map_err(|_| Error::TransferFailed)?;
            self.env()
                .transfer(self.addresses[1], result.1)
                .map_err(|_| Error::TransferFailed)?;
            self.env()
                .transfer(self.addresses[2], result.2)
                .map_err(|_| Error::TransferFailed)?;

            Ok(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use delegate_call_divider::delegate_call_divider::DelegateCallDividerRef;
        use delegate_call_exploiter::delegate_call_exploiter::DelegateCallExploiterRef;
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn constructor_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let bob_account_id: ink::primitives::AccountId =
                ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            let charlie_account_id: ink::primitives::AccountId =
                ink_e2e::charlie::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            let dave_account_id: ink::primitives::AccountId =
                ink_e2e::dave::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();

            let constructor = DelegateCallRef::new(
                bob_account_id,
                charlie_account_id,
                dave_account_id,
                10,
                20,
                70,
            );
            let contract_acc_id = client
                .instantiate("delegate-call", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Act
            let get_addresses_call = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.get_addresses());
            let addresses = client
                .call_dry_run(&ink_e2e::alice(), &get_addresses_call, 0, None)
                .await;

            let get_percentages_call = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.get_percentages());
            let percentages = client
                .call_dry_run(&ink_e2e::alice(), &get_percentages_call, 0, None)
                .await;

            // Assert
            assert_eq!(
                addresses.return_value(),
                [bob_account_id, charlie_account_id, dave_account_id]
            );
            assert_eq!(percentages.return_value(), (10, 20, 70));

            Ok(())
        }

        #[ink_e2e::test]
        async fn payout_works_with_correct_divider(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            // Arrange
            let bob_account_id: ink::primitives::AccountId =
                ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            let bob_initial_balance = client.balance(bob_account_id).await.unwrap();
            let charlie_account_id: ink::primitives::AccountId =
                ink_e2e::charlie::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            let charlie_initial_balance = client.balance(charlie_account_id).await.unwrap();
            let dave_account_id: ink::primitives::AccountId =
                ink_e2e::dave::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            let dave_initial_balance = client.balance(dave_account_id).await.unwrap();

            let constructor = DelegateCallRef::new(
                bob_account_id,
                charlie_account_id,
                dave_account_id,
                10,
                20,
                70,
            );
            let contract_acc_id = client
                .instantiate("delegate-call", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let divider_constructor = DelegateCallDividerRef::new(
                bob_account_id,
                charlie_account_id,
                dave_account_id,
                10,
                20,
                70,
            );
            let divider_contract_acc_id = client
                .instantiate(
                    "delegate-call-divider",
                    &ink_e2e::alice(),
                    divider_constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;
            let divider_contract_codehash_call =
                build_message::<DelegateCallDividerRef>(divider_contract_acc_id.clone())
                    .call(|contract| contract.codehash());
            let divider_contract_codehash = client
                .call_dry_run(&ink_e2e::alice(), &divider_contract_codehash_call, 0, None)
                .await
                .return_value();

            // Act
            let ask_payouts_call = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.ask_payouts(divider_contract_codehash));
            let ask_payouts = client
                .call(&ink_e2e::alice(), ask_payouts_call, 100, None)
                .await;

            // Assert
            let bob_current_balance = client.balance(bob_account_id).await.unwrap();
            let charlie_current_balance = client.balance(charlie_account_id).await.unwrap();
            let dave_current_balance = client.balance(dave_account_id).await.unwrap();

            assert!(ask_payouts.is_ok());
            assert_eq!(bob_current_balance - bob_initial_balance, 10);
            assert_eq!(charlie_current_balance - charlie_initial_balance, 20);
            assert_eq!(dave_current_balance - dave_initial_balance, 70);

            Ok(())
        }

        #[ink_e2e::test]
        async fn payout_works_with_exploit(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let bob_account_id: ink::primitives::AccountId =
                ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            let bob_initial_balance = client.balance(bob_account_id).await.unwrap();
            let charlie_account_id: ink::primitives::AccountId =
                ink_e2e::charlie::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            let charlie_initial_balance = client.balance(charlie_account_id).await.unwrap();
            let dave_account_id: ink::primitives::AccountId =
                ink_e2e::dave::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();
            let dave_initial_balance = client.balance(dave_account_id).await.unwrap();

            let constructor = DelegateCallRef::new(
                bob_account_id,
                charlie_account_id,
                dave_account_id,
                10,
                20,
                70,
            );
            let contract_acc_id = client
                .instantiate("delegate-call", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let exploiter_constructor = DelegateCallExploiterRef::new(
                bob_account_id,
                charlie_account_id,
                dave_account_id,
                10,
                20,
                70,
            );
            let exploiter_contract_acc_id = client
                .instantiate(
                    "delegate-call-exploiter",
                    &ink_e2e::alice(),
                    exploiter_constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;
            let exploiter_contract_call =
                build_message::<DelegateCallExploiterRef>(exploiter_contract_acc_id.clone())
                    .call(|contract| contract.codehash());
            let exploiter_contract_codehash = client
                .call_dry_run(&ink_e2e::alice(), &exploiter_contract_call, 0, None)
                .await
                .return_value();

            // Act
            let ask_payouts_call = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.ask_payouts(exploiter_contract_codehash));
            let ask_payouts = client
                .call(&ink_e2e::alice(), ask_payouts_call, 100, None)
                .await;

            // Assert
            let bob_current_balance = client.balance(bob_account_id).await.unwrap();
            let charlie_current_balance = client.balance(charlie_account_id).await.unwrap();
            let dave_current_balance = client.balance(dave_account_id).await.unwrap();

            assert!(ask_payouts.is_ok());
            assert_eq!(bob_current_balance - bob_initial_balance, 70);
            assert_eq!(charlie_current_balance - charlie_initial_balance, 10);
            assert_eq!(dave_current_balance - dave_initial_balance, 20);

            Ok(())
        }
    }
}
