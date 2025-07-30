use bevy::log::{error, info};
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
use std::f32::consts::PI;
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

// New resource to track drag state and positions
#[derive(Resource, Default)]
pub struct DragState {
    pub dragging: bool,
    pub start_pos: Option<Vec2>,
    pub current_pos: Option<Vec2>,
}

callback_labels!(
    OnGmAction => "on_gm_action"
);

pub struct GMActionsManager;

impl Plugin for GMActionsManager {
    fn build(&self, app: &mut App) {
        app.init_resource::<GMActions>();
        app.init_resource::<GMCurrentAction>();
        app.init_resource::<DragState>();
        app.add_systems(Update, (track_drag_state, draw_gizmo));
        app.add_systems(
            Update,
            event_handler::<OnGmAction, LuaScriptingPlugin>.after(track_drag_state),
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

/// Tracks dragging state and updates DragState resource with start and current positions
fn track_drag_state(
    buttons: Res<ButtonInput<MouseButton>>,
    current_action: Res<GMCurrentAction>,
    mut events: EventWriter<ScriptCallbackEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut egui_contexts: EguiContexts,
    mut drag_state: ResMut<DragState>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            // Drag started
            let ctx = egui_contexts.ctx_mut();
            if ctx.wants_pointer_input() || ctx.wants_keyboard_input() {
                return;
            }
        
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                drag_state.dragging = true;
                drag_state.start_pos = Some(world_pos);
                drag_state.current_pos = Some(world_pos);
            }
        } else if buttons.pressed(MouseButton::Left) && drag_state.dragging {
            // Drag ongoing
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                drag_state.current_pos = Some(world_pos);
            }
        } else if buttons.just_released(MouseButton::Left) {
            // Drag ended
            if let (Some(start_pos), Some(stop_pos)) = (drag_state.start_pos, drag_state.current_pos)
            {
                send_on_gm_action(
                    current_action,
                    events,
                    start_pos,
                    (start_pos-stop_pos).to_angle()+PI/2.0,
                    (start_pos-stop_pos).length()
                )
            }
            drag_state.dragging = false;
            drag_state.start_pos = None;
            drag_state.current_pos = None;

        }
    }
}

/// Sends the gm_action event as before (unchanged logic except no egui context checks)
pub fn send_on_gm_action(
    current_action: Res<GMCurrentAction>,
    mut events: EventWriter<ScriptCallbackEvent>,
    pos: Vec2,
    angle: f32,
    size: f32,
) {
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
                ScriptValue::Float(pos.x.into()),
                ScriptValue::Float(pos.y.into()),
                ScriptValue::Float(angle.into()),
                ScriptValue::Float(size.into()),
            ],
        ));
    }
}

/// Draws a circle and a line gizmo showing drag size and direction
fn draw_gizmo(
    mut gizmos: Gizmos,
    drag_state: Res<DragState>,
) {
    if !drag_state.dragging {
        return;
    }

    let start = match drag_state.start_pos {
        Some(pos) => pos,
        None => return,
    };
    let current = match drag_state.current_pos {
        Some(pos) => pos,
        None => return,
    };

    // Draw a circle at the start drag position with radius equal to drag distance
    let radius = start.distance(current);
    gizmos.circle(start.extend(0.0), radius, Color::WHITE);

    // Draw a line from start to current position
    gizmos.line(start.extend(0.0), current.extend(0.0), Color::WHITE);
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
