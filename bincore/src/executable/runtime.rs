use std::collections::HashMap;
use crate::data::function::Callable;
use crate::executable::runnable::{Instruction};
use crate::data::object::{Object, ObjectBuilder};
use crate::data::value::Value;

pub struct Runtime {
    pub stack: Vec<Value>,
    pub functions: HashMap<String, Callable>,
    pub heap: HashMap<String, Value>,
    pub object_builders: HashMap<String, ObjectBuilder>,
    pub object_init_counter: usize,
    pub objects: HashMap<usize, Object>
}

impl Runtime {
    pub fn new() -> Runtime {
        let mut object_builders = HashMap::new();

        // macro_rules! literals_object_builder_add {
        //     ($type:ty) => {
        //         object_builders.insert(stringify!($type).to_string(),
        //             ObjectBuilder::new(stringify!($type).to_string(), ObjectDescriptor {
        //                 name: stringify!($type).to_string(),
        //                 members: HashMap::from([("__value__".to_string(), 0)])
        //             })
        //         );
        //     };
        // }
        //
        // literals_object_builder_add!(int);
        // literals_object_builder_add!(float);
        // literals_object_builder_add!(str);
        // literals_object_builder_add!(bool);
        // literals_object_builder_add!(list);
        //


        Runtime {
            object_init_counter: 0,
            stack: Vec::new(),
            functions: HashMap::new(),
            heap: HashMap::new(),
            object_builders,
            objects: Default::default(),
        }
    }

    pub fn value_deref_object(&self, value: &Value) -> &Object {
        match value {
            Value::ObjectRef(object) => {
                self.objects.get(&object).unwrap()
            }
            _ => {
                panic!("Expected object")
            }
        }
    }

    pub fn value_deref_mut_object(&mut self, value: &Value) -> &mut Object {
        match value {
            Value::ObjectRef(object) => {
                self.objects.get_mut(&object).unwrap()
            }
            _ => {
                panic!("Expected object")
            }
        }
    }


    pub fn register_function(&mut self, name: String, function: Callable) {
        self.functions.insert(name, function);
    }

    pub fn object_to_literal_value(&mut self, object: &Object) -> Value {
        match object.descriptor.name.as_str() {
            "int" | "float" | "str" | "bool" | "list" => {
                object.get_member("__value__").clone()
            }
            _ => {
                panic!("Cannot convert object {} to literal value", object.descriptor.name)
            }
        }
    }

    pub fn load_from_heap(&mut self, name: String) -> Value {
        let names = name.split(".").collect::<Vec<&str>>();

        let mut value = self.heap.get(names[0]).unwrap();

        for i in 1..names.len() {
            let res = self.value_deref_object(value);
            value = res.get_member(names[i]);
        }

        value.clone()
    }

    pub fn execute(&mut self, instruction: Instruction) -> Option<Value> {
        match instruction {
            Instruction::Push { value } => {
                self.stack.push(value);
            }
            Instruction::Call { function } => {
                let function = self.functions.get(&function).unwrap().clone();
                match function {
                    Callable::Function(func) => {
                        for arg in func.args.iter().rev() {
                            let value = self.stack.pop().unwrap();
                            self.heap.insert(arg.clone(), value);
                        }
                        if let Some(value) = self.execute_instructions(func.instructions.clone()) {
                            self.stack.push(value);
                        }
                    }
                    Callable::NativeFunction(native_func) => {
                        native_func(self);
                    }
                }
            }
            Instruction::Store { name } => {
                let names = name.split(".").collect::<Vec<&str>>();

                if names.len() == 1 {
                    let value = self.stack.pop().unwrap();

                    self.heap.insert(name, value);
                } else {
                    let value = self.stack.pop().unwrap();

                    let mut object_ref = self.heap.get(names[0]).unwrap().clone();

                    for i in 1..names.len() - 1 {
                        let res = self.value_deref_object(&object_ref);
                        object_ref = res.get_member(names[i]).clone();
                    }

                    self.objects.get_mut(object_ref.as_object_ref().unwrap())
                        .unwrap()
                        .set_member(names[names.len() - 1], value);
                }

            }
            Instruction::Load { name } => {
                let value = self.load_from_heap(name);
                self.stack.push(value);
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
            Instruction::Ret => {
                return Some(self.stack.pop().unwrap());
            }
            Instruction::CreateObject { name } => {
                let object_builder = self.object_builders.get(&name).unwrap();
                let len = object_builder.descriptor.members.len();

                let mut args = vec![];
                for _ in 0..len {
                    args.push(self.stack.pop().unwrap());
                }

                let object = object_builder.build(args);
                let object_ref = self.object_init_counter;

                self.object_init_counter += 1;

                self.objects.insert(object_ref, object);
                self.stack.push(Value::ObjectRef(object_ref));
            }
        }
        None
    }

    pub fn execute_instructions(&mut self, instructions: Vec<Instruction>) -> Option<Value> {
        for instruction in instructions {
            if let Some(value) = self.execute(instruction) {
                return Some(value);
            }
        }

        None
    }

    pub fn run(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            self.execute(instruction);
        }
    }
}