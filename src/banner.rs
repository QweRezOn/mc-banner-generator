use anyhow::{bail, Result};
use enum_map::Enum;
use enum_ordinalize::Ordinalize;
use image::{Rgb, Rgba, RgbaImage};
use serde::Serialize;
use strum_macros::{Display, EnumString};
use thiserror::Error;

#[derive(Debug, Display, EnumString, Enum, PartialEq, Eq, Ordinalize, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Color {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

impl Color {
    pub fn parse_color(string: &str) -> Result<Color> {
        let color_code = u8::from_str_radix(string, 16)?;

        let color = Color::from_ordinal(color_code as i8)
            .ok_or(ParseError::InvalidColor(string.to_string()))?;

        Ok(color)
    }
    
    pub fn to_rgb(&self) -> Rgb<u8> {
        let hex = self.to_hex();

        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;

        let rgb = Rgb::from([r, g, b]);
        rgb
    }

    pub fn to_hex(&self) -> u32 {
        match self {
            Color::White => 0xF9FFFE,
            Color::Orange => 0xF9801D,
            Color::Magenta => 0xC74EBD,
            Color::LightBlue => 0x3AB3DA,
            Color::Yellow => 0xFED83D,
            Color::Lime => 0x80C71F,
            Color::Pink => 0xF38BAA,
            Color::Gray => 0x474F52,
            Color::LightGray => 0x9D9D97,
            Color::Cyan => 0x169C9C,
            Color::Purple => 0x8932B8,
            Color::Blue => 0x3C44AA,
            Color::Brown => 0x835432,
            Color::Green => 0x5E7C16,
            Color::Red => 0xB02E26,
            Color::Black => 0x1D1D21
        }
    }
}

#[derive(Debug, Display, EnumString, Enum, PartialEq, Eq, Ordinalize, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Pattern {
    Base,
    Border,
    Bricks,
    Circle,
    Creeper,
    Cross,
    CurlyBorder,
    DiagonalLeft,
    DiagonalRight,
    DiagonalUpLeft,
    DiagonalUpRight,
    Flow,
    Flower,
    Globe,
    Gradient,
    GradientUp,
    Guster,
    HalfHorizontal,
    HalfHorizontalBottom,
    HalfVertical,
    HalfVerticalRight,
    Mojang,
    Piglin,
    Rhombus,
    Skull,
    SmallStripes,
    SquareBottomLeft,
    SquareBottomRight,
    SquareTopLeft,
    SquareTopRight,
    StraightCross,
    StripeBottom,
    StripeCenter,
    StripeDownleft,
    StripeDownright,
    StripeLeft,
    StripeMiddle,
    StripeRight,
    StripeTop,
    TrianglesBottom,
    TrianglesTop,
    TriangleBottom,
    TriangleTop,
}

impl Pattern {
    pub fn parse_pattern(string: &str) -> Result<Pattern> {
        let pattern_code = u8::from_str_radix(string, 16)?;

        let pattern = Pattern::from_ordinal(pattern_code as i8)
            .ok_or(ParseError::InvalidPattern(string.to_string()))?;

        if pattern == Pattern::Base {
            bail!(ParseError::InvalidPattern(string.to_string()))
        }

        Ok(pattern)
    }
}

#[derive(Debug, Serialize)]
pub struct BannerPattern {
    pub pattern: Pattern,
    pub color: Color,
}

#[derive(Debug, Serialize)]
pub struct Banner {
    pub color: Color,
    pub patterns: Vec<BannerPattern>,
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("string can't be empty")]
    Empty,
    #[error("invalid banner string")]
    InvalidBannerString,
    #[error("invalid color {0}")]
    InvalidColor(String),
    #[error("invalid pattern {0}")]
    InvalidPattern(String),
}

impl Banner {
    pub fn parse_banner(banner_string: &str) -> Result<Banner> {
        if banner_string.is_empty() {
            bail!(ParseError::Empty)
        }

        if !banner_string.is_ascii() {
            bail!(ParseError::InvalidBannerString)
        }
        
        let banner_color = Color::parse_color(&banner_string[0..1])?;
        let mut banner_patterns: Vec<BannerPattern> = Vec::new();

        let mut index = 1;
        while index < banner_string.len().saturating_sub(2) && banner_patterns.len() < 16 {
            let pattern = Pattern::parse_pattern(
                &banner_string[index..(index + 2)]
            )?;
            index += 2;

            let color = Color::parse_color(
                &banner_string[index..(index + 1)]
            )?;
            index += 1;

            banner_patterns.push(BannerPattern { color, pattern });
        }

        Ok(
            Banner {
                color: banner_color,
                patterns: banner_patterns,
            }
        )
    }
}

pub fn apply_pattern(image: &mut RgbaImage, pattern: &RgbaImage, color: Rgb<u8>) {
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let pattern_pixel = pattern.get_pixel(x, y);
        if pattern_pixel[3] == 0 {
            continue;
        }

        let brightness_r = pattern_pixel[0] as f32 / 255.0;
        let brightness_g = pattern_pixel[1] as f32 / 255.0;
        let brightness_b = pattern_pixel[2] as f32 / 255.0;
        
        if pixel[3] == 0 {
            let blended_pixel = Rgba([
                (color[0] as f32 * brightness_r) as u8,
                (color[1] as f32 * brightness_g) as u8,
                (color[2] as f32 * brightness_b) as u8,
                255,
            ]);
            *pixel = blended_pixel;
        } else {
            let alpha = pattern_pixel[3] as f32 / 255.0;

            let blended_pixel = Rgba([
                (((pixel[0] as f32) * (1.0 - alpha) + color[0] as f32 * alpha) * brightness_r) as u8,
                (((pixel[1] as f32) * (1.0 - alpha) + color[1] as f32 * alpha) * brightness_g) as u8,
                (((pixel[2] as f32) * (1.0 - alpha) + color[2] as f32 * alpha) * brightness_b) as u8,
                255,
            ]);

            *pixel = blended_pixel;
        }
    }
}
