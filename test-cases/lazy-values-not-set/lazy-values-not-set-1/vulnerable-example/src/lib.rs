#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod lazy_values_not_set {
    use ink::storage::Lazy;
    use ink::storage::Mapping;

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        Error,
    }

    #[ink(storage)]
    pub struct LazyVaulesGetNotSet {
        mapping_values_asdf: Mapping<AccountId, u64>,
        lazy_val: Lazy<u64>,
    }

    impl LazyVaulesGetNotSet {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                mapping_values_asdf: Mapping::default(),
                lazy_val: Lazy::default(),
            }
        }

        #[ink(message)]
        pub fn mapping_sum_value_directly(&mut self, value: u64) -> Result<(), Error> {
            let key = self.env().caller();
            let mut _val = self.mapping_values_asdf.get(key).unwrap_or_default();
            _val += value;
            Ok(())
        }

        #[ink(message)]
        pub fn mapping_sum_value_indirectly(&mut self, value: u64) -> Result<(), Error> {
            let key = self.env().caller();
            let val = self.mapping_values_asdf.get(key).unwrap_or_default();
            self.mapping_sum_value_indirectly_step2(val + value)
        }

        fn mapping_sum_value_indirectly_step2(&mut self, _value: u64) -> Result<(), Error> {
            let _key = self.env().caller();
            Ok(())
        }

        #[ink(message)]
        pub fn lazy_sum_value_directly(&mut self, value: u64) -> Result<(), Error> {
            let mut _val = self.lazy_val.get().unwrap_or_default();
            _val += value;
            Ok(())
        }

        #[ink(message)]
        pub fn lazy_sum_value_indirectly(&mut self, value: u64) -> Result<(), Error> {
            let val = self.lazy_val.get().unwrap_or_default();
            self.lazy_sum_value_indirectly_step2(val + value)
        }

        fn lazy_sum_value_indirectly_step2(&mut self, _value: u64) -> Result<(), Error> {
            Ok(())
        }
    }

    impl Default for LazyVaulesGetNotSet {
        fn default() -> Self {
            Self::new()
        }
    }
}
