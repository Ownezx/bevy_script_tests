use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_scripting::{core::{bindings::ScriptValue, callback_labels, event::ScriptCallbackEvent, handler::event_handler}, lua::LuaScriptingPlugin};
use bevy::log::info;


#[derive(Resource, Default, Reflect, Clone)]
pub struct GMActions {
    command_list: Vec<String>,
}

callback_labels!(
    OnGmAction => "on_gm_action"
);

pub struct GMActionsManager;

impl Plugin for GMActionsManager {
    fn build(&self, app: &mut App) {
        app.init_resource::<GMActions>();
        app.add_systems(Update, send_on_gm_action);
        app.add_systems(
            Update,
            event_handler::<OnGmAction, LuaScriptingPlugin>.after(send_on_gm_action),
        );
    }
}


pub fn send_on_gm_action(
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
            OnGmAction,
            vec![
                ScriptValue::String("test".to_string().into()),
                ScriptValue::String("cruiser".to_string().into()),
                ScriptValue::Integer(world_pos.x as i64),
                ScriptValue::Integer(world_pos.y as i64),
            ],
        ));
    }
}


