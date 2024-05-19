use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use robotics_lib::world::environmental_conditions::WeatherType;

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
mod visweather;

pub(crate)  const PIXEL: f32 = 64.0;
pub(crate) const SCALE: f32 = 0.5;
pub(crate) const BP_SCALE: f32 = 0.5;
pub(crate) const TOP_OFFSET: f32 = 32.0;

///struct containing data to be transmitted between threads
pub struct VisData {
    recv_energy: usize,
    recv_coordinates: (usize, usize),
    recv_discovered_tiles: Option<Vec<Vec<Option<Tile>>>>,
    recv_backpack: Option<HashMap<Content, usize>>,
    recv_weather: Option<WeatherType>,
}

impl VisData {
    /// - energy : {    }
    /// - coordinates : {   }
    /// - discovered_tiles : None
    /// - backpack : None
    /// - weather : None
    pub fn new_energy(energy: usize, coordinates: (usize, usize)) -> Self {
        Self {
            recv_energy: energy,
            recv_coordinates: coordinates,
            recv_discovered_tiles: None,
            recv_backpack: None,
            recv_weather: None,
        }
    }
    /// - energy : {    }
    /// - coordinates : {   }
    /// - discovered_tiles : {   }
    /// - backpack : None
    /// - weather : None
    pub fn new_discover(energy: usize, coordinates: (usize, usize), discovered_tiles: Vec<Vec<Option<Tile>>>) -> Self {
        Self {
            recv_energy: energy,
            recv_coordinates: coordinates,
            recv_discovered_tiles: Some(discovered_tiles),
            recv_backpack: None,
            recv_weather: None,
        }
    }
    /// - energy : {    }
    /// - coordinates : {   }
    /// - discovered_tiles : None
    /// - backpack : {   }
    /// - weather : None
    pub fn new_backpack(energy: usize, coordinates: (usize, usize), backpack: HashMap<Content, usize>) -> Self {
        Self {
            recv_energy: energy,
            recv_coordinates: coordinates,
            recv_discovered_tiles: None,
            recv_backpack: Some(backpack),
            recv_weather: None,
        }
    }
    pub fn new_weather(energy: usize, coordinates: (usize, usize), weather_type: WeatherType) -> Self {
        Self{
            recv_energy: energy,
            recv_coordinates: coordinates,
            recv_discovered_tiles: None,
            recv_backpack: None,
            recv_weather: Some(weather_type),
        }
    }

}

pub struct Visualizer {
    map: VisMap,
    energy: VisEnergy,
    backpack: VisBackPack,

    receiver: Receiver<VisData>,
    show_backpack: bool,
    map_pos: (f32, f32),
    scale: f32,

    //to be removed
    pub(crate) test: i32,
    test_receiver: Receiver<i32>,
}

impl Visualizer {
    pub fn new(ctx: &mut Context, test: i32, size: usize, receiver: Receiver<VisData>,
               //to be removed
               test_receiver: Receiver<i32>
    ) -> tetra::Result<Visualizer> {
        Ok(
            Self {
                //map: VisMap::new(ctx, size),
                map : VisMap::new(ctx, 150),
                energy: VisEnergy::new(ctx),
                backpack: VisBackPack::new(ctx, 10),
                receiver,
                show_backpack: false,
                map_pos: (0.0, 0.0),
                scale: SCALE,
                //to be removed
                test_receiver,
                test

            }
        )
    }
    pub(crate) fn update_map(&mut self, new_discovered: Vec<Vec<Option<Tile>>>, ctx: &mut Context) {
        //self.map.update_map(new_discovered, ctx)
        self.map.update_map(new_discovered, ctx)
    }
    pub(crate) fn update_robot_pos(&mut self, new_pos: (usize, usize)) {
        self.map.update_robot_pos(new_pos)
    }
    pub(crate) fn update_energy(&mut self, energy: usize) {
        self.energy.update(energy)
    }
    pub(crate) fn update_backpack(&mut self, backpack: HashMap<Content, usize>) {
        self.backpack.update(backpack);
    }
    pub(crate) fn update_weather(&mut self, weather_type: WeatherType){
        self.map.update_weather(weather_type);
    }
}

impl State for Visualizer {
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        //received data handling
        let data_res = self.receiver.try_recv();
        match data_res {
            Ok(data) => {
                self.update_energy(data.recv_energy);
                println!("received {}", data.recv_energy);
                self.update_robot_pos(data.recv_coordinates);
                if let Some(view) = data.recv_discovered_tiles {
                    self.update_map(view, ctx)
                }
                if let Some(backp) = data.recv_backpack {
                    self.update_backpack(backp)
                }
                if let Some(w) = data.recv_weather {
                    self.update_weather(w)
                }
            }
            Err(e) => {
                println!("{e}")
            }
        }

        //key input handling
        let scale = self.scale;
        if input::get_keys_pressed(ctx).next().is_some() {
            match input::get_keys_pressed(ctx).next().unwrap() {
                //map movement
                Key::A | Key::Left => { self.map_pos.0 -= PIXEL * scale; }
                Key::D | Key::Right => { self.map_pos.0 += PIXEL * scale; }
                Key::W | Key::Up => { self.map_pos.1 -= PIXEL * scale; }
                Key::S | Key::Down => { self.map_pos.1 += PIXEL * scale; }
                //backpack showing
                Key::X | Key::Space => { self.show_backpack = !self.show_backpack; }
                //zoom
                Key::I => { self.scale += 0.1; self.map_pos.1 += PIXEL * scale; self.map_pos.0 += PIXEL * scale;}
                Key::O => { self.scale -= 0.1 }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        //self.map.draw(ctx, self.map_pos, self.scale);
        self.map.new_draw(ctx, self.map_pos, self.scale);
        self.energy.draw(ctx);
        if self.show_backpack {
            self.backpack.draw(ctx, (0.0, 0.0))
        }
        Ok(())
    }
}
