use ff_core::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct EditorInput {
    pub action: bool,
    pub back: bool,
    pub context_menu: bool,
    pub camera_move_direction: Vec2,
    pub camera_mouse_move: bool,
    pub camera_zoom: f32,
    pub cursor_move_direction: Vec2,
    pub undo: bool,
    pub redo: bool,
    pub toggle_menu: bool,
    pub toggle_draw_grid: bool,
    pub toggle_snap_to_grid: bool,
    pub toggle_disable_parallax: bool,
    pub save: bool,
    pub save_as: bool,
    pub load: bool,
    pub delete: bool,
}

pub fn collect_editor_input() -> EditorInput {
    let mut input = EditorInput {
        action: is_mouse_button_down(MouseButton::Left),
        camera_mouse_move: is_mouse_button_down(MouseButton::Middle),
        context_menu: is_mouse_button_pressed(MouseButton::Right),
        ..Default::default()
    };

    let mouse_wheel = mouse_wheel();
    if mouse_wheel.y < 0.0 {
        input.camera_zoom = -1.0;
    } else if mouse_wheel.y > 0.0 {
        input.camera_zoom = 1.0;
    }

    if is_key_down(KeyCode::LeftControl) {
        if is_key_pressed(KeyCode::Z) {
            if is_key_down(KeyCode::LeftShift) {
                input.redo = true;
            } else {
                input.undo = true;
            }
        }

        input.toggle_snap_to_grid = is_key_pressed(KeyCode::G);

        if is_key_pressed(KeyCode::S) {
            if is_key_down(KeyCode::LeftShift) {
                input.save_as = true;
            } else {
                input.save = true;
            }
        }

        if is_key_pressed(KeyCode::L) {
            input.load = true;
        }
    } else {
        if is_key_pressed(KeyCode::Escape) {
            input.toggle_menu = true;
            input.back = true;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            input.camera_move_direction.x = -1.0;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            input.camera_move_direction.x = 1.0;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            input.camera_move_direction.y = -1.0;
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            input.camera_move_direction.y = 1.0;
        }

        input.toggle_draw_grid = is_key_pressed(KeyCode::G);

        input.toggle_disable_parallax = is_key_pressed(KeyCode::P);

        input.delete = is_key_pressed(KeyCode::Delete);
    }

    /*
    let gamepad_system = storage::get_mut::<fishsticks::GamepadContext>();

    let ix =

    if let Some(ix) = ix {
        let gamepad = gamepad_system.gamepad(ix);

        if let Some(gamepad) = gamepad {
            input.action = gamepad.digital_inputs.activated(Button::East);
            input.back = gamepad.digital_inputs.activated(Button::South);
            input.context_menu = gamepad.digital_inputs.activated(Button::West);

            input.camera_move_direction = {
                let direction_x = gamepad.analog_inputs.value(Axis::LeftStickX);
                let direction_y = gamepad.analog_inputs.value(Axis::LeftStickY);

                let direction = vec2(direction_x, direction_y);

                direction.normalize_or_zero()
            };

            input.cursor_move_direction = {
                let direction_x = gamepad.analog_inputs.value(Axis::RightStickX);
                let direction_y = gamepad.analog_inputs.value(Axis::RightStickY);

                let direction = vec2(direction_x, direction_y);

                direction.normalize_or_zero()
            };
        }
    }
     */

    input
}
