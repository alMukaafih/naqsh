#![allow(dead_code)]
/// A Pixel in an Image.
pub struct Pixel {
    buf: Vec<u8>
}

pub struct Row {
    pub length: usize,
    pixels: Vec<Pixel>,
    buf: Vec<u8>
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
        self.pixels.push(Pixel {buf});
        let length = self.pixels.len() - 1;
        self.pixels.get_mut(length)
    }
}

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
            width, height, format, rows: vec![], buf: vec![0; width * height]
        }
    }

    pub fn get_row(&mut self, index: usize) -> Option<&mut Row> {
        let start = index * self.width;
        let end = start + self.width;
        let row = self.buf.get_mut(start..end);
        let buf = row?;
        let ptr = buf.as_mut_ptr();
        let length = self.width;
        let buf = unsafe {
            Vec::from_raw_parts(ptr, length, length)
        };
        self.rows.push(Row {
            length, pixels: vec![], buf
        });
        let length = self.rows.len() - 1;
        self.rows.get_mut(length)
    }
}
