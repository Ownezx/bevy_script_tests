use bevy::prelude::*;
use bevy_mod_scripting::BMSPlugin;
use bevy::{
    prelude::*,
    window::PresentMode,
};

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

    app.add_plugins(BMSPlugin);

    app.run();
}
