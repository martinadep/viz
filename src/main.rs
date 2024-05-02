/*// Cargo.toml
//
// [dependencies]
// tetra = "0.7"

use std::sync::mpsc;
use std::thread;
use tetra::graphics::{self, Color};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

struct GameState {
    message_receiver: mpsc::Receiver<String>,
    messages: Vec<String>,

    instruction_sender : mpsc::Sender<i32>,
    value : i32,
}

impl GameState {
    fn new(message_receiver: mpsc::Receiver<String>, instruction_sender : mpsc::Sender<i32>) -> Self {
        GameState {
            message_receiver,
            messages: Vec::new(),
            instruction_sender,
            value : 0,
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Process incoming messages
        while let Ok(message) = self.message_receiver.try_recv() {
            self.messages.push(message);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::WHITE);

        if let Some(k) = input::get_keys_pressed(ctx).next(){
            match k {
                Key::A => {
                    self.instruction_sender.send(self.value + 1).expect_err("aa");
                    println!("tasto premuto");
                }
                _ => {}
            }

        }
        /*for (index, message) in self.messages.iter().enumerate() {
            println!("Message {}: {}", index + 1, message);
        }*/

        Ok(())
    }
}

fn main() -> tetra::Result {
    // Create a channel for sending messages
    let (message_sender, message_receiver) = mpsc::channel();
    let (instruction_sender, instruction_receiver) = mpsc::channel();
    // Spawn a thread for sending messages
    thread::spawn(move || {
        // Simulate sending messages
        for i in 1..=5 {
            thread::sleep(std::time::Duration::from_secs(1));
            message_sender.send(format!("Hello from thread! Message {}", i)).unwrap();
        }
        let value = instruction_receiver.try_recv().expect("TODO: panic message");
        println!("{value}")
    });

    ContextBuilder::new("Message Passing in Tetra", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(|ctx| Ok(GameState::new(message_receiver, instruction_sender)))
}
*/

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use rand::Rng;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::{Generator, World};
use tetra::{Context, ContextBuilder};
use tyrannousarust_rex_world_generator::WorldGenerator;
use crate::start_fn::start;
use crate::visualizer::{VisData, Visualizer};

mod visualizer;
mod start_fn;

pub const WINDOW_WIDTH : i32 = 600;
pub const WINDOW_HEIGHT : i32 = 600;

pub const WORLD_DEF_SIZE : usize = 10;

fn main() {
    let (vis_sender, vis_receiver) = mpsc::channel();
    let (sender_test, receiver_test) = mpsc::channel();

    thread::spawn(move ||{
        loop {
            vis_sender.send(VisData::new_energy(rand::thread_rng().gen_range(0..100), (1, 4)));
            sender_test.send(rand::thread_rng().gen_range(0..100));
        }
    });
    start(receiver_test, vis_receiver);
}
