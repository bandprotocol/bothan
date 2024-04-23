use serde::{Deserialize, Serialize};

/// Enum representing the possible operations that can be performed.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Operation {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Subtract,
    #[serde(rename = "*")]
    Multiply,
    #[serde(rename = "/")]
    Divide,
}

impl Operation {
    /// Executes the operation on two f64 numbers and returns the result.
    pub fn execute(&self, a: f64, b: f64) -> f64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

/// Route is value in a sequence of operations of which the operation is performed on.
/// For example, if the sequence is [a, b, c] and the operations are [+, *, -], the result
/// would be (((input + a) * b) - c).
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Route {
    /// The signal id.
    pub signal_id: String,
    /// The operation to be performed.
    pub operation: Operation,
}

/// Struct representing a source.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Source {
    /// The source id.
    pub source_id: String,
    /// The id.
    pub id: String,
    /// The routes associated with the source.
    pub routes: Vec<Route>,
}
