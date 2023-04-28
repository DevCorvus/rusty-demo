use reqwasm::http;
use serde::Deserialize;

static API_URL: &str = "http://localhost:8000/api";

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn api_login(data: &str) -> Result<LoginResponse, String> {
    let res = http::Request::post(format!("{}/auth/login", API_URL).as_str())
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await
        .unwrap();

    let res_json = res.json::<LoginResponse>().await.unwrap();

    Ok(res_json)
}
