use serde::{Deserialize, Serialize};

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
    pub fn execute(&self, a: f64, b: f64) -> f64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Route {
    pub signal_id: String,
    pub operation: Operation,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Source {
    pub source_id: String,
    pub id: String,
    pub routes: Vec<Route>,
}
