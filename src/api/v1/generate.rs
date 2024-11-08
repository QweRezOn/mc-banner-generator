use crate::api::error::AppError;
use crate::api::state::AppState;
use crate::banner::{apply_pattern, Banner, Pattern};
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::Response;
use image::RgbaImage;
use std::io::Cursor;

pub async fn generate(
    State(app_state): State<AppState>,
    Path(banner_string): Path<String>,
) -> Result<Response, AppError> {
    let banner = Banner::parse_banner(&banner_string)?;

    let mut image = RgbaImage::new(20, 40);
    let base_pattern_image = &app_state.patterns[Pattern::Base];

    let color = banner.color.to_rgb();

    apply_pattern(&mut image, base_pattern_image, color);

    for pattern in banner.patterns {
        let pattern_image = &app_state.patterns[pattern.pattern];

        apply_pattern(&mut image, pattern_image, pattern.color.to_rgb());
    }

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    image.write_to(&mut cursor, image::ImageFormat::Png)?;

    Ok(
        Response::builder()
            .header(header::CONTENT_TYPE, "image/png")
            .body(buf.into())?
    )
}
