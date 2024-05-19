use robotics_lib::energy::Energy;
use tetra::Context;
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use crate::start_fn::WINDOW_WIDTH;
use crate::visualizer::{PIXEL, SCALE};

///visualizable energy structure
pub struct VisEnergy{
    energy : usize,
    text : Text,
    font : Font,
}
impl VisEnergy{
    pub fn new(ctx : &mut Context) -> Self{
        let f = Font::vector(ctx, "./utils/fonts/roboto.ttf", 17.0)
            .expect("failed to upload font");
        let e = Energy::default();
        Self{
            energy : e.get_energy_level(),
            font : f.clone(),
            text : Text::new(format!("Robot Energy : {}", e.get_energy_level()), f)
        }
    }
    pub fn draw(&mut self, ctx : &mut Context){
        self.text.draw(ctx, Vec2::new(WINDOW_WIDTH as f32 / 1.5 - PIXEL * SCALE, 0.0));
    }
    pub fn update(&mut self, new_energy : usize){
        self.energy = new_energy;
        self.text.set_content(format!("Robot Energy : {}", self.energy));
    }
}
