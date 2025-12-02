use utoipa::OpenApi;

use crate::routes::prices::__path_get_average;
use crate::models::{AverageQuery, DailyAverage};

#[derive(OpenApi)]
#[openapi(
    paths(get_average),
    components(schemas(AverageQuery, DailyAverage))
)]
pub struct ApiDoc;