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
use crate::components::sensor_trace::SensorTrace;
use crate::components::subsystem_sensor::SubsystemSensor;

#[derive(Debug, Resource, Default)]
pub struct LoadedScripts(pub Vec<Handle<ScriptAsset>>);

/// Prepares any scripts by loading them and storing the handles.
pub fn load_script_assets(
    asset_server: Res<AssetServer>,
    mut loaded_scripts: ResMut<LoadedScripts>,
) {
    loaded_scripts.0.extend(vec![
        asset_server.load("lua/library/mainSettings.lua"),
        asset_server.load("lua/library/Template.luau"),
        asset_server.load("lua/library/FirstTemplates.luau"),
        asset_server.load("lua/GMActions/AddEntityOnClickPos.lua"),
        asset_server.load("lua/scenarios/test.lua"),
    ]);
}

fn spawn_loaded_scripts(mut commands: Commands) {
    commands.spawn(ScriptComponent::new(vec![
        "lua/library/mainSettings.lua",
        "lua/GMActions/AddEntityOnClickPos.lua",
]));
}

// define the label, you can define as many as you like here
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

fn main() {
    // Set the LUA_PATH env variable
    let mut assets_path = std::env::current_dir().expect("Failed to get current dir");
    assets_path.push("assets");

    let assets_str = assets_path
        .to_str()
        .expect("Failed to convert path to str")
        .replace("\\", "/");

    let luau_package_path = format!("{}{}", assets_str, "/lua/?.luau");

    unsafe{
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
    app.init_resource::<LoadedScripts>();
    app.add_systems(Startup, load_script_assets);
    app.add_systems(Startup, spawn_loaded_scripts.after(load_script_assets));
    app.add_systems(Update, send_on_click);
    app.add_systems(
        Update,
        event_handler::<OnClick, LuaScriptingPlugin>.after(send_on_click),
    );

    app.register_type::<SubsystemSensor>();
    app.register_type::<SensorTrace>();

    app.add_plugins(BMSPlugin);


    app.run();
}
