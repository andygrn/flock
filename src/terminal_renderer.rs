use std;
use std::io::Write;
use std::cell::RefCell;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, color, cursor, style};

use tile::TileMap;
use tile::TileMapView;
use tile::TileStyle;

use player::Player;

use renderable::Renderable;

struct TermTileStyle {
    pub colour_bg: color::Bg<color::Rgb>,
    pub colour_fg: color::Fg<color::Rgb>,
    pub char_gen: (fn(f32, f32) -> char),
}

impl TermTileStyle {
    pub fn new(
        colour_bg: [u8; 3],
        colour_fg: [u8; 3],
        char_gen: (fn(f32, f32) -> char),
    ) -> TermTileStyle {
        TermTileStyle {
            colour_bg: color::Bg(color::Rgb(colour_bg[0], colour_bg[1], colour_bg[2])),
            colour_fg: color::Fg(color::Rgb(colour_fg[0], colour_fg[1], colour_fg[2])),
            char_gen: char_gen,
        }
    }
}

pub struct Renderer {
    stdout: RefCell<RawTerminal<std::io::Stdout>>,
    tile_styles: Vec<TermTileStyle>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let output = std::io::stdout().into_raw_mode().unwrap();
        Renderer {
            stdout: RefCell::new(output),
            tile_styles: vec![
                // rock high
                TermTileStyle::new([107, 103, 98], [117, 113, 107], |offset, _rand| {
                    if offset > 0.8 { '#' } else if offset > 0.6 { '%' } else if offset > 0.4 { '=' } else { ' ' }
                }),
                // rock low
                TermTileStyle::new([91, 88, 84], [117, 113, 107], |offset, _rand| {
                    if offset > 0.8 { '#' } else if offset > 0.6 { '%' } else if offset > 0.4 { '=' } else { ' ' }
                }),
                // dirt
                TermTileStyle::new([71, 56,  19], [122, 97, 33], |offset, _rand| {
                    if offset > 0.8 { '~' } else if offset > 0.6 { 'o' } else if offset > 0.4 { '.' } else { ' ' }
                }),
                // dirt farmed
                TermTileStyle::new([71, 56,  19], [136, 181, 48], |offset, _rand| {
                    if offset > 0.8 { 'v' } else if offset > 0.5 { '\'' } else if offset > 0.25 { '.' } else { '~' }
                }),
                // tree
                TermTileStyle::new([99, 130, 35], [76, 91, 47], |offset, _rand| {
                    if offset > 0.8 { '*' } else if offset > 0.6 { 'V' } else if offset > 0.4 { ':' } else { 'Y' }
                }),
                // grass plain
                TermTileStyle::new([99, 130, 35], [136, 181, 48], |offset, rand| {
                    if offset > 0.8 { 'v' } else if offset > 0.7 { ',' } else if offset > 0.4 { '.' } else if offset > 0.15 { ' ' } else if (rand + offset) % 1.0 > 0.5 { ',' } else { '.' }
                }),
                // grass coastal
                TermTileStyle::new([99, 130, 35], [136, 181, 48], |offset, rand| {
                    if offset > 0.9 { ',' } else if offset > 0.75 { '.' } else if offset > 0.15 { ' ' } else if (rand + offset) % 1.0 > 0.5 { ',' } else { '.' }
                }),
                // sand
                TermTileStyle::new([140, 134, 107], [165, 158, 127], |offset, _rand| {
                    if offset > 0.8 { '~' } else if offset > 0.6 { '-' } else if offset > 0.4 { '.' } else { ' ' }
                }),
                // water shallow
                TermTileStyle::new([84, 116, 122], [102, 141, 147], |offset, rand| {
                    if (rand + offset) % 1.0 > 0.8 { '~' } else { ' ' }
                }),
                // water deep
                TermTileStyle::new([77, 106, 112], [102, 141, 147], |offset, rand| {
                    if (rand + offset) % 1.0 > 0.8 { '~' } else { ' ' }
                })
            ]
        }
    }

    fn get_tile_style(&self, tile_style: &TileStyle) -> &TermTileStyle {
        match *tile_style {
            TileStyle::RockHigh     => &self.tile_styles[0],
            TileStyle::RockLow      => &self.tile_styles[1],
            TileStyle::Dirt         => &self.tile_styles[2],
            TileStyle::DirtFarmed   => &self.tile_styles[3],
            TileStyle::Tree         => &self.tile_styles[4],
            TileStyle::GrassPlain   => &self.tile_styles[5],
            TileStyle::GrassCoastal => &self.tile_styles[6],
            TileStyle::Sand         => &self.tile_styles[7],
            TileStyle::WaterShallow => &self.tile_styles[8],
            TileStyle::WaterDeep    => &self.tile_styles[9],
        }
    }
}

impl Renderable for Renderer {
    fn set_up(&self) {
        write!(self.stdout.borrow_mut(), "{}", cursor::Hide).unwrap();
    }

    fn render_frame(
        &self,
        ref map: &TileMap,
        ref map_view: &TileMapView,
        ref player: &Player,
        &rand: &f32,
    ) {
        let mut buffer = String::with_capacity(map_view.width * map_view.height * 45);
        buffer.push_str(&format!("{}", clear::All));
        {
            for row in map_view.get_tile_ranges().iter() {
                for tile in map.tiles[row.start..row.end].iter() {
                    let tile_style = self.get_tile_style(&tile.style);
                    let tile_coord = map_view.world_to_view_coord(tile.x, tile.y);
                    buffer.push_str(&format!(
                        "{}{}{}{}",
                        cursor::Goto(tile_coord.x as u16 + 1, tile_coord.y as u16 + 1),
                        tile_style.colour_bg,
                        tile_style.colour_fg,
                        (tile_style.char_gen)(tile.rand_offset, rand)
                    ));
                }
            }
        }
        {
            let player_coord = map_view.world_to_view_coord(player.x, player.y);
            buffer.push_str(&format!(
                "{}{}{}&",
                cursor::Goto((player_coord.x + 1) as u16, (player_coord.y + 1) as u16),
                color::Bg(color::Black),
                color::Fg(color::White)
            ));
        }
        let mut stdout = self.stdout.borrow_mut();
        write!(stdout, "{}", buffer).unwrap();
        write!(stdout, "{}{}", cursor::Goto(1, 1), buffer.len()).unwrap();
        write!(stdout, "{}{}", cursor::Goto(8, 1), buffer.capacity()).unwrap();
        write!(stdout, "{}{},{}", cursor::Goto(15, 1), player.x, player.y).unwrap();
        stdout.flush().unwrap();
    }

    fn tear_down(&self) {
        let mut stdout = self.stdout.borrow_mut();
        write!(stdout, "{}", style::Reset).unwrap();
        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(stdout, "{}", cursor::Show).unwrap();
    }
}
