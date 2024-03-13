use reqwest::{RequestBuilder, Response};

use crate::helpers::Error;

pub async fn send_request(request_builder: RequestBuilder) -> Result<Response, Error> {
    let response = request_builder.send().await?;

    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(Error::Http(status));
    }

    Ok(response)
}

pub async fn parse_response<T: serde::de::DeserializeOwned>(
    response: Response,
) -> Result<T, Error> {
    Ok(response.json::<T>().await?)
}
