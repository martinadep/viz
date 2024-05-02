use std::ffi::c_ushort;
use robotics_lib::interface::go;
use robotics_lib::runner::Robot;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Content::Market;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType::ShallowWater;
use robotics_lib::world::world_generator::{Generator, World};
use tetra::{Context};
use tetra::graphics::{DrawParams, Texture};
use tetra::math::Vec2;
use tyrannousarust_rex_world_generator::WorldGenerator;
use crate::visualizer::{PIXEL, TOP_OFFSET};
use crate::visualizer::textures::Texturizable;

pub struct VisMap{
    discovered_map: Vec<Vec<Option<(Texture, Texture)>>>,
    weather : WeatherType,

    world_size : usize,
    robot_texture : Texture,
    robot_position: (usize, usize)
}
impl VisMap{
    ///create a new viusalizable [size x size] map of 'None' contents
    pub fn new (ctx : &mut Context, size : usize) -> Self{
        //let mappa = from_world_to_map(WorldGenerator::new().set_size(10).set_seed(5).gen(), ctx);
        let mut mappa= vec![vec![None; size]; size];
        Self{
            discovered_map: mappa,
            weather : WeatherType::Sunny,
            robot_texture : Texture::new(ctx, "./utils/robot.png").expect("failed to upload robot image"),
            robot_position: (1,1),
            world_size : size
        }
    }

    ///updates the map with the discovered tiles
    pub fn update_map(&mut self, view: Vec<Vec<Option<Tile>>>, ctx: &mut Context){
        let (new_row,new_col) = (self.robot_position.0, self.robot_position.1);
        view.iter().enumerate().for_each(|(i, vector)| {
            vector.iter().enumerate().for_each(|(j, _)| {
                    let row = new_row + i - 1;
                    let col = new_col + j - 1;
                    if let Some(tile) = view[i][j].clone(){
                        self.discovered_map[row][col] = Some((tile.tile_type.get_texture(ctx),
                                                                      tile.content.get_texture(ctx)));
                    }
            })
        });
    }
    ///updates the robot (texture pointer) position on the map
    pub fn update_robot_pos(&mut self, new_pos : (usize, usize)){
        println!("robot pos updated from [{},{}] to [{},{}]",self.robot_position.0, self.robot_position.1, new_pos.0, new_pos.1);
        self.robot_position = new_pos;

    }

    ///draws the map and the robot (texture pointer)
    pub fn draw(&mut self, ctx : &mut Context, map_pos : (f32, f32), scale : f32){
        let mut y_pixel = map_pos.1;
        let mut x_pixel = map_pos.0;
        for (xrobot, row) in self.discovered_map.iter().enumerate() {
            for (yrobot, opt_tile) in row.iter().enumerate() {
                if opt_tile.is_some() {
                    let (tiletype_texture, content_texture ) = opt_tile.clone().unwrap();
                    tiletype_texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(x_pixel, TOP_OFFSET + y_pixel))
                            .scale(Vec2::new(scale, scale)),
                    );
                    content_texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(x_pixel, TOP_OFFSET + y_pixel))
                            .scale(Vec2::new(scale, scale)),
                    );
                }
                if yrobot == self.robot_position.1 && xrobot == self.robot_position.0 {
                    self.robot_texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(x_pixel, TOP_OFFSET + y_pixel))
                            .scale(Vec2::new(scale, scale)),
                    );
                }
                y_pixel += PIXEL * scale;
            }
            y_pixel = map_pos.1;
            x_pixel += PIXEL * scale;
        }
        /*match self.weather {
            WeatherType::Sunny => { graphics::clear(ctx, Color::rgba(0.8, 0.8, 0.1, 0.8)); }
            WeatherType::Rainy => { graphics::clear(ctx, Color::rgba(0.2, 0.2, 0.5, 0.2)); }
            WeatherType::Foggy => { graphics::clear(ctx, Color::rgba(0.7, 0.7, 0.7, 0.2)); }
            WeatherType::TropicalMonsoon => { graphics::clear(ctx, Color::rgba(0.8, 0.2, 0.4, 0.2)); }
            WeatherType::TrentinoSnow => { graphics::clear(ctx, Color::rgba(1.0, 1.0, 1.0, 0.2)); }
        }*/
    }
    pub(crate) fn get_size(&self) -> usize{
        self.world_size
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