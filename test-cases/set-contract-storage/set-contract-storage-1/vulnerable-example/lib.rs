#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod erc20 {
    use ink::env;
    use ink::storage::traits::ManualKey;
    use ink::storage::Mapping;

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Account does not have enough ballance to fulfill the request.
        InsufficientBalance,
        /// Account does not have enough allowance to fulfill the request.
        InsufficientAllowance,
        /// An overflow was produced.
        Overflow,
        /// An underflow was produced.
        Underflow,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink::trait_definition]
    pub trait MisusedSetContractStorage {
        /// Manually set the contract storage.
        #[ink(message)]
        fn misused_set_contract_storage(
            &mut self,
            user_input_key: [u8; 68],
            user_input_data: u128,
        ) -> Result<()>;
    }

    /// Trait implemented by all ERC-20 respecting smart contracts.
    #[ink::trait_definition]
    pub trait BaseErc20 {
        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance;

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance;

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()>;

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()>;

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>;
    }

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// Total token supply.
        total_supply: Balance,
        /// Mapping from owner to number of owned token.
        balances: Mapping<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: Mapping<(AccountId, AccountId), Balance, ManualKey<255>>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    impl Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
            }
        }
    }

    impl BaseErc20 for Erc20 {
        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set.
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);
            env::debug_println!(
                "{:?}",
                (
                    AsRef::<[u8]>::as_ref(&owner),
                    AsRef::<[u8]>::as_ref(&spender)
                )
            );
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the account balance of `from`.
        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(&from, &to, value)?;
            self.allowances
                .insert((&from, &caller), &(allowance - value));
            Ok(())
        }
    }

    impl MisusedSetContractStorage for Erc20 {
        #[ink(message)]
        fn misused_set_contract_storage(
            &mut self,
            user_input_key: [u8; 68],
            user_input_data: u128,
        ) -> Result<()> {
            env::set_contract_storage(&user_input_key, &user_input_data);
            Ok(())
        }
    }

    #[ink(impl)]
    impl Erc20 {
        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        ///
        /// # Note
        ///
        /// Prefer to call this method over `balance_of` since this
        /// works using references which are more efficient in Wasm.
        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set.
        ///
        /// # Note
        ///
        /// Prefer to call this method over `allowance` since this
        /// works using references which are more efficient in Wasm.
        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of_impl(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_impl(to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });
            Ok(())
        }
    }

    #[cfg(feature = "std")]
    pub mod test_utils {
        use super::*;
        use ink::{
            env::hash::{Blake2x256, CryptoHash, HashOutput},
            primitives::Clear,
        };

        pub type Event = <Erc20 as ::ink::reflect::ContractEventBase>::Type;

        /// For calculating the event topic hash.
        pub struct PrefixedValue<'a, 'b, T> {
            pub prefix: &'a [u8],
            pub value: &'b T,
        }

        impl<X> scale::Encode for PrefixedValue<'_, '_, X>
        where
            X: scale::Encode,
        {
            #[inline]
            fn size_hint(&self) -> usize {
                self.prefix.size_hint() + self.value.size_hint()
            }

            #[inline]
            fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
                self.prefix.encode_to(dest);
                self.value.encode_to(dest);
            }
        }

        pub fn set_caller(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }

        pub fn assert_transfer_event(
            event: &ink::env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }

            fn encoded_into_hash<T>(entity: &T) -> Hash
            where
                T: scale::Encode,
            {
                let mut result = Hash::CLEAR_HASH;
                let len_result = result.as_ref().len();
                let encoded = entity.encode();
                let len_encoded = encoded.len();
                if len_encoded <= len_result {
                    result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                    return result;
                }
                let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
                <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
                let copy_len = core::cmp::min(hash_output.len(), len_result);
                result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
                result
            }

            let expected_topics = [
                encoded_into_hash(&PrefixedValue {
                    prefix: b"",
                    value: b"Erc20::Transfer",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::value",
                    value: &expected_value,
                }),
            ];
            for (n, (actual_topic, expected_topic)) in
                event.topics.iter().zip(expected_topics).enumerate()
            {
                let topic = <Hash as scale::Decode>::decode(&mut &actual_topic[..])
                    .expect("encountered invalid topic encoding");
                assert_eq!(topic, expected_topic, "encountered invalid topic at {n}");
            }
        }

        pub fn new_works(initial_supply: Balance) -> Erc20 {
            // Act
            let erc20 = Erc20::new(initial_supply);

            // Assert
            // The `BaseErc20` trait has indeed been implemented.
            assert_eq!(<Erc20 as BaseErc20>::total_supply(&erc20), initial_supply);

            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            test_utils::assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                initial_supply,
            );

            erc20
        }

        pub fn total_supply_works(initial_supply: Balance) -> Erc20 {
            // Act
            let erc20 = Erc20::new(initial_supply);

            // Assert
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            test_utils::assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                initial_supply,
            );
            assert_eq!(erc20.total_supply(), initial_supply);

            erc20
        }

        pub fn balance_of_works(initial_supply: Balance) -> Erc20 {
            // Arrange
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Act
            let erc20 = Erc20::new(initial_supply);

            // Assert
            assert_eq!(erc20.balance_of(accounts.alice), initial_supply);
            assert_eq!(erc20.balance_of(accounts.bob), 0);

            // Transfer event triggered during initial construction
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            test_utils::assert_transfer_event(
                &emitted_events[0],
                None,
                Some(accounts.alice),
                initial_supply,
            );

            erc20
        }

        pub fn transfer_works(initial_supply: Balance, tokens_to_transfer: Balance) -> Erc20 {
            // Arrange
            let mut erc20 = Erc20::new(initial_supply);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Act
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            erc20
                .transfer(accounts.bob, tokens_to_transfer)
                .expect("Transfer from alice to bob failed");

            // Assert
            assert_eq!(erc20.balance_of(accounts.bob), 10);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // Check first transfer event related to ERC-20 instantiation.
            test_utils::assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                initial_supply,
            );
            // Check the second transfer event relating to the actual trasfer.
            test_utils::assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                tokens_to_transfer,
            );

            erc20
        }

        pub fn invalid_transfer_should_fail(
            initial_supply: Balance,
            tokens_to_transfer: Balance,
        ) -> Erc20 {
            // Constructor works.
            let mut erc20 = Erc20::new(initial_supply);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            set_caller(accounts.bob);

            // Bob fails to transfers `tokens_to_transfer` tokens to Eve.
            assert_eq!(
                erc20.transfer(accounts.eve, tokens_to_transfer),
                Err(Error::InsufficientBalance)
            );
            // Alice owns all the tokens.
            assert_eq!(erc20.balance_of(accounts.alice), initial_supply);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);

            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            test_utils::assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                initial_supply,
            );

            erc20
        }

        pub fn transfer_from_works(initial_supply: Balance, tokens_to_transfer: Balance) -> Erc20 {
            // Constructor works.
            let mut erc20 = Erc20::new(initial_supply);
            // Transfer event triggered during initial construction.
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Bob fails to transfer tokens owned by Alice.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, tokens_to_transfer),
                Err(Error::InsufficientAllowance)
            );
            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // The approve event takes place.
            assert_eq!(ink::env::test::recorded_events().count(), 2);

            // Set Bob as caller.
            set_caller(accounts.bob);

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, tokens_to_transfer),
                Ok(())
            );
            // Eve owns tokens.
            assert_eq!(erc20.balance_of(accounts.eve), tokens_to_transfer);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            test_utils::assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                initial_supply,
            );
            // The second event `emitted_events[1]` is an Approve event that we skip checking.
            test_utils::assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                tokens_to_transfer,
            );

            erc20
        }

        pub fn allowance_must_not_change_on_failed_transfer(initial_supply: Balance) -> Erc20 {
            let mut erc20 = Erc20::new(initial_supply);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice approves Bob for token transfers on her behalf.
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

            // Set Bob as caller.
            set_caller(accounts.bob);

            // Bob tries to transfer tokens from Alice to Eve.
            let emitted_events_before = ink::env::test::recorded_events();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
                Err(Error::InsufficientBalance)
            );
            // Allowance must have stayed the same
            assert_eq!(
                erc20.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // No more events must have been emitted
            let emitted_events_after = ink::env::test::recorded_events();
            assert_eq!(emitted_events_before.count(), emitted_events_after.count());

            erc20
        }

        pub fn misuse_contract_storage(
            initial_supply: Balance,
            bob_initial_allowance: Balance,
            bob_exploited_allowance: Balance,
            storage_location: [u8; 68],
        ) -> Erc20 {
            // Arrange
            let mut erc20 = Erc20::new(initial_supply);
            // Using ink_e2e default accounts as those are used when setting the contract storage
            let alice_account_id: AccountId = [
                212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44,
                133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
            ]
            .into();
            let bob_account_id: AccountId = [
                142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97,
                54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
            ]
            .into();

            // Set Bob's allowance for Alice to `bob_initial_allowance`
            let allowance = erc20.allowance(alice_account_id, bob_account_id);
            assert_eq!(allowance, 0);

            set_caller(alice_account_id);
            erc20
                .approve(bob_account_id, bob_initial_allowance)
                .expect("Approve failed");

            let allowance = erc20.allowance(alice_account_id, bob_account_id);
            assert_eq!(allowance, bob_initial_allowance);

            // Act
            set_caller(bob_account_id);
            erc20
                .misused_set_contract_storage(storage_location, bob_exploited_allowance)
                .expect("Set contract storage failed");

            // Assert - assertion was moved to the test function to allow for fuzzing
            // let allowance = erc20.allowance(alice_account_id, bob_account_id);
            // assert_eq!(allowance, bob_exploited_allowance);

            erc20
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// The default constructor does its job.
        #[ink::test]
        fn new_works() {
            test_utils::new_works(100);
        }

        /// The total supply was applied.
        #[ink::test]
        fn total_supply_works() {
            test_utils::total_supply_works(100);
        }

        /// Get the actual balance of an account.
        #[ink::test]
        fn balance_of_works() {
            test_utils::balance_of_works(100);
        }

        #[ink::test]
        fn transfer_works() {
            test_utils::transfer_works(100, 10);
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            test_utils::invalid_transfer_should_fail(100, 10);
        }

        #[ink::test]
        fn transfer_from_works() {
            test_utils::transfer_from_works(100, 10);
        }

        #[ink::test]
        fn allowance_must_not_change_on_failed_transfer() {
            test_utils::allowance_must_not_change_on_failed_transfer(100);
        }

        #[ink::test]
        fn misuse_contract_storage() {
            let alice_account_id: AccountId = ink_e2e::alice::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let bob_account_id: AccountId = ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let initial_supply = 100;
            let bob_initial_allowance = 10;
            let bob_exploited_allowance = 20;
            let storage_location: [u8; 68] = {
                let mut storage_location = [0; 68];
                let (mapping_location, mapping_key_value) = storage_location.split_at_mut(4);
                let (mapping_key, mapping_value) = mapping_key_value.split_at_mut(32);

                mapping_location.copy_from_slice(&[255, 0, 0, 0]);
                mapping_key.copy_from_slice(alice_account_id.as_ref());
                mapping_value.copy_from_slice(bob_account_id.as_ref());

                storage_location
            };
            let erc20 = test_utils::misuse_contract_storage(
                initial_supply,
                bob_initial_allowance,
                bob_exploited_allowance,
                storage_location,
            );
            let allowance = erc20.allowance(alice_account_id, bob_account_id);
            assert_eq!(allowance, bob_exploited_allowance);
        }

        #[ink::test]
        fn misuse_contract_storage_fails() {
            let alice_account_id: AccountId = ink_e2e::alice::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let bob_account_id: AccountId = ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let charlie_account_id: AccountId = ink_e2e::charlie::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let initial_supply = 100;
            let bob_initial_allowance = 10;
            let bob_exploited_allowance = 20;
            let storage_location: [u8; 68] = {
                let mut storage_location = [0; 68];
                let (mapping_location, mapping_key_value) = storage_location.split_at_mut(4);
                let (mapping_key, mapping_value) = mapping_key_value.split_at_mut(32);

                mapping_location.copy_from_slice(&[255, 0, 0, 0]);
                mapping_key.copy_from_slice(alice_account_id.as_ref());
                mapping_value.copy_from_slice(charlie_account_id.as_ref());

                storage_location
            };
            let erc20 = test_utils::misuse_contract_storage(
                initial_supply,
                bob_initial_allowance,
                bob_exploited_allowance,
                storage_location,
            );
            let allowance = erc20.allowance(alice_account_id, bob_account_id);
            assert_eq!(allowance, bob_initial_allowance);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        fn misuse_contract_storage_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            let constructor = Erc20Ref::new(100);
            let contract_acc_id = client
                .instantiate(
                    "set-contract-storage",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("Contract instantiation failed")
                .account_id;
            let alice_account_id: AccountId = ink_e2e::alice::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();
            let bob_account_id: AccountId = ink_e2e::bob::<ink_e2e::PolkadotConfig>()
                .account_id()
                .0
                .into();

            // Set Bob's allowance for Alice to 10
            let allowance_msg = build_message::<Erc20Ref>(contract_acc_id.clone())
                .call(|contract| contract.allowance(alice_account_id, bob_account_id));
            let allowance = client
                .call_dry_run(&ink_e2e::alice(), &allowance_msg, 0, None)
                .await;
            assert_eq!(allowance.return_value(), 0);

            let approve_msg = build_message::<Erc20Ref>(contract_acc_id.clone())
                .call(|contract| contract.approve(bob_account_id, 10));
            client
                .call(&ink_e2e::alice(), approve_msg, 0, None)
                .await
                .expect("Approve failed");

            let allowance_msg = build_message::<Erc20Ref>(contract_acc_id.clone())
                .call(|contract| contract.allowance(alice_account_id, bob_account_id));
            let allowance = client
                .call_dry_run(&ink_e2e::alice(), &allowance_msg, 0, None)
                .await;
            assert_eq!(allowance.return_value(), 10);

            // Act
            let misused_msg = build_message::<Erc20Ref>(contract_acc_id.clone()).call(|contract| {
                contract.misused_set_contract_storage(
                    [
                        255, 0, 0, 0, 212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169,
                        159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109,
                        162, 125, 142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37,
                        252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242,
                        106, 72,
                    ],
                    1000,
                )
            });
            client
                .call(&ink_e2e::bob(), misused_msg, 0, None)
                .await
                .expect("Set contract storage failed");

            // Assert
            let allowance_msg = build_message::<Erc20Ref>(contract_acc_id.clone())
                .call(|contract| contract.allowance(alice_account_id, bob_account_id));
            let allowance = client
                .call_dry_run(&ink_e2e::alice(), &allowance_msg, 0, None)
                .await;
            assert_eq!(allowance.return_value(), 1000);

            Ok(())
        }
    }
}
