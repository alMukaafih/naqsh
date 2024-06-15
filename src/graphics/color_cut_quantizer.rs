#![allow(dead_code)]

use std::{cmp::Ordering, collections::BinaryHeap};

use super::{color::Color, color_utils::ColorUtils, Filter, Swatch};

/// Represents a tightly fitting box around a color space.
#[derive(Default, Eq)]
struct Vbox {
    m_lower_index: i32,
    m_upper_index: i32,

    m_population: i32,

    m_min_red: i32,
    m_max_red: i32,

    m_min_green: i32,
    m_max_green: i32,

    m_min_blue: i32,
    m_max_blue: i32,
}

impl Vbox {
    fn new(lower_index: i32, upper_index: i32, ccq: &ColorCutQuantizer) -> Self {
        let mut vbox = Self {
            m_lower_index: lower_index,
            m_upper_index: upper_index,
            ..Default::default()
        };
        vbox.fit_box(ccq);
        vbox
    }

    fn get_volume(&self) -> i32 {
        (self.m_max_red - self.m_min_red + 1) * (self.m_max_green - self.m_min_green + 1) *
            (self.m_max_blue - self.m_min_blue + 1)
    }

    fn get_color_count(&self) -> i32 {
        1 + self.m_upper_index - self.m_lower_index
    }

    fn can_split(&self) -> bool {
        self.get_color_count() > 1
    }

    /// Recomputes the boundaries of this box to tightly fit the colors within the box.
    fn fit_box(&mut self, ccq: &ColorCutQuantizer) {
        let colors = &ccq.m_colors;
        let hist = &ccq.m_histogram;

        let mut min_red = i32::MAX;
        let mut min_green = i32::MAX;
        let mut min_blue = i32::MAX;

        let mut max_red = i32::MIN;
        let mut max_green = i32::MIN;
        let mut max_blue = i32::MIN;

        let mut count = 0;

        let mut i = self.m_lower_index as usize;
        while i <= self.m_upper_index as usize {
            let color = colors[i];
            count += hist[color as usize];

            let r = ColorCutQuantizer::quantized_red(color);
            let g = ColorCutQuantizer::quantized_green(color);
            let b = ColorCutQuantizer::quantized_blue(color);

            if r > max_red {
                max_red = r;
            }
            if r < min_red {
                min_red = r;
            }
            if g > max_green {
                max_green = g;
            }
            if g < min_green {
                min_green = g;
            }
            if b > max_blue {
                max_blue = b;
            }
            if b < min_blue {
                min_blue = b;
            }

            i += 1;
        }

        self.m_min_red = min_red;
        self.m_max_red = max_red;
        self.m_min_green = min_green;
        self.m_max_green = max_green;
        self.m_min_blue = min_blue;
        self.m_max_blue = max_blue;
        self.m_population = count;
    }

    fn split_box(&mut self, ccq: &mut ColorCutQuantizer) -> Self {
        if !self.can_split() {
            panic!("Can not split a box with only 1 color")
        }

        // find median along the longest dimension
        let split_point = self.find_split_point(ccq);

        let newbox = Vbox::new(split_point, self.m_upper_index + 1, ccq);

        // Now change this box's upperIndex and recompute the color boundaries
        self.m_upper_index = split_point;
        self.fit_box(ccq);

        newbox
    }

    /// Returns the dimension which this box is largest in
    fn get_longest_color_dimension(&self) -> Component {
        let red_length = self.m_max_red - self.m_min_red;
        let green_length = self.m_max_green - self.m_min_green;
        let blue_length = self.m_max_blue - self.m_min_blue;

        if red_length >= green_length && red_length >= green_length {
            return Component::Red;
        } else if green_length >= red_length && green_length >= blue_length {
            return Component::Green;
        } else {
            return  Component::Blue;
        }
    }

