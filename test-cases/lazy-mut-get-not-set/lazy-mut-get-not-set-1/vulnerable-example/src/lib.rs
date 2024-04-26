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
        pub fn something(&self) -> u32 {
            let mut this_shows = self.value.get().unwrap();
            this_shows += 1;
            this_shows
        }

        #[ink(message)]
        pub fn something_set(&mut self) -> u32 {
            let mut only_get = self.value.get();

            only_get.unwrap_or(0)
        }

        #[ink(message)]
        pub fn really_long_method(&mut self) -> u32 {
            self.value
                .get()
                .unwrap_or(0)
                .checked_add(1)
                .unwrap_or(0)
                .checked_mul(10)
                .unwrap_or(0)
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
