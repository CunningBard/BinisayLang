use std::collections::HashMap;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::data::object::{ObjectBuilder, ObjectDescriptor};
use crate::executable::runnable::Instruction;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObjectBuilderLayout {
    pub fields: Vec<String>,
    pub members: HashMap<String, usize>,
    pub name: String
}

impl ObjectBuilderLayout {
    pub fn into_builder(self) -> ObjectBuilder {
        ObjectBuilder {
            name: self.name.clone(),
            descriptor: Rc::new(ObjectDescriptor {
                name: self.name.clone(),
                members: self.members
            })
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub object_builder: Vec<ObjectBuilderLayout>
}