use crate::ArgValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArgMap {
    pub values: HashMap<String, Option<ArgValue>>,
}

impl ArgMap {
    /// Returns `Option<ArgValue>` from the
    ///
    /// # Panic
    /// * When accessing an unregistered Argument, (register via ArgReader.register())
    pub fn get(&self, key: &str) -> Option<ArgValue> {
        match self.values.get(key) {
            None => {
                eprintln!("Register '{}', via ArgReader.register({})", key, key);
                panic!("Unregistered Argument being accessed: '{}'", key);
            }
            Some(value) => return value.clone(),
        }
    }

    pub fn get_as_float(&self, key: &str) -> Option<f64> {
        let value = self.get(key);
        match value {
            None => None,
            Some(arg_value) => arg_value.as_float(),
        }
    }

    pub fn get_as_int(&self, key: &str) -> Option<i64> {
        let value = self.get(key);
        match value {
            None => None,
            Some(arg_value) => arg_value.as_int(),
        }
    }

    pub fn get_as_string(&self, key: &str) -> Option<String> {
        let value = self.get(key);
        match value {
            None => None,
            Some(arg_value) => arg_value.as_string(),
        }
    }

    pub fn get_as_bool(&self, key: &str) -> Option<bool> {
        let value = self.get(key);
        match value {
            None => None,
            Some(arg_value) => arg_value.as_bool(),
        }
    }

    pub fn flag_is_set(&self, key: &str) -> bool {
        let value = self.get(key);
        match value {
            None => false,
            Some(arg_value) => arg_value.as_bool().unwrap_or(false),
        }
    }
}
