use std::sync::mpsc;
use std::thread;

use rand::Rng;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::world_generator::Generator;
use tetra::input::KeyLabel::V;

use crate::start_fn::start;
use crate::visualizer::VisData;

mod visualizer;
mod start_fn;

pub const WORLD_DEF_SIZE: usize = 10;

fn main() {
    let (vis_sender, vis_receiver) = mpsc::channel();
    let (sender_test, receiver_test) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let x = rand::thread_rng().gen_range(0..5000000);
            if x == 14 {
                vis_sender.send(VisData::new_energy(14, (1,4)));
            }
            if x == 38 {
                vis_sender.send(VisData::new_energy(38, (3,8)));
            }
            if x == 25 {
                vis_sender.send(VisData::new_weather(25, (1,4), WeatherType::Foggy));
            }
            if x == 45 {
                vis_sender.send(VisData::new_weather(25, (1,4), WeatherType::TrentinoSnow));
            }
            if x == 55 {
                vis_sender.send(VisData::new_weather(25, (1,4), WeatherType::TropicalMonsoon));
            }
            sender_test.send(rand::thread_rng().gen_range(0..100));
        }
    });
    start(receiver_test, vis_receiver);
}
