use wasm_graph;
use parity_wasm::elements::{Type, FunctionType, ValueType};

use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: {} input_file.wasm output_file.wasm", args[0]);
        return;
    }

    let mut module = wasm_graph::parse_file(&args[1]).expect("Failed to load");

    let type_ref = module.types.push(
        Type::Function(FunctionType::new(
            vec![ValueType::I32, ValueType::I32],
            None,
        ))
    );

    module.funcs.push(
        wasm_graph::Func {
            type_ref: type_ref.clone(),
            origin: wasm_graph::ImportedOrDeclared::Imported("env".to_string(), "debug".to_string())
        }
    );

    wasm_graph::generate_file(&module, &args[2]).expect("Failed to save");
}
