use crate::data::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Copy)]
pub enum Instruction {
    Nop,
    Push { value: Value },
    ExternCall { string_id: usize },
    Store { address: usize },
    Load { address: usize },
    AccessMember { index: usize },
    SetMember { index: usize },
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
    Call { address: usize },
    Jump { address: usize },
    JumpIfTrue { address: usize },
    JumpIfFalse { address: usize },
    CreateObject { descriptor: usize },
}
