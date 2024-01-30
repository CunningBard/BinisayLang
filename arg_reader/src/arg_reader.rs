use crate::arg_map::ArgMap;
use crate::arg_value::ArgValue;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum ReaderError {
    MissingRequiredArgument(String),
    UnknownArgument(String),
}

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

    /// Register an argument
    ///
    /// # Arguments
    /// * `arg` - The name of the argument
    ///
    /// # Examples
    /// ```
    /// use arg_reader::ArgReader;
    ///
    /// let env_args = vec!["--verbose".to_string()];
    /// // retrieved from std::env::args().skip(1).collect()
    ///
    ///
    /// let args = ArgReader::new()
    ///      .register("verbose")
    ///      .bind(vec!["v", "verbose"]) // will turn verbose true if -v or --verbose is passed
    ///      .read_args(env_args);
    ///
    ///
    /// assert_eq!(args.get("verbose").unwrap().as_ref().unwrap().as_bool(), Some(true));
    /// ```
    pub fn register(mut self, arg: &str) -> Self {
        if self.current_arg.is_some() {
            self.args
                .insert(self.current_arg.as_deref().unwrap().to_string(), None);
        }

        self.current_arg = Some(arg.to_string());
        self
    }

    /// Add a binding for the current argument
    ///
    /// # Arguments
    /// * `bindings` - A vector of strings representing the bindings for the current argument
    ///
    /// # Note
    ///
    /// * bindings will be appended with `--` or `-`, pass binding without the `--` or `-`
    /// * if a binding is a single character, `-` will be appended else `--`
    /// * passing `o` will convert it to `-o`
    /// * passing `output` will convert it to `--output`
    ///
    /// # Example
    ///
    /// ```
    /// use arg_reader::ArgReader;
    ///
    /// let env_args = vec!["--verbose".to_string()];
    /// // retrieved from std::env::args().skip(1).collect()
    ///
    ///
    /// let args = ArgReader::new()
    ///      .register("verbose")
    ///      .bind(vec!["v", "verbose"])
    ///      .read_args(env_args)
    ///      .unwrap();
    ///
    ///
    /// assert_eq!(args.flag_is_set("verbose"), true);
    /// ```
    pub fn bind(mut self, bindings: Vec<&str>) -> Self {
        let arg = self.current_arg.as_deref().unwrap();

        for binding in bindings {
            self.bindings
                .insert(correct_binding(binding), arg.to_string());
        }

        self
    }

    /// Add a binding with required parameter for the current argument
    ///
    /// # Arguments
    /// * `bindings` - A vector of strings representing the bindings for the current argument
    ///
    /// # Note
    ///
    /// * bindings will be appended with `--` or `-`, pass binding without the `--` or `-`
    /// * if a binding is a single character, `-` will be appended else `--`
    /// * passing `o` will convert it to `-o`
    /// * passing `output` will convert it to `--output`
    ///
    /// # Example
    ///
    /// ```
    /// use arg_reader::ArgReader;
    ///
    /// let env_args = vec!["--verbose".to_string()];
    /// // retrieved from std::env::args().skip(1).collect()
    ///
    ///
    /// let args = ArgReader::new()
    ///      .register("verbose")
    ///      .bind(vec!["v", "verbose"])
    ///      .read_args(env_args)
    ///      .unwrap();
    ///
    ///
    /// assert_eq!(args.flag_is_set("verbose"), true);
    /// ```
    pub fn bind_with_required(mut self, bindings: Vec<&str>) -> Self {
        let arg = self.current_arg.as_deref().unwrap();

        for binding in bindings {
            self.bindings_with_required
                .insert(correct_binding(binding), arg.to_string());
        }

        self
    }

    pub fn bind_positional(mut self, arg: &str) -> Self {
        self.positional_args.push_back(arg.to_string());
        self
    }

    /// Read arguments from the command line and return a ArgMap
    ///
    /// # Panics
    ///
    /// Panics if a required argument is not provided
    /// Panics if an unknown argument is provided
    ///
    /// # Arguments
    /// * `args` - A vector of strings representing the arguments passed to the program, *does not include* the program name
    ///
    /// # Returns
    ///
    /// A ArgMap, you can extract the values from the ArgMap
    pub fn read_args(mut self, args: Vec<String>) -> Result<ArgMap, ReaderError> {
        if self.current_arg.is_some() {
            self.args
                .insert(self.current_arg.as_deref().unwrap().to_string(), None);
        }

        let mut args = args.iter();

        while let Some(arg) = args.next() {
            if let Some(arg) = self.bindings.get(&*arg) {
                self.args
                    .insert(arg.to_string(), Some(ArgValue::from(true)));
            } else if let Some(arg) = self.bindings_with_required.get(&*arg) {
                if let Some(value) = args.next() {
                    self.args
                        .insert(arg.to_string(), Some(ArgValue::parse(value)));
                } else {
                    return Err(ReaderError::MissingRequiredArgument(arg.to_string()));
                }
            } else if arg.contains("=") {
                let mut args = arg.split("=");
                let arg_name = args.next().unwrap();
                let value = args.next().unwrap();

                if let Some(arg) = self.bindings_with_required.get(&*arg_name) {
                    self.args
                        .insert(arg.to_string(), Some(ArgValue::parse(value)));
                } else {
                    return Err(ReaderError::UnknownArgument(arg.to_string()));
                }
            } else {
                if arg.starts_with("-") {
                    return Err(ReaderError::UnknownArgument(arg.to_string()));
                } else if let Some(arg_name) = self.positional_args.pop_front() {
                    self.args
                        .insert(arg_name.to_string(), Some(ArgValue::parse(&arg)));
                } else {
                    return Err(ReaderError::UnknownArgument(arg.to_string()));
                }
            }
        }

        return Ok(ArgMap { values: self.args });
    }
}
