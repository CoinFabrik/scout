#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod iterators_over_indexing {

    #[ink(storage)]
    pub struct IteratorsOverIndexing {
        value: Vec<u8>,
    }

    impl IteratorsOverIndexing {
        #[ink(constructor)]
        pub fn new(value1: u8, value2: u8, value3: u8) -> Self {
            Self {
                value: vec![value1, value2, value3],
            }
        }

        #[ink(message)]
        pub fn index_bad(&self) {
            ink::env::debug_println!("this will panic");
            for i in 0..3 {
                ink::env::debug_println!("item: {:?}", self.value[i]);
            }
        }
    }
}
