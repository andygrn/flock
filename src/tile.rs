use std::ops::Range;

pub enum TileStyle {
    RockHigh,
    RockLow,
    Dirt,
    Tree,
    GrassPlain,
    GrassCoastal,
    Sand,
    WaterShallow,
    WaterDeep,
}

pub struct Coord {
    pub x: isize,
    pub y: isize,
}

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub style: TileStyle,
    pub rand_offset: f32,
}

pub struct TileMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> TileMap {
        TileMap {
            width: width,
            height: height,
            tiles: Vec::new(),
        }
    }

    pub fn fill_tiles<F>(&mut self, factory: F)
    where
        F: Fn(usize, usize) -> Tile,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                self.tiles.push(factory(x, y));
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        let index = (y * self.width) + x;
        if index > self.tiles.len() - 1 {
            return None;
        }
        Some(&self.tiles[index])
    }
}

pub struct TileMapView {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    map_width: usize,
    limit_x: usize,
    limit_y: usize,
}

impl TileMapView {
    pub fn new(map: &TileMap, width: usize, height: usize, x: usize, y: usize) -> TileMapView {
        TileMapView {
            width: width,
            height: height,
            x: x,
            y: y,
            map_width: map.width,
            limit_x: map.width - width,
            limit_y: map.height - height,
        }
    }

    pub fn get_tile_ranges(&self) -> Vec<Range<usize>> {
        let mut ranges = Vec::new();
        for y in 0..self.height {
            let left_i = ((self.y + y) * self.map_width) + self.x;
            ranges.push(left_i..(left_i + self.width));
        }
        ranges
    }

    pub fn world_to_view_coord(&self, x: usize, y: usize) -> Coord {
        let view_x: isize = x as isize - self.x as isize;
        let view_y: isize = y as isize - self.y as isize;
        Coord {
            x: view_x,
            y: view_y,
        }
    }

    pub fn go_north(&mut self, steps: usize) {
        if self.y.checked_sub(steps).is_none() {
            return;
        }
        self.y -= steps;
    }

    pub fn go_east(&mut self, steps: usize) {
        if self.x + steps > self.limit_x {
            return;
        }
        self.x += steps;
    }

    pub fn go_south(&mut self, steps: usize) {
        if self.y + steps > self.limit_y {
            return;
        }
        self.y += 1;
    }

    pub fn go_west(&mut self, steps: usize) {
        if self.x.checked_sub(steps).is_none() {
            return;
        }
        self.x -= 1;
    }
}
