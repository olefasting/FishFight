use std::path::Path;

use ff_core::macroquad::hash;
use ff_core::macroquad::ui::{widgets, Ui};
use ff_core::prelude::*;

use super::{ButtonParams, EditorAction, EditorContext, Window, WindowParams};
use ff_core::map::{map_name_to_filename, Map, MAP_EXPORTS_DEFAULT_DIR, MAP_EXPORTS_EXTENSION};

pub struct SaveMapWindow {
    params: WindowParams,
    name: String,
    should_overwrite: bool,
}

impl SaveMapWindow {
    pub fn new(current_name: &str) -> Self {
        let params = WindowParams {
            title: Some("Save Map".to_string()),
            size: vec2(350.0, 350.0),
            ..Default::default()
        };

        SaveMapWindow {
            params,
            name: current_name.to_string(),
            should_overwrite: false,
        }
    }
}

impl Window for SaveMapWindow {
    fn get_params(&self) -> &WindowParams {
        &self.params
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        _size: Vec2,
        _map: &Map,
        _ctx: &EditorContext,
    ) -> Option<EditorAction> {
        let id = hash!("save_map_window");

        {
            let size = vec2(173.0, 25.0);

            widgets::InputText::new(hash!(id, "name_input"))
                .size(size)
                .ratio(1.0)
                .label("Name")
                .ui(ui, &mut self.name);

            {
                let assets_dir = assets_dir();
                let path = Path::new(&assets_dir)
                    .join(MAP_EXPORTS_DEFAULT_DIR)
                    .join(map_name_to_filename(&self.name))
                    .with_extension(MAP_EXPORTS_EXTENSION);

                widgets::Label::new(path.to_string_lossy().as_ref()).ui(ui);
            }
        }

        ui.separator();
        ui.separator();
        ui.separator();
        ui.separator();

        widgets::Checkbox::new(hash!(id, "overwrite_input"))
            .label("Overwrite Existing")
            .ui(ui, &mut self.should_overwrite);

        None
    }

    fn get_buttons(&self, _map: &Map, _ctx: &EditorContext) -> Vec<ButtonParams> {
        let mut res = Vec::new();

        let path = Path::new(MAP_EXPORTS_DEFAULT_DIR)
            .join(map_name_to_filename(&self.name))
            .with_extension(MAP_EXPORTS_EXTENSION);

        let mut action = None;
        if ff_core::map::is_valid_map_export_path(&path, self.should_overwrite) {
            let save_action = EditorAction::SaveMap(Some(self.name.clone()));
            let batch = self.get_close_action().then(save_action);

            action = Some(batch);
        }

        res.push(ButtonParams {
            label: "Save",
            action,
            ..Default::default()
        });

        res.push(ButtonParams {
            label: "Cancel",
            action: Some(self.get_close_action()),
            ..Default::default()
        });

        res
    }
}
