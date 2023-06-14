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

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        NotEnoughMoney,
        ErrorInvoking,
        TransferError,
    }

    impl DelegateCall {
        #[ink(constructor)]
        pub fn new(
            address1: AccountId,
            address2: AccountId,
            address3: AccountId,
            p1: u128,
            p2: u128,
            p3: u128,
        ) -> Self {
            Self {
                admin: Self::env().caller(),
                addresses: [address1, address2, address3],
                percent1: p1,
                percent2: p2,
                percent3: p3,
            }
        }

        #[ink(message)]
        pub fn get_percents(&self, target: Hash) -> Result<(u128, u128, u128), Error>  {
            let result: (u128, u128, u128) = build_call::<DefaultEnvironment>()
                .delegate(target)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "get_percents"
                ))))
                .returns::<(u128, u128, u128)>()
                .try_invoke()
                .map_err(|_e| Error::ErrorInvoking)?;

            Ok(result)
        }

        #[ink(message, payable)]
        pub fn get_msg_money(&self) -> u128 {
            let amount = self.env().transferred_value();
            amount
        }

        #[ink(message, payable)]
        pub fn ask_payouts(&mut self, target: Hash) -> Result<(Balance, Balance, Balance), Error> {
            let amount = self.env().transferred_value();

            let result: (Balance, Balance, Balance) = build_call::<DefaultEnvironment>()
                .delegate(target)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("payouts")))
                        .push_arg(amount),
                )
                .returns::<(Balance, Balance, Balance)>()
                .try_invoke()
                .map_err(|_e| Error::ErrorInvoking)?;



            let total = result.0 + result.1 + result.2;

            if total > amount {
                return Err(Error::NotEnoughMoney);
            }

            self.env().transfer(self.addresses[0], result.0).map_err(|_e| Error::TransferError)?;
            self.env().transfer(self.addresses[1], result.1).map_err(|_e| Error::TransferError)?;
            self.env().transfer(self.addresses[2], result.2).map_err(|_e| Error::TransferError)?;

            Ok(result)
        }
    }
}
