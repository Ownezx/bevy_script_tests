use bevy::prelude::*;

use crate::plugins::script_manager::LoadedScripts;


#[derive(Resource, Default, Reflect, Clone)]
pub struct GameSettings {
    pub grid_cell_size: f32,
    pub grid_width: isize,
    pub grid_height: isize,
    pub map_icon_base_scale: f32,
    pub camera_move_speed: f32,
    pub camera_zoom_speed: f32,
    pub camera_min_zoom: f32,
    pub camera_max_zoom: f32,
    pub camera_edge_percent_x: f32,
    pub camera_edge_percent_y: f32,
}


// Define a plugin to insert the resource
pub struct GameSettingsPlugin;

impl Plugin for GameSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
        app.register_type::<GameSettings>();
        app.add_systems(Startup, load_settings_script);
    }
}

pub fn load_settings_script(
    asset_server: Res<AssetServer>,
    mut loaded_scripts: ResMut<LoadedScripts>,
) {
    loaded_scripts.0.extend(vec![
        asset_server.load("lua/mainSettings.luau"),
    ]);
}
