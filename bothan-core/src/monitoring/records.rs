use std::collections::hash_map::Entry;
use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::registry::post_processor::PostProcessError;
use crate::registry::processor::ProcessError;
use crate::registry::source::Operation;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SignalComputationRecords<T, U> {
    #[serde(flatten)]
    inner: HashMap<String, SignalComputationRecord<T, U>>,
}

impl<T, U> SignalComputationRecords<T, U>
where
    T: Serialize + DeserializeOwned,
    U: Serialize + DeserializeOwned,
{
    pub fn entry(&mut self, key: String) -> Entry<'_, String, SignalComputationRecord<T, U>> {
        self.inner.entry(key)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SignalComputationRecord<T, U>
where
    T: Sized,
    U: Sized,
{
    pub sources: HashMap<String, SourceRecord<T>>,
    pub process_result: Option<Result<U, ProcessError>>,
    pub post_process_result: Option<Result<U, PostProcessError>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceRecord<T: Sized> {
    pub query_id: String,
    pub raw_source_value: T,
    pub operations: Vec<OperationRecord>,
    pub final_value: Option<T>,
}

impl<T> SourceRecord<T> {
    pub fn new(
        query_id: String,
        raw_source_value: T,
        operations: Vec<OperationRecord>,
        final_value: Option<T>,
    ) -> Self {
        SourceRecord {
            query_id,
            raw_source_value,
            operations,
            final_value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationRecord {
    pub signal_id: String,
    pub operation: Operation,
    pub value: Decimal,
}

impl OperationRecord {
    pub fn new(signal_id: String, operation: Operation, value: Decimal) -> Self {
        OperationRecord {
            signal_id,
            operation,
            value,
        }
    }
}
