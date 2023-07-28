#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(unknown_lints)]

#[ink::contract]
mod avoid_std_and_core_mem {

    #[ink(storage)]
    pub struct AvoidStdAndCoreMem {
        value: bool,
    }

    impl AvoidStdAndCoreMem {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[allow(forgetting_copy_types)]
        #[ink(message)]
        pub fn forget_value(&mut self) {
            let forgotten_value = self.value;
            self.value = false;
            core::mem::forget(forgotten_value);
        }

        #[ink(message)]
        pub fn get_value(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn forget_value_works() {
            let mut avoid_std_and_core_mem = AvoidStdAndCoreMem::new(false);
            avoid_std_and_core_mem.forget_value();
        }
    }
}
