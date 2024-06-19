#![allow(dead_code)]

use std::mem::ManuallyDrop;
use std::ops::{AddAssign, Deref, DerefMut, Index, IndexMut};

/// A Pixel in an Image.
#[repr(transparent)]
pub struct Pixel {
    buf: ManuallyDrop<Vec<u8>>
}

impl Index<usize> for Pixel {
    type Output = u8;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.buf[idx]
    }
}

impl IndexMut<usize> for Pixel {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.buf[idx]
    }
}

impl AddAssign for Pixel {
    fn add_assign(&mut self, pixel: Self) {
        let alpha = pixel[3] as u64;
        /* red */
        self[0] = ((alpha * pixel[0] as u64) + ((255 - alpha) * self[0] as u64)).div_ceil(255) as u8;
        /* green */
        self[1] = ((alpha * pixel[1] as u64) + ((255 - alpha) * self[1] as u64)).div_ceil(255) as u8;
        /* blue */
        self[2] = ((alpha * pixel[2] as u64) + ((255 - alpha) * self[2] as u64)).div_ceil(255) as u8;
        /* alpha */
        self[3] = (alpha * 255 + ((255 - alpha) * self[3] as u64)).div_ceil(255) as u8;
    }
}

pub struct Row {
    pub length: usize,
    pixels: Vec<Pixel>,
    buf: ManuallyDrop<Vec<u8>>
}

impl Row {
    pub fn get_pixel(&mut self, index: usize) -> Option<&mut Pixel> {
        let start = index * 4;
        let end = start + 4;
        let pixel = self.buf.get_mut(start..end);
        let buf = pixel?;
        let ptr = buf.as_mut_ptr();
        let length = 4;
        let buf = unsafe {
            Vec::from_raw_parts(ptr, length, length)
        };
        self.pixels.push(Pixel {buf: ManuallyDrop::new(buf)});
        let length = self.pixels.len() - 1;
        self.pixels.get_mut(length)
    }
}

impl Deref for Row {
    type Target = ManuallyDrop<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

#[derive(Default)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    format: String,
    rows: Vec<Row>,
    buf: Vec<u8>
}

impl Image {
    pub fn new(width: usize, height: usize, format: String) -> Image {
        Image {
            width, height, format, rows: vec![], buf: vec![0; width * height * 4]
        }
    }

    pub fn get_row(&mut self, index: usize) -> Option<&mut Row> {
        let start = index * self.width * 4;
        let end = start + self.width * 4;
        let row = self.buf.get_mut(start..end);
        let buf = row?;
        let length = self.width * 4;
        let ptr = buf.as_mut_ptr();
        let buf = unsafe {
            Vec::from_raw_parts(ptr, length, length)
        };
        self.rows.push(Row {
            length, pixels: vec![], buf: ManuallyDrop::new(buf)
        });
        let length = self.rows.len() - 1;
        self.rows.get_mut(length)
    }

    pub fn as_color_ints(&mut self) -> Vec<i32> {
        todo!()
    }
}

impl From<(usize, usize, Vec<u8>)> for Image {
    fn from(data: (usize, usize, Vec<u8>)) -> Self {
        Image {
            width: data.0, height: data.1, format: String::new(), rows: vec![], buf: data.2
        }
    }
}

impl IntoIterator for Image {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.buf.into_iter()
    }
}

impl Deref for Image {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

#[test]
fn test_rows_in_image() {
    let mut img = Image::from((5, 5, vec![
        01, 02, 03, 04,   05, 06, 07, 08,   09, 10, 11, 12,   13, 14, 15, 16,   17, 18, 19, 20,

        21, 22, 23, 24,   25, 26, 27, 28,   29, 30, 31, 32,   33, 34, 35, 36,   37, 38, 39, 40,

        41, 42, 43, 44,   45, 46, 47, 48,   49, 50, 51, 52,   53, 54, 55, 56,   57, 58, 59, 60,

        61, 62, 63, 64,   65, 66, 67, 68,   69, 70, 71, 72,   73, 74, 75, 76,   77, 78, 79, 80,

        81, 82, 83, 84,   85, 86, 87, 88,   89, 90, 91, 92,   93, 94, 95, 96,   97, 98, 99, 100
    ]));

    let row_1 = img.get_row(0).unwrap();
    let expected_row_1 = vec![
        01, 02, 03, 04,   05, 06, 07, 08,   09, 10, 11, 12,   13, 14, 15, 16,   17, 18, 19, 20
    ];
    assert_eq!(*row_1.buf, expected_row_1);

    let row_2 = img.get_row(1).unwrap();
    let expected_row_2 = vec![
        21, 22, 23, 24,   25, 26, 27, 28,   29, 30, 31, 32,   33, 34, 35, 36,   37, 38, 39, 40
    ];
    assert_eq!(*row_2.buf, expected_row_2);

    let row_3 = img.get_row(2).unwrap();
    let expected_row_3 = vec![
        41, 42, 43, 44,   45, 46, 47, 48,   49, 50, 51, 52,   53, 54, 55, 56,   57, 58, 59, 60
    ];
    assert_eq!(*row_3.buf, expected_row_3);

    let row_4 = img.get_row(3).unwrap();
    let expected_row_4 = vec![
        61, 62, 63, 64,   65, 66, 67, 68,   69, 70, 71, 72,   73, 74, 75, 76,   77, 78, 79, 80
    ];
    assert_eq!(*row_4.buf, expected_row_4);

    let row_5 = img.get_row(4).unwrap();
    let expected_row_5 = vec![
        81, 82, 83, 84,   85, 86, 87, 88,   89, 90, 91, 92,   93, 94, 95, 96,   97, 98, 99, 100
    ];
    assert_eq!(*row_5.buf, expected_row_5);

    let invalid_row = img.get_row(5);
    assert!(invalid_row.is_none());
}

#[test]
fn test_pixels_in_row() {
    let mut img = Image::from((5, 1, vec![
        01, 02, 03, 04,   05, 06, 07, 08,   09, 10, 11, 12,   13, 14, 15, 16,   17, 18, 19, 20
    ]));

    let row = img.get_row(0).unwrap();

    let pixel_1 = row.get_pixel(0).unwrap();
    assert_eq!(*pixel_1.buf, vec![01, 02, 03, 04]);

    let pixel_2 = row.get_pixel(1).unwrap();
    assert_eq!(*pixel_2.buf, vec![05, 06, 07, 08]);

    let pixel_3 = row.get_pixel(2).unwrap();
    assert_eq!(*pixel_3.buf, vec![09, 10, 11, 12]);

    let pixel_4 = row.get_pixel(3).unwrap();
    assert_eq!(*pixel_4.buf, vec![13, 14, 15, 16]);

    let pixel_5 = row.get_pixel(4).unwrap();
    assert_eq!(*pixel_5.buf, vec![17, 18, 19, 20]);

    let invalid_pixel = row.get_pixel(5);
    assert!(invalid_pixel.is_none());
}