use tonic::Code;

pub fn code_to_str(code: Code) -> String {
    match code {
        Code::Ok => "Ok".to_string(),
        Code::Cancelled => "Cancelled".to_string(),
        Code::Unknown => "Unknown".to_string(),
        Code::InvalidArgument => "InvalidArgument".to_string(),
        Code::DeadlineExceeded => "DeadlineExceeded".to_string(),
        Code::NotFound => "NotFound".to_string(),
        Code::AlreadyExists => "AlreadyExists".to_string(),
        Code::PermissionDenied => "PermissionDenied".to_string(),
        Code::ResourceExhausted => "ResourceExhausted".to_string(),
        Code::FailedPrecondition => "FailedPrecondition".to_string(),
        Code::Aborted => "Aborted".to_string(),
        Code::OutOfRange => "OutOfRange".to_string(),
        Code::Unimplemented => "Unimplemented".to_string(),
        Code::Internal => "Internal".to_string(),
        Code::Unavailable => "Unavailable".to_string(),
        Code::DataLoss => "DataLoss".to_string(),
        Code::Unauthenticated => "Unauthenticated".to_string(),
    }
}
