use std::fmt;
use std::ops::{Add, AddAssign};
use std::ops::{BitAnd, BitAndAssign};
use std::ops::{BitOr, BitOrAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Shl, ShlAssign};
use std::ops::{Sub, SubAssign};
use std::num::Wrapping;

///<p>The <code>Color</code> struct provides methods for creating, converting and manipulating colors.
/// Colors have three different representations:</p>
/// <ul>
///     <li>Color ints, the most common representation</li>
///     <li>Color longs</li>
///     <li><code>Color</code> instances</li>
/// </ul>
/// <p>The section below describe each representation in detail.</p>
///
/// <h3>Color ints</h3>
/// <p>Color ints are the most common representation of colors on Android and
/// have been used since {@link android.os.Build.VERSION_CODES#BASE API level 1}.</p>
///
/// <p>A color int always defines a color in the {@link ColorSpace.Named#SRGB sRGB}
/// color space using 4 components packed in a single 32 bit integer value:</p>
///
/// <table summary="Color int definition">
///     <tr>
///         <th>Component</th><th>Name</th><th>Size</th><th>Range</th>
///     </tr>
///     <tr><td>A</td><td>Alpha</td><td>8 bits</td><td>\([0..255]\)</td></tr>
///     <tr><td>R</td><td>Red</td><td>8 bits</td><td>\([0..255]\)</td></tr>
///     <tr><td>G</td><td>Green</td><td>8 bits</td><td>\([0..255]\)</td></tr>
///     <tr><td>B</td><td>Blue</td><td>8 bits</td><td>\([0..255]\)</td></tr>
/// </table>
///
/// <p>The components in this table are listed in encoding order (see below),
/// which is why color ints are called ARGB colors.</p>
///
/// <h4>Usage in code</h4>
/// <p>To avoid confusing color ints with arbitrary integer values, it is a
/// good practice to annotate them with the <code>@Wrapping</code> annotation
/// found in the Android Support Library.</p>
///
/// <h4>Encoding</h4>
/// <p>The four components of a color int are encoded in the following way:</p>
/// <pre struct="prettyprint">
/// int color = (A & 0xff) << 24 | (R & 0xff) << 16 | (G & 0xff) << 8 | (B & 0xff);
/// </pre>
///
/// <p>Because of this encoding, color ints can easily be described as an integer
/// constant in source. For instance, opaque blue is <code>0xff0000ff</code>
/// and yellow is <code>0xffffff00</code>.</p>
///
/// <p>To easily encode color ints, it is recommended to use the static methods
/// {@link #argb(int, int, int, int)} and {@link #rgb(int, int, int)}. The second
/// method omits the alpha component and assumes the color is opaque (alpha is 255).
/// As a convenience this struct also offers methods to encode color ints from components
/// defined in the \([0..1]\) range: {@link #argb(float, float, float, float)} and
/// {@link #rgb(float, float, float)}.</p>
///
/// <p>Color longs (defined below) can be easily converted to color ints by invoking
/// the {@link #toArgb(long)} method. This method performs a color space conversion
/// if needed.</p>
///
/// <p>It is also possible to create a color int by invoking the method {@link #toArgb()}
/// on a color instance.</p>
///
/// <h4>Decoding</h4>
/// <p>The four ARGB components can be individually extracted from a color int
/// using the following expressions:</p>
/// <pre struct="prettyprint">
/// int A = (color >> 24) & 0xff; // or color >>> 24
/// int R = (color >> 16) & 0xff;
/// int G = (color >>  8) & 0xff;
/// int B = (color      ) & 0xff;
/// </pre>
///
/// <p>This struct offers convenience methods to easily extract these components:</p>
/// <ul>
///     <li>{@link #alpha(int)} to extract the alpha component</li>
///     <li>{@link #red(int)} to extract the red component</li>
///     <li>{@link #green(int)} to extract the green component</li>
///     <li>{@link #blue(int)} to extract the blue component</li>
/// </ul>
///
/// <h3>Color longs</h3>
/// <p>Color longs are a representation introduced in
/// {@link android.os.Build.VERSION_CODES#O Android O} to store colors in different
/// {@link ColorSpace color spaces}, with more precision than color ints.</p>
///
/// <p>A color long always defines a color using 4 components packed in a single
/// 64 bit long value. One of these components is always alpha while the other
/// three components depend on the color space's {@link ColorSpace.Model color model}.
/// The most common color model is the {@link ColorSpace.Model#RGB RGB} model in
/// which the components represent red, green and blue values.</p>
///
/// <p struct="note"><b>Component ranges:</b> the ranges defined in the tables
/// below indicate the ranges that can be encoded in a color long. They do not
/// represent the actual ranges as they may differ per color space. For instance,
/// the RGB components of a color in the {@link ColorSpace.Named#DISPLAY_P3 Display P3}
/// color space use the \([0..1]\) range. Please refer to the documentation of the
/// various {@link ColorSpace.Named color spaces} to find their respective ranges.</p>
///
/// <p struct="note"><b>Alpha range:</b> while alpha is encoded in a color long using
/// a 10 bit integer (thus using a range of \([0..1023]\)), it is converted to and
/// from \([0..1]\) float values when decoding and encoding color longs.</p>
///
/// <p struct="note"><b>sRGB color space:</b> for compatibility reasons and ease of
/// use, color longs encoding {@link ColorSpace.Named#SRGB sRGB} colors do not
/// use the same encoding as other color longs.</p>
///
/// <table summary="Color long definition">
///     <tr>
///         <th>Component</th><th>Name</th><th>Size</th><th>Range</th>
///     </tr>
///     <tr><td colspan="4">{@link ColorSpace.Model#RGB RGB} color model</td></tr>
///     <tr><td>R</td><td>Red</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>G</td><td>Green</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>B</td><td>Blue</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>A</td><td>Alpha</td><td>10 bits</td><td>\([0..1023]\)</td></tr>
///     <tr><td></td><td>Color space</td><td>6 bits</td><td>\([0..63]\)</td></tr>
///     <tr><td colspan="4">{@link ColorSpace.Named#SRGB sRGB} color space</td></tr>
///     <tr><td>A</td><td>Alpha</td><td>8 bits</td><td>\([0..255]\)</td></tr>
///     <tr><td>R</td><td>Red</td><td>8 bits</td><td>\([0..255]\)</td></tr>
///     <tr><td>G</td><td>Green</td><td>8 bits</td><td>\([0..255]\)</td></tr>
///     <tr><td>B</td><td>Blue</td><td>8 bits</td><td>\([0..255]\)</td></tr>
///     <tr><td>X</td><td>Unused</td><td>32 bits</td><td>\(0\)</td></tr>
///     <tr><td colspan="4">{@link ColorSpace.Model#XYZ XYZ} color model</td></tr>
///     <tr><td>X</td><td>X</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>Y</td><td>Y</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>Z</td><td>Z</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>A</td><td>Alpha</td><td>10 bits</td><td>\([0..1023]\)</td></tr>
///     <tr><td></td><td>Color space</td><td>6 bits</td><td>\([0..63]\)</td></tr>
///     <tr><td colspan="4">{@link ColorSpace.Model#XYZ Lab} color model</td></tr>
///     <tr><td>L</td><td>L</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>a</td><td>a</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>b</td><td>b</td><td>16 bits</td><td>\([-65504.0, 65504.0]\)</td></tr>
///     <tr><td>A</td><td>Alpha</td><td>10 bits</td><td>\([0..1023]\)</td></tr>
///     <tr><td></td><td>Color space</td><td>6 bits</td><td>\([0..63]\)</td></tr>
///     <tr><td colspan="4">{@link ColorSpace.Model#CMYK CMYK} color model</td></tr>
///     <tr><td colspan="4">Unsupported</td></tr>
/// </table>
///
/// <p>The components in this table are listed in encoding order (see below),
/// which is why color longs in the RGB model are called RGBA colors (even if
/// this doesn't quite hold for the special case of sRGB colors).</p>
///
/// <p>The color long encoding relies on half-precision float values (fp16). If you
/// wish to know more about the limitations of half-precision float values, please
/// refer to the documentation of the {@link Half} struct.</p>
///
/// <h4>Usage in code</h4>
/// <p>To avoid confusing color longs with arbitrary long values, it is a
/// good practice to annotate them with the <code>@ColorLong</code> annotation
/// found in the Android Support Library.</p>
///
/// <h4>Encoding</h4>
///
/// <p>Given the complex nature of color longs, it is strongly encouraged to use
/// the various methods provided by this struct to encode them.</p>
///
/// <p>The most flexible way to encode a color long is to use the method
/// {@link #pack(float, float, float, float, ColorSpace)}. This method allows you
/// to specify three color components (typically RGB), an alpha component and a
/// color space. To encode sRGB colors, use {@link #pack(float, float, float)}
/// and {@link #pack(float, float, float, float)} which are the
/// equivalent of {@link #rgb(int, int, int)} and {@link #argb(int, int, int, int)}
/// for color ints. If you simply need to convert a color int into a color long,
/// use {@link #pack(int)}.</p>
///
/// <p>It is also possible to create a color long value by invoking the method
/// {@link #pack()} on a color instance.</p>
///
/// <h4>Decoding</h4>
///
/// <p>This struct offers convenience methods to easily extract the components
/// of a color long:</p>
/// <ul>
///     <li>{@link #alpha(long)} to extract the alpha component</li>
///     <li>{@link #red(long)} to extract the red/X/L component</li>
///     <li>{@link #green(long)} to extract the green/Y/a component</li>
///     <li>{@link #blue(long)} to extract the blue/Z/b component</li>
/// </ul>
///
/// <p>The values returned by these methods depend on the color space encoded
/// in the color long. The values are however typically in the \([0..1]\) range
/// for RGB colors. Please refer to the documentation of the various
/// {@link ColorSpace.Named color spaces} for the exact ranges.</p>
///
/// <h3>Color instances</h3>
/// <p>Color instances are a representation introduced in
/// {@link android.os.Build.VERSION_CODES#O Android O} to store colors in different
/// {@link ColorSpace color spaces}, with more precision than both color ints and
/// color longs. Color instances also offer the ability to store more than 4
/// components if necessary.</p>
///
/// <p>Colors instances are immutable and can be created using one of the various
/// <code>valueOf</code> methods. For instance:</p>
/// <pre struct="prettyprint">
/// // sRGB
/// Color opaqueRed = Color.valueOf(0xffff0000); // from a color int
/// Color translucentRed = Color.valueOf(1.0f, 0.0f, 0.0f, 0.5f);
///
/// // Wide gamut color
/// {@literal @}ColorLong long p3 = pack(1.0f, 1.0f, 0.0f, 1.0f, colorSpaceP3);
/// Color opaqueYellow = Color.valueOf(p3); // from a color long
///
/// // CIE L*a*b* color space
/// ColorSpace lab = ColorSpace.get(ColorSpace.Named.LAB);
/// Color green = Color.valueOf(100.0f, -128.0f, 128.0f, 1.0f, lab);
/// </pre>
///
/// <p>Color instances can be converted to color ints ({@link #toArgb()}) or
/// color longs ({@link #pack()}). They also offer easy access to their various
/// components using the following methods:</p>
/// <ul>
///     <li>{@link #alpha()}, returns the alpha component value</li>
///     <li>{@link #red()}, returns the red component value (or first
///     component value in non-RGB models)</li>
///     <li>{@link #green()}, returns the green component value (or second
///     component value in non-RGB models)</li>
///     <li>{@link #blue()}, returns the blue component value (or third
///     component value in non-RGB models)</li>
///     <li>{@link #getComponent(int)}, returns a specific component value</li>
///     <li>{@link #getComponents()}, returns all component values as an array</li>
/// </ul>
///
/// <h3>Color space conversions</h3>
/// <p>You can convert colors from one color space to another using
/// {@link ColorSpace#connect(ColorSpace, ColorSpace)} and its variants. However,
/// the <code>Color</code> struct provides a few convenience methods to simplify
/// the process. Here is a brief description of some of them:</p>
/// <ul>
///     <li>{@link #convert(ColorSpace)} to convert a color instance in a color
///     space to a new color instance in a different color space</li>
///     <li>{@link #convert(float, float, float, float, ColorSpace, ColorSpace)} to
///     convert a color from a source color space to a destination color space</li>
///     <li>{@link #convert(long, ColorSpace)} to convert a color long from its
///     built-in color space to a destination color space</li>
///     <li>{@link #convert(int, ColorSpace)} to convert a color int from sRGB
///     to a destination color space</li>
/// </ul>
///
/// <p>Please refere to the {@link ColorSpace} documentation for more
/// information.</p>
///
/// <h3>Alpha and transparency</h3>
/// <p>The alpha component of a color defines the level of transparency of a
/// color. When the alpha component is 0, the color is completely transparent.
/// When the alpha is component is 1 (in the \([0..1]\) range) or 255 (in the
/// \([0..255]\) range), the color is completely opaque.</p>
///
/// <p>The color representations described above do not use pre-multiplied
/// color components (a pre-multiplied color component is a color component
/// that has been multiplied by the value of the alpha component).
/// For instance, the color int representation of opaque red is
/// <code>0xffff0000</code>. For semi-transparent (50%) red, the
/// representation becomes <code>0x80ff0000</code>. The equivalent color
/// instance representations would be <code>(1.0, 0.0, 0.0, 1.0)</code>
/// and <code>(1.0, 0.0, 0.0, 0.5)</code>.</p>
pub struct Color();

