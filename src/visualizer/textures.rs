use robotics_lib::world::environmental_conditions::WeatherType;
use tetra::{Context};
use tetra::graphics::{DrawParams, Rectangle, Texture};
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::tile::TileType::*;


pub fn upload_tileset(ctx : &mut Context) -> Texture{
    //Texture::new(ctx,"./resources/tileset.png").expect("failed to upload texture")
    Texture::new(ctx,"./resources/ts_minecraft.png").expect("failed to upload texture")
}
pub fn upload_contentset(ctx : &mut Context) -> Texture{
    Texture::new(ctx,"./resources/contentset.png").expect("failed to upload texture")
}

fn draw_rect(texture: Texture, ctx : &mut Context, region : Rectangle, draw_params: DrawParams){
    texture.draw_region(ctx, region, draw_params)
}
pub trait Drawable {
    fn draw(&self,texture : Texture, ctx : &mut Context, draw_params: DrawParams);
}

impl Drawable for TileType {
    fn draw(&self, texture : Texture, ctx: &mut Context, draw_params: DrawParams) {
        let r = match self {
            DeepWater => {Rectangle::new(0.0,0.0,64.0,64.0)}
            ShallowWater => {Rectangle::new(64.0,0.0,64.0,64.0)}
            Sand => {Rectangle::new(128.0,0.0,64.0,64.0)}
            Grass => {Rectangle::new(192.0,0.0,64.0,64.0)}
            Street => {Rectangle::new(0.0,64.0,64.0,64.0)}
            Hill => {Rectangle::new(64.0,64.0,64.0,64.0)}
            Mountain => {Rectangle::new(128.0,64.0,64.0,64.0)}
            Snow => {Rectangle::new(192.0,64.0,64.0,64.0)}
            Lava => {Rectangle::new(0.0,128.0,64.0,64.0)}
            Teleport(_) => {Rectangle::new(64.0,128.0,64.0,64.0)}
            Wall => {Rectangle::new(128.0,128.0,64.0,64.0)}
        };
        draw_rect(texture, ctx, r, draw_params)
    }
}

impl Drawable for Content {
    fn draw(&self, texture: Texture, ctx: &mut Context, draw_params: DrawParams) {
        let r = match self {
            Content::Rock(_) => {Rectangle::new(0.0,0.0,64.0,64.0)}
            Content::Tree(_) => {Rectangle::new(64.0,0.0,64.0,64.0)}
            Content::Garbage(_) => {Rectangle::new(128.0,0.0,64.0,64.0)}
            Content::Fire => {Rectangle::new(192.0,0.0,64.0,64.0)}
            Content::Coin(_) => {Rectangle::new(0.0,64.0,64.0,64.0)}
            Content::Bin(_) => {Rectangle::new(64.0,64.0,64.0,64.0)}
            Content::Crate(_) => {Rectangle::new(128.0,64.0,64.0,64.0)}
            Content::Bank(_) => {Rectangle::new(192.0,64.0,64.0,64.0)}
            Content::Market(_) => {Rectangle::new(0.0,128.0,64.0,64.0)}
            Content::Fish(_) => {Rectangle::new(64.0,128.0,64.0,64.0)}
            Content::Building => {Rectangle::new(128.0,128.0,64.0,64.0)}
            Content::Bush(_) => {Rectangle::new(192.0,128.0,64.0,64.0)}
            Content::Scarecrow => {Rectangle::new(0.0,192.0,64.0,64.0)}
            _ => {Rectangle::new(192.0,192.0,64.0,64.0)}
        };
        draw_rect(texture, ctx, r, draw_params)
    }
}

//-------------
pub(crate) trait Texturizable {
    ///matches an object with its texture
    fn get_texture(&self, ctx: &mut Context) -> Texture;
}
impl Texturizable for TileType{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        let tmp = match self {
            DeepWater => {Texture::new(ctx, "./utils/tiletype/DeepWater.png")}
            ShallowWater => {Texture::new(ctx, "./utils/tiletype/ShallowWater.png")}
            Sand => {Texture::new(ctx, "./utils/tiletype/Sand.png")}
            Grass => {Texture::new(ctx, "./utils/tiletype/Grass.png")}
            Street => {Texture::new(ctx, "./utils/tiletype/Street.png")}
            Hill => {Texture::new(ctx, "./utils/tiletype/Hill.png")}
            Mountain => {Texture::new(ctx, "./utils/tiletype/Mountain.png")}
            Snow => {Texture::new(ctx, "./utils/tiletype/Snow.png")}
            Lava => {Texture::new(ctx, "./utils/tiletype/Lava.png")}
            Teleport(_) => {Texture::new(ctx, "./utils/tiletype/Teleport.png")}
            Wall => {Texture::new(ctx, "./utils/tiletype/Wall.png")}
        };
        tmp.expect("failed to upload tile type texture")
    }
}

impl Texturizable for Content{
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        let tmp = match self {
            Content::Rock(_) => {Texture::new(ctx, "./utils/content/Rock.png")}
            Content::Tree(_) => {Texture::new(ctx, "./utils/content/Tree.png")}
            Content::Garbage(_) => {Texture::new(ctx, "./utils/content/Garbage.png")}
            Content::Fire => {Texture::new(ctx, "./utils/content/Fire.png")}
            Content::Coin(_) => {Texture::new(ctx, "./utils/content/Coin.png")}
            Content::Bin(_) => {Texture::new(ctx, "./utils/content/Bin.png")}
            Content::Crate(_) => {Texture::new(ctx, "./utils/content/Crate.png")}
            Content::Bank(_) => {Texture::new(ctx, "./utils/content/Bank.png")}
            Content::Market(_) => {Texture::new(ctx, "./utils/content/Market.png")}
            Content::Fish(_) => {Texture::new(ctx, "./utils/content/Fish.png")}
            Content::Building => {Texture::new(ctx, "./utils/content/Building.png")}
            Content::Bush(_) => {Texture::new(ctx, "./utils/content/Bush.png")}
            Content::Scarecrow => {Texture::new(ctx, "./utils/content/Scarecrow.png")}
            _ => {Texture::new(ctx, "./utils/content/None.png")}
        };
        tmp.expect("failed to upload tile type texture")
    }
}

impl Texturizable for WeatherType {
    fn get_texture(&self, ctx: &mut Context) -> Texture {
        let tmp = match self {
            WeatherType::Sunny => {Texture::new(ctx, "./utils/sun.png")}
            WeatherType::Foggy => {Texture::new(ctx, "./utils/fog.png")}
            WeatherType::TrentinoSnow => {Texture::new(ctx, "./utils/snow.png")}
            _ => {Texture::new(ctx, "./utils/drop.png")}
        };
        tmp.expect("failed to upload weather texture")
    }
}

