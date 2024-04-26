#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod delegate_call {

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        invoke_contract_v1, DefaultEnvironment,
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

            let accounts = ink::env::tests

            invoke_contract_v1(&accounts);

            //let total = result.0 + result.1 + result.2;
            //if total > amount {
            //    return Err(Error::NotEnoughMoney);
            //}

            //self.env()
            //    .transfer(self.addresses[0], result.0)
            //    .map_err(|_| Error::TransferFailed)?;
            //self.env()
            //    .transfer(self.addresses[1], result.1)
            //    .map_err(|_| Error::TransferFailed)?;
            //self.env()
            //    .transfer(self.addresses[2], result.2)
            //    .map_err(|_| Error::TransferFailed)?;

            Ok(())
        }
    }
}