impl Color {
    pub const BLACK: i32       = 0xFF000000u32 as i32;
    pub const WHITE: i32       = 0xFFFFFFFFu32 as i32;

    /// Return the alpha component of a color int. This is the same as saying
    /// `((color >> 24) & 0xff) as u8`
    pub fn alpha(color: i32) -> u8 {
        ((color >> 24) & 0xff) as u8
    }

    /// Return the red component of a color int. This is the same as saying
    /// `((color >> 16) & 0xFF) as u8`
    pub fn red(color: i32) -> u8 {
        ((color >> 16) & 0xFF) as u8
    }

    /// Return the green component of a color int. This is the same as saying
    /// `((color >> 8) & 0xFF) as u8`
    pub fn green(color: i32) -> u8 {
        ((color >> 8) & 0xFF) as u8
    }

    /// Return the blue component of a color int. This is the same as saying
    /// `(color >> 8 & 0xFF) as u8`
    pub fn blue(color: i32) -> u8 {
        (color >> 8 & 0xFF) as u8
    }

    /// Returns a color-int from red, green, blue components.
    /// The alpha component is implicitly 255 (fully opaque).
    /// `red` is the red component  of the color.
    /// `green` is the green component  of the color.
    /// `blue` is the blue component  of the color.
    pub fn rgb(red: u8, green: u8, blue: u8) -> i32 {
        let red = Wrapping(red as i32);
        let green = Wrapping(green as i32);
        let blue = Wrapping(blue as i32);
        (Wrapping(0xff000000u32 as i32) | (red << 16) | (green << 8) | blue).0
    }

