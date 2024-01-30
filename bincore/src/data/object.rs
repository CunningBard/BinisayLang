use crate::data::value::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectDescriptor {
    pub name: String,
    pub members: HashMap<String, usize>,
    pub members_by_index: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub descriptor: Rc<ObjectDescriptor>,
    pub members: Vec<Value>,
}
