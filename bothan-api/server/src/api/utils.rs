use tonic::Response;

use crate::proto::query::{Price, PriceStatus, UpdateRegistryResponse, UpdateStatusCode};

pub fn registry_resp(status: UpdateStatusCode) -> Response<UpdateRegistryResponse> {
    let update_registry_response = UpdateRegistryResponse {
        code: status.into(),
    };
    Response::new(update_registry_response)
}

pub fn price<T: Into<String>>(signal_id: T, status: PriceStatus, price: i64) -> Price {
    Price {
        signal_id: signal_id.into(),
        status: status.into(),
        price,
    }
}
