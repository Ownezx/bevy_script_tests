use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow},
};
use bevy_mod_scripting::BMSPlugin;
use bevy_mod_scripting::core::script::ScriptComponent;
use bevy_mod_scripting::core::{
    asset::ScriptAsset, bindings::script_value::ScriptValue, callback_labels,
    event::ScriptCallbackEvent, handler::event_handler,
};
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use std::env;

mod components;
mod plugins;
use crate::{components::subsystem_sensor::SubsystemSensor, plugins::{game_settings::GameSettingsPlugin, scripting::ScriptPlugin}};
use crate::plugins::map_icon_loader::MapIconLoader;
use crate::{
    components::sensor_trace::SensorTrace,
    plugins::{ map_grid::MapGrid},
};

callback_labels!(
    OnClick => "on_click"
);

pub fn send_on_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut events: EventWriter<ScriptCallbackEvent>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = q_windows.single();
        let pos = window.cursor_position().unwrap_or_default();
        let x = pos.x as u32;
        let y = pos.y as u32;
        info!("Bevy on clic");
        events.send(ScriptCallbackEvent::new_for_all(
            OnClick,
            vec![
                ScriptValue::Integer(x as i64),
                ScriptValue::Integer(y as i64),
            ],
        ));
    }
}

fn setup_map_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    });
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

    app.add_systems(Startup, setup_map_camera);
    app.add_systems(Update, send_on_click);
    app.add_systems(
        Update,
        event_handler::<OnClick, LuaScriptingPlugin>.after(send_on_click),
    );

    app.register_type::<SubsystemSensor>();
    app.register_type::<SensorTrace>();

    app.add_plugins(BMSPlugin);
    app.add_plugins(MapIconLoader);
    app.add_plugins(ScriptPlugin);
    app.add_plugins(GameSettingsPlugin);
    app.add_plugins(MapGrid);

    app.run();
}
