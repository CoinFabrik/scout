#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unprotected_mapping_operation {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct UnprotectedMappingOperation {
        balances: Mapping<AccountId, Balance>,
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
            }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();
            if let Some(current_bal) = self.balances.get(caller) {
                self.balances.insert(caller, &(current_bal + amount));
            } else {
                self.balances.insert(caller, &amount);
            }
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), Error> {
            let caller = self.env().caller();
            let current_bal = self.balances.take(caller).unwrap_or(0);
            if current_bal >= amount {
                self.balances.insert(caller, &(current_bal - amount));
                self.env()
                    .transfer(caller, current_bal)
                    .map_err(|_| Error::TransferError)
            } else {
                Err(Error::BalanceNotEnough)
            }
        }

        #[ink(message)]
        pub fn withdraw_all(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            let current_bal = self.balances.get(caller).unwrap_or(0);
            self.balances.remove(caller);
            self.env()
                .transfer(caller, current_bal)
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

            let deposit = build_message::<UnprotectedMappingOperationRef>(umop.clone())
                .call(|contract| contract.deposit());
            let _alice_deposit = client
                .call(&mut ink_e2e::alice(), deposit, 1000, None)
                .await
                .expect("Alice deposit failed");

            let withdraw_all = build_message::<UnprotectedMappingOperationRef>(umop.clone())
                .call(|contract| contract.withdraw_all());
            let _alice_withdraw_all = client
                .call(&mut ink_e2e::alice(), withdraw_all, 0, None)
                .await
                .expect("Alice withdraw all");

            let deposit2 = build_message::<UnprotectedMappingOperationRef>(umop.clone())
                .call(|contract| contract.deposit());
            let _alice_deposit = client
                .call(&mut ink_e2e::alice(), deposit2, 1000, None)
                .await
                .expect("Alice deposit failed");

            let withdraw = build_message::<UnprotectedMappingOperationRef>(umop.clone())
                .call(|contract| contract.withdraw(500));
            let _alice_withdraw = client
                .call(&mut ink_e2e::alice(), withdraw, 0, None)
                .await
                .expect("Alice withdraw failed");

            Ok(())
        }
    }
}
