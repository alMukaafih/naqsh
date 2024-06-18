#![allow(dead_code)]
//! A set of color-related utility methods, building upon those available in [Color].

use crate::graphics::color::Color;

use super::ColorInt;

/// A set of color-related utility methods, building upon those available in [Color].
pub struct ColorUtils();

impl ColorUtils {
    const MIN_ALPHA_SEARCH_MAX_ITERATIONS: i32 = 10;
    const MIN_ALPHA_SEARCH_PRECISION: i32 = 1;

    pub fn constrain(amount: f32, low: f32, high: f32) -> f32 {
        if amount < low {
            low
        } else {
            f32::min(amount, high)
        }
    }

    /// Convert RGB components to HSL (hue-saturation-lightness).
    /// <ul>
    /// <li>out_hsl[0] is Hue [0, 360)</li>
    /// <li>out_hsl[1] is Saturation [0, 1]</li>
    /// <li>out_hsl[2] is Lightness [0, 1]</li>
    /// </ul>
    pub fn rgb_to_hsl(r: u8, g: u8, b: u8, out_hsl: &mut [f32;3]) {
        let rf = (r as f32) / 255f32;
        let gf = (g as f32) / 255f32;
        let bf = (b as f32) / 255f32;

        let max = f32::max(rf, f32::max(gf, bf));
        let min = f32::min(rf, f32::max(gf, bf));
        let delta_max_min = max - min;

        let mut h: f32;
        let s: f32;
        let l = (max + min) / 2f32;

        if max == min {
            // Monochromatic
            h = 0f32;
            s = 0f32;
        } else {
            if max == rf {
                h = ((gf - bf) / delta_max_min) % 6f32;
            } else if max == gf {
                h = ((bf - rf) / delta_max_min) + 2f32;
            } else {
                h = ((rf - gf) / delta_max_min) + 4f32;
            }

            s = delta_max_min / (1f32 - f32::abs(2f32 * l - 1f32));
        }

        h = (h * 60f32) % 360f32;
        if h < 0.0 {
            h += 360f32;
        }

        out_hsl[0] = Self::constrain(h, 0f32, 360f32);
        out_hsl[1] = Self::constrain(s, 0f32, 1f32);
        out_hsl[2] = Self::constrain(l, 0f32, 1f32);

    }

    /// Convert the ARGB color to its HSL (hue-saturation-lightness) components.
    /// <ul>
    /// <li>outHsl[0] is Hue [0, 360)</li>
    /// <li>outHsl[1] is Saturation [0, 1]</li>
    /// <li>outHsl[2] is Lightness [0, 1]</li>
    /// </ul>
    ///
    /// `color` is the ARGB color to convert. The alpha component is ignored.
    /// `out_hsl` is a 3-element array which holds the resulting HSL components
    pub fn color_to_hsl(color: ColorInt, out_hsl: &mut [f32;3]) {
        Self::rgb_to_hsl(Color::red(color), Color::green(color), Color::blue(color), out_hsl)
    }

    /// Convert RGB components to its CIE XYZ representative components.
    ///
    /// <p>The resulting XYZ representation will use the D65 illuminant and the CIE
    ///° Standard Observer (1931).</p>
    ///
    /// <ul>
    /// <li>out_xyz[0] is X [0, 95.047)</li>
    /// <li>out_xyz[1] is Y [0, 100)</li>
    /// <li>out_xyz[2] is Z [0, 108.883)</li>
    /// </ul>
    /// `r` is the red component value.
    /// `g` is the green component value.
    /// `b` is the blue component value.
    /// out_xyz 3-element array which holds the resulting XYZ components
    pub fn rgb_to_xyz(r: u8, g: u8, b: u8, out_xyz: &mut [f64;3]) {
        let mut sr = r as f64 / 255.0;
        sr = {
            if sr < 0.04045 {
                sr / 12.92
            } else {
                f64::powf((sr + 0.055) / 1.055, 2.4)
            }
        };
        let mut sg = g as f64 / 255.0;
        sg = {
            if sg < 0.04045 {
                sg / 12.92
            } else {
                f64::powf((sg + 0.055) / 1.055, 2.4)
            }
        };
        let mut sb = b as f64 / 255.0;
        sb = {
            if sb < 0.04045 {
                sb / 12.92
            } else {
                f64::powf((sb + 0.055) / 1.055, 2.4)
            }
        };

        out_xyz[0] = 100f64 * (sr * 0.4124 + sg * 0.3576 + sb * 0.1805);
        out_xyz[1] = 100f64 * (sr * 0.2126 + sg * 0.7152 + sb * 0.0722);
        out_xyz[2] = 100f64 * (sr * 0.0193 + sg * 0.1192 + sb * 0.9505);
    }

    /// Convert the ARGB color to its CIE XYZ representative components.
    ///
    /// <p>The resulting XYZ representation will use the D65 illuminant and the CIE
    /// 2° Standard Observer (1931).</p>
    ///
    /// <ul>
    /// <li>out_xyz[0] is X [0, 95.047)</li>
    /// <li>out_xyz[1] is Y [0, 100)</li>
    /// <li>out_xyz[2] is Z [0, 108.883)</li>
    /// </ul>
    /// color the ARGB color to convert. The alpha component is ignored
    /// out_xyz 3-element array which holds the resulting LAB components
    pub fn color_to_xyz(color: ColorInt, out_xyz: &mut [f64;3]) {
        Self::rgb_to_xyz(Color::red(color), Color::green(color), Color::blue(color), out_xyz)
    }

