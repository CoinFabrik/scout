#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod zerocheck {

    #[ink(storage)]
    pub struct Zerocheck {
        admin: AccountId,
    }

    impl Zerocheck {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { admin: admin }
        }

        #[ink(message)]
        pub fn modify_admin(&mut self, admin: AccountId) -> Result<AccountId, &'static str> {
            if self.admin != self.env().caller() {
                return Err("Only admin can call this function");
            }
            if admin == AccountId::from([0x0; 32]) {
                return Err("Admin address cannot be empty");
            }
        
            self.admin = admin;
            Ok(self.admin)
        }
        
        
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::DefaultAccounts;
        type AccountId = <ink::env::DefaultEnvironment as ink::env::Environment>::AccountId;

        #[ink::test]
        fn default_works() {
            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let zerocheck = Zerocheck::new(accounts.alice);
            assert_eq!(zerocheck.admin, accounts.alice);
        }

        //check contract panics
        #[ink::test]
        fn allows_zero_account() {
            let zero_address = AccountId::from([0x0; 32]);

            let zerocheck = Zerocheck::new(zero_address);
            assert!(zerocheck.admin == zero_address);
        }

        #[ink::test]
        fn allows_default_accounts() {
            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let zerocheck = Zerocheck::new(accounts.alice);
            assert_eq!(zerocheck.admin, accounts.alice);


        }


        #[ink::test]
        #[should_panic(expected = "Only admin can call this function")]
        fn modify_admin_fails_if_caller_not_admin() {
            let zero_address = AccountId::from([0x0; 32]);

            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut zerocheck = Zerocheck::new(zero_address);
            zerocheck.modify_admin(accounts.alice);
        }

        #[ink::test]
        fn modify_admin_doesnt_fails_if_setting_admin_to_zero() {
            let zero_address = AccountId::from([0x0; 32]);

            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut zerocheck = Zerocheck::new(accounts.alice);
            zerocheck.modify_admin(zero_address);
        }

    }
}
