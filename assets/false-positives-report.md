# False Positives Report

## Summary
We wanted to test our tool in code that is being used out in the wild. We checked several sources including [Parity's awesome-ink](https://github.com/paritytech/awesome-ink), and the [use-ink examples](https://use.ink/examples/dapps). The main purpose of these runs was to experiment with realistic smart contracts, ensuring that cargo scout ran without problems and measuring false positives (fixing them when possible). 

We selected 5 smart contracts, found a some vulnerabilities and two false positives. We fixed one of the false positives, which was a fuction that appeared to return a Result type not of the enum variant (Ok/Err). The second false positive required more work on a detector and remains unfixed (and added to our backlog).

## Detailed Description
We selected five smart contracts to work on.
- The **Phat Bricks Version of Oracle** smart contract. Find the code [here](https://github.com/Phala-Network/phat-bricks) and its deployment [here](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpoc5.phala.network%2Fws#/explorer) in the Phala Network.
- The **psp22**, **trading_pair_psp22**, **vesting_contract** and **multi_sig** smart contracts, all to be found in [this](https://github.com/RottenKiwi/Panorama-Swap-INK-SC) repository. We did not find this were deployed.

Interestingly, in the phat bricks contract we found a false positive in `action_evm_transaction` with the `unused_return_enum` detector. We had to add a new check in order to fix this.

For the panorama swap contracts, **scout audit** detected integer overflow/underflows, an [unsafe expect](https://coinfabrik.github.io/scout/docs/detectors/unsafe-expect) in `psp22`.
