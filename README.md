# skani-lib-example
Example for how to use skani as a rust library. 

## Introduction

skani is a tool for calculating ANI from metagenomic sequences very quickly using sparse chaining. 

skani is also available on crates.io here: https://crates.io/crates/skani. Due to requests for using skani through a rust API, I have included
in this repository an example of how to use skani as a library. 

Depending on the interest, I may clean up the API and keep the https://docs.rs/skani/0.1.0/skani/ docs more updated.. For now, just see the example and check out [pyskani](https://github.com/althonos/pyskani/tree/main/pyskani) for hints on how to wrap skani's internals. 

## Usage

see `src/main.rs` for the main example and `Cargo.toml` for how to add skani as a dependency:

```rust
[dependencies]
skani = "0.1.0"
```
 
You'll probably want to use a specific version right now (0.1.0).

simply run 
```sh
cd skani-lib-example
cargo run
```
