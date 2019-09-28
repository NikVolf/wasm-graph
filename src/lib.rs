//! WebAssembly binary format graph representation

#![warn(missing_docs)]

extern crate parity_wasm;

mod graph;
mod ref_list;

pub use graph::{Module, parse, generate};