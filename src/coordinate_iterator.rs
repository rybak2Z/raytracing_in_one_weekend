pub struct CoordinateIterator {
    x: i32,
    y: u32,
    max_width: u32,
}

impl CoordinateIterator {
    pub fn new(max_width: u32, max_height: u32) -> CoordinateIterator {
        CoordinateIterator { x: -1, y: max_height - 1, max_width, }
    }

    pub fn next(&mut self) -> (u32, u32) {
        if self.x as u32 == self.max_width - 1 {
            self.x = 0;
            self.y -= 1;
        } else {
            self.x += 1;
        }

        (self.x as u32, self.y)
    }
}