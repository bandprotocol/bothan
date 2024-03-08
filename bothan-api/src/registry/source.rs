use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch]
pub trait Operator {
    fn operate(&self, a: f64, b: f64) -> f64;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Add {}

impl Operator for Add {
    fn operate(&self, a: f64, b: f64) -> f64 {
        a + b
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Subtract {}

impl Operator for Subtract {
    fn operate(&self, a: f64, b: f64) -> f64 {
        a - b
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Multiply {}

impl Operator for Multiply {
    fn operate(&self, a: f64, b: f64) -> f64 {
        a * b
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Divide {}

impl Operator for Divide {
    fn operate(&self, a: f64, b: f64) -> f64 {
        a / b
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[enum_dispatch(Operator)]
pub enum Operation {
    #[serde(rename = "+")]
    Add(Add),
    #[serde(rename = "-")]
    Subtract(Subtract),
    #[serde(rename = "*")]
    Multiply(Multiply),
    #[serde(rename = "/")]
    Divide(Divide),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Route {
    pub signal_id: String,
    pub operation: Operation,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Source {
    pub source_id: String,
    pub id: String,
    pub routes: Option<Vec<Route>>,
}
