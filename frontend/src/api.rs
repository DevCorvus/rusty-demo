use common::{ErrorResponse, LoginDto, LoginResponse};
use reqwasm::http;
use serde_json;
use thiserror::Error;

static API_URL: &str = "http://localhost:8000/api";

#[derive(Error, Debug)]
enum RequestError {
    #[error("Failed to make request")]
    Failed,
    #[error("Failed to parse dto")]
    ParseDtoFailed,
    #[error("Failed to parse response")]
    ParseResponseFailed,
    #[error("Failed to parse error response")]
    ParseErrorResponseFailed,
}

pub async fn api_login(data: LoginDto) -> Result<LoginResponse, String> {
    let parsed_data =
        serde_json::to_string(&data).map_err(|_| RequestError::ParseDtoFailed.to_string())?;

    let res = http::Request::post(format!("{}/auth/login", API_URL).as_str())
        .header("Content-Type", "application/json")
        .body(parsed_data)
        .send()
        .await
        .map_err(|_| RequestError::Failed.to_string())?;

    if res.status() != 201 {
        let err_res = res
            .json::<ErrorResponse>()
            .await
            .map_err(|_| RequestError::ParseErrorResponseFailed.to_string())?;

        return Err(err_res.message);
    }

    let res_json = res
        .json::<LoginResponse>()
        .await
        .map_err(|_| RequestError::ParseResponseFailed.to_string())?;

    Ok(res_json)
}
