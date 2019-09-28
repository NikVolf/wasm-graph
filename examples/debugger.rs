extern crate wasm_graph;

use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: {} input_file.wasm output_file.wasm", args[0]);
        return;
    }

    let module = wasm_graph::parse_file(&args[1]).expect("Failed to load");
}
