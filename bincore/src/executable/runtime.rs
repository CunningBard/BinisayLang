use std::collections::HashMap;
use std::rc::Rc;
use crate::executable::runnable::{FunctionSignature, Instruction};
use crate::object::{Object, ObjectBuilder, ObjectDescriptor, Value};

pub struct Runtime {
    pub stack: Vec<Value>,
    pub functions: HashMap<String, Rc<Box<FunctionSignature>>>,
    pub heap: HashMap<String, Object>,
    pub object_builders: HashMap<String, ObjectBuilder>
}

impl Runtime {
    pub fn new() -> Runtime {
        let mut object_builders = HashMap::new();

        macro_rules! literals_object_builder_add {
            ($type:ty) => {
                object_builders.insert(stringify!($type).to_string(),
                    ObjectBuilder::new(stringify!($type).to_string(), ObjectDescriptor {
                        name: stringify!($type).to_string(),
                        members: HashMap::from([("__value__".to_string(), 0)])
                    })
                );
            };
        }

        literals_object_builder_add!(int);
        literals_object_builder_add!(float);
        literals_object_builder_add!(str);
        literals_object_builder_add!(bool);
        literals_object_builder_add!(list);



        Runtime {
            stack: Vec::new(),
            functions: HashMap::new(),
            heap: HashMap::new(),
            object_builders
        }
    }

    pub fn register_function(&mut self, name: String, function: Rc<Box<FunctionSignature>>) {
        self.functions.insert(name, function);
    }

    pub fn execute(&mut self, instruction: Instruction){
        match instruction {
            Instruction::Push { value } => {
                self.stack.push(value);
            }
            Instruction::Call { function } => {
                let function = self.functions.get(&function).unwrap().clone();
                function(self);
            }
            Instruction::Store { name } => {
                let names = name.split(".").collect::<Vec<&str>>();

                if names.len() == 1 {
                    let value = self.stack.pop().unwrap();

                    match value {
                        Value::Int(values) => {
                            let object = self.object_builders.get("int").unwrap().build(vec![Value::Int(values)]);
                            self.heap.insert(name, object);
                        }
                        Value::Float(values) => {
                            let object = self.object_builders.get("float").unwrap().build(vec![Value::Float(values)]);
                            self.heap.insert(name, object);
                        }
                        Value::Str(values) => {
                            let object = self.object_builders.get("str").unwrap().build(vec![Value::Str(values)]);
                            self.heap.insert(name, object);
                        }
                        Value::Bool(values) => {
                            let object = self.object_builders.get("bool").unwrap().build(vec![Value::Bool(values)]);
                            self.heap.insert(name, object);
                        }
                        Value::List(values) => {
                            let object = self.object_builders.get("list").unwrap().build(vec![Value::List(values)]);
                            self.heap.insert(name, object);
                        }
                        Value::Object(_) => {
                            panic!("Expected literal value, not object")
                        }
                    }
                } else {
                    let mut object = self.heap.get_mut(names[0]).unwrap();
                    for i in 1..names.len() - 1 {
                        object = object.get_member_mut(names[i]).as_object_mut().unwrap();
                    }
                    let value = self.stack.pop().unwrap();
                    object.set_member(names[names.len() - 1], value);
                }

            }
            Instruction::Load { name } => {
                let names = name.split(".").collect::<Vec<&str>>();

                if names.len() == 1 {
                    let object = self.heap.get(&name).unwrap().clone();
                    let value = object.get_member("__value__").clone();
                    self.stack.push(value);
                } else {
                    let mut object = self.heap.get(names[0]).unwrap();
                    for i in 1..names.len() - 1 {
                        object = object.get_member(names[i]).as_object().unwrap();
                    }
                    let value = object.get_member(names[names.len() - 1]).clone();

                    if value.is_object() {
                        panic!("Expected literal value, not object")
                    } else {
                        self.stack.push(value);
                    }
                }
            }
            Instruction::Add => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack.push(Value::Int(left + right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack.push(Value::Float(left + right));
                    }
                    (Value::Str(left), Value::Str(right)) => {
                        self.stack.push(Value::Str(left + &right));
                    }
                    (Value::List(mut left), Value::List(right)) => {
                        left.extend(right);
                        self.stack.push(Value::List(left));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Sub => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack.push(Value::Int(left - right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack.push(Value::Float(left - right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Mul => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack.push(Value::Int(left * right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack.push(Value::Float(left * right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Div => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack.push(Value::Int(left / right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack.push(Value::Float(left / right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Mod => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack.push(Value::Int(left % right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack.push(Value::Float(left % right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Pow => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack.push(Value::Int(left.pow(right as u32)));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack.push(Value::Float(left.powf(right)));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
        }
    }

    pub fn run(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            self.execute(instruction);
        }
    }
}