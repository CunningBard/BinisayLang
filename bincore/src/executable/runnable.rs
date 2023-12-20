use crate::executable::runtime::Runtime;
use crate::object::{Value};

pub type FunctionSignature = dyn Fn(&mut Runtime);

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
}