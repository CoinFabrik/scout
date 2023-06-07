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

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// External contract call failed.
        ContractInvokeFailed,
        /// Insufficient balance to perform operation.
        InsufficientBalance,
        /// An overflow was produced.
        Overflow,
        /// Transfer failed.
        TransferFailed,
        /// An underflow was produced.
        Underflow,
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
        pub fn deposit(&mut self) -> Result<Balance, Error> {
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            let updated_balance = caller_balance
                .checked_add(self.env().transferred_value())
                .ok_or(Error::Overflow)?;
            self.balances.insert(caller_addr, &updated_balance);
            Ok(updated_balance)
        }

        /// Returns the current balance of the given account.
        #[ink(message)]
        pub fn balance(&mut self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }

        /// Withdraws the given amount from the vault.
        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<Balance, Error> {
            let caller_addr = self.env().caller();
            let caller_balance = self.balance(caller_addr);
            if amount > caller_balance {
                return Err(Error::InsufficientBalance);
            }

            let updated_balance = caller_balance.checked_sub(amount).ok_or(Error::Underflow)?;
            self.env()
                .transfer(self.env().caller(), amount)
                .map_err(|_| Error::TransferFailed)?;
            self.balances.insert(caller_addr, &updated_balance);
            Ok(updated_balance)
        }

        /// Calls the given address with the given amount and selector.
        #[ink(message)]
        pub fn call_with_value(
            &mut self,
            address: AccountId,
            amount: Balance,
            selector: u32,
        ) -> Result<Balance, Error> {
            ink::env::debug_println!(
                "call_with_value function called from {:?}",
                self.env().caller()
            );
            let caller_addr = self.env().caller();
            let caller_balance = self.balance(caller_addr);

            if amount > caller_balance {
                return Ok(caller_balance);
            }

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
                .map_err(|_| Error::ContractInvokeFailed)?
                .map_err(|_| Error::ContractInvokeFailed)?;

            let new_balance = caller_balance.checked_sub(amount).ok_or(Error::Underflow)?;
            self.balances.insert(caller_addr, &new_balance);

            Ok(new_balance)
        }
    }
}
