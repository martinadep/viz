use tetra::Context;
use tetra::graphics::{DrawParams, Texture};
use tetra::math::Vec2;
use crate::visualizer::{PIXEL, SCALE, TOP_OFFSET};

pub struct VisBackPack{
    texture : Texture,
}
impl VisBackPack {
    pub fn new(ctx : &mut Context) -> Self{
        Self{
            texture : Texture::new(ctx, "./utils/robot.png").expect("failed to upload robot image"),
        }
    }
    pub fn draw (&self, ctx : &mut Context){
        self.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(0.0, 0.0))
                .scale(Vec2::new(SCALE, SCALE)),
        );
    }
}