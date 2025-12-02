use serde::{Serialize, Deserialize};
use crate::models::daily_average::DailyAverage;

#[derive(Debug, Serialize, Deserialize)]
pub struct PythonApiError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PythonApiResponse {
    Ok { data: Vec<DailyAverage> },
    Err { code: String, message: String },
}