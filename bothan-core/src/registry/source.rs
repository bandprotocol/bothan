use std::ops::{Add, Div, Mul, Sub};

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
    /// Executes the operation on two numbers and returns the result.
    pub fn execute<T>(&self, a: T, b: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
    {
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
pub struct OperationRoute {
    /// The signal id of the value to be used in the operation.
    pub signal_id: String,
    /// The operation to be performed.
    pub operation: Operation,
}

impl OperationRoute {
    /// Creates a new OperationRoute.
    pub fn new<T: Into<String>>(signal_id: T, operation: Operation) -> Self {
        OperationRoute {
            signal_id: signal_id.into(),
            operation,
        }
    }
}

/// Struct representing a source.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SourceQuery {
    /// The source id.
    pub source_id: String,
    /// The id to query the source.
    #[serde(rename = "id")]
    pub query_id: String,
    /// The operation routes to execute on the source query results.
    pub routes: Vec<OperationRoute>,
}

impl SourceQuery {
    /// Creates a new SourceQuery.
    pub fn new<T, U>(source_id: T, query_id: U, routes: Vec<OperationRoute>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        SourceQuery {
            source_id: source_id.into(),
            query_id: query_id.into(),
            routes,
        }
    }
}
