#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod unsafe_unwrap {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct UnsafeUnwrap {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Account has insufficient balance.
        InsufficientBalance,
    }

    impl UnsafeUnwrap {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);

            Self {
                total_supply,
                balances,
            }
        }

        /// Returns the balance of a given account.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap()
        }

        /// Transfers tokens from the caller to the given `to` account.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<(), Error> {
            let from_balance = self.balances.get(self.env().caller()).unwrap();

            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            let new_from_balance = from_balance - amount;
            self.balances.insert(self.env().caller(), &new_from_balance);

            let new_to_balance = self.balance_of(to) + amount;
            self.balances.insert(to, &new_to_balance);

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;
        use ink::env::DefaultEnvironment;

        #[ink::test]
        fn constructor_works() {
            // Arrange
            let initial_balance = 100;

            // Act
            let contract = UnsafeUnwrap::new(initial_balance);

            // Assert
            let alice_balance: Balance =
                contract.balance_of(test::default_accounts::<DefaultEnvironment>().alice);
            assert_eq!(alice_balance, initial_balance);
        }

        #[ink::test]
        #[should_panic]
        fn balance_of_unwraps_on_unknown_account() {
            // Arrange
            let initial_balance = 100;
            let contract = UnsafeUnwrap::new(initial_balance);

            // Act
            contract.balance_of(test::default_accounts::<DefaultEnvironment>().bob);
        }

        #[ink::test]
        #[should_panic]
        fn transfer_unwraps_to_unknown_account() {
            // Arrange
            let initial_balance = 100;
            let transfer_amount = 20;
            let mut contract = UnsafeUnwrap::new(initial_balance);

            // Act
            contract
                .transfer(
                    test::default_accounts::<DefaultEnvironment>().bob,
                    transfer_amount,
                )
                .unwrap();
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn constructor_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let initial_balance = 100;

            // Act
            let constructor = UnsafeUnwrapRef::new(initial_balance);
            let contract_acc_id = client
                .instantiate("unsafe-unwrap", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Assert
            let alice_account_id = ink_e2e::alice::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let balance_of_alice_call = build_message::<UnsafeUnwrapRef>(contract_acc_id.clone())
                .call(|contract| contract.balance_of(alice_account_id));
            let balance_of_alice = client
                .call_dry_run(&ink_e2e::alice(), &balance_of_alice_call, 0, None)
                .await;

            assert_eq!(balance_of_alice.return_value(), initial_balance);

            Ok(())
        }

        #[ink_e2e::test]
        async fn balance_of_unwraps_on_unknown_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            // Arrange
            let initial_balance = 100;

            // Act
            let constructor = UnsafeUnwrapRef::new(initial_balance);
            let contract_acc_id = client
                .instantiate("unsafe-unwrap", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Assert
            let bob_account_id = ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let balance_of_bob_call = build_message::<UnsafeUnwrapRef>(contract_acc_id.clone())
                .call(|contract| contract.balance_of(bob_account_id));
            let balance_of_bob = client
                .call(&ink_e2e::alice(), balance_of_bob_call, 0, None)
                .await;

            assert!(balance_of_bob.is_err());

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_unwraps_to_unknown_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            // Arrange
            let initial_balance = 100;
            let transfer_amount = 40;

            let bob_account_id = ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();

            // Act
            let constructor = UnsafeUnwrapRef::new(initial_balance);
            let contract_acc_id = client
                .instantiate("unsafe-unwrap", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let transfer_call = build_message::<UnsafeUnwrapRef>(contract_acc_id.clone())
                .call(|contract| contract.transfer(bob_account_id, transfer_amount));
            let transfer_result = client.call(&ink_e2e::alice(), transfer_call, 0, None).await;

            // Assert
            assert!(transfer_result.is_err());

            Ok(())
        }
    }
}
