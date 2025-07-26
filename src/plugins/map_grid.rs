use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const GRID_CELL_SIZE: f32 = 100.0;
const GRID_WIDTH: i32 = 10;
const GRID_HEIGHT: i32 = 10;

pub struct MapGrid;

impl Plugin for MapGrid {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
           .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Build grid path
    let mut path_builder = PathBuilder::new();

    for x in 0..=GRID_WIDTH {
        let x_pos = x as f32 * GRID_CELL_SIZE;
        path_builder.move_to(Vec2::new(x_pos, 0.0));
        path_builder.line_to(Vec2::new(x_pos, GRID_HEIGHT as f32 * GRID_CELL_SIZE));
    }

    for y in 0..=GRID_HEIGHT {
        let y_pos = y as f32 * GRID_CELL_SIZE;
        path_builder.move_to(Vec2::new(0.0, y_pos));
        path_builder.line_to(Vec2::new(GRID_WIDTH as f32 * GRID_CELL_SIZE, y_pos));
    }

    let path = path_builder.build();

    // Center the grid by offsetting it negatively
    let grid_offset = Vec3::new(
        -GRID_WIDTH as f32 * GRID_CELL_SIZE / 2.0,
        -GRID_HEIGHT as f32 * GRID_CELL_SIZE / 2.0,
        0.0,
    );

    commands.spawn((
        ShapeBundle {
            path,
            transform: Transform::from_translation(grid_offset),
            ..default()
        },
        Stroke::new(Color::rgba(1.0,1.0,1.0,0.2), 1.0),
    ));
}
