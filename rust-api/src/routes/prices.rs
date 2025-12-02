use actix_web::{get, HttpResponse, web};

use crate::config::AppConfig;
use crate::errors::ApiError;
use crate::models::DailyAverage;
use crate::models::query::AverageQuery;
use crate::utils::validate_dates;
use crate::services::call_python_api;

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
pub async fn get_average(
    query: web::Query<AverageQuery>,
    cfg: web::Data<AppConfig>,
) -> Result<HttpResponse, ApiError> {
    validate_dates(&query.start_date, &query.end_date)?;

    let upstream = call_python_api(&cfg, &query.start_date, &query.end_date).await?;

    match upstream {
        crate::models::PythonApiResponse::Ok { data } =>
            Ok(HttpResponse::Ok().json(serde_json::json!({ "data": data }))),

        crate::models::PythonApiResponse::Err { code, message } =>
            Err(ApiError::UpstreamError(format!("{}: {}", code, message))),

    }
}
