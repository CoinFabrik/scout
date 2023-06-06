#![cfg_attr(not(feature = "std"), no_std)]

pub use self::vault::{Vault, VaultRef};

#[ink::contract]
mod vault {
    use ink::{
        env::call::{build_call, Selector},
        storage::Mapping,
    };

    #[ink(storage)]
    pub struct Vault {
        /// Balances of accounts.
        balances: Mapping<AccountId, Balance>,
    }

    impl Vault {
        /// Creates a new instance of the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::default(),
            }
        }

        /// Deposits the sent amount into the vault.
        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Balance {
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            let updated_balance = caller_balance + self.env().transferred_value();
            self.balances.insert(caller_addr, &updated_balance);
            return updated_balance;
        }

        /// Returns the current balance of the given account.
        #[ink(message)]
        pub fn balance(&mut self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }

        /// Withdraws the given amount from the vault.
        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Balance {
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            if amount <= caller_balance {
                let updated_balance = caller_balance - amount;
                if self.env().transfer(self.env().caller(), amount).is_err() {
                    panic!("requested transfer failed.")
                }
                self.balances.insert(caller_addr, &updated_balance);
                return updated_balance;
            } else {
                panic!("amount > balance")
            }
        }

        /// Calls the given address with the given amount and selector.
        pub fn call_with_value(
            &mut self,
            address: AccountId,
            amount: Balance,
            selector: u32,
        ) -> Balance {
            ink::env::debug_println!(
                "call_with_value function called from {:?}",
                self.env().caller()
            );
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            if amount <= caller_balance {
                //The balance is updated before the contract call
                self.balances
                    .insert(caller_addr, &(caller_balance - amount));
                let call = build_call::<ink::env::DefaultEnvironment>()
                    .call(address)
                    .transferred_value(amount)
                    .exec_input(ink::env::call::ExecutionInput::new(Selector::new(
                        selector.to_be_bytes(),
                    )))
                    .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                    .returns::<()>()
                    .params();
                self.env()
                    .invoke_contract(&call)
                    .unwrap_or_else(|err| panic!("Err {:?}", err))
                    .unwrap_or_else(|err| panic!("LangErr {:?}", err));

                return caller_balance - amount;
            } else {
                return caller_balance;
            }
        }
    }
}
