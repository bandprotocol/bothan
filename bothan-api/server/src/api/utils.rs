use tonic::Response;

use crate::proto::query::{UpdateRegistryResponse, UpdateStatusCode};

pub fn registry_resp(status: UpdateStatusCode) -> Response<UpdateRegistryResponse> {
    let update_registry_response = UpdateRegistryResponse {
        code: status.into(),
    };
    Response::new(update_registry_response)
}
