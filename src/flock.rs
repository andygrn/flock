
use rand::{thread_rng, Rng};

use noise::{NoiseModule, Seedable, Perlin, Add, ScaleBias, ScalePoint};

use tile::Tile;
use tile::TileMap;
use tile::TileStyle;

pub fn generate_tilemap(width: usize, height: usize) -> TileMap {

    let mut map = TileMap::new(width, height);

    let noise_perlin = Perlin::new().set_seed(thread_rng().next_u32() as usize);
    let noise_perlin_2 = ScaleBias::new(ScalePoint::new(&noise_perlin).set_x_scale(2.0).set_y_scale(2.0)).set_scale(0.5);
    let noise_perlin_3 = ScaleBias::new(ScalePoint::new(&noise_perlin).set_x_scale(4.0).set_y_scale(4.0)).set_scale(0.25);
    let noise_perlin_4 = ScaleBias::new(ScalePoint::new(&noise_perlin).set_x_scale(8.0).set_y_scale(8.0)).set_scale(0.125);
    let noise_perlin_5 = ScaleBias::new(ScalePoint::new(&noise_perlin).set_x_scale(16.0).set_y_scale(16.0)).set_scale(0.0625);
    let noise_gen = Add::new(
        noise_perlin,
        Add::new(
            noise_perlin_2,
            Add::new(
                noise_perlin_3,
                Add::new(
                    noise_perlin_4,
                    noise_perlin_5
                )
            )
        )
    );

    map.fill_tiles(move |x, y| {
        let mut rand = thread_rng();
        let noise_val = noise_gen.get([x as f32 / 64.0, y as f32 / 64.0]);
        Tile {
            x: x,
            y: y,
            style: if noise_val > 0.6 {
                if rand.next_f32() > (((noise_val - 0.65) / 0.4) * 0.8) { TileStyle::RockLow } else { TileStyle::RockHigh }
            } else if noise_val > 0.4 {
                TileStyle::Dirt
            } else if noise_val > -0.4 {
                if rand.next_f32() > 0.05 { TileStyle::GrassPlain } else { TileStyle::Tree }
            } else if noise_val > -0.6 {
                TileStyle::GrassCoastal
            } else if noise_val > -0.7 {
                TileStyle::Sand
            } else if noise_val > -0.95 {
                TileStyle::WaterShallow
            } else {
                TileStyle::WaterDeep
            },
            rand_offset: rand.next_f32()
        }
    });

    map

}
