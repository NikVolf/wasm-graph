//! WebAssembly binary format graph representation

#![warn(missing_docs)]

mod graph;
mod ref_list;

pub use graph::{Module, parse, generate};

// TODO: feature gate io
pub fn parse_file<P: AsRef<std::path::Path>>(p: P) -> Result<Module, graph::Error> {
    use parity_wasm::elements::Deserialize;

    let mut f = ::std::fs::File::open(p)
        .map_err(|e| parity_wasm::elements::Error::HeapOther(format!("Can't read from the file: {:?}", e)))?;

    graph::Module::from_elements(&parity_wasm::elements::Module::deserialize(&mut f)?)
}
