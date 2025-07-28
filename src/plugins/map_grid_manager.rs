use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;


use crate::plugins::game_settings::GameSettings;


#[derive(Component)]
struct Grid;

pub struct MapGridManager;

impl Plugin for MapGridManager {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, update_grid_on_settings_change);
    }
}

fn setup(mut commands: Commands, settings: Res<GameSettings>) {
    spawn_grid(&mut commands, &settings);
}

fn update_grid_on_settings_change(
    mut commands: Commands,
    settings: Res<GameSettings>,
    grid_query: Query<Entity, With<Grid>>,
) {
    // Only run if GameSettings changed this frame
    if !settings.is_changed() {
        return;
    }

    // Despawn old grid
    for entity in grid_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Spawn updated grid
    spawn_grid(&mut commands, &settings);
}

fn spawn_grid(commands: &mut Commands, settings: &GameSettings) {
    let mut path_builder = PathBuilder::new();

    for x in 0..=settings.grid_width {
        let x_pos = x as f32 * settings.grid_cell_size;
        path_builder.move_to(Vec2::new(x_pos, 0.0));
        path_builder.line_to(Vec2::new(x_pos, settings.grid_height as f32 * settings.grid_cell_size));
    }

    for y in 0..=settings.grid_height {
        let y_pos = y as f32 * settings.grid_cell_size;
        path_builder.move_to(Vec2::new(0.0, y_pos));
        path_builder.line_to(Vec2::new(settings.grid_width as f32 * settings.grid_cell_size, y_pos));
    }

    let path = path_builder.build();

    let grid_offset = Vec3::new(
        -settings.grid_width as f32 * settings.grid_cell_size / 2.0,
        -settings.grid_height as f32 * settings.grid_cell_size / 2.0,
        0.0,
    );

    commands.spawn((
        ShapeBundle {
            path,
            transform: Transform::from_translation(grid_offset),
            ..default()
        },
        Stroke::new(Color::srgba(1.0, 1.0, 1.0, 0.2), 1.0),
        Grid,
    ));
}
