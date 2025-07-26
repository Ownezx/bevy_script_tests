use bevy::prelude::*;
use bevy_mod_scripting::core::script::ScriptComponent;
use bevy_mod_scripting::core::{
    asset::ScriptAsset, bindings::script_value::ScriptValue, callback_labels,
    event::ScriptCallbackEvent, handler::event_handler,
};
use bevy_mod_scripting::lua::LuaScriptingPlugin;

#[derive(Debug, Resource, Default)]
pub struct LoadedScripts(pub Vec<Handle<ScriptAsset>>);

pub struct ScriptPlugin;

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedScripts>();
        app.add_systems(Startup, load_script_assets);
        app.add_systems(Startup, spawn_loaded_scripts.after(load_script_assets));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(
        asset_server.load("map_icons/corvette.png"),
    ));
}

pub fn load_script_assets(
    asset_server: Res<AssetServer>,
    mut loaded_scripts: ResMut<LoadedScripts>,
) {
    loaded_scripts.0.extend(vec![
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