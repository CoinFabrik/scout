#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod delegate_call_divider {

    #[ink(storage)]
    pub struct DelegateCallDivider {
        admin: AccountId,
        addresses: [AccountId; 3],
        percent1: u128,
        percent2: u128,
        percent3: u128,
    }

    impl DelegateCallDivider {
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

        /// Returns the values to pay dependant on the saved percents
        #[ink(message, payable)]
        pub fn payouts(&mut self, amount: Balance) -> (Balance, Balance, Balance) {
            let amount1 = amount * self.percent1 / 100;
            let amount2 = amount * self.percent2 / 100;
            let amount3 = amount * self.percent3 / 100;
            (amount1, amount2, amount3)
        }

        /// Returns the codehash of the contract
        #[ink(message)]
        pub fn codehash(&self) -> Hash {
            self.env()
                .code_hash(&self.env().account_id())
                .expect("Failed to get code hash")
        }
    }
}
