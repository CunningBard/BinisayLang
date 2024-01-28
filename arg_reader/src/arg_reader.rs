use std::collections::{HashMap, VecDeque};
use crate::arg_value::ArgValue;


pub fn correct_binding(binding: &str) -> String {
    if binding.len() == 1 {
        format!("-{}", binding)
    } else {
        format!("--{}", binding.replace("_", "-").replace(" ", "-"))
    }
}



#[derive(Debug, Clone)]
pub struct ArgReader {
    args: HashMap<String, Option<ArgValue>>,
    bindings: HashMap<String, String>,
    bindings_with_required: HashMap<String, String>,
    positional_args: VecDeque<String>,
    current_arg: Option<String>,
}


impl ArgReader {
    pub fn new() -> Self {
        Self {
            args: HashMap::new(),
            bindings: HashMap::new(),
            bindings_with_required: HashMap::new(),
            positional_args: VecDeque::new(),
            current_arg: None,
        }
    }

    pub fn register(mut self, arg: &str) -> Self {
        self.current_arg = Some(arg.to_string());
        self
    }

    pub fn bind(mut self, bindings: Vec<&str>) -> Self {
        let arg = self.current_arg.unwrap();

        for binding in bindings {
            self.bindings.insert(correct_binding(binding), arg.to_string());
        }

        self
    }


    pub fn bind_with_required(mut self, bindings: Vec<&str>) -> Self {
        let arg = self.current_arg.unwrap();

        for binding in bindings {
            self.bindings_with_required.insert(correct_binding(binding), arg.to_string());
        }

        self
    }

    pub fn bind_positional(mut self, arg: &str) -> Self {
        self.positional_args.push_back(arg.to_string());
        self
    }

    pub fn read_args(mut self) -> HashMap<String, Option<ArgValue>> {
        let mut args = std::env::args().skip(1).peekable();

        while let Some(arg) = args.next() {
            if let Some(arg) = self.bindings.get(&arg) {
                self.args.insert(arg.to_string(), None);
            } else if let Some(arg) = self.bindings_with_required.get(&arg) {
                if let Some(value) = args.peek() {
                    self.args.insert(arg.to_string(), Some(ArgValue::parse(value)));
                    args.next();
                } else {
                    panic!("Expected value for argument {}", arg);
                }
            } else if arg.contains("=") {
                let mut args = arg.split("=");
                let arg_name = args.next().unwrap();
                let value = args.next().unwrap();

                if let Some(arg) = self.bindings_with_required.get(&arg_name) {
                    self.args.insert(arg.to_string(), Some(ArgValue::parse(value)));
                } else {
                    panic!("Unknown argument, no matching required arg found {}", arg_name);
                }
            }
            else
            {
                if let Some(arg) = self.positional_args.pop_front() {
                    self.args.insert(arg.to_string(), Some(ArgValue::parse(&arg)));
                } else {
                    panic!("Unknown argument {}", arg);
                }
            }
        }

        self.args
    }
}
