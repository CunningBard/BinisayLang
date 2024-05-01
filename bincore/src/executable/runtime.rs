use crate::data::function::FunctionSignature;
use crate::data::object::{Object, ObjectDescriptor};
use crate::data::value::Value;
use crate::executable::runnable::Instruction;
use std::collections::HashMap;
use std::rc::Rc;


const STACK_SIZE: usize = 1024;
const STACK_THRESHOLD: usize = 10;

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
    pub strings: Vec<String>,

    pub stack: Vec<Value>,
    pub stack_pointer: usize,
    pub functions: HashMap<String, FunctionSignature>,
    pub call_stack: Vec<usize>,
    pub heap: Vec<Value>,

    pub object_descriptor: Vec<ObjectDescriptor>,
    pub object_init_counter: usize,
    pub objects: HashMap<usize, Object>,

    pub list_init_counter: usize,
    pub lists: HashMap<usize, Vec<Value>>,
    pub string_object_init_counter: usize,
    pub string_objects: HashMap<usize, String>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            instructions: vec![],
            instruction_pointer: 1,
            strings: vec![],

            stack: vec![Value::Int(0); STACK_SIZE],
            stack_pointer: 0,
            functions: HashMap::new(),
            call_stack: vec![],
            heap: vec![],

            object_descriptor: Vec::new(),
            object_init_counter: 0,
            objects: Default::default(),

            list_init_counter: 0,
            lists: Default::default(),
            string_object_init_counter: 0,
            string_objects: Default::default(),
        }
    }

    #[inline]
    pub fn register_function(&mut self, name: String, function: FunctionSignature) {
        self.functions.insert(name, function);
    }

    #[inline]
    pub fn load_from_heap(&mut self, address: usize) -> Value {
        self.heap[address]
    }

    #[inline]
    pub fn new_string(&mut self, string: String) -> Value {
        let string_id = self.string_object_init_counter;
        self.string_object_init_counter += 1;

        self.string_objects.insert(string_id, string.clone());

        Value::StrRef(string_id)
    }

    #[inline]
    pub fn stack_pop(&mut self) -> Value {
        self.stack_pointer -= 1;
        let value = self.stack[self.stack_pointer];
        value
    }

    #[inline]
    pub fn stack_push(&mut self, value: Value) {
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    #[inline]
    pub fn execute(&mut self, instruction: Instruction) -> Option<Value> {
        if self.stack_pointer + STACK_THRESHOLD >= self.stack.len() {
            self.stack
                .resize(self.stack.len() + STACK_SIZE, Value::Int(0));
        }

        match instruction {
            Instruction::Push { value } => {
                self.stack_push(value);
            }
            Instruction::ExternCall { string_id } => {
                let name = &*self.strings[string_id];
                let function = match self.functions.get(name) {
                    Some(function) => function,
                    None => panic!("Function '{}' not found", name),
                };

                function(self);
            }
            Instruction::Store { address } => {
                let value = self.stack_pop();
                self.heap[address] = value;
            }
            Instruction::Load { address } => {
                let value = self.load_from_heap(address);
                self.stack_push(value);
            }
            Instruction::Add => {
                let right = self.stack_pop();
                let left = self.stack_pop();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack_push(Value::Int(left + right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack_push(Value::Float(left + right));
                    }
                    (Value::Char(left), Value::Char(right)) => {
                        let new_string = left.to_string() + &right.to_string();
                        let str_ref = self.new_string(new_string);
                        self.stack_push(str_ref);
                    }
                    (Value::StrRef(left), Value::StrRef(right)) => {
                        let left = self.string_objects.get(&left).unwrap();
                        let right = self.string_objects.get(&right).unwrap();

                        let new_string = left.to_string() + right;
                        let str_ref = self.new_string(new_string);
                        self.stack_push(str_ref);
                    }
                    (Value::StrRef(left), Value::Char(right)) => {
                        let left = self.string_objects.get(&left).unwrap();

                        let new_string = left.to_string() + &right.to_string();
                        let str_ref = self.new_string(new_string);
                        self.stack_push(str_ref);
                    }
                    (Value::Char(left), Value::StrRef(right)) => {
                        let right = self.string_objects.get(&right).unwrap();

                        let new_string = left.to_string() + &right;
                        let str_ref = self.new_string(new_string);
                        self.stack_push(str_ref);
                    }
                    (Value::ListRef(left), Value::ListRef(right)) => {
                        let mut list = self.lists.get(&left).unwrap().clone();
                        list.extend(self.lists.get(&right).unwrap().clone());
                        self.stack_push(Value::ListRef(left));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Sub => {
                let right = self.stack_pop();
                let left = self.stack_pop();
                self.stack_push(bin_op_2!(left, right, -));
            }
            Instruction::Mul => {
                let right = self.stack_pop();
                let left = self.stack_pop();
                self.stack_push(bin_op_2!(left, right, *));
            }
            Instruction::Div => {
                let right = self.stack_pop();
                let left = self.stack_pop();
                self.stack_push(bin_op_2!(left, right, /));
            }
            Instruction::Mod => {
                let right = self.stack_pop();
                let left = self.stack_pop();
                self.stack_push(bin_op_2!(left, right, %));
            }
            Instruction::Pow => {
                let right = self.stack_pop();
                let left = self.stack_pop();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack_push(Value::Int(left.pow(right as u32)));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack_push(Value::Float(left.powf(right)));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Ret => {
                self.instruction_pointer = self.call_stack.pop().unwrap();
            }
            Instruction::Gt => {
                let right = self.stack_pop();
                let left = self.stack_pop();
                self.stack_push(bin_op_2_comp!(left, right, >));
            }
            Instruction::Lt => {
                let right = self.stack_pop();
                let left = self.stack_pop();
                self.stack_push(bin_op_2_comp!(left, right, <));
            }
            Instruction::Gte => {
                let right = self.stack_pop();
                let left = self.stack_pop();
                self.stack_push(bin_op_2_comp!(left, right, >=));
            }
            Instruction::Lte => {
                let right = self.stack_pop();
                let left = self.stack_pop();
                self.stack_push(bin_op_2_comp!(left, right, <=));
            }
            Instruction::Eq => {
                let right = self.stack_pop();
                let left = self.stack_pop();

                match (left, right) {
                    (Value::Char(left), Value::Char(right)) => {
                        self.stack_push(Value::Bool(left == right));
                    }
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack_push(Value::Bool(left == right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack_push(Value::Bool(left == right));
                    }
                    (Value::StrRef(left), Value::StrRef(right)) => {
                        let left = self.string_objects.get(&left).unwrap();
                        let right = self.string_objects.get(&right).unwrap();

                        self.stack_push(Value::Bool(left == right));
                    }
                    (Value::Bool(left), Value::Bool(right)) => {
                        self.stack_push(Value::Bool(left == right));
                    }
                    (Value::StrRef(left), Value::Char(right)) => {
                        let left = self.string_objects.get(&left).unwrap();

                        self.stack_push(Value::Bool(left == &right.to_string()));
                    }
                    (Value::Char(left), Value::StrRef(right)) => {
                        let right = self.string_objects.get(&right).unwrap();

                        self.stack_push(Value::Bool(&left.to_string() == right));
                    }
                    (Value::ListRef(left), Value::ListRef(right)) => {
                        let left = self.lists.get(&left).unwrap();
                        let right = self.lists.get(&right).unwrap();

                        self.stack_push(Value::Bool(left == right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Neq => {
                let right = self.stack_pop();
                let left = self.stack_pop();

                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        self.stack_push(Value::Bool(left != right));
                    }
                    (Value::Float(left), Value::Float(right)) => {
                        self.stack_push(Value::Bool(left != right));
                    }
                    (Value::StrRef(left), Value::StrRef(right)) => {
                        let left = self.string_objects.get(&left).unwrap();
                        let right = self.string_objects.get(&right).unwrap();

                        self.stack_push(Value::Bool(left != right));
                    }
                    (Value::Bool(left), Value::Bool(right)) => {
                        self.stack_push(Value::Bool(left != right));
                    }
                    (Value::ListRef(left), Value::ListRef(right)) => {
                        let left = self.lists.get(&left).unwrap();
                        let right = self.lists.get(&right).unwrap();

                        self.stack_push(Value::Bool(left != right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::And => {
                let right = self.stack_pop();
                let left = self.stack_pop();

                match (left, right) {
                    (Value::Bool(left), Value::Bool(right)) => {
                        self.stack_push(Value::Bool(left && right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Or => {
                let right = self.stack_pop();
                let left = self.stack_pop();

                match (left, right) {
                    (Value::Bool(left), Value::Bool(right)) => {
                        self.stack_push(Value::Bool(left || right));
                    }
                    _ => {
                        panic!("Expected two values of the same type")
                    }
                }
            }
            Instruction::Not => {
                let value = self.stack_pop();

                match value {
                    Value::Bool(value) => {
                        self.stack_push(Value::Bool(!value));
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
                let value = self.stack_pop();

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
                let value = self.stack_pop();

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
            Instruction::AccessMember { index } => {
                let object = self.stack_pop();
                let object = self.objects.get(&object.as_object_ref().unwrap()).unwrap();
                let value = object.members[index].clone();
                self.stack_push(value);
            }
            Instruction::SetMember { index } => {
                let value = self.stack_pop();
                let object = self.stack_pop();
                let object = self
                    .objects
                    .get_mut(&object.as_object_ref().unwrap())
                    .unwrap();
                object.members[index] = value;
            }
            Instruction::CreateObject { descriptor } => {
                let object_id = self.object_init_counter;
                self.object_init_counter += 1;

                let descriptor = self.object_descriptor[descriptor].clone();
                let mut members = vec![];

                for _ in 0..descriptor.members.len() {
                    let val = self.stack_pop();
                    members.push(val);
                }

                let object = Object {
                    descriptor: Rc::new(descriptor),
                    members,
                };

                self.objects.insert(object_id, object);
                self.stack_push(Value::ObjectRef(object_id));
            }
        }
        None
    }

    pub fn run(&mut self) {
        let instructions_length = self.instructions.len();
        while self.instruction_pointer < instructions_length {
            let instruction = self.instructions[self.instruction_pointer];
            self.instruction_pointer += 1;

            self.execute(instruction);
        }
    }
}
