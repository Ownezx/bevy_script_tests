use bevy::prelude::*;

pub struct MapIconLoader;

impl Plugin for MapIconLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(
        asset_server.load("map_icons/corvette.png"),
    ));
}