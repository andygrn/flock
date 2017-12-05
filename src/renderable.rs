use specs::World;

use tile::TileMap;
use tile::TileMapView;

use player::Player;

pub trait Renderable {
    fn set_up(&self);
    fn render_map(&self, &TileMap, &TileMapView, &Player, &f32);
    fn render_world(&self, &World);
    fn tear_down(&self);
}
