#![allow(unused_variables, dead_code)]
use crate::{image::Image, object::Rect};

pub struct Canvas {
    image: Image,
}

pub trait Draw<T> {
    fn draw(self, object: T, origin: (i32, i32)) -> Self;
}

impl Draw<Image> for Canvas {
    fn draw(mut self, mut object: Image, origin: (i32, i32)) -> Self {
        // where to start drawing in canvas
        let start_x;
        // how many columns should i skip while drawing image
        let skip_x;
        if origin.0 < 0 {
            start_x = (object.width - origin.0 as usize) * 4;
            skip_x = 0;
        } else {
            start_x = 0;
            skip_x = (origin.0 as usize) * 4
        }

        // where to stop drawing
        let stop_x;
        if object.width < self.image.width {
            stop_x = object.width * 4;
        } else {
            stop_x = self.image.width * 4;
        }

        let start_y;
        let skip_y;
        if origin.1 < 0 {
            start_y = (object.height - origin.1 as usize) * 4;
            skip_y = 0;
        } else {
            start_y = 0;
            skip_y = (origin.1 as usize) * 4;
        }

        let stop_y;
        if object.height < self.image.height {
            stop_y = object.height * 4;
        } else {
            stop_y = self.image.height * 4;
        }

        let width = self.image.width;
        let mut y = 0;
        let mut ix;
        let mut iy = 0;
        for row in self.image.chunks_mut(width) {
            if y < start_y {
                continue;
            }
            if y > stop_y {
                break;
            }

            ix = 0;
            let chunk = object.get_row(iy + skip_y).unwrap();
            for x in start_x..stop_x {
                row[x] = *chunk.get(ix + skip_x).unwrap();

                ix += 1;
            }

            y += 1;
            iy += 1;
        }

        self
    }
}

impl Draw<Rect> for Canvas {
    fn draw(self, object: Rect, origin: (i32, i32)) -> Self {
        todo!()
    }
}