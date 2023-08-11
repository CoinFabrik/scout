#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod delegate_call {

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };
    use ink::storage::{traits::*, Lazy};

    #[ink(storage)]
    pub struct DelegateCall {
        //var1: Lazy<u128, ManualKey<0xcafebabe> >
        var1: u128
    }

    impl DelegateCall {
        /// Creates a new instance of the contract
        #[ink(constructor)]
        pub fn new(
            var1: u128,
        ) -> Self {
            Self { var1: var1 }
            /*let mut instance = Self::default();
            instance.var1.set(&var1);
            instance*/
        }

        #[ink(message)]
        pub fn get_var(&self) -> u128 {
            //self.var1.get().unwrap()
            self.var1
        }

        #[ink(message)]
        pub fn get_arbitrary_var(&self) -> u128 {
            let a = ink::env::get_contract_storage(&0u32)
                .unwrap_or_else(|_|panic!("primero"));
            if a.is_some() {
                return a.unwrap()
            } else {
                panic!("segundo")
            }
        }

        /// Delegates the fee calculation and pays the results to the corresponding addresses
        #[ink(message)]
        pub fn do_delegate(&mut self, target: Hash, value: u128) -> Result<u128, ink::LangError> {
            build_call::<DefaultEnvironment>()
                .delegate(target)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("delegate")))
                        .push_arg(value),
                )
                .returns::<u128>()
                .try_invoke().unwrap_or_else(|_|Err(ink::LangError::CouldNotReadInput))
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use delegate_call_delegated::DelegateCallDelegatedRef;
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test(additional_contracts = "../!test-example-delegated/Cargo.toml")]
        async fn constructor_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = DelegateCallRef::new(
                70,
            );
            let contract_acc_id = client
                .instantiate("delegate-call", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Act
            let get_var_call = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.get_var());
            let var = client
                .call_dry_run(&ink_e2e::alice(), &get_var_call, 0, None)
                .await;

            
            // Assert
            assert_eq!(
                var.return_value(),
                70
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn delegate_call_works(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = DelegateCallRef::new(
                70,
            );
            let contract_acc_id = client
                .instantiate("delegate-call", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let delegated_constructor = DelegateCallDelegatedRef::new(
                20,
            );
            let delegated_contract_acc_id = client
                .instantiate(
                    "delegate-call-delegated",
                    &ink_e2e::alice(),
                    delegated_constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;
            let delegated_contract_codehash_call =
                build_message::<DelegateCallDelegatedRef>(delegated_contract_acc_id.clone())
                    .call(|contract| contract.codehash());
            let delegated_contract_codehash: Hash = client
                .call_dry_run(&ink_e2e::alice(), &delegated_contract_codehash_call, 0, None)
                .await
                .return_value();
            
            let get_var_call = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.get_var());
            let var = client
                .call_dry_run(&ink_e2e::alice(), &get_var_call, 0, None)
                .await;
            // Assert
            assert_eq!(
                var.return_value(),
                70
            );

            let delegated_contract_delegate_call =
                build_message::<DelegateCallRef>(contract_acc_id.clone())
                    .call(|contract| contract.do_delegate(delegated_contract_codehash, 10));
            let delegate_contract_res = client
                .call(&ink_e2e::alice(), delegated_contract_delegate_call, 0, None)
                .await
                .expect("delegate call failed")
                .return_value();
            assert_eq!(
                delegate_contract_res.unwrap(),
                10
            );

            let get_arbitrary_var_call = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.get_arbitrary_var(0u32));
            let get_arbitrary_var_res = client
                .call_dry_run(&ink_e2e::alice(), &get_arbitrary_var_call, 0, None)
                .await;

            assert_eq!(
                get_arbitrary_var_res.return_value(),
                10
            );

            let var2 = client
                .call_dry_run(&ink_e2e::alice(), &get_var_call, 0, None)
                .await;
            
            assert_eq!(
                var2.return_value(),
                10
            );
            Ok(())
            /*// Act
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

            Ok(())*/
        }
    }
}
