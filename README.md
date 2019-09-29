# wasm-graph

WebAssembly format in graph representation!

[![Build Status](https://travis-ci.org/nikvolf/wasm-graph.svg?branch=master)](https://travis-ci.org/paritytech/parity-wasm)
[![crates.io link](https://img.shields.io/crates/v/wasm-graph.svg)](https://crates.io/crates/parity-wasm)

## Rust WebAssembly format serializing/deserializing

Add to Cargo.toml

```toml
[dependencies]
wasm-graph= "0.1"
```

and then

```rust

let module = wasm_graph::parse_file("hello.wasm").unwrap();
assert!(module.funcs.len() > 0);
```

# License

`wasm-graph` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in wasm-graph by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
