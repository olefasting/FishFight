use ff_core::prelude::*;

use ff_core::gui::{get_gui_theme, theme::LIST_BOX_ENTRY_HEIGHT, ELEMENT_MARGIN};
use ff_core::map::{Map, MapBackgroundLayer};

use ff_core::macroquad::hash;
use ff_core::macroquad::ui::{widgets, Ui};

use super::{ButtonParams, EditorAction, EditorContext, Window, WindowParams};

pub struct BackgroundPropertiesWindow {
    params: WindowParams,
    color: Color,
    layers: Vec<MapBackgroundLayer>,
    layer_texture_id: Option<String>,
    layer_depth: f32,
    selected_layer: Option<usize>,
}

impl BackgroundPropertiesWindow {
    pub fn new(color: Color, layers: Vec<MapBackgroundLayer>) -> Self {
        let params = WindowParams {
            title: Some("Background Properties".to_string()),
            size: vec2(360.0, 500.0),
            ..Default::default()
        };

        BackgroundPropertiesWindow {
            params,
            color,
            layers,
            layer_texture_id: None,
            layer_depth: 0.0,
            selected_layer: None,
        }
    }
}

impl Window for BackgroundPropertiesWindow {
    fn get_params(&self) -> &WindowParams {
        &self.params
    }

