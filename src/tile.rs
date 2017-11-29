use std::ops::Range;
use std::cmp;

pub enum TileStyle {
    RockHigh,
    RockLow,
    Dirt,
    DirtFarmed,
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
    x: isize,
    y: isize,
    map_width: usize,
    map_height: usize,
}

impl TileMapView {
    pub fn new(map: &TileMap, width: usize, height: usize) -> TileMapView {
        TileMapView {
            width: width,
            height: height,
            x: 0,
            y: 0,
            map_width: map.width,
            map_height: map.height,
        }
    }

    pub fn get_tile_ranges(&self) -> Vec<Range<usize>> {
        let mut ranges = Vec::new();
        let x_offset;
        let y_offset;
        let rows;
        let cols;
        if self.y > 0 {
            y_offset = self.y;
            rows = cmp::min(self.height as isize, self.map_height as isize - self.y);
        } else {
            y_offset = 0;
            rows = self.height as isize + self.y;
        }
        if self.x > 0 {
            x_offset = self.x;
            cols = cmp::min(self.width as isize, self.map_width as isize - self.x);
        } else {
            x_offset = 0;
            cols = self.width as isize + self.x;
        }

        for y in 0..rows as isize {
            let left_i = ((y_offset + y) * self.map_width as isize) + x_offset;
            ranges.push((left_i as usize)..((left_i + cols) as usize));
        }
        ranges
    }

    pub fn world_to_view_coord(&self, x: usize, y: usize) -> Coord {
        Coord {
            x: (x as isize - self.x),
            y: (y as isize - self.y),
        }
    }

    pub fn centre_on_map_point(&mut self, x: usize, y: usize) {
        self.x = x as isize - (self.width / 2) as isize;
        self.y = y as isize - (self.height / 2) as isize;
    }
}
