use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AverageQuery {
    pub start_date: String,
    pub end_date: String,
}