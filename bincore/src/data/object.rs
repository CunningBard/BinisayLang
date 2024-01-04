use std::collections::HashMap;
use std::rc::Rc;
use crate::data::value::Value;

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
            panic!("Wrong number of arguments for data {}", self.name);
        }

        Object {
            members: args,
            descriptor: self.descriptor.clone()
        }
    }
}

