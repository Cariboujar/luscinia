//! Format value types

/// Value that can be formatted
#[derive(Debug, Clone)]
pub enum FormatValue {
    /// Number value
    Number(f64),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
}

impl From<f64> for FormatValue {
    fn from(value: f64) -> Self {
        FormatValue::Number(value)
    }
}

impl From<i64> for FormatValue {
    fn from(value: i64) -> Self {
        FormatValue::Number(value as f64)
    }
}

impl From<i32> for FormatValue {
    fn from(value: i32) -> Self {
        FormatValue::Number(value as f64)
    }
}

impl From<String> for FormatValue {
    fn from(value: String) -> Self {
        FormatValue::String(value)
    }
}

impl From<&str> for FormatValue {
    fn from(value: &str) -> Self {
        FormatValue::String(value.to_string())
    }
}

impl From<bool> for FormatValue {
    fn from(value: bool) -> Self {
        FormatValue::Boolean(value)
    }
}
