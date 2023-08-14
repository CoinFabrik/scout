#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unprotected_mapping_operation {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct UnprotectedMappingOperation {
        balances: Mapping<AccountId, Balance>,
        another_mapping: Mapping<u128, AccountId>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        TransferError,
        BalanceNotEnough,
    }

    impl UnprotectedMappingOperation {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::new(),
                another_mapping: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn this_should_not_trigger(&mut self, key: u128, value: AccountId) {
            self.another_mapping.insert(key, &value);
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self, dest: AccountId) {
            let amount: Balance = self.env().transferred_value();
            if let Some(current_bal) = self.balances.get(dest) {
                self.balances.insert(dest, &(current_bal + amount));
            } else {
                self.balances.insert(dest, &amount);
            }
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance, from: AccountId) -> Result<(), Error> {
            let current_bal = self.balances.take(from).unwrap_or(0);
            if current_bal >= amount {
                self.balances.insert(from, &(current_bal - amount));
                self.env()
                    .transfer(from, current_bal)
                    .map_err(|_| Error::TransferError)
            } else {
                Err(Error::BalanceNotEnough)
            }
        }

        #[ink(message)]
        pub fn withdraw_all(&mut self, from: AccountId) -> Result<(), Error> {
            let current_bal = self.balances.get(from).unwrap_or(0);
            self.balances.remove(from);
            self.env()
                .transfer(from, current_bal)
                .map_err(|_| Error::TransferError)
        }
    }

    impl Default for UnprotectedMappingOperation {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use ink_e2e::build_message;

        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_test(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = UnprotectedMappingOperationRef::new();
            let umop = client
                .instantiate(
                    "unprotected-mapping-operation",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let bob_account_id: AccountId = ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let charlie_account_id: AccountId = ink_e2e::charlie::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();

            let deposit_on_bob = build_message::<UnprotectedMappingOperationRef>(umop.clone())
                .call(|contract| contract.deposit(bob_account_id));
            let _alice_deposit_on_bob = client
                .call(&mut ink_e2e::alice(), deposit_on_bob, 1000, None)
                .await
                .expect("Alice deposit failed");

            let deposit_charlie = build_message::<UnprotectedMappingOperationRef>(umop.clone())
                .call(|contract| contract.deposit(charlie_account_id));
            let _charlie_deposit = client
                .call(&mut ink_e2e::charlie(), deposit_charlie, 1000, None)
                .await
                .expect("Charlie deposit failed");

            let withdraw = build_message::<UnprotectedMappingOperationRef>(umop.clone())
                .call(|contract| contract.withdraw(1000, bob_account_id));
            let _alice_withdraw_on_bob = client
                .call(&mut ink_e2e::alice(), withdraw, 0, None)
                .await
                .expect("Alice withdraw on bob failed");

            let withdraw_all = build_message::<UnprotectedMappingOperationRef>(umop.clone())
                .call(|contract| contract.withdraw_all(bob_account_id));
            let _charlie_withdraw_all = client
                .call(&mut ink_e2e::charlie(), withdraw_all, 0, None)
                .await
                .expect("Charlie withdraw all");
            /*let escrow_constructor = UnrestrictedTransferFromRef::new(
                ink_e2e::account_id(ink_e2e::AccountKeyring::Bob),
                ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie),
                ink_e2e::account_id(ink_e2e::AccountKeyring::Alice),
                token_account_id,
                1000,
            );

            let escrow_account_id = client
                .instantiate(
                    "unrestricted_transfer_from",
                    &mut ink_e2e::alice(),
                    escrow_constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let approve = build_message::<psp22::psp22::ContractRef>(token_account_id.clone())
                .call(|contract| contract.approve(escrow_account_id, 1000));
            let bob_approve = client.call(bob_borrow, approve, 0, None).await;
            assert_eq!(bob_approve.is_ok(), true);

            let balance = build_message::<psp22::psp22::ContractRef>(token_account_id.clone())
                .call(|contract| {
                    contract.balance_of(ink_e2e::account_id(ink_e2e::AccountKeyring::Bob))
                });
            let bob_balance = client.call_dry_run(bob_borrow, &balance, 0, None).await;
            assert_eq!(bob_balance.return_value(), 10000);

            let deposit = build_message::<UnrestrictedTransferFromRef>(escrow_account_id.clone())
                .call(|contract| {
                    contract.deposit(ink_e2e::account_id(ink_e2e::AccountKeyring::Bob))
                });
            let bob_deposit = client.call(bob_borrow, deposit, 0, None).await;
            assert_eq!(bob_deposit.is_ok(), true);

            let unlock = build_message::<UnrestrictedTransferFromRef>(escrow_account_id.clone())
                .call(|contract| contract.unlock());
            let alice_unlock = client.call(&mut ink_e2e::alice(), unlock, 0, None).await;
            assert_eq!(alice_unlock.is_ok(), true);

            let release = build_message::<UnrestrictedTransferFromRef>(escrow_account_id.clone())
                .call(|contract| contract.release());
            let charlie_release = client.call(&mut ink_e2e::charlie(), release, 0, None).await;
            assert_eq!(charlie_release.is_ok(), true);

            let charlie_balance = build_message::<psp22::psp22::ContractRef>(
                token_account_id.clone(),
            )
            .call(|contract| {
                contract.balance_of(ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie))
            });
            let charlie_balance_ret = client
                .call_dry_run(&mut ink_e2e::charlie(), &charlie_balance, 0, None)
                .await;
            assert_eq!(charlie_balance_ret.return_value(), 1000);
            */
            Ok(())
        }
    }
}
