use std::collections::HashMap;

use crate::graphics::{Color, ColorUtils, Target, TargetKind};
use crate::graphics::sparse_boolean_array::SparseBooleanArray;
use crate::image::Image;
use crate::object::Rect;

/// Represents a color swatch generated from an image's palette. The RGB color can be retrieved
/// by calling [Palette::get_rgb].
#[derive(Default, Clone)]
pub struct Swatch {
    m_red: u8,
    m_green: u8,
    m_blue: u8,
    m_rgb: i32,
    m_population: i32,
    m_generated_text_colors: bool,
    m_title_text_color: i32,
    m_body_text_color: i32,
    m_hsl: [f32;3]
}

impl Swatch {
    const MIN_CONTRAST_TITLE_TEXT: f32 = 3.0;
    const MIN_CONTRAST_BODY_TEXT: f32 = 4.5;

    pub fn new(color: i32, population: i32) -> Swatch {
        Swatch {
            m_red: Color::red(color),
            m_green: Color::green(color),
            m_blue: Color::blue(color),
            m_rgb: color,
            m_population: population,
            ..Default::default()
        }
    }

    /// Returns this swatch's RGB color value
    pub fn get_rgb(&self) -> i32 {
        self.m_rgb
    }

    /// Return this swatch's HSL values.
    ///
    /// hsv\[0\] is Hue \[0 .. 360\]
    ///
    /// hsv\[1\] is Saturation \[0...1\]
    ///
    /// hsv\[2\] is Lightness \[0...1\]
    pub fn get_hsl(&mut self) -> [f32;3] {
        ColorUtils::rgb_to_hsl(self.m_red, self.m_green, self.m_blue, &mut self.m_hsl);
        self.m_hsl
    }

    /// Returns the number of pixels represented by this swatch
    pub fn get_population(&self) -> i32 {
        self.m_population
    }

    fn ensure_text_colors_generated(&mut self) {
        if !self.m_generated_text_colors {
            // First check white, as most colors will be dark

            let light_body_alpha = ColorUtils::calculate_minimum_alpha(
                    Color::WHITE, self.m_rgb, Self::MIN_CONTRAST_BODY_TEXT);
            let light_title_alpha = ColorUtils::calculate_minimum_alpha(
                    Color::WHITE, self.m_rgb, Self::MIN_CONTRAST_TITLE_TEXT);

            if light_body_alpha != -1 && light_title_alpha != -1 {
                // If we found valid light values, use them and return
                self.m_body_text_color = ColorUtils::set_alpha_component(Color::WHITE, light_body_alpha as u8);
                self.m_title_text_color = ColorUtils::set_alpha_component(Color::WHITE, light_title_alpha as u8);
                self.m_generated_text_colors = true;
                return;
            }

            let dark_body_alpha = ColorUtils::calculate_minimum_alpha(
                    Color::BLACK, self.m_rgb, Self::MIN_CONTRAST_BODY_TEXT);
            let dark_title_alpha = ColorUtils::calculate_minimum_alpha(
                    Color::BLACK, self.m_rgb, Self::MIN_CONTRAST_TITLE_TEXT);

            if dark_body_alpha != -1 && dark_title_alpha != -1 {
                // If we found valid dark values, use them and return
                self.m_body_text_color = ColorUtils::set_alpha_component(Color::BLACK, dark_body_alpha as u8);
                self.m_title_text_color = ColorUtils::set_alpha_component(Color::BLACK, dark_title_alpha as u8);
                self.m_generated_text_colors = true;
                return;
            }

            // If we reach here then we can not find title and body values which use the same
            // lightness, we need to use mismatched values
            self.m_body_text_color = {
                if light_body_alpha != -1 {
                    ColorUtils::set_alpha_component(Color::WHITE, light_body_alpha as u8)
                } else {
                    ColorUtils::set_alpha_component(Color::BLACK, dark_body_alpha as u8)
                }
            };
            self.m_title_text_color = {
                if light_title_alpha != -1 {
                    ColorUtils::set_alpha_component(Color::WHITE, light_title_alpha as u8)
                } else {
                    ColorUtils::set_alpha_component(Color::BLACK, dark_title_alpha as u8)
                }
            };
            self.m_generated_text_colors = true;

        }
    }

    /// Returns an appropriate color to use for any 'title' text which is displayed over this
    /// [Swatch]'s color. This color is guaranteed to have sufficient contrast.
    pub fn get_title_text_color(&mut self) -> i32 {
        self.ensure_text_colors_generated();
        self.m_title_text_color
    }

    /// Returns an appropriate color to use for any 'body' text which is displayed over this
    /// [Swatch]'s color. This color is guaranteed to have sufficient contrast.
    pub fn get_body_text_color(&mut self) -> i32 {
        self.ensure_text_colors_generated();
        self.m_body_text_color
    }
}

/// A Filter provides a mechanism for exercising fine-grained control over which colors
/// are valid within a resulting [Palette].
pub trait Filter {
    /// Hook to allow clients to be able filter colors from resulting palette.
    /// `rgb` is the color in RGB888.
    /// `hsl` is HSL representation of the color.
    ///
    /// Returns true if the color is allowed, false if not.
    ///
    /// See also [PaletteBuilder::add_filter]
    fn is_allowed(&self, rgb: i32, hsl: [f32;3]) -> bool;
}

struct DefaultFilter();

impl DefaultFilter {
    const BLACK_MAX_LIGHTNESS: f32 = 0.05;
    const WHITE_MIN_LIGHTNESS: f32 = 0.95;

