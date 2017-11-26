
use tile::TileMap;
use tile::TileMapView;

use player::Player;

pub trait Renderable {
    fn set_up(&self);
    fn render_frame(&self, &TileMap, &TileMapView, &Player, &f32);
    fn tear_down(&self);
}
