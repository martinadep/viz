use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::world_generator::{Generator, World};
use tetra::{Context};
use tetra::graphics::{DrawParams, Texture};
use tetra::math::Vec2;
use tyrannousarust_rex_world_generator::WorldGenerator;
use crate::visualizer::{PIXEL, SCALE, TOP_OFFSET};
use crate::visualizer::textures::Texturizable;

pub struct VisMap{
    map : Vec<Vec<Option<(Texture, Texture)>>>,
    weather : WeatherType,
}
impl VisMap{
    pub fn new (ctx : &mut Context) -> Self{
        let mappa = from_world_to_map(WorldGenerator::new().set_size(20).gen(), ctx);
        Self{
            map: mappa,
            weather : WeatherType::Sunny,
        }
    }
    pub fn update_map(&mut self, discovered_map : &Vec<Vec<Option<Tile>>>, ctx : &mut Context){
        let mut final_matrix = vec![];
        for row in discovered_map {
            let mut row_to_add: Vec<Option<(Texture, Texture)>> = vec![];
            for opt_tile in row {
                match opt_tile {
                    None => {row_to_add.push(None)}
                    Some(tile) => {
                        row_to_add.push(Some((
                            tile.tile_type.get_texture(ctx),
                            tile.content.get_texture(ctx)))
                        )
                    }
                }
            }
            final_matrix.push(row_to_add)
        }
        self.map = final_matrix
    }
    pub fn draw(&mut self, ctx : &mut Context){
        let mut y = 0.0;
        let mut x = 0.0;
        for row in &self.map {
            for opt_tile in row {
                if opt_tile.is_some() {
                    let (tiletype_texture, content_texture ) = opt_tile.clone().unwrap();
                    tiletype_texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new( x, TOP_OFFSET + y))
                            .scale(Vec2::new(SCALE, SCALE)),
                    );
                    content_texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new( x, TOP_OFFSET + y))
                            .scale(Vec2::new(SCALE, SCALE)),
                    );

                }
                x += PIXEL * SCALE;
            }
            x = 0.0;
            y += PIXEL * SCALE;
        }
        /*match self.weather {
            WeatherType::Sunny => { graphics::clear(ctx, Color::rgba(0.8, 0.8, 0.1, 0.8)); }
            WeatherType::Rainy => { graphics::clear(ctx, Color::rgba(0.2, 0.2, 0.5, 0.2)); }
            WeatherType::Foggy => { graphics::clear(ctx, Color::rgba(0.7, 0.7, 0.7, 0.2)); }
            WeatherType::TropicalMonsoon => { graphics::clear(ctx, Color::rgba(0.8, 0.2, 0.4, 0.2)); }
            WeatherType::TrentinoSnow => { graphics::clear(ctx, Color::rgba(1.0, 1.0, 1.0, 0.2)); }
        }*/
    }
}

pub fn from_world_to_map(world : World, ctx : &mut Context) -> Vec<Vec<Option<(Texture, Texture)>>>{
    println!("--- from world to map: ");
    let wlen = world.0.len();
    let mut final_matrix = vec![];
    for (i, row) in world.0.iter().enumerate() {
        let mut row_to_add= vec![];
        for tile in row {
            row_to_add.push(Some((
                tile.tile_type.get_texture(ctx),
                tile.content.get_texture(ctx)))
            );
            //print!("{:?} | ", tile.tile_type);
        }
        println!("\trow {}/{wlen} added to map", i+1);
        final_matrix.push(row_to_add)
    }
    println!("--- map initialized");
    final_matrix
}