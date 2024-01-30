use crate::data::object::ObjectDescriptor;
use crate::data::value::Value;
use crate::executable::runnable::Instruction;
use crate::executable::runtime::Runtime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub strings: Vec<String>,
    pub heap_size: usize,

    pub object_descriptor: Vec<ObjectDescriptor>,
}

impl Program {
    pub fn into_runtime(self) -> Runtime {
        let mut runtime = Runtime {
            instructions: self.instructions,
            strings: self.strings.clone(),
            object_descriptor: self.object_descriptor,
            heap: vec![Value::Int(0); self.heap_size],

            ..Runtime::new()
        };

        for (index, string) in self.strings.iter().enumerate() {
            runtime.string_objects.insert(index, string.clone());
        }

        runtime.string_object_init_counter = self.strings.len();

        runtime
    }
}
