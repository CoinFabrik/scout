#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unstable_interface {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct UnstableInterface {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InvalidSignature,
    }

    impl UnstableInterface {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);

            Self {
                total_supply,
                balances,
            }
        }

        /// Returns the balance of a given account.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap()
        }

        /// Transfers tokens from the caller to the given `to` account.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<(), Error> {
            let from_balance = self.balances.get(self.env().caller()).unwrap();

            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            let new_from_balance = from_balance - amount;
            self.balances.insert(self.env().caller(), &new_from_balance);

            let new_to_balance = self.balance_of(to) + amount;
            self.balances.insert(to, &new_to_balance);

            Ok(())
        }

        #[ink(message)]
        pub fn example(&self) {
            let signature: [u8; 64] = [
                184, 49, 74, 238, 78, 165, 102, 252, 22, 92, 156, 176, 124, 118, 168, 116, 247, 99,
                0, 94, 2, 45, 9, 170, 73, 222, 182, 74, 60, 32, 75, 64, 98, 174, 69, 55, 83, 85,
                180, 98, 208, 75, 231, 57, 205, 62, 4, 105, 26, 136, 172, 17, 123, 99, 90, 255,
                228, 54, 115, 63, 30, 207, 205, 131,
            ];
            let message: &[u8; 11] = b"hello world";
            let pub_key: [u8; 32] = [
                212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44,
                133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
            ];

            let result = ink::env::sr25519_verify(&signature, message.as_slice(), &pub_key);
            //assert!(result.is_ok())
        }
    }
}
