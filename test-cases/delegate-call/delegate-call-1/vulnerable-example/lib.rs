#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod delegate_call {

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DelegateCall {
        admin: AccountId,
        addresses: [AccountId; 3],
        percent1: u128,
        percent2: u128,
        percent3: u128,
    }

    impl DelegateCall {

        #[ink(constructor)]
        pub fn new(address1: AccountId, address2: AccountId, address3: AccountId, p1: u128, p2: u128, p3: u128) -> Self {
            Self {
                admin: Self::env().caller(),
                addresses: [address1, address2, address3],
                percent1: p1,
                percent2: p2,
                percent3: p3
            }
        }

        #[ink(message)]
        pub fn get_percents(&self, target: Hash) -> (u128, u128, u128) {
            let result: (u128, u128, u128) = build_call::<DefaultEnvironment>()
                .delegate(target)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("get_percents")))
                )
                .returns::<(u128, u128, u128)>()
                .invoke();

            result

        }

        #[ink(message, payable)]
        pub fn get_msg_money(&self) -> u128 {
            let amount = self.env().transferred_value();
            amount
        }


        #[ink(message, payable)]
        pub fn ask_payouts(&mut self, target: Hash) -> (Balance, Balance, Balance) {
            let amount = self.env().transferred_value();

            ink::env::debug_println!("amount sent: {}", amount);

            let result: (Balance, Balance, Balance) = build_call::<DefaultEnvironment>()
                .delegate(target)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("payouts")))
                        .push_arg(amount)
                )
                .returns::<(Balance, Balance, Balance)>()
                .invoke();

                let total = result.0 + result.1 + result.2;

                ink::env::debug_println!("total: {}", total);

                assert!(total <= amount, "Not enough money");


            self.env().transfer(self.addresses[0],total).unwrap();


            result
        }

    }

/*         #[cfg(test)]
        mod tests {
            use super::*;
            use ink::env::test::DefaultAccounts;

            type AccountId = <ink::env::DefaultEnvironment as ink::env::Environment>::AccountId;


            #[ink::test]
            fn constructor_works() {
                let accounts: DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
                let alice: AccountId = accounts.alice;
                let bob: AccountId = accounts.bob;
                let charlie: AccountId = accounts.charlie;
                let dave: AccountId = accounts.eve;
                let contract = DelegateCall::new(alice, bob, charlie, 33, 33, 34);
                assert_eq!(contract.admin, alice);
                assert_eq!(contract.addresses, [alice, bob, charlie]);
            }

        }

        #[cfg(all(test, feature = "e2e-tests"))]

        mod e2e_tests {
            /// Imports all the definitions from the outer scope so we can use them here.
            use super::*;

            /// A helper function used for calling contract messages.
            use ink_e2e::build_message;

            /// The End-to-End test `Result` type.
            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            /// We test that we can read and write a value from the on-chain contract contract.
            #[ink_e2e::test(additional_contracts = "../divider/Cargo.toml")]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {



                let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
                let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
                let charlie_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);
                let eve_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Eve);
                let dave_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Dave);


                let constructor_contract = DelegateCallRef::new(
                    bob_account,
                    charlie_account,
                    eve_account,
                    33,
                    33,
                    34,
                );

                let hs = client.upload("divider", &ink_e2e::dave(), None)
                    .await
                    .expect("upload failed")
                    .code_hash;


                let construct_original = divider::DividerRef::new(
                    bob_account,
                    charlie_account,
                    eve_account,
                    33,
                    33,
                    34,
                );

                let contract_account_id = client
                    .instantiate("delegate-call", &ink_e2e::dave(), constructor_contract, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                let original_account_id = client
                    .instantiate("divider", &ink_e2e::dave(), construct_original, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;




                //let hash_original : Hash = ink::env::code_hash::<ink::env::DefaultEnvironment>(&original_account_id).unwrap();


                //let a = build_message::<divider::DividerRef>(original_account_id.clone())
                //     .call(|contract| contract.codehash());

                // let supuesto_hash = client.call(&ink_e2e::bob(), a, 0, None).await.unwrap();




                let test_original = build_message::<DelegateCallRef>(contract_account_id.clone())
                    .call(|contract| contract.ask_payouts(hs));



                let _original_result = client.call(&ink_e2e::bob(), test_original, 300, None).await;

                //read balance from bob, charlie, eve

                let balance_bob = client.balance(bob_account).await.unwrap();
                let balance_charlie = client.balance(charlie_account).await.unwrap();
                let balance_eve = client.balance(eve_account).await.unwrap();

                assert!(matches!(balance_bob, 33));
                assert!(matches!(balance_charlie, 33));
                assert!(matches!(balance_eve, 34));
                Ok(())
            }
        } */
}