    fn is_black(&self, hsl_color: [f32;3]) -> bool {
        hsl_color[2] <= Self::BLACK_MAX_LIGHTNESS
    }

    fn is_white(&self, hsl_color: [f32;3]) -> bool {
        hsl_color[2] <= Self::WHITE_MIN_LIGHTNESS
    }

    fn is_near_red_iline(&self, hsl_color: [f32;3]) -> bool {
        hsl_color[0] >= 10f32 && hsl_color[0] <= 37f32 && hsl_color[1] <= 0.82f32
    }
}

impl Filter for DefaultFilter {
    fn is_allowed(&self, rgb: i32, hsl: [f32;3]) -> bool {
        let _ = rgb;
        !self.is_white(hsl) && !self.is_black(hsl) && !self.is_near_red_iline(hsl)
    }
}

/// A helper struct to extract prominent colors from an image.
///
/// A number of colors with different profiles are extracted from the image:
/// <ul>
///     <li>Vibrant</li>
///     <li>Vibrant Dark</li>
///     <li>Vibrant Light</li>
///     <li>Muted</li>
///     <li>Muted Dark</li>
///     <li>Muted Light</li>
/// </ul>
/// These can be retrieved from the appropriate getter method.
///
///
/// Instances are created with a [PaletteBuilder] which supports several options to tweak the
/// generated Palette. See that class' documentation for more information.
///
/// Generation should always be completed on a background thread, ideally the one in
/// which you load your image on. [PaletteBuilder] supports both synchronous and asynchronous
/// generation:
///
/// <pre>
/// // Synchronous
/// Palette p = Palette.from(bitmap).generate();
///
/// // Asynchronous
/// Palette.from(bitmap).generate(new PaletteAsyncListener() {
///     public void onGenerated(Palette p) {
///         // Use generated instance
///     }
/// });
/// </pre>

pub struct Palette {
    m_swatches: Vec<Swatch>,
    m_targets: Vec<Target>,
    m_selected_swatches: HashMap<Target, Swatch>,
    m_used_colors: SparseBooleanArray,
    m_dominant_swatch: Swatch,
}

impl Palette {
    pub fn new(swatches: Vec<Swatch>, targets: Vec<Target>) -> Self {
        let swatch = Self::find_dominant_swatch(&swatches);
        Self {
            m_swatches: swatches,
            m_targets: targets,
            m_selected_swatches: HashMap::new(),
            m_used_colors: SparseBooleanArray::default(),
            m_dominant_swatch: swatch
        }
    }

    /// Returns all of the swatches which make up the palette.
    pub fn get_swatches(&self) -> &Vec<Swatch> {
        &self.m_swatches
    }

    pub fn get_targets(&self) -> &Vec<Target> {
        &self.m_targets
    }

    fn find_dominant_swatch(swatches: &Vec<Swatch>) -> Swatch {
        let mut max_pop = i32::MIN;
        let mut max_swatch = Swatch::default();
        let mut i = 0;
        let count = swatches.len();
        while i < count {
            let swatch = swatches.get(i).unwrap();
            if swatch.get_population() > max_pop {
                max_swatch = swatch.clone();
                max_pop = swatch.get_population();
            }

            i += 1;
        }

        max_swatch
    }
}

impl From<Vec<Swatch>> for Palette {
    /// Generate a [Palette] from the pre-generated list of [Swatch] swatches.
    /// This is useful for testing, or if you want to resurrect a [Palette] instance from a
    /// list of swatches. Will return null if the `swatches` is null.
    fn from(swatches: Vec<Swatch>) -> Self {
        todo!()
    }
}

/// PaletteBuilder class for generating [Palette] instances.
pub struct PaletteBuilder {
    m_swatches: Vec<Swatch>,
    m_image: Image,
    m_targets: Vec<Target>,
    m_max_colors: i32,
    m_resize_area: i32,
    m_resize_max_dimension: i32,
    m_filters: Vec<Box<dyn Filter>>,
    m_region: Rect
}

impl PaletteBuilder {
    const DEFAULT_RESIZE_IMAGE_AREA: i32 = 112 * 112;
    const DEFAULT_CALCULATE_NUMBER_COLORS: i32 = 16;

    pub fn new(image: Image) -> Self {
        let mut builder = Self::default();
        builder.m_image = image;
        builder.m_filters.push(Box::new(DefaultFilter()));

        // Add the default targets
        builder.m_targets.push(Target::new(TargetKind::LightVibrant));
        builder.m_targets.push(Target::new(TargetKind::Vibrant));
        builder.m_targets.push(Target::new(TargetKind::DarkVibrant));
        builder.m_targets.push(Target::new(TargetKind::LightMuted));
        builder.m_targets.push(Target::new(TargetKind::Muted));
        builder.m_targets.push(Target::new(TargetKind::DarkMuted));

        builder
    }
}

impl Default for PaletteBuilder {
    fn default() -> Self {
        Self {
            m_swatches: Default::default(),
            m_image: Default::default(),
            m_targets: Default::default(),
            m_max_colors: Self::DEFAULT_CALCULATE_NUMBER_COLORS,
            m_resize_area: Self::DEFAULT_RESIZE_IMAGE_AREA,
            m_resize_max_dimension: -1,
            m_filters: Default::default(),
            m_region: Default::default()
        }
    }
}

impl From<Image> for PaletteBuilder {
    /// Start generating a [Palette] with the returned [PaletteBuilder] instance.
    fn from(image: Image) -> Self {
        todo!()
    }
}
