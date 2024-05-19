use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use rand::Rng;

use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::{Generator, World};
use tetra::{Context, ContextBuilder};
use tyrannousarust_rex_world_generator::WorldGenerator;

use crate::visualizer::{VisData, Visualizer};

pub const WINDOW_WIDTH: i32 = 600;
pub const WINDOW_HEIGHT: i32 = 600;
pub const WORLD_DEF_SIZE: usize = 10;

///Builds and runs the window
pub fn start(receiver_test: Receiver<i32>, vis_receiver: Receiver<VisData>) {
        let mut c = ContextBuilder::new("tyrex", WINDOW_WIDTH, WINDOW_HEIGHT)
            .show_mouse(true)
            .quit_on_escape(true)
            .resizable(true)
            .build().expect("failed to build context");

        let mut vis = Visualizer::new(&mut c, 188, WORLD_DEF_SIZE, vis_receiver,
                                      //to be removed
                                      receiver_test
        ).expect("failed to create visualizer");

        //delete these if initializing by from_world_to_map() method
        let world = WorldGenerator::new().set_size(WORLD_DEF_SIZE).set_seed(265).gen();
        //tests(&mut vis, &mut c, &world);

        c.run(|_ctx| {
            println!("TEST {}", vis.test);
            Ok(vis)
        }).expect("failed to run");
}

/// used for testing, to be removed later
///
/// it adds some tiles to the discovered map, content to backpack and updates energy
fn tests(vis: &mut Visualizer, mut c: &mut Context, world: &World) {
    ///TEST - UPDATE MAP
    //fra (1,1) e (size - 2, size - 2) che non ho il controllo dei bounds nella robot view di test
    let discovered = robot_view_test(&world, (1, 1));
    vis.update_robot_pos((1, 1));
    vis.update_map(discovered, &mut c);
    let discovered = robot_view_test(&world, (3, 1));
    vis.update_robot_pos((3, 1));
    vis.update_map(discovered, &mut c);
    let discovered = robot_view_test(&world, (1, 4));
    vis.update_robot_pos((1, 4));
    vis.update_map(discovered, &mut c);

    ///TEST - UPDATE ENERGY
    vis.update_energy(20);

    ///TEST - UPDATE BACKPACK
    let mut newb = HashMap::new();
    newb.insert(Content::Tree(10), 10);
    newb.insert(Content::Tree(3), 3);
    newb.insert(Content::Rock(20), 20);
    vis.update_backpack(newb);
}
///used for testing, to be removed later
fn robot_view_test(world: &World, coordinates: (usize, usize)) -> Vec<Vec<Option<Tile>>> {
    let mut tmp: [[bool; 3]; 3] = [[false; 3]; 3];
    let mut out: Vec<Vec<Option<Tile>>> = vec![vec![None; 3]; 3]; //Matrix of Option <Tile>
    let (robot_row, robot_col) = (coordinates.0, coordinates.1);

    if robot_row == 0 {
        tmp[0][0] = true;
        tmp[0][1] = true;
        tmp[0][2] = true;
        out[0][0] = None;
        out[0][1] = None;
        out[0][2] = None;
    }
    if robot_col == 0 {
        tmp[0][0] = true;
        tmp[1][0] = true;
        tmp[2][0] = true;
        out[0][0] = None;
        out[1][0] = None;
        out[2][0] = None;
    }
    if robot_row == world.0.len() - 1 {
        tmp[2][0] = true;
        tmp[2][1] = true;
        tmp[2][2] = true;
        out[2][0] = None;
        out[2][1] = None;
        out[2][2] = None;
    }
    if robot_col == world.0.len() - 1 {
        tmp[0][2] = true;
        tmp[1][2] = true;
        tmp[2][2] = true;
        out[0][2] = None;
        out[1][2] = None;
        out[2][2] = None;
    }

    tmp.iter().enumerate().for_each(|(i, vector)| {
        vector.iter().enumerate().for_each(|(j, elem)| {
            if !elem {
                let row = robot_row + i - 1;
                let col = robot_col + j - 1;
                out[i][j] = Some(world.0[row][col].clone());
            }
        })
    });
    out
}
