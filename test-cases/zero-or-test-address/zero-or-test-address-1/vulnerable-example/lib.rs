#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod zerocheck {
    #[ink(storage)]
    pub struct Zerocheck {
        admin: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not not authorized.
        NotAuthorized,
        /// Address is invalid.
        InvalidAddress,
    }

    impl Zerocheck {
        #[ink(constructor)]
        pub fn new() -> Self {
            let admin = Self::env().caller();
            Self { admin }
        }

        /// Changes the admin and returns the new admin. Can set to 0x0...
        #[ink(message)]
        pub fn modify_admin(&mut self, admin: AccountId) -> Result<AccountId, Error> {
            if self.admin != self.env().caller() {
                return Err(Error::NotAuthorized);
            }

            self.admin = admin;
            Ok(self.admin)
        }
    }

    impl Default for Zerocheck {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::test::DefaultAccounts;

        use super::*;
        type AccountId = <ink::env::DefaultEnvironment as ink::env::Environment>::AccountId;

        #[ink::test]
        fn default_works() {
            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let zerocheck = Zerocheck::new();
            assert_eq!(zerocheck.admin, accounts.alice);
        }

        #[ink::test]
        fn allows_default_accounts() {
            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let zerocheck = Zerocheck::new();
            assert_eq!(zerocheck.admin, accounts.alice);
        }

        #[ink::test]
        fn modify_admin_fails_if_caller_not_admin() {
            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut zerocheck = Zerocheck::new();
            let res = zerocheck.modify_admin(accounts.alice);
            assert_eq!(res, Ok(accounts.alice));
        }

        #[ink::test]
        fn modify_admin_doesnt_fails_if_setting_admin_to_zero() {
            let zero_address = AccountId::from([0x0; 32]);

            let mut zerocheck = Zerocheck::new();
            let res = zerocheck.modify_admin(zero_address);
            assert_eq!(res, Ok(zero_address));
        }
    }
}
