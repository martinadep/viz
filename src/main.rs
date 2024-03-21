use tetra::ContextBuilder;
use crate::visualizer::Visualizer;

mod visualizer;

pub const WINDOW_WIDTH : i32 = 600;
pub const WINDOW_HEIGHT : i32 = 600;

fn main() {
    ContextBuilder::new("tyrex", WINDOW_WIDTH,WINDOW_HEIGHT)
        .show_mouse(true)
        .quit_on_escape(true)
        .resizable(true)
        .build().expect("failed to build context")
        .run(Visualizer::new).expect("failed to run context");
    println!("TEST");
}
