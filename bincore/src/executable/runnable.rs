use serde::{Deserialize, Serialize};
use crate::data::value::Value;


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Instruction {
    Nop,
    Push {
        value: Value
    },
    ExternCall {
        function: String
    },
    Store {
        name: String
    },
    Load {
        name: String
    },
    CreateObject {
        name: String
    },
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Ret,
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
    And,
    Or,
    Not,
    Call {
        address: usize
    },
    Jump {
        address: usize
    },
    JumpIfTrue {
        address: usize
    },
    JumpIfFalse {
        address: usize
    },
}