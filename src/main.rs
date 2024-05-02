use std::sync::mpsc;
use std::thread;

use rand::Rng;
use robotics_lib::world::world_generator::Generator;

use crate::start_fn::start;
use crate::visualizer::VisData;

mod visualizer;
mod start_fn;

pub const WINDOW_WIDTH: i32 = 600;
pub const WINDOW_HEIGHT: i32 = 600;

pub const WORLD_DEF_SIZE: usize = 10;

fn main() {
    let (vis_sender, vis_receiver) = mpsc::channel();
    let (sender_test, receiver_test) = mpsc::channel();

    thread::spawn(move || {
        loop {
            vis_sender.send(VisData::new_energy(rand::thread_rng().gen_range(0..100), (1, 4)));
            sender_test.send(rand::thread_rng().gen_range(0..100));
        }
    });
    start(receiver_test, vis_receiver);
}
