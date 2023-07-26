#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ink_version {

    /// The content of this contract is irrelevant, the detection
    /// of the ink! version is the important thing here.
    /// It is found in the Cargo.toml file.
    #[ink(storage)]
    pub struct InkVersion {
        value: bool,
    }

    impl InkVersion {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
