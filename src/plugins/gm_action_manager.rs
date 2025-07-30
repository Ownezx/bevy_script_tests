use bevy::log::error;
use bevy::log::info;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;
use bevy_mod_scripting::core::bindings::FunctionCallContext;
use bevy_mod_scripting::core::commands::AddStaticScript;
use bevy_mod_scripting::{
    core::{
        bindings::{GlobalNamespace, NamespaceBuilder, ScriptValue},
        callback_labels,
        event::ScriptCallbackEvent,
        handler::event_handler,
    },
    lua::LuaScriptingPlugin,
};
use std::fs;
use std::path::Path;

use crate::plugins::script_manager::LoadedScripts;

#[derive(Resource, Debug, Default)]
pub struct GMCurrentAction {
    pub template_category: Option<String>,
    pub template_name: Option<String>,
    pub command: Option<String>,
}

#[derive(Resource, Default, Reflect, Clone)]
pub struct GMActions {
    pub command_list: Vec<String>,
}

callback_labels!(
    OnGmAction => "on_gm_action"
);

pub struct GMActionsManager;

impl Plugin for GMActionsManager {
    fn build(&self, app: &mut App) {
        app.init_resource::<GMActions>();
        app.init_resource::<GMCurrentAction>();
        app.add_systems(Update, send_on_gm_action);
        app.add_systems(
            Update,
            event_handler::<OnGmAction, LuaScriptingPlugin>.after(send_on_gm_action),
        );
        app.add_systems(Startup, setup);
        let world = app.world_mut();
        NamespaceBuilder::<GlobalNamespace>::new_unregistered(world)
            .register("register_gm_function", register_gm_function);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loaded_scripts: ResMut<LoadedScripts>,
) {
    let script_dir = Path::new("assets/lua/GMActions");

    if let Ok(entries) = fs::read_dir(script_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Accept only `.lua` and `.luau` files
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext == "lua" || ext == "luau" {
                    if let Some(relative_path) =
                        path.strip_prefix("assets").ok().and_then(|p| p.to_str())
                    {
                        let handle = asset_server.load(relative_path);
                        loaded_scripts.0.push(handle.clone());
                        commands.queue(AddStaticScript::new(relative_path.to_string()));
                    }
                }
            }
        }
    } else {
        error!("Could not read script directory: {:?}", script_dir);
    }
}

pub fn send_on_gm_action(
    mut egui_contexts: EguiContexts,
    buttons: Res<ButtonInput<MouseButton>>,
    current_action: Res<GMCurrentAction>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut events: EventWriter<ScriptCallbackEvent>,
) {
    let ctx = egui_contexts.ctx_mut();
    if ctx.wants_pointer_input() || ctx.wants_keyboard_input() {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) || buttons.just_pressed(MouseButton::Right) {
        let (camera, camera_transform) = *camera_query;
        let window = q_windows.single();

        let Some(cursor_position) = window.cursor_position() else {
            return;
        };

        // Calculate a world position based on the cursor's position.
        let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };

        let Some(command) = (*current_action).command.clone() else {return;};

        if let (Some(category), Some(name)) = (
            (*current_action).template_category.clone(),
            current_action.template_name.clone(),
        ) {
            events.send(ScriptCallbackEvent::new_for_all(
                OnGmAction,
                vec![
                    ScriptValue::String(command.into()),
                    ScriptValue::String(name.into()),
                    ScriptValue::Integer(world_pos.x as i64),
                    ScriptValue::Integer(world_pos.y as i64),
                    ScriptValue::Float(3.0),
                    ScriptValue::Float(400.0),
                ],
            ));
        }
    }
}

fn register_gm_function(ctx: FunctionCallContext, name: ScriptValue) {
    let ScriptValue::String(function_name) = name else {
        error!(
            "Gm function should be registered by string, received {}",
            name.type_name()
        );
        return;
    };

    let Ok(world) = ctx.world() else {
        error!("Could not access world in register_gm_function.");
        return;
    };

    world.with_resource_mut(|mut gm_action: Mut<GMActions>| {
        if gm_action.command_list.iter().any(|s| s == function_name.as_ref()) {
            info!("gm_action \"{}\" already registered", function_name);
        } else {
            info!("Registered gm_action \"{}\"", function_name);
            gm_action.command_list.push(function_name.to_string());
        }
    }).unwrap();
}
