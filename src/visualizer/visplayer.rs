use tetra::Context;
use tetra::graphics::{DrawParams, Texture};
use tetra::math::Vec2;
use crate::visualizer::{PIXEL, SCALE, TOP_OFFSET};

pub struct VisPlayer{
    texture : Texture,
    position : (usize, usize)
}
impl VisPlayer{
    pub fn new(ctx : &mut Context) -> Self{
        Self{
            texture : Texture::new(ctx, "./utils/robot.png").expect("failed to upload robot image"),
            position : (10, 10)
        }
    }
    pub fn draw (&self, ctx : &mut Context){
        let (x, y) = (self.position.0 as f32, self.position.1 as f32);
        self.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(x * PIXEL * SCALE, TOP_OFFSET + y * PIXEL * SCALE))
                .scale(Vec2::new(SCALE, SCALE)),
        );
    }
}