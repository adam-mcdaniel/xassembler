use crate::{program, Compile, Target};

use alloc::string::String;

pub fn compile<T: Target>(script: &str) -> Result<String, String> {
    match program().parse(script) {
        Ok(ast) => match Compile::<T>::compile(ast) {
            Ok(code_gen) => Ok(code_gen.replace(";", ";\n\t")),
            Err(e) => Err(format!("{:?}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}
