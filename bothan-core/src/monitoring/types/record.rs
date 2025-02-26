use std::sync::Arc;

use bothan_lib::registry::post_processor::PostProcessError;
use bothan_lib::registry::processor::ProcessError;
use bothan_lib::registry::source::Operation;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalTransactionRecord<T, U> {
    pub tx_hash: String,
    pub records: Arc<Vec<SignalComputationRecord<T, U>>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalComputationRecord<T, U> {
    pub signal_id: String,
    pub sources: Vec<SourceRecord<T, U>>,
    pub process_result: Option<ProcessRecord<U, ProcessError>>,
    pub post_process_result: Option<Vec<ProcessRecord<U, PostProcessError>>>,
}

impl<T, U> SignalComputationRecord<T, U> {
    pub(crate) fn new(signal_id: String) -> Self {
        SignalComputationRecord {
            signal_id,
            sources: Vec::new(),
            process_result: None,
            post_process_result: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessRecord<T, E> {
    pub function: String,
    pub result: Result<T, E>,
}

impl<T, E> ProcessRecord<T, E> {
    pub fn new(function: String, result: Result<T, E>) -> Self {
        ProcessRecord { function, result }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceRecord<T, U> {
    pub source_id: String,
    pub query_id: String,
    pub raw_source_value: Option<T>,
    pub operations: Vec<OperationRecord>,
    pub final_value: Option<U>,
}

impl<T, U> SourceRecord<T, U> {
    pub fn new(
        source_id: String,
        query_id: String,
        raw_source_value: Option<T>,
        operations: Vec<OperationRecord>,
        final_value: Option<U>,
    ) -> Self {
        SourceRecord {
            source_id,
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
