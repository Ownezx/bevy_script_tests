use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, mouse::MouseWheel};

use crate::plugins::game_settings::GameSettings;

pub struct RtsCameraManager;

#[derive(Component)]
pub struct RtsCamera; // Marker component only

impl Plugin for RtsCameraManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_movement, camera_zoom, camera_edge_scrolling));
    }
}

fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<RtsCamera>>,
) {
    for (mut transform, projection) in &mut query {
        let mut dir = Vec2::ZERO;
        if keys.pressed(KeyCode::ArrowLeft) {
            dir.x -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            dir.x += 1.0;
        }
        if keys.pressed(KeyCode::ArrowUp) {
            dir.y += 1.0;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            dir.y -= 1.0;
        }

        // Scale movement speed by the current zoom level (projection.scale)
        let movement_speed = settings.camera_move_speed * projection.scale;
        let movement = dir.normalize_or_zero() * movement_speed * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

fn camera_edge_scrolling(
    windows: Query<&Window>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<RtsCamera>>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };

    let width = window.width();
    let height = window.height();

    for (mut transform, projection) in &mut query {
        let mut dir = Vec2::ZERO;

        if cursor_pos.x < width * settings.camera_edge_percent_x {
            dir.x -= 1.0;
        }
        if cursor_pos.x > width * (1.0 - settings.camera_edge_percent_x) {
            dir.x += 1.0;
        }
        if cursor_pos.y < height * settings.camera_edge_percent_y {
            dir.y += 1.0;
        }
        if cursor_pos.y > height * (1.0 - settings.camera_edge_percent_y) {
            dir.y -= 1.0;
        }

        // Scale movement speed by the current zoom level
        let movement_speed = settings.camera_move_speed * projection.scale;
        let movement = dir.normalize_or_zero() * movement_speed * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

fn camera_zoom(
    mut scroll: EventReader<MouseWheel>,
    windows: Query<&Window>,
    mut query: Query<(&mut OrthographicProjection, &mut Transform, &Camera, &GlobalTransform), With<RtsCamera>>,
    settings: Res<GameSettings>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };

    for ev in scroll.read() {
        for (mut projection, mut transform, camera, camera_transform) in &mut query {
            let before = camera
                .viewport_to_world_2d(camera_transform, cursor_pos)
                .unwrap_or(Vec2::ZERO);

            projection.scale = (projection.scale - ev.y * settings.camera_zoom_speed)
                .clamp(settings.camera_min_zoom, settings.camera_max_zoom);

            let after = camera
                .viewport_to_world_2d(camera_transform, cursor_pos)
                .unwrap_or(before);

            let delta = before - after;
            transform.translation += delta.extend(0.0);
        }
    }
}
