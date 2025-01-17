#[path = "macroquad/config.rs"]
pub mod config;

#[path = "macroquad/text.rs"]
pub mod text;

#[path = "macroquad/image.rs"]
pub mod image;

#[path = "macroquad/texture.rs"]
pub mod texture;

#[path = "macroquad/input.rs"]
pub mod input;

#[path = "macroquad/file.rs"]
pub mod file;

#[path = "macroquad/error.rs"]
pub mod error;

#[path = "macroquad/color.rs"]
pub mod color;

#[path = "macroquad/render.rs"]
pub mod render;

#[path = "macroquad/window.rs"]
pub mod window;

#[path = "macroquad/math.rs"]
pub mod math;

#[path = "macroquad/game.rs"]
pub mod game;

#[path = "macroquad/event.rs"]
pub mod event;

pub mod video {}

pub mod context {}

pub mod gl {}

pub mod gui {
    pub use macroquad::ui::*;
}

pub use macroquad::ui;

pub use macroquad::experimental::scene;

pub use ff_particles as particles;
