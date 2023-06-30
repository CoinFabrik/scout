# Insufficiently random values

## Description

- Vulnerability Category: `Block attributes`
- Vulnerability Severity: `Critical`
- Detectors: [`insufficiently-random-values`](https://github.com/CoinFabrik/scout/tree/main/detectors/insufficiently-random-values)
- Test Cases: [`insufficiently-random-values-1`](https://github.com/CoinFabrik/scout/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1)

Using block attributes like `block_timestamp` or `block_number` for random number generation in ink! Substrate smart contracts is not recommended due to the predictability of these values. Block attributes are publicly visible and deterministic, making it easy for malicious actors to anticipate their values and manipulate outcomes to their advantage. Furthermore, validators could potentially influence these attributes, further exacerbating the risk of manipulation. For truly random number generation, it's important to use a source that is both unpredictable and external to the blockchain environment, reducing the potential for malicious exploitation.

## Exploit Scenario

Consider the following `ink!` contract:

```rust
#[ink(message, payable)]
pub fn bet_single(&mut self, number: u8) -> Result<bool> {
    let inputs = self.check_inputs(36, 0, 36, number);
    if inputs.is_err() {
        return Err(inputs.unwrap_err());
    }

    let pseudo_random: u8 = (self.env().block_number() % 37).try_into().unwrap();
    if pseudo_random == number {
        return self
            .env()
            .transfer(self.env().caller(), self.env().transferred_value() * 36)
            .map(|_| true)
            .map_err(|_e| Error::TransferFailed);
    }
    return Ok(false);
}
```

The vulnerability in this `bet_single` function arises from the use of `self.env().block_number() % 37` for pseudo-random number generation. Given the public visibility and predictability of block numbers, this method exposes the function to potential manipulation. 

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1/vulnerable-example).

## Remediation

Avoid using block attributes like `block_timestamp` or `block_number` for randomness generation, and consider using oracles instead.


## References

- https://dasp.co/#item-6
- https://blog.sigmaprime.io/solidity-security.html#SP-6
- [SWC-120](https://swcregistry.io/docs/SWC-120)
- [SWC-116](https://swcregistry.io/docs/SWC-116)
- [Ethernaut: Coinflip](https://ethernaut.openzeppelin.com/level/0x4dF32584890A0026e56f7535d0f2C6486753624f)
- [Slither: Weak PRNG](https://github.com/crytic/slither/wiki/Detector-Documentation#weak-PRNG)
- [Slither: Dangerous usage of block.timestamp](https://github.com/crytic/slither/wiki/Detector-Documentation#block-timestamp)