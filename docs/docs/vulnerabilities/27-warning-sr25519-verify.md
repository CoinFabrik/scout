# Warning sr25519_verify

## Description

- Vulnerability Category: `Known Bugs`
- Vulnerability Severity: `Medium`
- Detectors: [`warning-sr25519-verify`](https://github.com/CoinFabrik/scout/tree/main/detectors/warning-sr25519-verify)
- Test Cases: [`warning-sr25519-verify-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/warning-sr25519-verify/warning-sr25519-verify-1)

This function is from the unstable interface, which is unsafe and normally is not available on production chains.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
    #[ink(message)]
    pub fn example(&self) -> bool {
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

        ink::env::sr25519_verify(&signature, message.as_slice(), &pub_key).is_ok()
    }
```
`sr25519_verify` is a function not available on production chains. If used, it will cause the contract to fail.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/warning-sr25519-verify/warning-sr25519-verify-1/vulnerable-example).

## Remediation

Because of that do not use the `sr25519_verify` function.

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/warning-sr25519-verify/warning-sr25519-verify-1/remediated-example).


