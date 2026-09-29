# RPG Character & Data Designer

## Proof of Concept

This repository contains a proof-of-concept implementation of the
RPG Character & Data Designer.

The POC demonstrates:

- RPG data represented using Rust structures
- Race and class data loaded from JSON
- Character creation
- Automatic stat calculation
- Serialization/deserialization using Serde

Current limitations: 
The prototype uses a predefined character and fixed race/class selections. Users can inspect the available definitions and see calculated character statistics, but interactive character creation, data validation, and character saving are not yet implemented.

## Requirements

- Windows 10/11
- Rust 1.96.0
- Cargo 1.96.0

## Build & Run

Clone the repository and run:

```bash
cargo build
cargo run
