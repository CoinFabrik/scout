#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod delegate_call {
    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct DelegateCall {
        admin: AccountId,
        balances: Mapping<AccountId, Balance>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        NotAnAdmin,
        DelegateCallFailed,
    }

    impl DelegateCall {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
                balances: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn get_admin(&self) -> AccountId {
            self.admin
        }

        #[ink(message, payable)]
        pub fn change_admin(
            &mut self,
            target: Hash,
            new_admin: AccountId,
        ) -> Result<AccountId, Error> {
            if self.admin != self.env().caller() {
                return Err(Error::NotAnAdmin);
            }

            let res = build_call::<DefaultEnvironment>()
                .delegate(target)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("change_admin")))
                        .push_arg(new_admin),
                )
                .returns::<AccountId>()
                .try_invoke()
                .map_err(|_| Error::DelegateCallFailed)?
                .map_err(|_| Error::DelegateCallFailed)?;

            Ok(res)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use delegate_call_exploiter::delegate_call_exploiter::DelegateCallExploiterRef;
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn delegate_change_of_admin(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let bob_account_id: ink::primitives::AccountId =
                ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                    .account_id()
                    .0
                    .into();

            let constructor = DelegateCallRef::new();

            let contract_acc_id = client
                .instantiate("delegate-call", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            //call get_admin
            let previous_admin = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.get_admin())
                .account_id()
                .clone();

            // Act
            let exploit_contract = DelegateCallExploiterRef::new();

            let exploit_contract_acc_id = client
                .instantiate(
                    "delegate-call-exploiter",
                    &ink_e2e::alice(),
                    exploit_contract,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let exploiter_contract_call =
                build_message::<DelegateCallExploiterRef>(exploit_contract_acc_id.clone())
                    .call(|contract| contract.codehash());

            let exploiter_contract_codehash = client
                .call_dry_run(&ink_e2e::alice(), &exploiter_contract_call, 0, None)
                .await
                .return_value();

            let change_admin_call =
                build_message::<DelegateCallRef>(contract_acc_id.clone()).call(|contract| {
                    contract.change_admin(exploiter_contract_codehash, bob_account_id)
                });

            let change_admin = client
                .call(&ink_e2e::alice(), change_admin_call, 0, None)
                .await;

            let new_admin = build_message::<DelegateCallRef>(contract_acc_id.clone())
                .call(|contract| contract.get_admin())
                .account_id()
                .clone();

            // Assert
            assert!(change_admin.is_ok());
            assert_eq!(previous_admin, new_admin);
            Ok(())
        }
    }
}
