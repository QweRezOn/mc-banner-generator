use crate::banner::Pattern;
use anyhow::Result;
use enum_map::EnumMap;
use image::{ImageReader, RgbaImage};
use std::fs;

pub fn load_patterns() -> Result<EnumMap<Pattern, RgbaImage>> {
    let mut patterns = EnumMap::default();

    let dir = fs::read_dir("./patterns")?;

    for dir_path in dir {
        let path = dir_path?.path();

        let image = ImageReader::open(&path)?
            .decode()?
            .to_rgba8();

        let pattern_name = path.file_stem()
            .expect("filename expected")
            .to_str()
            .expect("string expected");
        let pattern: Pattern = pattern_name.parse()?;

        patterns[pattern] = image;
    }

    Ok(patterns)
}
