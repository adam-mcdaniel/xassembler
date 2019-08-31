use crate::{program, Compile};

use alloc::string::String;

pub fn compile(script: &str) -> Result<String, String> {
    match program().parse(script) {
        Ok(ast) => {
            match ast.compile() {
                Ok(code_gen) => Ok(code_gen.replace(";", ";\n\t")),
                Err(e) => Err(format!("{:?}", e))
            }
        },
        Err(e) => {
            Err(format!("{:?}", e))
        }
    }
}
