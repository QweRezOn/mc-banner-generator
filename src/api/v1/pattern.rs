use crate::api::error::AppError;
use crate::api::state::AppState;
use crate::banner::Pattern;
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::Response;
use std::io::Cursor;

const CACHE_TIME: i32 = 30 * 24 * 60 * 60;

pub async fn pattern(
    State(app_state): State<AppState>,
    Path(pattern_name): Path<String>,
) -> Result<Response, AppError> {
    let pattern_without_extension = pattern_name
        .chars()
        .take_while(|&ch| ch != '.')
        .collect::<String>();
    let pattern: Pattern = pattern_without_extension.parse()?;

    let pattern_image = &app_state.patterns[pattern];

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    pattern_image.write_to(&mut cursor, image::ImageFormat::Png)?;

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "image/png")
        .header(header::CACHE_CONTROL, format!("public, max-age={}", CACHE_TIME))
        .body(buf.into())?)
}
