use bevy::prelude::*;

use crate::plugins::scripting::LoadedScripts;


#[derive(Resource, Default, Reflect)]
pub struct GameSettings {
    pub grid_cell_size: isize,
    pub grid_width: f32,
    pub grid_height: f32,
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
