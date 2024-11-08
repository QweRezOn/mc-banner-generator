use crate::api::error::AppError;
use crate::banner::Banner;
use axum::extract::Path;
use axum::Json;

pub async fn meta(
    Path(banner_string): Path<String>,
) -> Result<Json<Banner>, AppError> {
    let banner = Banner::parse_banner(&banner_string)?;

    Ok(Json(banner))
}
