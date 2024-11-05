use std::sync::Arc;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::registry::post_processor::PostProcessError;
use crate::registry::processor::ProcessError;
use crate::registry::source::Operation;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalRecordsWithTxHash<T, U> {
    pub tx_hash: String,
    pub records: Arc<Vec<SignalComputationRecord<T, U>>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalComputationRecord<T, U>
where
    T: Sized,
    U: Sized,
{
    pub signal_id: String,
    pub sources: Vec<SourceRecord<T>>,
    pub process_result: Option<Result<U, ProcessError>>,
    pub post_process_result: Option<Result<U, PostProcessError>>,
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
pub struct SourceRecord<T: Sized> {
    pub source_id: String,
    pub query_id: String,
    pub raw_source_value: T,
    pub operations: Vec<OperationRecord>,
    pub final_value: Option<T>,
}

impl<T> SourceRecord<T> {
    pub fn new(
        source_id: String,
        query_id: String,
        raw_source_value: T,
        operations: Vec<OperationRecord>,
        final_value: Option<T>,
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