    /// Finds the point within this box's lowerIndex and upperIndex index of where to split.
    ///
    /// This is calculated by finding the longest color dimension, and then sorting the
    /// sub-array based on that dimension value in each color. The colors are then iterated over
    /// until a color is found with at least the midpoint of the whole box's dimension midpoint.
    ///
    /// @return the index of the colors array to split from
    fn find_split_point(&self, ccq: &mut ColorCutQuantizer) -> i32 {
        let longest_dimension = self.get_longest_color_dimension();
        let colors = &mut ccq.m_colors;
        let hist = &ccq.m_histogram;

        // We need to sort the colors in this box based on the longest color dimension.
        // As we can't use a Comparator to define the sort logic, we modify each color so that
        // its most significant is the desired dimension
        ColorCutQuantizer::modify_significant_octet(colors, longest_dimension.clone(), self.m_lower_index, self.m_upper_index);

        // Now sort... slice.sort uses a exclusive toIndex so we need to add 1
        let lower = self.m_lower_index as usize;
        let upper = self.m_upper_index as usize + 1;
        let index = colors.get_mut(lower..upper).unwrap();
        index.sort();

        // Now revert all of the colors so that they are packed as RGB again
        ColorCutQuantizer::modify_significant_octet(colors, longest_dimension, self.m_lower_index, self.m_upper_index);

        let midpoint = self.m_population / 2;
        let mut i = self.m_lower_index;
        let mut count = 0;
        while i <= self.m_upper_index {
            count += hist[colors[i as usize] as usize];
            if count >= midpoint {
                // we never want to split on the upperIndex, as this will result in the same
                // box
                return i32::min(self.m_upper_index - 1, i);
            }
            i += 1;
        }

        self.m_lower_index
    }

    /// Returns the average color of this box.
    fn get_average_color(&self, ccq: &mut ColorCutQuantizer) -> Swatch {
        let colors = &mut ccq.m_colors;
        let hist = &ccq.m_histogram;
        let mut red_sum = 0;
        let mut green_sum = 0;
        let mut blue_sum = 0;
        let mut total_population = 0;

        let mut i = self.m_lower_index;
        while i <= self.m_upper_index {
            let color = colors[i as usize];
            let color_population = hist[color as usize];

            total_population += color_population;
            red_sum += color_population * ColorCutQuantizer::quantized_red(color);
            green_sum += color_population * ColorCutQuantizer::quantized_green(color);
            blue_sum += color_population * ColorCutQuantizer::quantized_blue(color);

            i += 1;
        }

        let red_mean = f32::round(red_sum as f32 / total_population as f32) as i32;
        let green_mean = f32::round(green_sum as f32 / total_population as f32) as i32;
        let blue_mean = f32::round(blue_sum as f32 / total_population as f32) as i32;

        Swatch::new(ColorCutQuantizer::approximate_to_rgb888_1(red_mean, green_mean, blue_mean), total_population)
    }
}

impl Ord for Vbox {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_volume().cmp(&other.get_volume())
    }
}

impl PartialOrd for Vbox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vbox {
    fn eq(&self, other: &Self) -> bool {
        self.get_volume() == other.get_volume()
    }
}

#[derive(Clone)]
enum Component {
    Red = -3,
    Green = -2,
    Blue = -1,
}

/// A color quantizer based on the Median-cut algorithm, but optimized for picking out distinct
/// colors rather than representation colors.
///
/// The color space is represented as a 3-dimensional cube with each dimension being an RGB
/// component. The cube is then repeatedly divided until we have reduced the color space to the
/// requested number of colors. An average color is then generated from each cube.
///
/// What makes this different to median-cut is that median-cut divided cubes so that all of the cubes
/// have roughly the same population, where this quantizer divides boxes based on their color volume.
/// This means that the color space is divided into distinct colors, rather than representative
/// colors.
#[derive(Default)]
pub struct ColorCutQuantizer {
    m_colors: Vec<i32>,
    m_histogram: Vec<i32>,
    m_quantized_colors: Vec<Swatch>,
    m_filters: Vec<Box<dyn Filter>>,
    m_temp_hsl: [f32;3]
}

impl ColorCutQuantizer {
    const COMPONENT_RED: i32 = -3;
    const COMPONENT_GREEN: i32 = -2;
    const COMPONENT_BLUE: i32 = -1;

    const QUANTIZE_WORD_WIDTH: i32 = 5;
    const QUANTIZE_WORD_MASK: i32 = (1 << Self::QUANTIZE_WORD_WIDTH) - 1;

