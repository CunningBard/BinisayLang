use serde::{Deserialize, Serialize};
use crate::executable::runnable::Instruction;
use crate::executable::runtime::Runtime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub instructions: Vec<Instruction>
    // pub last_is_variadic: bool,
}

pub type FunctionSignature = fn(&mut Runtime);

pub enum Callable {
    Function(Function),
    NativeFunction(FunctionSignature)
}