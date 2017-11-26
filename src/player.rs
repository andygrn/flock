
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub limit_x: usize,
    pub limit_y: usize,
}

impl Player {
    pub fn try_north(&mut self) {
        if self.y == 0 {
            return
        }
        self.y -= 1;
    }
    pub fn try_east(&mut self) {
        if self.x == self.limit_x {
            return
        }
        self.x += 1;
    }
    pub fn try_south(&mut self) {
        if self.y == self.limit_y {
            return
        }
        self.y += 1;
    }
    pub fn try_west(&mut self) {
        if self.x == 0 {
            return
        }
        self.x -= 1;
    }
}
