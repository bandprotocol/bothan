//! Utility functions for metrics modules.
//!
//! This module provides helper functions for working with gRPC codes and other metrics-related utilities.

use tonic::Code;

/// Converts a gRPC [`Code`] to a human-readable string for labeling metrics.
///
/// # Arguments
///
/// * `code` - The gRPC status code to convert.
///
/// # Returns
///
/// A string representation of the gRPC code suitable for use as a metric label.
pub fn code_to_str(code: Code) -> String {
    match code {
        Code::Ok => "ok".to_string(),
        Code::Cancelled => "cancelled".to_string(),
        Code::Unknown => "unknown".to_string(),
        Code::InvalidArgument => "invalid_argument".to_string(),
        Code::DeadlineExceeded => "deadline_exceeded".to_string(),
        Code::NotFound => "not_found".to_string(),
        Code::AlreadyExists => "already_exists".to_string(),
        Code::PermissionDenied => "permission_denied".to_string(),
        Code::ResourceExhausted => "resource_exhausted".to_string(),
        Code::FailedPrecondition => "failed_precondition".to_string(),
        Code::Aborted => "aborted".to_string(),
        Code::OutOfRange => "out_of_range".to_string(),
        Code::Unimplemented => "unimplemented".to_string(),
        Code::Internal => "internal".to_string(),
        Code::Unavailable => "unavailable".to_string(),
        Code::DataLoss => "data_loss".to_string(),
        Code::Unauthenticated => "unauthenticated".to_string(),
    }
}
