use wasm_graph::{parse_file, generate_file, ImportedOrDeclared, Func, Instruction};
use parity_wasm::elements::{self, Type, FunctionType, ValueType};

use std::env;

//
// This example adds logging of index of called function, to each function!
//

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: {} input_file.wasm output_file.wasm", args[0]);
        return;
    }

    let mut module = parse_file(&args[1]).expect("Failed to load");

    let type_ref = module.types.push(
        Type::Function(FunctionType::new(
            vec![ValueType::I32],
            None,
        ))
    );

    let debug_func = module.funcs.push(
        Func {
            type_ref: type_ref.clone(),
            origin: ImportedOrDeclared::Imported("env".to_string(), "debug".to_string())
        }
    );

    for f in module.funcs.iter() {
        let idx = f.order().expect("All iterable should not be detached");
        let mut f = f.write();
        match f.origin {
            ImportedOrDeclared::Declared(ref mut b) => {
                b.code.insert(0, Instruction::Plain(elements::Instruction::I32Const(idx as i32)));
                b.code.insert(1, Instruction::Call(debug_func.clone()));
            },
            _ => continue,
        }
    }

    generate_file(&module, &args[2]).expect("Failed to save");
}
