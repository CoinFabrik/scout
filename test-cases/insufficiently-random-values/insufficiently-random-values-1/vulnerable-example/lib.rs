#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod weak_prng {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct WeakPrng {
        /// Stores a single `bool` value on the storage.
        owner: AccountId,
        max_bet: Balance,
        min_bet: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        NumberOutOfRange,
        BetTooLow,
        BetTooHigh,
        TransferFailed,
    }
    pub type Result<T> = core::result::Result<T, Error>;

    impl WeakPrng {
        #[ink(constructor)]
        pub fn new(min: Balance, max: Balance) -> Self {
            Self {
                owner: Self::env().caller(),
                max_bet: max,
                min_bet: min,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(1, 1000000000)
        }

        pub fn check_inputs(
            &mut self,
            multiply_times: u128,
            min_num: u8,
            max_num: u8,
            num: u8,
        ) -> Result<bool> {
            if num < min_num || num > max_num {
                Err(Error::NumberOutOfRange)
            } else if self.env().transferred_value() < self.min_bet {
                Err(Error::BetTooLow)
            } else if self.env().transferred_value() > self.max_bet {
                Err(Error::BetTooHigh)
            } else if self.env().transferred_value() * multiply_times > self.env().balance() {
                Err(Error::InsufficientBalance)
            } else {
                Ok(true)
            }
        }

        #[ink(message, payable)]
        pub fn bet_single(&mut self, number: u8) -> Result<bool> {
            let inputs = self.check_inputs(36, 0, 36, number);
            inputs?;

            let pseudo_random: u8 = (self.env().block_number() % 37).try_into().unwrap();
            if pseudo_random == number {
                return self
                    .env()
                    .transfer(self.env().caller(), self.env().transferred_value() * 36)
                    .map(|_| true)
                    .map_err(|_e| Error::TransferFailed);
            }
            Ok(false)
        }

        #[ink(message, payable)]
        pub fn bet_dozen(&mut self, dozen_n: u8) -> Result<bool> {
            let inputs = self.check_inputs(3, 0, 2, dozen_n);
            inputs?;

            let pseudo_random: u8 = (self.env().block_timestamp() % 37).try_into().unwrap();
            if pseudo_random != 0
                && pseudo_random > (12 * dozen_n)
                && pseudo_random <= (12 * (dozen_n + 1))
            {
                return self
                    .env()
                    .transfer(self.env().caller(), self.env().transferred_value() * 3)
                    .map(|_| true)
                    .map_err(|_e| Error::TransferFailed);
            }
            Ok(false)
        }

        #[ink(message, payable)]
        pub fn bet_red_or_black(&mut self, red: bool) -> Result<bool> {
            let inputs = self.check_inputs(2, 0, 0, 0);
            inputs?;

            let pseudo_random: u8 = (self.env().block_timestamp() % 37).try_into().unwrap();
            let won = pseudo_random != 0
                && if red {
                    if pseudo_random <= 10 || (20..=28).contains(&pseudo_random) {
                        pseudo_random % 2 == 1
                    } else {
                        pseudo_random % 2 == 0
                    }
                } else if pseudo_random <= 10 || (20..=28).contains(&pseudo_random) {
                    pseudo_random % 2 == 0
                } else {
                    pseudo_random % 2 == 1
                };
            if won {
                return self
                    .env()
                    .transfer(self.env().caller(), self.env().transferred_value() * 2)
                    .map(|_| true)
                    .map_err(|_e| Error::TransferFailed);
            }
            Ok(false)
        }

        #[ink(message, payable)]
        pub fn bet_even_or_odd(&mut self, even: bool) -> Result<bool> {
            let inputs = self.check_inputs(2, 0, 0, 0);
            inputs?;

            let pseudo_random: u8 = (self.env().block_timestamp() % 37).try_into().unwrap();

            if pseudo_random != 0 && pseudo_random % 2 == if even { 0 } else { 1 } {
                return self
                    .env()
                    .transfer(self.env().caller(), self.env().transferred_value() * 2)
                    .map(|_| true)
                    .map_err(|_e| Error::TransferFailed);
            }
            Ok(false)
        }

        #[ink(message, payable)]
        pub fn bet_low_or_high(&mut self, low: bool) -> Result<bool> {
            let inputs = self.check_inputs(2, 0, 0, 0);
            inputs?;

            let pseudo_random: u8 = (self.env().block_timestamp() % 37).try_into().unwrap();
            let won = pseudo_random != 0
                && if low {
                    pseudo_random <= 18
                } else {
                    pseudo_random > 18
                };

            if won {
                return self
                    .env()
                    .transfer(self.env().caller(), self.env().transferred_value() * 2)
                    .map(|_| true)
                    .map_err(|_e| Error::TransferFailed);
            }
            Ok(false)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn bet_single_test() {
            let mut contract = WeakPrng::new(0, 1000000);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(35);
            let bet = contract.bet_single(0);
            assert!(bet.is_ok());
            assert!(bet.unwrap());

            let bet = contract.bet_single(60);
            assert!(bet.is_err());

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(350);
            let bet = contract.bet_single(1);
            assert!(bet.is_ok());
            assert!(!bet.unwrap());
        }

        #[ink::test]
        fn bet_dozen() {
            let mut contract = WeakPrng::new(0, 1000000);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(12);
            let bet = contract.bet_dozen(0);
            assert!(bet.is_ok());
            assert!(bet.unwrap());

            let bet = contract.bet_dozen(6);
            assert!(bet.is_err());

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(370 + 24);
            let bet = contract.bet_dozen(2);
            assert!(bet.is_ok());
            assert!(!bet.unwrap());
        }

        #[ink::test]
        fn bet_even_or_odd() {
            let mut contract = WeakPrng::new(0, 1000000);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(12);
            let bet = contract.bet_even_or_odd(true);
            assert!(bet.is_ok());
            assert!(bet.unwrap());

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(37);
            let bet = contract.bet_even_or_odd(true);
            assert!(bet.is_ok());
            assert!(!bet.unwrap());

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(370 + 24);
            let bet = contract.bet_even_or_odd(false);
            assert!(bet.is_ok());
            assert!(!bet.unwrap());
        }

        #[ink::test]
        fn bet_low_or_high() {
            let mut contract = WeakPrng::new(0, 1000000);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(12);
            let bet = contract.bet_low_or_high(true);
            assert!(bet.is_ok());
            assert!(bet.unwrap());

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(37);
            let bet = contract.bet_low_or_high(true);
            assert!(bet.is_ok());
            assert!(!bet.unwrap());

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(370 + 14);
            let bet = contract.bet_low_or_high(false);
            assert!(bet.is_ok());
            assert!(!bet.unwrap());
        }

        #[ink::test]
        fn bet_red_or_black() {
            let mut contract = WeakPrng::new(0, 1000000);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(12);
            let bet = contract.bet_red_or_black(true);
            assert!(bet.is_ok());
            assert!(bet.unwrap());

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(37);
            let bet = contract.bet_red_or_black(true);
            assert!(bet.is_ok());
            assert!(!bet.unwrap());

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(370 + 14);
            let bet = contract.bet_red_or_black(false);
            assert!(bet.is_ok());
            assert!(!bet.unwrap());
        }
    }
}
