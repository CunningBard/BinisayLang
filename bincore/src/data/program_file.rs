use serde::{Deserialize, Serialize};
use crate::data::function::{Function};
use crate::executable::runnable::Instruction;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub functions: Vec<Function>
}