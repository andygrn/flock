
use rand::{thread_rng, Rng};

use noise::{NoiseModule, Seedable, Perlin, Add, ScaleBias, ScalePoint};

use tile::Tile;
use tile::TileMap;
use tile::TileStyle;

fn get_noise_map() -> Add<Perlin, Add<ScaleBias<ScalePoint<Perlin, f32>, f32>, Add<ScaleBias<ScalePoint<Perlin, f32>, f32>, Add<ScaleBias<ScalePoint<Perlin, f32>, f32>, ScaleBias<ScalePoint<Perlin, f32>, f32>>>>> {
    let perlin_1 = Perlin::new().set_seed(thread_rng().next_u32() as usize);
    let perlin_2 = ScaleBias::new(ScalePoint::new(perlin_1).set_x_scale(2.0).set_y_scale(2.0)).set_scale(0.5);
    let perlin_3 = ScaleBias::new(ScalePoint::new(perlin_1).set_x_scale(4.0).set_y_scale(4.0)).set_scale(0.25);
    let perlin_4 = ScaleBias::new(ScalePoint::new(perlin_1).set_x_scale(8.0).set_y_scale(8.0)).set_scale(0.125);
    let perlin_5 = ScaleBias::new(ScalePoint::new(perlin_1).set_x_scale(16.0).set_y_scale(16.0)).set_scale(0.0625);
    Add::new(
        perlin_1,
        Add::new(
            perlin_2,
            Add::new(
                perlin_3,
                Add::new(
                    perlin_4,
                    perlin_5
                )
            )
        )
    )
}

pub fn generate_tilemap(width: usize, height: usize) -> TileMap {

    let mut map = TileMap::new(width, height);

    let heightmap = get_noise_map();
    let terrain = get_noise_map();
    let vegetation = get_noise_map();

    map.fill_tiles(move |x, y| {
        let mut rand = thread_rng();
        let coord = [x as f32 / 64.0, y as f32 / 64.0];
        let tile_height = heightmap.get(coord);
        Tile {
            x: x,
            y: y,
            style: if tile_height > 0.75 {
                if rand.next_f32() > (((tile_height - 0.65) / 0.4) * 0.8) { TileStyle::RockLow } else { TileStyle::RockHigh }
            } else if tile_height > 0.55 {
                TileStyle::Dirt
            } else if tile_height > -0.5 {
                let tile_terrain = terrain.get(coord);
                if tile_terrain > 0.7 {
                    TileStyle::Dirt
                } else {
                    let tile_vegetation = vegetation.get(coord);
                    if tile_vegetation > 0.6 {
                        if rand.next_f32() > 0.15 { TileStyle::GrassPlain } else { TileStyle::Tree }
                    } else if tile_vegetation > 0.5 {
                        if rand.next_f32() > 0.05 { TileStyle::GrassPlain } else { TileStyle::Tree }
                    } else {
                        TileStyle::GrassCoastal
                    }
                }
            } else if tile_height > -0.6 {
                TileStyle::GrassCoastal
            } else if tile_height > -0.7 {
                TileStyle::Sand
            } else if tile_height > -0.95 {
                TileStyle::WaterShallow
            } else {
                TileStyle::WaterDeep
            },
            rand_offset: rand.next_f32()
        }
    });

    map

}
