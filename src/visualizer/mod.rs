use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::Generator;
use tetra::{Context, graphics, input, State, TetraError};
use tetra::graphics::Color;
use tetra::input::Key;

use crate::visualizer::textures::Texturizable;
use crate::visualizer::visbackpack::VisBackPack;
use crate::visualizer::visenergy::VisEnergy;
use crate::visualizer::vismap::VisMap;

mod textures;
mod vismap;
mod visenergy;
mod visbackpack;

pub const PIXEL: f32 = 64.0;
pub const SCALE: f32 = 0.5;
pub const BP_SCALE: f32 = 0.5;
pub const TOP_OFFSET: f32 = 32.0;

pub struct VisData {
    recv_energy: usize,
    recv_coordinates: (usize, usize),
    recv_discovered_tiles: Option<Vec<Vec<Option<Tile>>>>,
    recv_backpack: Option<HashMap<Content, usize>>,
}

impl VisData {
    pub fn new_energy(energy: usize, coordinates: (usize, usize)) -> Self {
        Self {
            recv_energy: energy,
            recv_coordinates: coordinates,
            recv_discovered_tiles: None,
            recv_backpack: None,
        }
    }
    pub fn new_discover(energy: usize, coordinates: (usize, usize), discovered_tiles: Option<Vec<Vec<Option<Tile>>>>) -> Self {
        Self {
            recv_energy: energy,
            recv_coordinates: coordinates,
            recv_discovered_tiles: discovered_tiles,
            recv_backpack: None,
        }
    }
    pub fn new_backpack(energy: usize, coordinates: (usize, usize), backpack: Option<HashMap<Content, usize>>) -> Self {
        Self {
            recv_energy: energy,
            recv_coordinates: coordinates,
            recv_discovered_tiles: None,
            recv_backpack: backpack,
        }
    }
}

pub struct Visualizer {
    test_receiver: Receiver<i32>,
    receiver: Receiver<VisData>,

    pub(crate) test: i32,

    map: VisMap,
    energy: VisEnergy,
    backpack: VisBackPack,

    show_backpack: bool,
    map_pos: (f32, f32),
    scale: f32,
}

impl Visualizer {
    pub fn new(ctx: &mut Context, test: i32, size: usize,
               test_receiver: Receiver<i32>, receiver: Receiver<VisData>,
    ) -> tetra::Result<Visualizer> {
        Ok(
            Self {
                test,
                map: VisMap::new(ctx, size),
                energy: VisEnergy::new(ctx),
                backpack: VisBackPack::new(ctx, 10),
                show_backpack: false,
                map_pos: (0.0, 0.0),
                scale: SCALE,
                test_receiver,
                receiver,
            }
        )
    }
    pub fn update_map(&mut self, new_discovered: Vec<Vec<Option<Tile>>>, ctx: &mut Context) {
        self.map.update_map(new_discovered, ctx)
    }
    pub fn update_robot_pos(&mut self, new_pos: (usize, usize)) {
        self.map.update_robot_pos(new_pos)
    }
    pub fn update_energy(&mut self, energy: usize) {
        self.energy.update(energy)
    }
    pub fn update_backpack(&mut self, backpack: HashMap<Content, usize>) {
        self.backpack.update(backpack);
    }
}

impl State for Visualizer {
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        let test_received_integer = self.test_receiver.try_recv();
        match test_received_integer {
            Ok(n) => {
                println!("received {n}");
            }
            Err(_) => {}
        }

        let data_res = self.receiver.try_recv();
        match data_res {
            Ok(data) => {
                self.energy.update(data.recv_energy);
                self.map.update_robot_pos(data.recv_coordinates);
                if let Some(view) = data.recv_discovered_tiles {
                    self.map.update_map(view, ctx);
                }
                if let Some(backp) = data.recv_backpack {
                    self.backpack.update(backp);
                }
            }
            Err(e) => {
                println!("{e}")
            }
        }

        let scale = self.scale;
        if input::get_keys_pressed(ctx).next().is_some() {
            match input::get_keys_pressed(ctx).next().unwrap() {
                //map movement
                Key::A | Key::Left => { self.map_pos.0 -= PIXEL * scale; }
                Key::D | Key::Right => { self.map_pos.0 += PIXEL * scale; }
                Key::W | Key::Up => { self.map_pos.1 -= PIXEL * scale; }
                Key::S | Key::Down => { self.map_pos.1 += PIXEL * scale; }
                //backpack
                Key::X | Key::Space => { self.show_backpack = !self.show_backpack; }
                //zoom
                Key::I => {
                    self.scale += 0.1;
                    //self.sender.send(VisData::new_map_scale(self.scale + 0.1));
                }
                Key::O => { self.scale -= 0.1 }
                _ => {}
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        self.map.draw(ctx, self.map_pos, self.scale);
        self.energy.draw(ctx);
        if self.show_backpack {
            self.backpack.draw(ctx, (0.0, 0.0))
        }
        Ok(())
    }
}
