#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod non_pble_transferred_value {

    #[ink(storage)]
    pub struct NonPbleTransferredValue {
        value: u32,
    }

    impl NonPbleTransferredValue {
        #[ink(constructor)]
        pub fn new(value: u32) -> Self {
            Self { value }
        }

        #[ink(message)]
        pub fn something(&self) -> bool {
            if self.env().transferred_value() > 0 {
                return true;
            }
            false
        }
    }
}
