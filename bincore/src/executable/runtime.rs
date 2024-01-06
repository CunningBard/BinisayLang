use std::collections::HashMap;
use crate::data::function::{FunctionSignature};
use crate::executable::runnable::{Instruction};
use crate::data::object::{Object, ObjectBuilder};
use crate::data::value::Value;

macro_rules! bin_op_2 {
    ($left:expr, $right:expr, $op:tt) => {
        match ($left, $right) {
            (Value::Int(left), Value::Int(right)) => {
                Value::Int(left $op right)
            }
            (Value::Float(left), Value::Float(right)) => {
                Value::Float(left $op right)
            }
            _ => {
                panic!("Expected two values of the same type")
            }
        }
    }
}

macro_rules! bin_op_2_comp {
    ($left:expr, $right:expr, $op:tt) => {
        match ($left, $right) {
            (Value::Int(left), Value::Int(right)) => {
                Value::Bool(left $op right)
            }
            (Value::Float(left), Value::Float(right)) => {
                Value::Bool(left $op right)
            }
            _ => {
                panic!("Expected two values of the same type")
            }
        }
    }
}

pub struct Runtime {
    pub instructions: Vec<Instruction>,
    pub instruction_pointer: usize,

    pub stack: Vec<Value>,
    pub functions: HashMap<String, FunctionSignature>,
    pub call_stack: Vec<usize>,
    pub heap: HashMap<String, Value>,
    pub object_builders: HashMap<String, ObjectBuilder>,
    pub object_init_counter: usize,
    pub objects: HashMap<usize, Object>,
    pub list_init_counter: usize,
    pub lists: HashMap<usize, Vec<Value>>
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            instructions: vec![Instruction::Nop],
            instruction_pointer: 1,

            object_init_counter: 0,
            stack: Vec::new(),
            functions: HashMap::new(),
            call_stack: vec![],
            heap: HashMap::new(),
            object_builders: HashMap::new(),
            objects: Default::default(),
            list_init_counter: 0,
            lists: Default::default(),
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


    pub fn register_function(&mut self, name: String, function: FunctionSignature) {
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
            Instruction::ExternCall { function } => {
                let function = match self.functions.get(&function){
                    Some(function) => function,
                    None => panic!("Function '{}' not found", function)
                };

                function(self);
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
                    (Value::ListRef(left), Value::ListRef(right)) => {
                        let mut list = self.lists.get(&left).unwrap().clone();
                        list.extend(self.lists.get(&right).unwrap().clone());
                        self.stack.push(Value::ListRef(left));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Sub => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(bin_op_2!(left, right, -));
            }
            Instruction::Mul => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(bin_op_2!(left, right, *));
            }
            Instruction::Div => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(bin_op_2!(left, right, /));
            }
            Instruction::Mod => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(bin_op_2!(left, right, %));
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
                self.instruction_pointer = self.call_stack.pop().unwrap();
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
            Instruction::Gt => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(bin_op_2_comp!(left, right, >));
            }
            Instruction::Lt => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(bin_op_2_comp!(left, right, <));
            }
            Instruction::Gte => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(bin_op_2_comp!(left, right, >=));
            }
            Instruction::Lte => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(bin_op_2_comp!(left, right, <=));
            }
            Instruction::Eq => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack.push(Value::Bool(left == right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack.push(Value::Bool(left == right));
                    }
                    (Value::Str(left), Value::Str(right)) => {
                        self.stack.push(Value::Bool(left == right));
                    }
                    (Value::Bool(left), Value::Bool(right)) => {
                        self.stack.push(Value::Bool(left == right));
                    }
                    (Value::ListRef(left), Value::ListRef(right)) => {
                        let left = self.lists.get(&left).unwrap();
                        let right = self.lists.get(&right).unwrap();

                        self.stack.push(Value::Bool(left == right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Neq => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack.push(Value::Bool(left != right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack.push(Value::Bool(left != right));
                    }
                    (Value::Str(left), Value::Str(right)) => {
                        self.stack.push(Value::Bool(left != right));
                    }
                    (Value::Bool(left), Value::Bool(right)) => {
                        self.stack.push(Value::Bool(left != right));
                    }
                    (Value::ListRef(left), Value::ListRef(right)) => {
                        let left = self.lists.get(&left).unwrap();
                        let right = self.lists.get(&right).unwrap();

                        self.stack.push(Value::Bool(left != right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::And => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Bool(left), Value::Bool(right)) => {
                        self.stack.push(Value::Bool(left && right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Or => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();

                match (left, right) {
                    (Value::Bool(left), Value::Bool(right)) => {
                        self.stack.push(Value::Bool(left || right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Not => {
                let value = self.stack.pop().unwrap();

                match value {
                    Value::Bool(value) => {
                        self.stack.push(Value::Bool(!value));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Jump { address } => {
                self.instruction_pointer = address;
            }
            Instruction::JumpIfTrue { address } => {
                let value = self.stack.pop().unwrap();

                match value {
                    Value::Bool(value) => {
                        if value {
                            self.instruction_pointer = address;
                        }
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::JumpIfFalse { address } => {
                let value = self.stack.pop().unwrap();

                match value {
                    Value::Bool(value) => {
                        if !value {
                            self.instruction_pointer = address;
                        }
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Nop => {}
            Instruction::Call { address } => {
                self.call_stack.push(self.instruction_pointer);
                self.instruction_pointer = address;
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

    pub fn run_as_function(&mut self, instructions: Vec<Instruction>) -> Option<Value> {
        self.instructions.extend(instructions);

        while self.instruction_pointer < self.instructions.len() {
            let instruction = self.instructions[self.instruction_pointer].clone();
            self.instruction_pointer += 1;

            match self.execute(instruction){
                Some(value) => {
                    return Some(value);
                }
                None => {}

            }
        }

        None
    }

    pub fn run(&mut self, instructions: Vec<Instruction>) {
        self.instructions.extend(instructions);

        while self.instruction_pointer < self.instructions.len() {
            let instruction = self.instructions[self.instruction_pointer].clone();
            self.instruction_pointer += 1;

            self.execute(instruction);
        }
    }
}