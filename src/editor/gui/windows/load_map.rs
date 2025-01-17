use ff_core::prelude::*;
use std::ops::Deref;

use ff_core::gui::{get_gui_theme, theme::LIST_BOX_ENTRY_HEIGHT, ELEMENT_MARGIN};
use ff_core::macroquad::hash;
use ff_core::macroquad::ui::{widgets, Ui};
use ff_core::map::{get_map, iter_maps, Map};

use super::{ButtonParams, EditorAction, EditorContext, Window, WindowParams};

pub struct LoadMapWindow {
    params: WindowParams,
    index: Option<usize>,
}

impl LoadMapWindow {
    pub fn new() -> Self {
        let params = WindowParams {
            title: Some("Open Map".to_string()),
            size: vec2(350.0, 350.0),
            ..Default::default()
        };

        LoadMapWindow {
            params,
            index: None,
        }
    }
}

impl Window for LoadMapWindow {
    fn get_params(&self) -> &WindowParams {
        &self.params
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        size: Vec2,
        _map: &Map,
        _ctx: &EditorContext,
    ) -> Option<EditorAction> {
        let id = hash!("load_map_window");

        {
            let gui_theme = get_gui_theme();
            ui.push_skin(&gui_theme.list_box_no_bg);
        }

        if let Some(index) = self.index {
            let btn_size = vec2(size.x, LIST_BOX_ENTRY_HEIGHT);
            let btn_position = vec2(0.0, 0.0);

            let back_btn = widgets::Button::new("")
                .size(btn_size)
                .position(btn_position);

            if back_btn.ui(ui) {
                self.index = None;
            }

            ui.label(btn_position, "< Back");

            ui.pop_skin();

            {
                let map_resource = get_map(index);

                let preview_size = map_resource.preview.size();

                let mut width = size.x;
                let mut height = (width / preview_size.width) * preview_size.height;

                let max_height = size.y - LIST_BOX_ENTRY_HEIGHT - (ELEMENT_MARGIN * 2.0);

                if height > max_height {
                    let preview_size = map_resource.preview.size();
                    height = max_height;
                    width = (height / preview_size.height) * preview_size.width;
                }

                let preview_position = vec2((size.x - width) / 2.0, btn_size.y + ELEMENT_MARGIN);

                widgets::Texture::new(map_resource.preview.deref().into())
                    .size(width, height)
                    .position(preview_position)
                    .ui(ui);
            }
        } else {
            let size = vec2(size.x, size.y - ELEMENT_MARGIN);
            widgets::Group::new(hash!(id, "list_box"), size)
                .position(Vec2::ZERO)
                .ui(ui, |ui| {
                    let entry_size = vec2(size.x, LIST_BOX_ENTRY_HEIGHT);

                    for (i, map_resource) in iter_maps().enumerate() {
                        let mut is_selected = false;
                        if let Some(index) = self.index {
                            is_selected = index == i;
                        }

                        if is_selected {
                            let gui_theme = get_gui_theme();
                            ui.push_skin(&gui_theme.list_box_selected);
                        }

                        let entry_position = vec2(0.0, i as f32 * entry_size.y);

                        let entry_btn = widgets::Button::new("")
                            .size(entry_size)
                            .position(entry_position);

                        if entry_btn.ui(ui) {
                            self.index = Some(i);
                        }

                        ui.label(entry_position, &map_resource.meta.path);

                        if is_selected {
                            ui.pop_skin();
                        }
                    }
                });

            ui.pop_skin();
        }

        None
    }

    fn get_buttons(&self, _map: &Map, _ctx: &EditorContext) -> Vec<ButtonParams> {
        let mut res = Vec::new();

        let mut open_action = None;
        let mut import_action = None;

        if let Some(index) = self.index {
            let open_batch = self.get_close_action().then(EditorAction::OpenMap(index));
            open_action = Some(open_batch);

            let import_batch = self
                .get_close_action()
                .then(EditorAction::OpenImportWindow(index));
            import_action = Some(import_batch);
        }

        res.push(ButtonParams {
            label: "Open",
            action: open_action,
            ..Default::default()
        });

        res.push(ButtonParams {
            label: "Import",
            action: import_action,
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

impl Default for LoadMapWindow {
    fn default() -> Self {
        Self::new()
    }
}
