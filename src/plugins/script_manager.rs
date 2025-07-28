use bevy::prelude::*;
use bevy_mod_scripting::core::asset::ScriptAsset;
use bevy_mod_scripting::core::script::ScriptComponent;

#[derive(Debug, Resource, Default)]
pub struct LoadedScripts(pub Vec<Handle<ScriptAsset>>);

pub struct ScriptManager;

impl Plugin for ScriptManager {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedScripts>();
        app.add_systems(Startup, load_script_assets);
        app.add_systems(Startup, spawn_loaded_scripts.after(load_script_assets));
    }
}

pub fn load_script_assets(
    asset_server: Res<AssetServer>,
    mut loaded_scripts: ResMut<LoadedScripts>,
) {
    loaded_scripts.0.extend(vec![
        asset_server.load("lua/library/Template.luau"),
        asset_server.load("lua/templates/FirstTemplates.luau"),
        asset_server.load("lua/templates/TemplateManager.luau"),
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
