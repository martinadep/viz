mod textures;
mod visplayer;
mod vismap;
mod visenergy;
mod visbackpack;

use std::sync::mpsc::Receiver;
use robotics_lib::energy::Energy;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::world_generator::{Generator};
use tetra::{Context, graphics, input, State, TetraError};
use tetra::graphics::{Color};
use tetra::input::Key;
use crate::visualizer::textures::Texturizable;
use crate::visualizer::visbackpack::VisBackPack;
use crate::visualizer::visenergy::VisEnergy;
use crate::visualizer::vismap::{VisMap};
use crate::visualizer::visplayer::VisPlayer;

pub const PIXEL : f32 = 64.0;
pub const SCALE : f32 = 0.5;
pub const TOP_OFFSET : f32 = 32.0;

type VisData = (Energy, Coordinate, Vec<Vec<Option<Tile>>>);

pub struct Visualizer{
    //receiver : Receiver<VisData>,

    map : VisMap,
    player : VisPlayer,
    energy : VisEnergy,

    show_backpack : bool,
    backpack : VisBackPack,

}
impl Visualizer{
    pub fn new (ctx : &mut Context) -> tetra::Result<Visualizer>{
        Ok(
            Self{
                map: VisMap::new(ctx),
                player : VisPlayer::new(ctx),
                energy : VisEnergy::new(ctx),
                backpack : VisBackPack::new(ctx),
                show_backpack : false
            }
        )
    }
    pub fn update_map(&mut self, discovered_map : &Vec<Vec<Option<Tile>>>, ctx : &mut Context){
        self.map.update_map(discovered_map, ctx)
    }
}
impl State for Visualizer{
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        if input::get_keys_pressed(ctx).next().is_some() {
            self.show_backpack = !self.show_backpack;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        graphics::clear(ctx, Color::rgb(0.0,0.0,0.0));
        self.map.draw(ctx);
        self.player.draw(ctx);
        self.energy.draw(ctx);
        if self.show_backpack {
            self.backpack.draw(ctx)
        }
        Ok(())
    }
}