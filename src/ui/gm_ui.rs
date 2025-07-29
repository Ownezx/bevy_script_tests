use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::plugins::{
    database_manager::GameDatabase,
    gm_action_manager::{GMCurrentAction, GMActions},
};

pub struct GmUi;

#[derive(Resource, Default)]
struct SelectedTab {
    name: Option<String>,
}

impl Plugin for GmUi {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedTab::default());
        app.add_systems(Update, draw_button_grid);
    }
}

fn draw_button_grid(
    mut contexts: EguiContexts,
    db: Res<GameDatabase>,
    actions: Res<GMActions>,
    mut current_action: ResMut<GMCurrentAction>,
    mut selected_tab: ResMut<SelectedTab>,
) {
    if selected_tab.name.is_none() {
        if let Some(first_key) = db.templates.keys().next() {
            selected_tab.name = Some(first_key.clone());
        }
    }

    let button_size = egui::vec2(60.0, 60.0);
    let grid_width = button_size.x * 3.0;
    let grid_height = button_size.y * 4.0 + 16.0; // 3 rows templates + 1 row commands + padding

    egui::Window::new("Grid")
        .anchor(egui::Align2::LEFT_BOTTOM, [10.0, 10.0])
        .resizable(false)
        .title_bar(false)
        .fixed_size([grid_width + 20.0, grid_height + 80.0])
        .show(contexts.ctx_mut(), |ui| {
            // Tabs
            ui.horizontal_wrapped(|ui| {
                for category in db.templates.keys() {
                    let selected = selected_tab
                        .name
                        .as_ref()
                        .map_or(false, |n| n == category);
                    if ui.selectable_label(selected, category).clicked() {
                        selected_tab.name = Some(category.clone());
                    }
                }
            });

            ui.separator();

            // Template grid (3x3)
            if let Some(tab_name) = &selected_tab.name {
                if let Some(inner_map) = db.templates.get(tab_name) {
                    let templates: Vec<_> = inner_map.values().collect();

                    egui::Grid::new("template_grid")
                        .spacing([4.0, 4.0])
                        .min_col_width(button_size.x)
                        .show(ui, |ui| {
                            for row in 0..3 {
                                for col in 0..3 {
                                    let idx = row * 3 + col;
                                    if let Some(template) = templates.get(idx) {
                                        if ui
                                            .add_sized(button_size, egui::Button::new(&template.name))
                                            .clicked()
                                        {
                                            current_action.template_category = Some(tab_name.clone());
                                            current_action.template_name = Some(template.name.clone());
                                            info!(
                                                tab_name, template.name
                                            );
                                        }
                                    } else {
                                        ui.add_sized(button_size, egui::Label::new(""));
                                    }
                                }
                                ui.end_row();
                            }
                        });
                }
            }

            ui.add_space(8.0);
            ui.separator();

            // Command grid (3x1)
            egui::Grid::new("command_grid")
                .spacing([4.0, 4.0])
                .min_col_width(button_size.x)
                .show(ui, |ui| {
                    for i in 0..3 {
                        if let Some(command) = actions.command_list.get(i) {
                            if ui
                                .add_sized(button_size, egui::Button::new(command))
                                .clicked()
                            {
                                current_action.command = Some(command.clone());
                            }
                        } else {
                            ui.add_sized(button_size, egui::Label::new(""));
                        }
                    }
                    ui.end_row();
                });
        });
}
