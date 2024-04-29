#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod non_pble_transferred_value {

    #[ink(storage)]
    pub struct NonPbleTransferredValue {}

    impl NonPbleTransferredValue {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn something(&self) -> bool {
            self.env().transferred_value() > 0
        }

        #[ink(message)]
        pub fn something_in_other_function(&self) -> bool {
            self.the_other_function()
        }

        pub fn the_other_function(&self) -> bool {
            self.env().transferred_value() > 0
        }
    }
}
