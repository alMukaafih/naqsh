use std::path::PathBuf;

use serde::Deserialize;

use crate::graphics::ColorInt;

#[derive(Debug, Deserialize)]
/// The representation of the Manifest file.
pub struct Manifest {
    /// The format of the generated Image.
    pub format: String,
    /// Size of the generated Image.
    pub size: (usize, usize),
    /// Background Color of the generated Image.
    pub color: ColorInt,
    /// Assets used during generation of the Image.
    pub assets: Option<Vec<Asset>>,
    /// Objects in the generated Image.
    pub objects: Vec<Object>,
}

#[derive(Debug, Deserialize)]
/// This is the representation of an Asset.
pub struct Asset {
    /// String for hashing the Asset.
    pub id: String,
    /// Source path of the Asset
    pub src: PathBuf,
}

#[derive(Debug, Deserialize)]
/// This is the representation of an Object.
pub struct Object {
    /// Name of Object.
    pub name: String,
    /// Source path of Object.
    pub src: Option<PathBuf>,
    /// Asset id of Object
    pub asset: Option<String>,
    /// Color of Object.
    pub color: Option<ColorInt>,
    /// Color of Object in rgba.
    pub rgba: Option<(u8, u8, u8, u8)>,
    /// Text Content of Object.
    pub text: Option<String>,
    /// Resize Object to coordinates with x and y values represented as percentages of width and height of Image respectively.
    /// If either of x value, y value is null, aspect-ratio of Object is maintained.
    pub size: Option<(Option<f64>, Option<f64>)>,
    /// The left, top, right, bottom coordinate of the Object. If size is specified, only the left and top coordinate is used.
    pub coordinates: Option<(f64, f64, f64, f64)>
}