extern crate noise;
extern crate rand;
extern crate termion;

mod tile;
mod player;
mod flock;
mod renderable;
mod terminal_renderer;

use std::{thread, time};
use std::io::Read;
use std::sync::{mpsc, Arc, Mutex};

use termion::event::Key;
use termion::input::TermRead;

use rand::{thread_rng, Rng};

use tile::TileMapView;

use player::Player;

use renderable::Renderable;

use terminal_renderer::Renderer;

fn main() {
    // Game setup
    let mut stdin = std::io::stdin();
    let map = Arc::new(Mutex::new(flock::generate_tilemap(300, 300)));
    let player = {
        let map = map.lock().unwrap();
        Arc::new(Mutex::new(Player {
            x: map.width / 2,
            y: map.height / 2,
            limit_x: map.width - 1,
            limit_y: map.height - 1,
        }))
    };

    // Thread control channels
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Spawn render thread
    let player_render = player.clone();
    let map_render = map.clone();
    thread::spawn(move || {
        let frame_sleep = time::Duration::from_millis(64); // ~15 fps

        let mut view = {
            let map = map_render.lock().unwrap();
            TileMapView::new(&map, 80, 40)
        };

        let renderer = Renderer::new();

        let mut frame_counter: u8 = 0;
        let mut rand: f32 = 0.0;

        renderer.set_up();

        loop {
            // If message is received from main thread, break the frame loop.
            if rx1.try_recv().is_ok() {
                break;
            }

            thread::sleep(frame_sleep);

            if frame_counter == 0 {
                rand = thread_rng().next_f32();
            }

            frame_counter += 1;
            frame_counter %= 10;

            {
                let player = player_render.lock().unwrap();
                view.centre_on_map_point(player.x, player.y);
            }

            {
                let player = player_render.lock().unwrap();
                let map = map_render.lock().unwrap();
                renderer.render_frame(&map, &view, &player, &rand);
            }
        }

        renderer.tear_down();

        // Notify main thread that render thread has finished tearing down.
        tx2.send(()).unwrap();
    });

    'gameloop: loop {
        for c in stdin.by_ref().keys() {
            let mut player = player.lock().unwrap();
            match c.unwrap() {
                Key::Up | Key::Char('w') => {
                    if player.y == 0 {
                        break;
                    }
                    if let Some(tile) = map.lock().unwrap().get_tile(player.x, player.y - 1) {
                        if flock::tile_is_passable(tile) {
                            player.go_north();
                        }
                    }
                }
                Key::Right | Key::Char('d') => {
                    if player.x == player.limit_x {
                        break;
                    }
                    if let Some(tile) = map.lock().unwrap().get_tile(player.x + 1, player.y) {
                        if flock::tile_is_passable(tile) {
                            player.go_east();
                        }
                    }
                }
                Key::Down | Key::Char('s') => {
                    if player.y == player.limit_y {
                        break;
                    }
                    if let Some(tile) = map.lock().unwrap().get_tile(player.x, player.y + 1) {
                        if flock::tile_is_passable(tile) {
                            player.go_south();
                        }
                    }
                }
                Key::Left | Key::Char('a') => {
                    if player.x == 0 {
                        break;
                    }
                    if let Some(tile) = map.lock().unwrap().get_tile(player.x - 1, player.y) {
                        if flock::tile_is_passable(tile) {
                            player.go_west();
                        }
                    }
                }
                _ => break 'gameloop,
            }
            break;
        }
    }

    // Now gameloop has stopped, tell render thread to tear down...
    tx1.send(()).unwrap();
    // ...then block until render thread sends confirmation.
    rx2.recv().unwrap();
}
