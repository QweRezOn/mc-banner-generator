use std::sync::Arc;
use enum_map::EnumMap;
use image::RgbaImage;
use crate::banner::Pattern;

#[derive(Clone)]
pub struct AppState {
    pub patterns: Arc<EnumMap<Pattern, RgbaImage>>,
}