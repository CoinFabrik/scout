#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::divider::{DelegateCall, DelegateCallRef};

#[ink::contract]
mod divider{


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
        pub fn get_percents(&self) -> (u128, u128, u128) {
            let p1 = self.percent1;
            let p2 = self.percent2;
            let p3 = self.percent3;
            ink::env::debug_println!("requested value: {:?}", p1);
            ink::env::debug_println!("requested value: {:?}", p2);
            ink::env::debug_println!("requested value: {:?}", p3);
            //print the memory address of the values
            ink::env::debug_println!("address of p1 in divider: {:?}", &p1 as *const u128);
            ink::env::debug_println!("address of p2 in divider: {:?}", &p2 as *const u128);
            ink::env::debug_println!("address of p3 in divider: {:?}", &p3 as *const u128);

            (p1, p2, p3)
        }

        #[ink(message, payable)]
        pub fn payouts(&mut self, amount: Balance) -> (Balance, Balance, Balance) {
            let amount1 = amount * self.percent1 / 100;
            let amount2 = amount * self.percent2 / 100;
            let amount3 = amount * self.percent3 / 100;
            (amount1, amount2, amount3)
        }

        #[ink(message)]
        pub fn codehash(&self) -> Hash {
            self.env().code_hash(&self.env().account_id()).expect("Failed to get code hash")

        }
    }
}
