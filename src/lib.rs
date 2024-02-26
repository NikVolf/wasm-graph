//! WebAssembly binary format graph representation

#![warn(missing_docs)]

mod graph;
mod ref_list;

pub use graph::{
    Module, Func, Memory, Table, Global, Export, ElementSegment,
    DataSegment, ImportedOrDeclared, Instruction, Error, parse, FuncBody,
    generate, ExportLocal, SegmentLocation,
};

pub use ref_list::{RefList, EntryRef};

/// Parse file to graph representation.
// TODO: feature gate io
pub fn parse_file<P: AsRef<std::path::Path>>(p: P) -> Result<Module, graph::Error> {
    use parity_wasm::elements::Deserialize;

    let mut f = ::std::fs::File::open(p)
        .map_err(|e| parity_wasm::elements::Error::HeapOther(format!("Can't read from the file: {:?}", e)))?;

    graph::Module::from_elements(&parity_wasm::elements::Module::deserialize(&mut f)?)
}

/// Save graph representation to file.
// TODO: feature gate io
pub fn generate_file<P: AsRef<std::path::Path>>(module: &Module, p: P) -> Result<(), graph::Error> {
    use parity_wasm::elements::Serialize;

    let mut f = ::std::fs::File::create(p)
        .map_err(|e| parity_wasm::elements::Error::HeapOther(format!("Can't create file: {:?}", e)))?;

    Ok(module.generate()?.serialize(&mut f)?)
}
