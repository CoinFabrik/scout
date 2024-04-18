#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incorrect_exponentiation {

    #[ink(storage)]
    pub struct IncorrectExponentiation {
        data: u128,
    }

    impl IncorrectExponentiation {
        #[ink(constructor)]
        pub fn new() -> Self {
            IncorrectExponentiation { data: 255 ^ 2 - 1 }
        }

        #[ink(message)]
        pub fn set_data(&mut self, new_data: Balance) {
            self.data = new_data;
        }

        #[ink(message)]
        pub fn exp_data_by_3(&mut self) {
            self.data = self.data ^ 3
        }
    }
}
