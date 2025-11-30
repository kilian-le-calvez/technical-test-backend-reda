use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use chrono::NaiveDate;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::path;
use thiserror::Error;
use std::str::FromStr;

mod config;
use crate::config::AppConfig;


// ------------------------------
// Error Handling
// ------------------------------
#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

impl ErrorResponse {
    fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Error, Debug)]
enum ApiError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("upstream error: {0}")]
    Upstream(String),

    #[error("internal error")]
    Internal(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        match self {
            ApiError::Validation(_) => StatusCode::BAD_REQUEST,
            ApiError::Upstream(_) => StatusCode::BAD_GATEWAY,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, msg) = match self {
            ApiError::Validation(m) => ("INVALID_REQUEST", m.to_string()),
            ApiError::Upstream(m) => ("UPSTREAM_ERROR", "Upstream service unavailable".into()),
            ApiError::Internal(_) => ("INTERNAL_ERROR", "Internal server error".to_string()),
        };

        HttpResponse::build(self.status_code()).json(ErrorResponse::new(code, &msg))
    }
}


// ------------------------------
// API Types
// ------------------------------
#[derive(Debug, Deserialize, ToSchema)]
struct AverageQuery {
    start_date: String,
    end_date: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DailyAverage {
    date: String,
    average_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PythonApiError {
    code: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PythonApiSuccess {
    data: Vec<DailyAverage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum PythonApiResponse {
    Ok { data: Vec<DailyAverage> },
    Err { code: String, message: String },
}

#[get("/swagger-test")]
async fn swagger_test() -> HttpResponse {
    HttpResponse::Ok().body("swagger route reachable")
}
// ------------------------------
// Route + Handler
// ------------------------------
#[utoipa::path(
    get,
    path = "/prices/average",
    params(
        ("start_date" = String, Query, description = "Start date in YYYY-MM-DD"),
        ("end_date" = String, Query, description = "End date in YYYY-MM-DD")
    ),
    responses(
        (status = 200, description = "Daily averages", body = [DailyAverage])
    )
)]
#[get("/prices/average")]
async fn get_average(
    query: web::Query<AverageQuery>,
    cfg: web::Data<AppConfig>,
) -> Result<HttpResponse, ApiError> {

    // Validate dates
    let start = NaiveDate::parse_from_str(&query.start_date, "%Y-%m-%d")
        .map_err(|_| ApiError::Validation("Invalid start_date format".into()))?;
    let end = NaiveDate::parse_from_str(&query.end_date, "%Y-%m-%d")
        .map_err(|_| ApiError::Validation("Invalid end_date format".into()))?;

    if start > end {
        return Err(ApiError::Validation(
            "start_date must be before or equal to end_date".into(),
        ));
    }

    // Request python-api
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(cfg.upstream.timeout_ms))
        .build()
        .map_err(|e| ApiError::Internal(Box::new(e)))?;

    let url = format!("{}/internal/average-prices", cfg.upstream.python_api_base_url);

    #[derive(Serialize)]
    struct UpstreamPayload<'a> {
        start_date: &'a str,
        end_date: &'a str,
    }

    let resp = client
        .post(&url)
        .json(&UpstreamPayload {
            start_date: &query.start_date,
            end_date: &query.end_date,
        })
        .send()
        .await
        .map_err(|e| ApiError::Upstream(format!("Failed to reach python-api: {}", e)))?;

    let status = resp.status();
    let body_text = resp
        .text()
        .await
        .map_err(|e| ApiError::Upstream(format!("Invalid upstream body: {}", e)))?;

    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<PythonApiError>(&body_text) {
            return Err(ApiError::Upstream(format!("{}: {}", err.code, err.message)));
        }
        return Err(ApiError::Upstream(format!("Upstream returned status {}", status)));
    }

    let parsed: PythonApiResponse = serde_json::from_str(&body_text)
        .map_err(|e| ApiError::Upstream(format!("Cannot parse upstream JSON: {}", e)))?;

    match parsed {
        PythonApiResponse::Ok { data } =>
            Ok(HttpResponse::Ok().json(serde_json::json!({ "data": data }))),
        PythonApiResponse::Err { code, message } =>
            Err(ApiError::Upstream(format!("{}: {}", code, message))),
    }
}


// ------------------------------
// Swagger Documentation
// ------------------------------
#[derive(OpenApi)]
#[openapi(
    paths(get_average),
    components(schemas(AverageQuery, DailyAverage))
)]
pub struct ApiDoc;


// ------------------------------
// Main
// ------------------------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = AppConfig::from_file("Config.toml").expect("Failed to load config");

    env_logger::Builder::new()
        .filter_level(cfg.logging.level.parse().unwrap_or(LevelFilter::Info))
        .init();

    let bind_addr = format!("{}:{}", cfg.server.bind_address, cfg.server.port);
    println!("Starting rust-api at {}", bind_addr);

    HttpServer::new(move || {
    App::new()
            .app_data(web::Data::new(cfg.clone()))
            .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(get_average)
    })
    .bind(bind_addr)?
    .run()
    .await
}
