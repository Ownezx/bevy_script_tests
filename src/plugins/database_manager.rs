use bevy::log::error;
use bevy::log::info;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_mod_scripting::core::bindings::FunctionCallContext;
use bevy_mod_scripting::core::bindings::{GlobalNamespace, NamespaceBuilder, ScriptValue};

#[derive(Reflect, Clone)]
pub struct Template {
    pub name: String,
}

#[derive(Resource, Default, Reflect, Clone)]
pub struct GameDatabase {
    pub templates: HashMap<String, HashMap<String, Template>>,
}

pub struct DatabaseManager;

impl Plugin for DatabaseManager {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameDatabase>();
        app.register_type::<GameDatabase>();
        let world = app.world_mut();
        NamespaceBuilder::<GlobalNamespace>::new_unregistered(world)
            .register("add_template_to_database", add_template_to_database);
    }
}

fn add_template_to_database(
    ctx: FunctionCallContext,
    template_library: ScriptValue,
    template: ScriptValue,
) {
    let Ok(world) = ctx.world() else {
        error!("Could not access world in add_sprite_to_entity.");
        return;
    };

    let ScriptValue::String(template_library) = template_library else {
        error!(
            "template_library should be ScriptValue::String, got {}",
            template_library.type_name()
        );
        return;
    };

    let ScriptValue::Map(map) = template else {
        error!(
            "template should be ScriptValue::Map, got {}",
            template.type_name()
        );
        return;
    };

    let ScriptValue::String(ref name) = map["name"] else {
        error!(
            "template.name should be ScriptValue::String, got {}",
            map["name"].type_name()
        );
        return;
    };

    world.with_resource_mut(|mut database: Mut<GameDatabase>| {
        let library_hash = database
            .templates
            .entry(template_library.to_string())
            .or_insert_with(HashMap::new);

        let template = Template {
            name: name.to_string(),
        };

        if library_hash.contains_key(name.as_ref()) {
            info!(
                "Overwriting \"{}\" in template library \"{}\"",
                name, template_library
            );
        } else {
            info!(
                "Adding \"{}\" in template library \"{}\"",
                name, template_library
            );
        }

        library_hash.insert(name.to_string(), template);
        ()
    }).unwrap();
}