    /// Returns a color-int from alpha, red, green, blue components.
    /// `alpha` is the alpha component  of the color.
    /// `red` is the red component  of the color.
    /// `green` is the green component  of the color.
    /// `blue` is the blue component  of the color.
    pub fn argb(alpha: u8, red: u8, green: u8, blue: u8) -> i32 {
        ((alpha as i32) << 24) | ((red as i32) << 16) | ((green as i32) << 8) | (blue as i32)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
#[repr(transparent)]
pub struct ColorInt(pub i32);

impl fmt::Debug for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Binary for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::LowerHex for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Add for ColorInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_add(rhs.0))
    }
}

impl AddAssign for ColorInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i32> for ColorInt {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + ColorInt(rhs);
    }
}

impl Sub for ColorInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_sub(rhs.0))
    }
}

impl SubAssign for ColorInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<i32> for ColorInt {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - ColorInt(rhs);
    }
}

impl Mul for ColorInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_mul(rhs.0))
    }
}

impl MulAssign for ColorInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<i32> for ColorInt {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * ColorInt(rhs);
    }
}

impl Div for ColorInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_div(rhs.0))
    }
}

impl DivAssign for ColorInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<i32> for ColorInt {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / ColorInt(rhs);
    }
}

impl BitOr for ColorInt {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let num = Wrapping(self.0) | Wrapping(rhs.0);
        Self(num.0)
    }
}

impl BitOrAssign for ColorInt {
    fn bitor_assign(&mut self, rhs: Self) {
        let num = Wrapping(self.0) | Wrapping(rhs.0);
        *self = Self(num.0);
    }
}

impl BitAnd for ColorInt {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let num = Wrapping(self.0) & Wrapping(rhs.0);
        Self(num.0)
    }
}

impl BitAndAssign for ColorInt {
    fn bitand_assign(&mut self, rhs: Self) {
        let num = Wrapping(self.0) & Wrapping(rhs.0);
        *self = Self(num.0);
    }
}

impl Shl<usize> for ColorInt {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let num = Wrapping(self.0) << rhs;
        Self(num.0)
    }
}