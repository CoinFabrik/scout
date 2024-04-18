#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod iterators_over_indexing {

    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct IteratorsOverIndexing {
        value: Vec<u8>,
    }

    impl IteratorsOverIndexing {
        #[ink(constructor)]
        pub fn new(value1: u8, value2: u8, value3: u8) -> Self {
            Self {
                value: Vec::from([value1, value2, value3]),
            }
        }
        #[ink(message)]
        pub fn iterator(&self) {
            for _item in self.value.iter() {
                ink::env::debug_println!("item: {:?}", _item);
            }
        }

        #[ink(message)]
        pub fn index_to_len(&self) {
            for _i in 0..self.value.len() {
                ink::env::debug_println!("item: {:?}", self.value[_i]);
            }
        }
    }
}
