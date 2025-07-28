use bevy::log::error;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_mod_scripting::core::bindings::{
    FunctionCallContext, GlobalNamespace, NamespaceBuilder, ReflectReference,
    ScriptComponentRegistration, ScriptResourceRegistration, ScriptTypeRegistration, Val,
};

use crate::plugins::game_settings::GameSettings;

pub struct MapIconLoader;

#[derive(Resource, Default, Reflect, Clone)]
pub struct IconLoaded {
    hash: HashMap<String, Handle<Image>>,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MapIcon;

impl Plugin for MapIconLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.init_resource::<IconLoaded>();
        app.register_type::<IconLoaded>();
        app.register_type::<MapIcon>();
        app.add_systems(Update, update_component_size);
        let world = app.world_mut();
        NamespaceBuilder::<GlobalNamespace>::new_unregistered(world)
            .register("add_sprite_to_entity", add_sprite_to_entity);
    }
}

fn setup(
    mut icon_loaded: ResMut<IconLoaded>,
    asset_server: Res<AssetServer>,
) {
    icon_loaded.hash.insert(
        "corvette".to_string(),
        asset_server.load("map_icons/corvette.png"),
    );
    icon_loaded.hash.insert(
        "cruiser".to_string(),
        asset_server.load("map_icons/cruiser.png"),
    );
    icon_loaded.hash.insert(
        "destroyer".to_string(),
        asset_server.load("map_icons/destroyer.png"),
    );
    icon_loaded.hash.insert(
        "frigate".to_string(),
        asset_server.load("map_icons/frigate.png"),
    );
}

fn add_sprite_to_entity(ctx: FunctionCallContext, entity: Val<Entity>, icon: String) {
    let Ok(world) = ctx.world() else {
        error!("Could not access world in add_sprite_to_entity.");
        return;
    };

    let game_settings: ScriptTypeRegistration = world.get_type_by_name("GameSettings").unwrap();
    let game_settings: ScriptResourceRegistration =
        world.get_resource_type(game_settings).unwrap().unwrap();
    let game_settings: ReflectReference = world
        .get_resource(game_settings.resource_id())
        .unwrap()
        .unwrap();
    let game_settings: GameSettings = game_settings.downcast(ctx.world().unwrap()).unwrap();

    let icon_loaded: ScriptTypeRegistration = world.get_type_by_name("IconLoaded").unwrap();
    let icon_loaded: ScriptResourceRegistration =
        world.get_resource_type(icon_loaded).unwrap().unwrap();
    let icon_loaded: ReflectReference = world
        .get_resource(icon_loaded.resource_id())
        .unwrap()
        .unwrap();

    let icon_loaded: IconLoaded = icon_loaded.downcast(ctx.world().unwrap()).unwrap();
    let Some(image) = icon_loaded.hash.get(&icon) else {
        error!("Image {icon} not loaded.");
        return;
    };
    let sprite: Sprite = Sprite::from_image(image.clone());
    let binding = world.allocator();

    let sprite_reference = {
        let mut allocator = (&binding).write();
        ReflectReference::new_allocated(sprite, &mut allocator)
    };

    let sprite_registration: ScriptTypeRegistration = world.get_type_by_name("Sprite").unwrap();
    let sprite_registration: ScriptComponentRegistration = world
        .get_component_type(sprite_registration)
        .unwrap()
        .unwrap();

    let map_icon = MapIcon;   
    let map_icon_reference = {
        let mut allocator = (&binding).write();
        ReflectReference::new_allocated(map_icon, &mut allocator)
    };
    let map_icon_registration: ScriptTypeRegistration = world.get_type_by_name("MapIcon").unwrap();
    let map_icon_registration: ScriptComponentRegistration = world
        .get_component_type(map_icon_registration)
        .unwrap()
        .unwrap();

    let mut transform = Transform::default();
    transform.scale = Vec3::splat(game_settings.map_icon_base_scale);
    let transform_reference = {
        let mut allocator = (&binding).write();
        ReflectReference::new_allocated(transform, &mut allocator)
    };
    let transform_registration: ScriptTypeRegistration = world.get_type_by_name("Transform").unwrap();
    let transform_registration: ScriptComponentRegistration = world
        .get_component_type(transform_registration)
        .unwrap()
        .unwrap();

    let child_entity = world.spawn().unwrap();
    let Ok(_) = world.insert_component(child_entity, sprite_registration, sprite_reference) else {
        error!("Unable to insert sprite {icon} on entity {}.", *entity);
        return;
    };
    let Ok(_) = world.insert_component(child_entity, map_icon_registration, map_icon_reference)
    else {
        error!("Unable to insert MapIcon on entity {}.", *entity);
        return;
    };
    let Ok(_) = world.insert_component(child_entity, transform_registration, transform_reference)
    else {
        error!("Unable to insert MapIcon on entity {}.", *entity);
        return;
    };

    let mut visibility = InheritedVisibility::default();
    let visibility_reference = {
        let mut allocator = (&binding).write();
        ReflectReference::new_allocated(visibility, &mut allocator)
    };
    let visibility_registration: ScriptTypeRegistration = world.get_type_by_name("InheritedVisibility").unwrap();
    let visibility_registration: ScriptComponentRegistration = world
        .get_component_type(visibility_registration)
        .unwrap()
        .unwrap();
    
    // This is to avoid problems with inherited visibilty.
    let Ok(_) = world.insert_component(*entity, visibility_registration, visibility_reference)
    else {
        error!("Unable to insert MapIcon on entity {}.", *entity);
        return;
    };

    world.push_children(*entity, &[child_entity]).unwrap();
}

fn update_component_size(
    settings: Res<GameSettings>,
    mut map_icons: Query<&mut Transform, With<MapIcon>>,
) {
    if !settings.is_changed() {
        return;
    }

    for mut transform in map_icons.iter_mut() {
        transform.scale = Vec3::splat(settings.map_icon_base_scale);
    }
}
