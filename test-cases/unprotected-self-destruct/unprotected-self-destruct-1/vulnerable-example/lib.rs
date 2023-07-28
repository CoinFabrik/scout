#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unprotected_self_destruct {

    #[ink(storage)]
    pub struct UnprotectedSelfDestruct {
        admin: AccountId,
    }

    impl UnprotectedSelfDestruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn delete_contract(&mut self, beneficiary: AccountId) {
            self.env().terminate_contract(beneficiary)
        }
    }

    impl Default for UnprotectedSelfDestruct {
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
            let zerocheck = UnprotectedSelfDestruct::new();
            assert_eq!(zerocheck.admin, accounts.alice);
        }
    }
}
