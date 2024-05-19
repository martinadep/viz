use std::fmt::{Debug, Formatter};
use std::path::Display;
use robotics_lib::world::environmental_conditions::WeatherType;
use tetra::{Context, State};
use rand::rngs::ThreadRng;
use rand::{self, Rng, thread_rng};
use robotics_lib::world::environmental_conditions::WeatherType::*;
use tetra::graphics::{DrawParams};
use tetra::math::Vec2;
use crate::start_fn::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::visualizer::textures::Texturizable;

// NOTE: Using a high number here yields worse performance than adding more bunnies over
// time - I think this is due to all of the RNG being run on the same tick...
const INITIAL_DROPS: usize = 100;
const GRAVITY: f32 = 2.5;

pub struct Drop {
    drop_type : WeatherType,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Drop {
    pub(crate) fn new(rng: &mut ThreadRng, drop_type: WeatherType) -> Drop {
        let velocity;
        let mut position = Vec2::new(rng.gen::<f32>() * WINDOW_WIDTH as f32, rng.gen::<f32>() * WINDOW_WIDTH as f32);
        match drop_type {
            Rainy => {
                velocity = Vec2::new(6.4, 8.7);
            }
            TrentinoSnow => {
                velocity = Vec2::new(4.3, 6.2);
            }
            TropicalMonsoon => {
                velocity = Vec2::new(8.4, 10.7);
            }
            Sunny => {
                position = Vec2::new(WINDOW_WIDTH as f32 - 60.0, 20.0);
                velocity = Vec2::new(0.0, 0.0);
            }
            _ => {
                velocity = Vec2::new(0.4, 0.0);
            }
        }

        Drop {
            drop_type,
            position : position,
            velocity: velocity,
        }
    }

}

pub struct VisWeather {
    pub droptype : WeatherType,
    rng : ThreadRng,
    drops: Vec<Drop>,
}

impl Debug for VisWeather {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.droptype)
    }
}

impl VisWeather {
    pub(crate) fn new(droptype: WeatherType) -> VisWeather {
        let mut rng = thread_rng();

        let mut drop_numbers = INITIAL_DROPS;
        match droptype {
            TropicalMonsoon => {
                drop_numbers = INITIAL_DROPS * 2;
            }
            Sunny => {
                drop_numbers = 1;
            }
            Foggy => {
                drop_numbers = 5;
            }
            _ => {}
        }

        let mut drops = Vec::with_capacity(drop_numbers);
        for _ in 0..drop_numbers {
            drops.push(Drop::new(&mut rng, droptype));
        }

        VisWeather {
            droptype,
            rng,
            drops,
        }
    }
}

impl State for VisWeather {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        for drop in &mut self.drops {
            drop.position += drop.velocity;

            if drop.position.x > WINDOW_WIDTH as f32 {
                drop.position.x = 0.0;
            }
            else if drop.position.x < 0.0 {
                drop.position.x = WINDOW_WIDTH as f32;
            }
            if drop.position.y > WINDOW_HEIGHT as f32 {
                drop.position.y = 0.0;
            }
            else if drop.position.y < 0.0 {
                drop.position.y = WINDOW_HEIGHT as f32;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let texture = self.droptype.get_texture(ctx);

        for d in &self.drops {
            match self.droptype {
                Sunny => {
                    texture
                        .draw(ctx, DrawParams::new()
                            .position(d.position)
                            .scale(Vec2::new(0.5, 0.5)));
                }
                Foggy => {
                    texture
                        .draw(ctx, DrawParams::new()
                            .position(d.position)
                            .scale(Vec2::new(1.5, 1.5)));
                }
                _ => {
                    texture
                        .draw(ctx, DrawParams::new()
                            .position(d.position)
                            .scale(Vec2::new(0.1, 0.1)));
                }
            }
        }

        Ok(())
    }
}
