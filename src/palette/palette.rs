#![allow(dead_code)]
use std::collections::HashMap;

use super::target::Target;

pub struct Swatch {
    m_red: i32,
    m_green: i32,
    m_blue: i32,
    m_rgb: i32,
    m_population: i32,
    m_generated_text_colors: bool,
    m_title_text_color: i32,
    m_body_text_color: i32,
    m_hsl: Vec<f32>
}

pub struct Builder {
    m_swatches: Vec<Swatch>,
    m_bitmap: (),
}

pub struct Palette {
    m_swatches: Vec<Swatch>,
    m_targets: Vec<Target>,
    m_selected_swatches: HashMap<String, String>,
    m_used_colors: Vec<bool>,
    m_dominant_swatch: String,
}

impl Palette {
    const DEFAULT_RESIZE_BITMAP_AREA: i32 = 112 * 112;
    const DEFAULT_CALCULATE_NUMBER_COLORS: i32 = 16;
    const MIN_CONTRAST_TITLE_TEXT: f32 = 3.0;
    const MIN_CONTRAST_BODY_TEXT: f32 = 4.5;
}
