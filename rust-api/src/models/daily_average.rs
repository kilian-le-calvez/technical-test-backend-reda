use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DailyAverage {
    pub date: String,
    pub average_price: f64,
}