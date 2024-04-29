#![cfg_attr(not(feature = "std"), no_std, no_main)]
// This is an example from ink docs.

#[ink::contract]
pub mod my_contract {
    // In order for this to actually work with another contract we'd need a way
    // to turn the `ink-as-dependency` crate feature on in doctests, which we
    // can't do.
    //
    // Instead we use our own contract's `Ref`, which is fine for this example
    // (just need something that implements the `ContractRef` trait).
    pub mod other_contract {
        pub use super::MyContractRef as OtherContractRef;
    }
    use ink::env::call::{build_create, ExecutionInput, Selector};
    use other_contract::OtherContractRef;

    #[ink(storage)]
    pub struct MyContract {}

    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn instantiate_contract(&self) -> MyContractRef {
            let create_params = build_create::<OtherContractRef>()
                .instantiate_v1()
                .code_hash(Hash::from([0x42; 32]))
                .gas_limit(500_000_000)
                .endowment(25)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("new")))
                        .push_arg(42)
                        .push_arg(true)
                        .push_arg(&[0x10u8; 32]),
                )
                .salt_bytes(&[0xCA, 0xFE, 0xBA, 0xBE])
                .returns::<OtherContractRef>()
                .params();
            self.env()
                .instantiate_contract_v1(&create_params)
                .unwrap_or_else(|error| {
                    panic!(
                        "Received an error from the Contracts pallet while instantiating: {:?}",
                        error
                    )
                })
                .unwrap_or_else(|error| {
                    panic!("Received a `LangError` while instatiating: {:?}", error)
                })
        }
    }
}
