#[cfg(feature = "macroquad-backend")]
use crate::macroquad::ui::{root_ui, widgets, Ui};

use crate::math::{Size, Vec2};
use crate::prelude::{draw_texture, viewport};
use crate::render::DrawTextureParams;
use crate::texture::get_texture;
use std::ops::Deref;

use crate::texture::Texture2D;

pub struct Background {
    textures: Vec<Texture2D>,
    size: Size<f32>,
    position: Vec2,
}

impl Background {
    pub fn new(size: Size<f32>, position: Vec2, textures: &[Texture2D]) -> Self {
        Background {
            textures: textures.to_vec(),
            size,
            position,
        }
    }

    #[cfg(feature = "macroquad-backend")]
    pub fn ui(&self, ui: &mut Ui) {
        for texture in &self.textures {
            widgets::Texture::new(texture.deref().into())
                .size(self.size.width, self.size.height)
                .position(self.position)
                .ui(ui);
        }
    }

    pub fn draw(&self) {
        for texture in &self.textures {
            draw_texture(
                self.position.x,
                self.position.y,
                *texture,
                DrawTextureParams {
                    dest_size: Some(self.size),
                    ..Default::default()
                },
            )
        }
    }
}

pub fn draw_main_menu_background() {
    let backgrounds = [
        get_texture("background_01"),
        get_texture("background_02"),
        get_texture("background_03"),
        get_texture("background_04"),
    ];

    let size = backgrounds[0].size();

    let height = viewport().height as f32;
    let width = (height / size.height) * size.width;

    let bg = Background::new(
        Size::new(width, height),
        Vec2::ZERO,
        &[
            backgrounds[3],
            backgrounds[2],
            backgrounds[1],
            backgrounds[0],
        ],
    );

    #[cfg(feature = "macroquad-backend")]
    bg.ui(&mut *root_ui());

    #[cfg(not(feature = "macroquad-backend"))]
    bg.draw();
}
