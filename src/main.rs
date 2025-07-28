use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_scripting::BMSPlugin;
use bevy_mod_scripting::core::{
    bindings::script_value::ScriptValue, callback_labels, event::ScriptCallbackEvent,
    handler::event_handler,
};
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use std::env;

mod components;
mod plugins;
use crate::plugins::{database_manager::DatabaseManager, map_icon_manager::MapIconManager};
use crate::{components::sensor_trace::SensorTrace, plugins::map_grid_manager::MapGridManager};
use crate::{
    components::subsystem_sensor::SubsystemSensor,
    plugins::{game_settings::GameSettingsPlugin, script_manager::ScriptManager},
};

callback_labels!(
    OnClick => "on_click"
);

pub fn send_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut events: EventWriter<ScriptCallbackEvent>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = *camera_query;
        let window = q_windows.single();
    
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
    
        // Calculate a world position based on the cursor's position.
        let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };
        events.send(ScriptCallbackEvent::new_for_all(
            OnClick,
            vec![
                ScriptValue::Integer(world_pos.x as i64),
                ScriptValue::Integer(world_pos.y as i64),
            ],
        ));
    }
}

fn setup_map_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
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
    app.add_systems(Update, send_on_click);
    app.add_systems(
        Update,
        event_handler::<OnClick, LuaScriptingPlugin>.after(send_on_click),
    );

    app.register_type::<SubsystemSensor>();
    app.register_type::<SensorTrace>();

    app.add_plugins(BMSPlugin);
    app.add_plugins(MapIconManager);
    app.add_plugins(DatabaseManager);
    app.add_plugins(ScriptManager);
    app.add_plugins(GameSettingsPlugin);
    app.add_plugins(MapGridManager);

    app.run();
}
