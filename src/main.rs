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

mod components;
use crate::components::subsystem_sensor::SubsystemSensor;

#[derive(Debug, Resource, Default)]
pub struct LoadedScripts(pub Vec<Handle<ScriptAsset>>);

/// Prepares any scripts by loading them and storing the handles.
pub fn load_script_assets(
    asset_server: Res<AssetServer>,
    mut loaded_scripts: ResMut<LoadedScripts>,
) {
    loaded_scripts.0.extend(vec![
        asset_server.load("scripts/mainSettings.lua"),
        asset_server.load("scripts/clickFunction.lua"),
        asset_server.load("scenarios/test.lua"),
    ]);
}

fn spawn_loaded_scripts(mut commands: Commands) {
    commands.spawn(ScriptComponent::new(vec!["scripts/clickFunction.lua"]));
    commands.spawn(ScriptComponent::new(vec!["scripts/mainSettings.lua"]));
    commands.spawn(ScriptComponent::new(vec!["scenarios/test.lua"]));
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

    app.add_plugins(BMSPlugin);

    app.run();
}
