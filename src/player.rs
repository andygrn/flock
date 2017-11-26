
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub limit_x: usize,
    pub limit_y: usize,
}

impl Player {
    pub fn go_north(&mut self) {
        self.y -= 1;
    }
    pub fn go_east(&mut self) {
        self.x += 1;
    }
    pub fn go_south(&mut self) {
        self.y += 1;
    }
    pub fn go_west(&mut self) {
        self.x -= 1;
    }
}
