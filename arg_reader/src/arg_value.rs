/// Represents a value that can be passed to a command line argument.
#[derive(Debug, Clone, PartialEq)]
pub enum ArgValue {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

impl ArgValue {
    /// Convert the value to a boolean
    ///
    /// Returns `Some(true)` if the value is `true`,
    /// `Some(false)` if the value is `false`,
    /// and `None` if the value is not a boolean
    ///
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ArgValue::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Convert the value to a string
    ///
    /// Returns `Some(String)` if the value is a string,
    /// and `None` if the value is not a string
    pub fn as_string(&self) -> Option<String> {
        match self {
            ArgValue::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// Convert the value to an integer
    ///
    /// Returns `Some(i64)` if the value is an integer,
    /// and `None` if the value is not an integer
    pub fn as_int(&self) -> Option<i64> {
        match self {
            ArgValue::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Convert the value to a float
    ///
    /// Returns `Some(f64)` if the value is a float,
    /// and `None` if the value is not a float
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ArgValue::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Parses the value into its corresponding Value Types
    ///
    ///
    ///
    /// Returns `ArgValue::Bool(boolean)` if
    pub fn parse(value: &str) -> Self {
        if value == "true" {
            Self::Bool(true)
        } else if value == "false" {
            Self::Bool(false)
        } else if let Ok(value) = value.parse::<i64>() {
            Self::Int(value)
        } else if let Ok(value) = value.parse::<f64>() {
            Self::Float(value)
        } else {
            Self::String(value.to_string())
        }
    }
}

impl From<String> for ArgValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ArgValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<bool> for ArgValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for ArgValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for ArgValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<&i64> for ArgValue {
    fn from(value: &i64) -> Self {
        Self::Int(*value)
    }
}

impl From<&f64> for ArgValue {
    fn from(value: &f64) -> Self {
        Self::Float(*value)
    }
}

impl From<&bool> for ArgValue {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}

impl From<&String> for ArgValue {
    fn from(value: &String) -> Self {
        Self::String(value.to_string())
    }
}
