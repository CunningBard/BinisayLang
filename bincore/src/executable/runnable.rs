use serde::{Deserialize, Serialize};
use crate::data::value::Value;


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Instruction {
    Push {
        value: Value
    },
    Call {
        function: String
    },
    Store {
        name: String
    },
    Load {
        name: String
    },
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Ret
}