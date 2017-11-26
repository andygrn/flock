extern crate noise;
extern crate termion;
extern crate rand;

mod tile;
mod player;
mod flock;

use std::io::{Write, Read};
use std::{thread, time};

use termion::raw::IntoRawMode;
use termion::{color, clear, style, cursor};
use termion::event::Key;
use termion::input::TermRead;

use rand::{thread_rng, Rng};

use std::sync::{Arc, Mutex};

use tile::TileStyle;
use tile::TileMapView;
use tile::TermTile;

use player::Player;

fn main() {

    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut stdin = std::io::stdin();

    let map = Arc::new(Mutex::new(flock::generate_tilemap(300, 300)));

    let player = {
        let map = map.lock().unwrap();
        Arc::new(Mutex::new(Player { x: 0, y: 0, limit_x: map.width - 1, limit_y: map.height - 1 }))
    };

    {
        let mut stdout = stdout.lock();
        write!(stdout, "{}", cursor::Hide).unwrap();
    }

    let player_render = player.clone();
    let map_render = map.clone();

    thread::spawn(move || {

        const VIEW_PADDING: usize = 10;
        let frame_sleep = time::Duration::from_millis(64);

        let material_rock_high = TermTile::new([107, 103, 98], [117, 113, 107], |offset, _rand| {
            if offset > 0.8 { '#' } else if offset > 0.6 { '%' } else if offset > 0.4 { '=' } else { ' ' }
        });
        let material_rock_low = TermTile::new([91, 88, 84], [117, 113, 107], |offset, _rand| {
            if offset > 0.8 { '#' } else if offset > 0.6 { '%' } else if offset > 0.4 { '=' } else { ' ' }
        });
        let material_dirt = TermTile::new([71, 56,  19], [122, 97, 33], |offset, _rand| {
            if offset > 0.8 { '~' } else if offset > 0.6 { 'o' } else if offset > 0.4 { '.' } else { ' ' }
        });
        let material_tree = TermTile::new([99, 130, 35], [76, 91, 47], |offset, _rand| {
            if offset > 0.8 { '*' } else if offset > 0.6 { 'V' } else if offset > 0.4 { ':' } else { 'Y' }
        });
        let material_grass_plain = TermTile::new([99, 130, 35], [136, 181, 48], |offset, rand| {
            if offset > 0.8 { 'v' } else if offset > 0.7 { ',' } else if offset > 0.4 { '.' } else if offset > 0.05 { ' ' } else if (rand + offset) % 1.0 > 0.9 { ',' } else { '.' }
        });
        let material_grass_coastal = TermTile::new([99, 130, 35], [136, 181, 48], |offset, rand| {
            if offset > 0.9 { ',' } else if offset > 0.75 { '.' } else if offset > 0.05 { ' ' } else if (rand + offset) % 1.0 > 0.9 { ',' } else { '.' }
        });
        let material_sand = TermTile::new([140, 134, 107], [165, 158, 127], |offset, _rand| {
            if offset > 0.8 { '~' } else if offset > 0.6 { '-' } else if offset > 0.4 { '.' } else { ' ' }
        });
        let material_water_shallow = TermTile::new([84, 116, 122], [102, 141, 147], |offset, rand| {
            if (rand + offset) % 1.0 > 0.8 { '~' } else { ' ' }
        });
        let material_water_deep = TermTile::new([77, 106, 112], [102, 141, 147], |offset, rand| {
            if (rand + offset) % 1.0 > 0.8 { '~' } else { ' ' }
        });

        let mut view = {
            let map = map_render.lock().unwrap();
            TileMapView::new(&map, 80, 30, 0, 0)
        };

        let mut frame_counter: u8 = 0;
        let mut rand: f32 = 0.0;

        loop {

            thread::sleep(frame_sleep);

            if frame_counter == 0 {
                rand = thread_rng().next_f32();
            }

            frame_counter += 1;
            frame_counter %= 10;

            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let player = player_render.lock().unwrap();
            let map = map_render.lock().unwrap();

            let player_coord = view.world_to_view_coord(player.x, player.y);
            if player_coord.x < VIEW_PADDING as isize {
                view.go_west(1);
            } else if player_coord.x > view.width as isize - VIEW_PADDING as isize {
                view.go_east(1);
            }
            if player_coord.y < VIEW_PADDING as isize {
                view.go_north(1);
            } else if player_coord.y > view.height as isize - VIEW_PADDING as isize {
                view.go_south(1);
            }

            for (y, row) in view.get_tile_ranges().iter().enumerate() {
                for (x, tile) in map.tiles[row.start..row.end].iter().enumerate() {
                    write!(
                        stdout,
                        "{}{}{}{}",
                        cursor::Goto(x as u16 + 1, y as u16 + 1),
                        match tile.style {
                            TileStyle::RockHigh     => material_rock_high.colour_bg,
                            TileStyle::RockLow      => material_rock_low.colour_bg,
                            TileStyle::Dirt         => material_dirt.colour_bg,
                            TileStyle::Tree         => material_tree.colour_bg,
                            TileStyle::GrassPlain   => material_grass_plain.colour_bg,
                            TileStyle::GrassCoastal => material_grass_coastal.colour_bg,
                            TileStyle::Sand         => material_sand.colour_bg,
                            TileStyle::WaterShallow => material_water_shallow.colour_bg,
                            TileStyle::WaterDeep    => material_water_deep.colour_bg,
                        },
                        match tile.style {
                            TileStyle::RockHigh     => material_rock_high.colour_fg,
                            TileStyle::RockLow      => material_rock_low.colour_fg,
                            TileStyle::Dirt         => material_dirt.colour_fg,
                            TileStyle::Tree         => material_tree.colour_fg,
                            TileStyle::GrassPlain   => material_grass_plain.colour_fg,
                            TileStyle::GrassCoastal => material_grass_coastal.colour_fg,
                            TileStyle::Sand         => material_sand.colour_fg,
                            TileStyle::WaterShallow => material_water_shallow.colour_fg,
                            TileStyle::WaterDeep    => material_water_deep.colour_fg,
                        },
                        match tile.style {
                            TileStyle::RockHigh     => (material_rock_high.char_gen)(tile.rand_offset, rand),
                            TileStyle::RockLow      => (material_rock_low.char_gen)(tile.rand_offset, rand),
                            TileStyle::Dirt         => (material_dirt.char_gen)(tile.rand_offset, rand),
                            TileStyle::Tree         => (material_tree.char_gen)(tile.rand_offset, rand),
                            TileStyle::GrassPlain   => (material_grass_plain.char_gen)(tile.rand_offset, rand),
                            TileStyle::GrassCoastal => (material_grass_coastal.char_gen)(tile.rand_offset, rand),
                            TileStyle::Sand         => (material_sand.char_gen)(tile.rand_offset, rand),
                            TileStyle::WaterShallow => (material_water_shallow.char_gen)(tile.rand_offset, rand),
                            TileStyle::WaterDeep    => (material_water_deep.char_gen)(tile.rand_offset, rand),
                        }
                    ).unwrap();
                }
            }

            write!(
                stdout,
                "{}{}{}&",
                cursor::Goto(
                    (player_coord.x + 1) as u16,
                    (player_coord.y + 1) as u16
                ),
                color::Bg(color::Black),
                color::Fg(color::White)
            ).unwrap();

            stdout.flush().unwrap();

        }
    });

    'gameloop: loop {

        for c in stdin.by_ref().keys() {
            let mut player = player.lock().unwrap();
            match c.unwrap() {
                // Exit.
                // Key::Char('q') => break,
                // Key::Char(c)   => println!("{}", c),
                // Key::Alt(c)    => println!("Alt-{}", c),
                // Key::Ctrl(c)   => println!("Ctrl-{}", c),
                Key::Up | Key::Char('w') => {
                    if player.y == 0 {
                        break
                    }
                    let map = map.lock().unwrap();
                    let target_tile = map.get_tile(player.x, player.y - 1);
                    if let Some(tile) = target_tile {
                        match tile.style {
                            TileStyle::RockLow | TileStyle::Dirt | TileStyle::GrassPlain | TileStyle::GrassCoastal | TileStyle::Sand => player.go_north(),
                            _ => {}
                        }
                    }
                },
                Key::Right | Key::Char('d') => {
                    if player.x == player.limit_x {
                        break
                    }
                    let map = map.lock().unwrap();
                    let target_tile = map.get_tile(player.x + 1, player.y);
                    if let Some(tile) = target_tile {
                        match tile.style {
                            TileStyle::RockLow | TileStyle::Dirt | TileStyle::GrassPlain | TileStyle::GrassCoastal | TileStyle::Sand => player.go_east(),
                            _ => {}
                        }
                    }
                },
                Key::Down | Key::Char('s') => {
                    if player.y == player.limit_y {
                        break
                    }
                    let map = map.lock().unwrap();
                    let target_tile = map.get_tile(player.x, player.y + 1);
                    if let Some(tile) = target_tile {
                        match tile.style {
                            TileStyle::RockLow | TileStyle::Dirt | TileStyle::GrassPlain | TileStyle::GrassCoastal | TileStyle::Sand => player.go_south(),
                            _ => {}
                        }
                    }
                },
                Key::Left | Key::Char('a') => {
                    if player.x == 0 {
                        break
                    }
                    let map = map.lock().unwrap();
                    let target_tile = map.get_tile(player.x - 1, player.y);
                    if let Some(tile) = target_tile {
                        match tile.style {
                            TileStyle::RockLow | TileStyle::Dirt | TileStyle::GrassPlain | TileStyle::GrassCoastal | TileStyle::Sand => player.go_west(),
                            _ => {}
                        }
                    }
                },
                _ => break 'gameloop,
            }
            break;
        }

    }

    {
        let mut stdout = stdout.lock();
        write!(stdout, "{}", style::Reset).unwrap();
        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(stdout, "{}", cursor::Show).unwrap();
    }

}
