use chrono::NaiveDate;
use crate::errors::ApiError;

pub fn validate_dates(start: &str, end: &str) -> Result<(NaiveDate, NaiveDate), ApiError> {
    let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d")
        .map_err(|_| ApiError::InvalidInput("Invalid start_date format".into()))?;

    let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d")
        .map_err(|_| ApiError::InvalidInput("Invalid end_date format".into()))?;

    if start_date > end_date {
        return Err(ApiError::InvalidInput(
            "start_date must be before or equal tso end_date".into(),
        ));
    }

    Ok((start_date, end_date))
}