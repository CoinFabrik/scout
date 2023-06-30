---
sidebar_position: 3
---

# Detectors

In this section we introduce our set of detectors powered by [Dylint](https://github.com/trailofbits/dylint) - a Rust linting tool. 

Similar to [Clippy](https://github.com/rust-lang/rust-clippy), Dylint can run lints to help identify potential issues in code. However, unlike Clippy, Dylint can run lints from user-specified dynamic libraries instead of just a statically predetermined set. This unique feature of Dylint makes it easier for developers to extend and customize their own personal lint collections, leading to reduced compile and run cycles.

Check our [Proof of Concept Study](https://github.com/CoinFabrik/web3-grant/tree/main/detectors) for a more detailed analysis of different detection techniques and tools.