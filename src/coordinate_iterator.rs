pub struct CoordinateIterator {
    row: u32,
    col: i32,
    max_width: u32,
}

impl CoordinateIterator {
    pub fn new(max_width: u32, max_height: u32) -> CoordinateIterator {
        CoordinateIterator {
            row: max_height - 1,
            col: -1,
            max_width,
        }
    }

    pub fn next(&mut self) -> Option<(u32, u32)> {
        if self.col as u32 == self.max_width - 1 && self.row == 0 {
            return None;
        }

        if self.col as u32 == self.max_width - 1 {
            self.col = 0;
            self.row -= 1;
        } else {
            self.col += 1;
        }

        Some((self.row, self.col as u32))
    }
}