    /// Set the alpha component of color to be alpha.
    pub fn set_alpha_component(color: ColorInt, alpha: u8) -> ColorInt {
        (color & 0x00ffffff) | ((alpha as i32) << 24)
    }

    fn composite_alpha(foreground_alpha: u8, background_alpha: u8) -> u8 {
        let background_alpha = background_alpha as i32;
        let foreground_alpha = foreground_alpha as i32;
        (0xFF - (((0xFF - background_alpha) * (0xFF - foreground_alpha)) / 0xFF)) as u8
    }

    fn composite_component(fg_c: u8, fg_a: u8, bg_c: u8, bg_a: u8, a: u8) -> u8 {
        if a == 0 {
            return 0;
        }
        let fg_c = fg_c as i32;
        let fg_a = fg_a as i32;
        let bg_c = bg_c as i32;
        let bg_a = bg_a as i32;
        let a = a as i32;
        (((0xFF * fg_c * fg_a) + (bg_c * bg_a * (0xFF - fg_a))) / (a * 0xFF)) as u8
    }

    pub fn composite_colors(foreground: ColorInt, background: ColorInt) -> i32 {
        let bg_alpha = Color::alpha(background);
        let fg_alpha = Color::alpha(foreground);
        let a = Self::composite_alpha(fg_alpha, bg_alpha);

        let r = Self::composite_component(Color::red(foreground), fg_alpha,
                Color::red(background), bg_alpha, a);
        let g = Self::composite_component(Color::green(foreground), fg_alpha,
                Color::green(background), bg_alpha, a);
        let b = Self::composite_component(Color::blue(foreground), fg_alpha,
                Color::blue(background), bg_alpha, a);

        return *Color::argb(a, r, g, b)
    }

    /// Returns the luminance of a color as a float between `0.0` and `1.0`.
    ///
    /// Defined as the Y component in the XYZ representation of `color`.
    pub fn calculate_luminance(color: ColorInt) -> f64 {
        let mut result: [f64;3] = Default::default();
        Self::color_to_xyz(color, &mut result);
        result[1] / 100f64
    }

    /// Returns the contrast ratio between `foreground` and `background`.
    /// `background` must be opaque.
    ///
    /// Formula defined
    /// <a href="http://www.w3.org/TR/2008/REC-WCAG20-20081211/#contrast-ratiodef">here</a>.
    pub fn calculate_contrast(mut foreground: ColorInt, background: ColorInt) -> f64 {
        if Color::alpha(background) != 255 {
            panic!()
        }
        if Color::alpha(foreground) < 255 {
            // If the foreground is translucent, composite the foreground over the background
            foreground = Self::composite_colors(foreground, background).into();
        }
        let luminance1 = Self::calculate_luminance(foreground) + 0.05;
        let luminance2 = Self::calculate_luminance(background) + 0.05;

        // Now return the lighter luminance divided by the darker luminance
        f64::max(luminance1, luminance2) / f64::min(luminance1, luminance2)
    }

    /// Calculates the minimum alpha value which can be applied to `foreground` so that would
    /// have a contrast value of at least `min_contrast_ratio` when compared to
    /// `background`.
    /// foreground the foreground color
    /// background the opaque background color
    /// minContrastRatio the minimum contrast ratio
    ///
    /// Returns the alpha value in the range \[0, 255] or -1 if no value could be calculated
    pub fn calculate_minimum_alpha(foreground: ColorInt, background: ColorInt, min_contrast_ratio: f32) -> i32 {
        if Color::alpha(background) != 255 {
            panic!()
        }

        // First lets check that a fully opaque foreground has sufficient contrast
        let mut test_foreground = Self::set_alpha_component(foreground, 255);
        let mut test_ratio = Self::calculate_contrast(test_foreground, background);
        if test_ratio < min_contrast_ratio.into() {
            // Fully opaque foreground does not have sufficient contrast, return error
            return -1;
        }

        let mut num_iterations = 0;
        let mut min_alpha: u8 = 0;
        let mut max_alpha: u8 = 255;

        while num_iterations <= ColorUtils::MIN_ALPHA_SEARCH_MAX_ITERATIONS &&
                i32::from(max_alpha - min_alpha) > ColorUtils::MIN_ALPHA_SEARCH_PRECISION {
            let test_alpha = ((min_alpha + max_alpha) / 2) as u8;

            test_foreground = Self::set_alpha_component(foreground, test_alpha);
            test_ratio = Self::calculate_contrast(test_foreground, background);

            if test_ratio < min_contrast_ratio .into(){
                min_alpha = test_alpha;
            } else {
                max_alpha = test_alpha;
            }

            num_iterations += 1;
        }

        // Conservatively return the max of the range of possible alphas, which is known to pass.
        return max_alpha.into();
    }

}