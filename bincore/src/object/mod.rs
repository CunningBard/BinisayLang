use std::collections::HashMap;
use std::rc::Rc;



#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
    List(Vec<Value>),

    Object(Object)
}

impl Value {
    pub fn is_object(&self) -> bool {
        match self {
            Value::Object(_) => true,
            _ => false
        }
    }
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Value::Int(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            Value::Float(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_str(&self) -> Option<&String> {
        match self {
            Value::Str(value) => Some(value),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(value) => Some(value),
            _ => None
        }
    }

    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Value::Object(object) => Some(object),
            _ => None
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Value::Object(object) => Some(object),
            _ => None
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObjectDescriptor {
    pub name: String,
    pub members: HashMap<String, usize>
}
#[derive(Clone, Debug)]
pub struct Object {
    pub members: Vec<Value>,
    pub descriptor: Rc<ObjectDescriptor>
}

impl Object  {
    pub fn get_member_index(&self, name: &str) -> usize {
        match self.descriptor.members.get(name) {
            Some(index) => *index,
            None => panic!("Object {} does not have member {}", self.descriptor.name, name)
        }
    }
    pub fn get_member(&self, name: &str) -> &Value  {
        let index = self.get_member_index(name);
        self.members.get(index).unwrap()
    }

    pub fn get_member_mut(&mut self, name: &str) -> &mut Value  {
        let index = self.get_member_index(name);
        self.members.get_mut(index).unwrap()
    }

    pub fn set_member(&mut self, name: &str, value: Value ) {
        let index = self.get_member_index(name);
        self.members[index] = value;
    }

    pub fn get_member_by_index(&self, index: usize) -> &Value  {
        self.members.get(index).unwrap()
    }

    pub fn set_member_by_index(&mut self, index: usize, value: Value ) {
        self.members[index] = value;
    }
}

#[derive(Clone, Debug)]
pub struct ObjectBuilder {
    pub name: String,
    pub descriptor: Rc<ObjectDescriptor>,
}

impl ObjectBuilder {
    pub fn new(name: String, object_descriptor: ObjectDescriptor) -> Self {
        Self {
            name: name.clone(),
            descriptor: Rc::new(object_descriptor)
        }
    }
    pub fn build(&self, args: Vec<Value>) -> Object {
        if args.len() != self.descriptor.members.len() {
            panic!("Wrong number of arguments for object {}", self.name);
        }

        Object {
            members: args,
            descriptor: self.descriptor.clone()
        }
    }
}

