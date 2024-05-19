use std::collections::HashMap;
use std::ops::{Add, Sub};
use robotics_lib::event::events::Event::RemovedFromBackpack;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::utils::LibError::NoContent;
use robotics_lib::world::tile::Content;
use tetra::Context;
use tetra::graphics::{DrawParams, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use crate::visualizer::{PIXEL, BP_SCALE, SCALE};
use crate::visualizer::textures::Texturizable;

const OFFSET : f32 = 0.5;

///visualizable backpack structure
pub struct VisBackPack{
    square: Texture,
    size : usize,
    contents : HashMap<Content, usize>,

    text : Text,
    scale : f32,
}
impl VisBackPack {
    pub fn new(ctx : &mut Context, size : usize) -> Self{
        Self{
            square: Texture::new(ctx, "./utils/backp.png").expect("failed to upload inventory's square image"),
            size,
            contents : HashMap::new(),
            scale : BP_SCALE,
            text : Text::new(format!(""),
                             Font::vector(ctx, "./utils/fonts/roboto.ttf", 17.0)
                                 .expect("failed to upload font")),
        }
    }
    pub fn draw (&mut self, ctx : &mut Context, pos : (f32, f32)){
        let mut x = pos.0;

        let diff = 0.12;
        let content_scale = self.scale - diff;
        let to_center_pos = PIXEL * self.scale - PIXEL * (SCALE - diff / 2.0);

        for (cont, quantity) in &self.contents {
            self.text.set_content(format!("{}", quantity));
            self.square.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(x + pos.0, pos.1))
                    .scale(Vec2::new(self.scale, self.scale)),
            );
            cont.get_texture(ctx).draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(x + to_center_pos + pos.0, pos.1 + to_center_pos))
                    .scale(Vec2::new(content_scale, content_scale)),
            );
            self.text.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(x + pos.0, pos.1))
                    .scale(Vec2::new(self.scale * 2.0, self.scale * 2.0)),
            );

            x += PIXEL * self.scale;
        }

        let voids = self.size - self.contents.len();
        for _ in 0..voids {
            self.square.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(x + pos.0, pos.1))
                    .scale(Vec2::new(self.scale, self.scale)),
            );
            x += PIXEL * self.scale;
        }
    }
    pub fn update(&mut self, new_backpack : HashMap<Content, usize>){
        self.contents = new_backpack;
    }
}