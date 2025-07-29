use bevy::{
    prelude::*,
    window::{PresentMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_scripting::BMSPlugin;
use std::env;

mod components;
mod plugins;
use crate::plugins::{database_manager::DatabaseManager, gm_action_manager::GMActionsManager, map_icon_manager::MapIconManager, rts_camera::{RtsCamera, RtsCameraManager}};
use crate::{components::sensor_trace::SensorTrace, plugins::map_grid_manager::MapGridManager};
use crate::{
    components::subsystem_sensor::SubsystemSensor,
    plugins::{game_settings::GameSettingsPlugin, script_manager::ScriptManager},
};


fn setup_map_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        RtsCamera,
    ));
}

fn main() {
    // Set the LUA_PATH env variable
    let mut assets_path = std::env::current_dir().expect("Failed to get current dir");
    assets_path.push("assets");

    let assets_str = assets_path
        .to_str()
        .expect("Failed to convert path to str")
        .replace("\\", "/");

    let luau_package_path = format!("{}{}", assets_str, "/lua/?.luau");

    unsafe {
        env::set_var("LUA_PATH", luau_package_path);
    }

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (1_000.0, 1_000.0).into(),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }));
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, setup_map_camera);

    app.register_type::<SubsystemSensor>();
    app.register_type::<SensorTrace>();

    app.add_plugins(BMSPlugin);
    app.add_plugins(MapIconManager);
    app.add_plugins(DatabaseManager);
    app.add_plugins(ScriptManager);
    app.add_plugins(GameSettingsPlugin);
    app.add_plugins(MapGridManager);
    app.add_plugins(RtsCameraManager);
    app.add_plugins(GMActionsManager);

    app.run();
}