    fn get_buttons(&self, _map: &Map, _ctx: &EditorContext) -> Vec<ButtonParams> {
        let mut res = Vec::new();

        let action = self
            .get_close_action()
            .then(EditorAction::UpdateBackground {
                color: self.color,
                layers: self.layers.clone(),
            });

        res.push(ButtonParams {
            label: "Save",
            action: Some(action),
            ..Default::default()
        });

        res.push(ButtonParams {
            label: "Cancel",
            action: Some(self.get_close_action()),
            ..Default::default()
        });

        res
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        size: Vec2,
        _map: &Map,
        _ctx: &EditorContext,
    ) -> Option<EditorAction> {
        let id = hash!("background_properties_window");

        widgets::Group::new(hash!(id, "color_group"), vec2(size.x * 0.4, size.y * 0.5))
            .position(vec2(0.0, 0.0))
            .ui(ui, |ui| {
                let mut r_str = format!("{:.1}", self.color.red);
                let mut g_str = format!("{:.1}", self.color.green);
                let mut b_str = format!("{:.1}", self.color.blue);
                let mut a_str = format!("{:.1}", self.color.alpha);

                widgets::InputText::new(hash!(id, "color_r_input"))
                    .ratio(1.0)
                    .label("r")
                    .ui(ui, &mut r_str);

                widgets::InputText::new(hash!(id, "color_g_input"))
                    .ratio(1.0)
                    .label("g")
                    .ui(ui, &mut g_str);

                widgets::InputText::new(hash!(id, "color_b_input"))
                    .ratio(1.0)
                    .label("b")
                    .ui(ui, &mut b_str);

                widgets::InputText::new(hash!(id, "color_a_input"))
                    .ratio(1.0)
                    .label("a")
                    .ui(ui, &mut a_str);

                if let Ok(r) = r_str.parse::<f32>() {
                    self.color.red = r;
                }

                if let Ok(g) = g_str.parse::<f32>() {
                    self.color.green = g;
                }

                if let Ok(b) = b_str.parse::<f32>() {
                    self.color.blue = b;
                }

                if let Ok(a) = a_str.parse::<f32>() {
                    self.color.alpha = a;
                }
            });

        let layer_list_size = vec2((size.x * 0.6) - ELEMENT_MARGIN, size.y * 0.5);
        let layer_list_entry_size = vec2(layer_list_size.x, LIST_BOX_ENTRY_HEIGHT);

        {
            let gui_theme = get_gui_theme();
            ui.push_skin(&gui_theme.list_box_no_bg);
        }

        widgets::Group::new(hash!(id, "layer_list"), layer_list_size)
            .position(vec2((size.x * 0.4) + ELEMENT_MARGIN, 0.0))
            .ui(ui, |ui| {
                let layers = self.layers.clone();
                for (i, layer) in layers.iter().enumerate() {
                    widgets::Group::new(hash!(id, "layer_list_entry", i), layer_list_entry_size)
                        .position(vec2(0.0, i as f32 * LIST_BOX_ENTRY_HEIGHT))
                        .ui(ui, |ui| {
                            let mut is_selected = false;
                            if let Some(index) = self.selected_layer {
                                is_selected = index == i;
                            }

                            if is_selected {
                                let gui_theme = get_gui_theme();
                                ui.push_skin(&gui_theme.list_box_selected);
                            }

                            let entry_btn = widgets::Button::new("")
                                .size(layer_list_entry_size)
                                .position(vec2(0.0, 0.0));

                            if entry_btn.ui(ui) {
                                if is_selected {
                                    self.selected_layer = None;
                                    self.layer_texture_id = None;
                                    self.layer_depth = 0.0;
                                } else {
                                    self.selected_layer = Some(i);
                                    self.layer_texture_id = Some(layer.texture_id.clone());
                                    self.layer_depth = layer.depth;
                                }
                            }

                            ui.label(vec2(0.0, 0.0), &layer.texture_id);

                            if is_selected {
                                ui.pop_skin();
                            }
                        });
                }
            });

        ui.pop_skin();

        widgets::Group::new(
            hash!(id, "layer_attributes"),
            vec2(size.x, (size.y * 0.5) - ELEMENT_MARGIN),
        )
        .position(vec2(0.0, (size.y * 0.5) + ELEMENT_MARGIN))
        .ui(ui, |ui| {
            let mut textures =
                iter_texture_ids_of_kind(TextureKind::Background).collect::<Vec<_>>();

            textures.sort_unstable();

            let mut texture_index = textures
                .iter()
                .enumerate()
                .find_map(|(i, id)| {
                    if let Some(texture_id) = &self.layer_texture_id {
                        if *id == *texture_id {
                            return Some(i);
                        }
                    }

                    None
                })
                .unwrap_or(0);

            let texture_ids = textures.iter().map(|str| str.as_str()).collect::<Vec<_>>();

            widgets::ComboBox::new(hash!(id, "layer_texture_input"), &texture_ids)
                .ratio(0.8)
                .label("Texture")
                .ui(ui, &mut texture_index);

            self.layer_texture_id = texture_ids.get(texture_index).map(|str| str.to_string());

            let mut depth_str = format!("{:.1}", self.layer_depth);

            widgets::InputText::new(hash!(id, "layer_depth_input"))
                .ratio(0.4)
                .label("Depth")
                .ui(ui, &mut depth_str);

            if let Ok(depth) = depth_str.parse::<f32>() {
                self.layer_depth = depth;
            }

            ui.same_line(0.0);

            if let Some(mut index) = self.selected_layer {
                {
                    let layer = self.layers.get_mut(index).unwrap();
                    layer.texture_id = self.layer_texture_id.clone().unwrap();
                    layer.depth = self.layer_depth;
                }

                let delete_btn = widgets::Button::new("Delete");

                if delete_btn.ui(ui) {
                    self.layers.remove(index);

                    self.selected_layer = None;
                    self.layer_texture_id = None;
                    self.layer_depth = 0.0;
                }

                ui.same_line(0.0);

                let up_btn = widgets::Button::new("Up");

                if up_btn.ui(ui) && index > 0 {
                    let layer = self.layers.remove(index);

                    index -= 1;
                    self.layers.insert(index, layer);

                    self.selected_layer = Some(index);
                }

                ui.same_line(0.0);

                let down_btn = widgets::Button::new("Down");

                if down_btn.ui(ui) && index < self.layers.len() {
                    let layer = self.layers.remove(index);

                    if index < self.layers.len() {
                        index += 1;
                        self.layers.insert(index, layer);
                    } else {
                        index = self.layers.len();
                        self.layers.push(layer);
                    }

                    self.selected_layer = Some(index);
                }
            } else {
                let add_btn = widgets::Button::new("Add");

                if add_btn.ui(ui) && self.layer_texture_id.is_some() {
                    let texture_id = self.layer_texture_id.take().unwrap();
                    let depth = self.layer_depth;

                    self.layer_depth = 0.0;

                    self.layers.push(MapBackgroundLayer {
                        texture_id,
                        depth,
                        offset: Vec2::ZERO,
                    });
                }
            }
        });

        None
    }
}
