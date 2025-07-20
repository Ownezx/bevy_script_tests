use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::ScriptValue;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LuaState {
    data: ScriptValue,
}
