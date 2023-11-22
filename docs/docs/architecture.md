---
sidebar_position: 5
---

# Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/CoinFabrik/scout/main/assets/scout-architecture.png" alt="Scout Architectural Diagram"/>
</p>


Scout is built on Trail of Bits’ [Dylint](https://github.com/trailofbits/dylint), featuring a new set of lints. Dylint is a static analyzer that interfaces with the Rust compiler, providing access to the High-Level Intermediate Representation and the Mid-Level Intermediate Representation. These representations enable the accurate capture of many vulnerabilities. The lints are specifically designed to detect certain vulnerability classes. They are files integrated into the tool during compilation, and adding new lints, or detectors as we call them, is straightforward for any contributor. We have also contributed to the Dylint project, enhancing its capabilities to produce outputs in various formats, including PDF reports.