    /// Constructor.
    ///
    /// @param pixels histogram representing an image's pixel data
    /// @param maxColors The maximum number of colors that should be in the result palette.
    ///
    /// @param filters Set of filters to use in the quantization stage
    pub fn new(mut pixels: Vec<i32>, max_colors: i32, filters: Vec<Box<dyn Filter>>) -> Self {

        let mut ccq: ColorCutQuantizer = Default::default();
        ccq.m_filters = filters;

        let mut hist = vec![0i32; 1 << (Self::QUANTIZE_WORD_WIDTH * 3)];

        let mut i = 0;
        while i < pixels.len() {
            let quantized_color = Self::quantize_from_rgb888(pixels[i]);
            // Now update the pixel value to the quantized value
            pixels[i] = quantized_color;
            // And update the histogram
            hist[quantized_color as usize] += 1;

            i += 1;
        }

        // Now let's count the number of distinct colors
        let mut distinct_color_count = 0;
        let mut color = 0;
        while color < hist.len() {
            if hist[color] > 0 && ccq.should_ignore_color_1(color as i32) {
                // If we should ignore the color, set the population to 0
                hist[color] = 0;
            }
            if hist[color] > 0 {
                // If the color has population, increase the distinct color count
                distinct_color_count += 1
            }

            color += 1;
        }

        // Now lets go through create an array consisting of only distinct colors
        let mut colors = vec![0; distinct_color_count];
        let mut distinct_color_index = 0;
        let mut color = 0;
        while color < hist.len() {
            if hist[color] > 0 {
                distinct_color_index += 1;
                colors[distinct_color_index] = color;
            }

            color += 1;
        }

        if distinct_color_count as i32 <= max_colors {
            // The image has fewer colors than the maximum requested, so just return the colors
            for color in colors {
                ccq.m_quantized_colors.push(
                    Swatch::new(Self::approximate_to_rgb888_2(color as i32), hist[color])
                )
            }
        } else {
            ccq.m_quantized_colors = ccq.quantize_pixels(max_colors)
        }

        ccq
    }

    fn quantize_pixels(&mut self, max_colors: i32) -> Vec<Swatch> {
        // Create the priority queue which is sorted by volume descending. This means we always
        // split the largest box in the queue
        let mut pq: BinaryHeap<Vbox> = BinaryHeap::with_capacity(max_colors as usize);

        // To start, offer a box which contains all of the colors
        pq.push(Vbox::new(0, (self.m_colors.len() - 1) as i32, self));

        // Now go through the boxes, splitting them until we have reached maxColors or there are no
        // more boxes to split
        self.split_boxes(&mut pq, max_colors as usize);

        // Finally, return the average colors of the color boxes
        self.generate_average_colors(pq)
    }

    /// Returns the list of quantized colors
    pub(crate) fn get_quantized_colors(&self) -> &Vec<Swatch> {
        &self.m_quantized_colors
    }

    /// Iterate through the [BinaryHeap], popping
    /// [Vbox] objects from the queue
    /// and splitting them. Once split, the new box and the remaining box are offered back to the
    /// queue.
    ///
    /// @param queue [BinaryHeap] to pop for boxes
    ///
    /// @param max_size Maximum amount of boxes to split
    fn split_boxes(&mut self, queue: &mut BinaryHeap<Vbox>, max_size: usize) {
        while queue.len() < max_size {
            let _vbox = queue.pop();

            if let Some(mut vbox) = _vbox {
                // First split the box, and push the result
                queue.push(vbox.split_box(self));
                // Then push the box back
                queue.push(vbox)
            } else {
                // If we get here then there are no more boxes to split, so return
                return;
            }
        }
    }

    fn generate_average_colors(&mut self, vboxes: BinaryHeap<Vbox>) -> Vec<Swatch> {
        let mut colors: Vec<Swatch> = Vec::with_capacity(vboxes.len());
        for vbox in vboxes {
            let mut swatch = vbox.get_average_color(self);
            if !self.should_ignore_color_2(&mut swatch) {
                // As we're averaging a color box, we can still get colors which we do not want, so
                // we check again here
                colors.push(swatch);
            }
        }
        colors
    }

