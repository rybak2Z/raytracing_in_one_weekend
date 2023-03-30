use crate::config::{IMAGE_HEIGHT, IMAGE_WIDTH};

pub struct CoordinateIterator {
    row: u32,
    col: i32,
}

impl CoordinateIterator {
    pub fn new() -> CoordinateIterator {
        CoordinateIterator {
            row: IMAGE_HEIGHT - 1,
            col: -1,
        }
    }

    pub fn next(&mut self) -> Option<(u32, u32)> {
        if self.col as u32 == IMAGE_WIDTH - 1 && self.row == 0 {
            return None;
        }

        if self.col as u32 == IMAGE_WIDTH - 1 {
            self.col = 0;
            self.row -= 1;
        } else {
            self.col += 1;
        }

        Some((self.row, self.col as u32))
    }
}
