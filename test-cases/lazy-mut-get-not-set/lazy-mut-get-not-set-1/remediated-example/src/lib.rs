#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(unused_mut)]
#[ink::contract]
mod lazy_mut_get_not_set {

    use ink::storage::Lazy;

    #[ink(storage)]
    pub struct LazyMutGetNotSet {
        value: Lazy<u32>,
    }

    impl LazyMutGetNotSet {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: Lazy::new() }
        }

        #[ink(message)]
        pub fn this_does_not_triggers(&mut self) -> u32 {
            let mut not = self.value.get().unwrap();
            not += 1;
            self.value.set(&not);
            not
        }
    }
}
