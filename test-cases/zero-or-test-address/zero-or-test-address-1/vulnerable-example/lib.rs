#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod zerocheck {

    #[ink(storage)]
    pub struct Zerocheck {
        admin: AccountId,
    }

    impl Zerocheck {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self { admin: admin }
        }

        #[ink(message)]
        pub fn modify_admin(&mut self, admin: AccountId) -> AccountId {
            assert_eq!(
                self.admin,
                self.env().caller(),
                "Only admin can call this function"
            );

            self.admin = admin;
            return self.admin;
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
            assert_eq!(zerocheck.admin, zero_address);
        }

        #[ink::test]
        fn allows_default_accounts() {
            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let zerocheck = Zerocheck::new(accounts.alice);
            assert_eq!(zerocheck.admin, accounts.alice);

            let zerocheck = Zerocheck::new(accounts.bob);
            assert_eq!(zerocheck.admin, accounts.bob);

            let zerocheck = Zerocheck::new(accounts.charlie);
            assert_eq!(zerocheck.admin, accounts.charlie);

            let zerocheck = Zerocheck::new(accounts.django);
            assert_eq!(zerocheck.admin, accounts.django);

            let zerocheck = Zerocheck::new(accounts.eve);
            assert_eq!(zerocheck.admin, accounts.eve);

            let zerocheck = Zerocheck::new(accounts.frank);
            assert_eq!(zerocheck.admin, accounts.frank);



        }


        #[ink::test]
        #[should_panic(expected = "Only admin can call this function")]
        fn calls_fails_if_zero_admin() {
            let zero_address = AccountId::from([0x0; 32]);

            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut zerocheck = Zerocheck::new(zero_address);
            zerocheck.modify_admin(accounts.alice);
        }
        #[ink::test]
        fn modify_admin_doesnt_fail_if_zero() {
            let zero_address = AccountId::from([0x0; 32]);

            let accounts: DefaultAccounts<ink::env::DefaultEnvironment> =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut zerocheck = Zerocheck::new(accounts.alice);
            zerocheck.modify_admin(zero_address);
        }

    }
}
