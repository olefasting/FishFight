use ff_core::macroquad::camera::Camera2D;
use ff_core::macroquad::experimental::scene;
use ff_core::macroquad::experimental::scene::{Node, RefMut};

use ff_core::prelude::*;

pub struct EditorCamera {
    pub position: Vec2,
    pub scale: f32,
}

impl EditorCamera {
    const FRUSTUM_PADDING: f32 = 64.0;
    const DEFAULT_SCALE: f32 = 1.0;

    pub fn new(position: Vec2) -> Self {
        EditorCamera {
            position,
            scale: Self::DEFAULT_SCALE,
        }
    }

    pub fn get_view_rect(&self) -> Rect {
        let window_size = window_size();
        let size = vec2(
            window_size.width as f32 / self.scale,
            window_size.height as f32 / self.scale,
        );

        let position = self.position - size / 2.0;

        Rect::new(position.x, position.y, size.x, size.y)
    }

    // This can be used for culling when drawing the map. Not strictly necessary with the small maps in FF
    pub fn get_padded_frustum(&self) -> Rect {
        let mut res = self.get_view_rect();
        res.move_to(res.point() - vec2(Self::FRUSTUM_PADDING, Self::FRUSTUM_PADDING));
        res.width += Self::FRUSTUM_PADDING * 2.0;
        res.height += Self::FRUSTUM_PADDING * 2.0;
        res
    }

    pub fn to_world_space(&self, position: Vec2) -> Vec2 {
        let rect = self.get_view_rect();
        position / self.scale + rect.point()
    }

    pub fn to_screen_space(&self, position: Vec2) -> Vec2 {
        let rect = self.get_view_rect();
        (position - rect.point()) * self.scale
    }
}

impl Node for EditorCamera {
    fn fixed_update(node: RefMut<Self>) {
        let viewport_size = viewport_size();

        let camera = Some(Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(node.position.x.round(), node.position.y.round()),
            zoom: vec2(
                node.scale / viewport_size.width,
                -node.scale / viewport_size.height,
            ) * 2.0,
            ..Camera2D::default()
        });

        scene::set_camera(0, camera);
    }
}