    /// Modify the significant octet in a packed color int. Allows sorting based on the value of a
    /// single color component. This relies on all components being the same word size.
    ///
    /// See also [Vbox::find_split_point]
    fn modify_significant_octet(a: &mut Vec<i32>, dimension: Component, lower: i32, upper: i32) {
        match dimension {
            Component::Red => {
                // Already in RGB, no need to do anything
            }
            Component::Green => {
                // We need to do a RGB to GRB swap, or vice-versa
                let mut i = lower as usize;
                while i <= upper as usize {
                    let color = a[i];
                    a[i] = Self::quantized_green(color) << (Self::QUANTIZE_WORD_WIDTH + Self::QUANTIZE_WORD_WIDTH)
                    | Self::quantized_red(color) << Self::QUANTIZE_WORD_WIDTH
                    | Self::quantized_blue(color);
                    i += 1;
                }
            }
            Component::Blue => {
                // We need to do a RGB to BGR swap, or vice-versa
                let mut i = lower as usize;
                while i <= upper as usize {
                    let color = a[i];
                    a[i] = Self::quantized_blue(color) << (Self::QUANTIZE_WORD_WIDTH + Self::QUANTIZE_WORD_WIDTH)
                    | Self::quantized_green(color) << Self::QUANTIZE_WORD_WIDTH
                    | Self::quantized_red(color);
                    i += 1;
                }
            }
        }
    }

    fn should_ignore_color_1(&mut self, color565: i32) -> bool {
        let rgb = Self::approximate_to_rgb888_2(color565);
        ColorUtils::color_to_hsl(rgb, &mut self.m_temp_hsl);
        self.should_ignore_color_3(rgb, self.m_temp_hsl)
    }

    fn should_ignore_color_2(&self, color: &mut Swatch) -> bool {
        self.should_ignore_color_3(color.get_rgb(), color.get_hsl())
    }

    fn should_ignore_color_3(&self, rgb: i32, hsl: [f32;3]) -> bool {
        if self.m_filters.len() > 0 {
            let mut i = 0;
            let count = self.m_filters.len();
            while i < count {
                if !self.m_filters[i].is_allowed(rgb, hsl) {
                    return true
                }
                i += 1;
            }
        }
        false
    }

    fn quantize_from_rgb888(color: i32) -> i32 {
        let r = Self::modify_word_width(Color::red(color).into(), 8, Self::QUANTIZE_WORD_WIDTH);
        let g = Self::modify_word_width(Color::green(color).into(), 8, Self::QUANTIZE_WORD_WIDTH);
        let b = Self::modify_word_width(Color::blue(color).into(), 8, Self::QUANTIZE_WORD_WIDTH);
        (r as i32) << (Self::QUANTIZE_WORD_WIDTH + Self::QUANTIZE_WORD_WIDTH) | (g as i32) << Self::QUANTIZE_WORD_WIDTH | (b as i32)
    }

    /// Quantized RGB888 values to have a word width of [Self::QUANTIZE_WORD_WIDTH].
    fn approximate_to_rgb888_1(r: i32, g: i32, b: i32) -> i32 {
        Color::rgb(
            Self::modify_word_width(r, Self::QUANTIZE_WORD_WIDTH, 8) as u8,
            Self::modify_word_width(g, Self::QUANTIZE_WORD_WIDTH, 8) as u8,
            Self::modify_word_width(b, Self::QUANTIZE_WORD_WIDTH, 8) as u8
        )
    }

    fn approximate_to_rgb888_2(color: i32) -> i32 {
        Self::approximate_to_rgb888_1(
            Self::quantized_red(color),
            Self::quantized_green(color),
            Self::quantized_blue(color)
        )
    }

    /// Returns red component of the quantized color
    pub fn quantized_red(color: i32) -> i32 {
        (color >> (Self::QUANTIZE_WORD_WIDTH + Self::QUANTIZE_WORD_WIDTH)) & Self::QUANTIZE_WORD_MASK
    }

    /// Returns green component of the quantized color
    pub fn quantized_green(color: i32) -> i32 {
        (color >> Self::QUANTIZE_WORD_WIDTH) & Self::QUANTIZE_WORD_MASK
    }

    /// Returns blue component of the quantized color
    pub fn quantized_blue(color: i32) -> i32 {
        color >> & Self::QUANTIZE_WORD_MASK
    }

    pub fn modify_word_width(value: i32, current_width: i32, target_width: i32) -> i32 {
        let new_value;
        if target_width > current_width {
            // If we're approximating up in word width, we'll shift up
            new_value = (value as i32) << (target_width - current_width);
        } else {
            // Else, we will just shift and keep the MSB
            new_value = (value as i32) >> (current_width - target_width);
        }
        new_value & ((1 << target_width) - 1)
    }
}