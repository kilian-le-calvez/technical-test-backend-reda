use reqwest::Client;
use serde::Serialize;

use crate::config::AppConfig;
use crate::errors::ApiError;
use crate::models::{PythonApiError, PythonApiResponse};


#[derive(Serialize)]
struct UpstreamPayload<'a> {
    start_date: &'a str,
    end_date: &'a str,
}

pub async fn call_python_api(
    cfg: &AppConfig,
    start: &str,
    end: &str,
) -> Result<PythonApiResponse, ApiError> {
    let client: Client = Client::builder()
        .timeout(std::time::Duration::from_millis(cfg.upstream.timeout_ms))
        .build()
        .map_err(|e| ApiError::Internal(Box::new(e)))?;

    let url: String = format!("{}/internal/average-prices", cfg.upstream.python_api_base_url);

    let resp: reqwest::Response = client
        .post(&url)
        .json(&UpstreamPayload { start_date: start, end_date: end })
        .send()
        .await
        .map_err(|e| {
        log::error!("Failed to reach python-api: {}", e);
        ApiError::UpstreamUnavailable
    })?;

    let status: reqwest::StatusCode = resp.status();
    let body: String = resp.text().await.map_err(|_| ApiError::UpstreamError("Invalid upstream body".into()))?;

    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<PythonApiError>(&body) {
            return Err(ApiError::UpstreamError(format!("{}: {}", err.code, err.message)));
        }
        return Err(ApiError::UpstreamError("Upstream service returned error".into()));
    }

    let parsed = serde_json::from_str::<PythonApiResponse>(&body)
        .map_err(|_| ApiError::UpstreamError("Invalid upstream JSON".into()))?;

    Ok(parsed)
}
