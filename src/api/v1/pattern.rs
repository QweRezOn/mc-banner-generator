use crate::api::error::AppError;
use crate::api::state::AppState;
use crate::banner::Pattern;
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::Response;
use std::io::Cursor;

pub async fn pattern(
    State(app_state): State<AppState>,
    Path(pattern_name): Path<String>,
) -> Result<Response, AppError> {
    let pattern: Pattern = pattern_name.parse()?;
    
    let pattern_image = &app_state.patterns[pattern];
    
    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    pattern_image.write_to(&mut cursor, image::ImageFormat::Png)?;
    
    Ok(
        Response::builder()
            .header(header::CONTENT_TYPE, "image/png")
            .body(buf.into())?
    )
}