#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod lazy_values_not_set {
    use ink::storage::Lazy;
    use ink::storage::Mapping;

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not not authorized.
        NotAuthorized,
        /// Address is invalid.
        InvalidAddress,
    }

    #[ink(storage)]
    pub struct LazyVaulesGetNotSet {
        values: Mapping<AccountId, u64>,
        lazy_val: Lazy<u64>,
    }

    impl LazyVaulesGetNotSet {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                values: Mapping::default(),
                lazy_val: Lazy::default(),
            }
        }

        #[ink(message)]
        pub fn mapping_sum_value_directly(&mut self, value: u64) -> Result<(), Error> {
            let key = self.env().caller();
            let val = self.values.get(key).unwrap_or_default();
            self.values.insert(key, &(val + value));
            Ok(())
        }

        #[ink(message)]
        pub fn mapping_sum_value_indirectly(&mut self, value: u64) -> Result<(), Error> {
            let key = self.env().caller();
            let val = self.values.get(key).unwrap_or_default();
            self.mapping_sum_value_indirectly_step2(val + value)
        }

        fn mapping_sum_value_indirectly_step2(&mut self, value: u64) -> Result<(), Error> {
            let key = self.env().caller();
            self.values.insert(key, &value);
            Ok(())
        }

        #[ink(message)]
        pub fn lazy_sum_value_directly(&mut self, value: u64) -> Result<(), Error> {
            let val = self.lazy_val.get().unwrap_or_default();
            self.lazy_val.set(&(val + value));
            Ok(())
        }

        #[ink(message)]
        pub fn lazy_sum_value_indirectly(&mut self, value: u64) -> Result<(), Error> {
            let val = self.lazy_val.get().unwrap_or_default();
            self.lazy_sum_value_indirectly_step2(val + value)
        }

        fn lazy_sum_value_indirectly_step2(&mut self, value: u64) -> Result<(), Error> {
            self.lazy_val.set(&value);
            Ok(())
        }

        #[ink(message)]
        pub fn lazy_sum_value_double_indirectly(&mut self, value: u64) -> Result<(), Error> {
            let val = self.lazy_val.get().unwrap_or_default();
            self.lazy_sum_value_double_indirectly_step2(val + value)
        }

        fn lazy_sum_value_double_indirectly_step2(&mut self, value: u64) -> Result<(), Error> {
            if value > 99999 {
                self.lazy_sum_value_double_indirectly_step3(value)
            } else {
                //test for recursive path
                self.lazy_sum_value_double_indirectly(value)
            }
        }

        fn lazy_sum_value_double_indirectly_step3(&mut self, value: u64) -> Result<(), Error> {
            self.lazy_val.set(&value);
            Ok(())
        }
    }

    impl Default for LazyVaulesGetNotSet {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::test::DefaultAccounts;

        use crate::lazy_values_not_set::LazyVaulesGetNotSet;

        #[ink::test]
        fn default_works() {
            let contract = LazyVaulesGetNotSet::new();
            assert_eq!(contract.lazy_val.get(), None);
        }

        #[ink::test]
        fn mapping_sum_value_directly_test() {
            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut contract = LazyVaulesGetNotSet::new();
            let res = contract.mapping_sum_value_directly(50);
            assert!(res.is_ok());
            assert_eq!(contract.values.get(accounts.alice), Some(50u64));
        }

        #[ink::test]
        fn lazy_sum_value_directly_test() {
            let mut contract = LazyVaulesGetNotSet::new();
            let res = contract.lazy_sum_value_directly(50);
            assert!(res.is_ok());
            assert_eq!(contract.lazy_val.get(), Some(50u64));
        }

        #[ink::test]
        fn mapping_sum_value_indirectly_test() {
            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut contract = LazyVaulesGetNotSet::new();
            let res = contract.mapping_sum_value_indirectly(50);
            assert!(res.is_ok());
            assert_eq!(contract.values.get(accounts.alice), Some(50u64));
        }

        #[ink::test]
        fn lazy_sum_value_indirectly_test() {
            let mut contract = LazyVaulesGetNotSet::new();
            let res = contract.lazy_sum_value_indirectly(50);
            assert!(res.is_ok());
            assert_eq!(contract.lazy_val.get(), Some(50u64));
        }
    }
}
