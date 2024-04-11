#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod avoid_unsafe_block {

    use ink::prelude::{string::String};
    use ink::storage::{StorageVec, Mapping};

    #[derive(Default)]
    #[ink(storage)]
    pub struct MyContract {
        on_chain_log: Mapping<AccountId, String>,
        donations: StorageVec<String>,

    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsertFailed,
        PeekFailed,
        PushFailed,
        ErrNone,
    }


    impl MyContract {

        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn unsafe_function(&mut self, n: u64) -> u64 {
            unsafe {
                let mut i = n as f64;
                let mut y = i.to_bits();
                y = 0x5fe6ec85e7de30da - (y >> 1);
                i = f64::from_bits(y);
                i *= 1.5 - 0.5 * n as f64 * i * i;
                i *= 1.5 - 0.5 * n as f64 * i * i;
    
                let result_ptr: *mut f64 = &mut i;
    
                (*result_ptr).to_bits()
            }
        }

    }
}
