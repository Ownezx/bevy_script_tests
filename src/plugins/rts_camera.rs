use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, mouse::MouseWheel};
pub struct RtsCameraManager;

#[derive(Component)]
pub struct RtsCamera {
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl Plugin for RtsCameraManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_movement, camera_zoom));
    }
}

fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &RtsCamera)>,
) {
    for (mut transform, camera) in &mut query {
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

        let movement = dir.normalize_or_zero() * camera.move_speed * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

fn camera_zoom(
    mut scroll: EventReader<MouseWheel>,
    windows: Query<&Window>,
    mut query: Query<(
        &mut OrthographicProjection,
        &mut Transform,
        &Camera,
        &GlobalTransform,
        &RtsCamera,
    )>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    for ev in scroll.read() {
        for (mut projection, mut transform, camera, camera_transform, settings) in &mut query {
            let before = camera
                .viewport_to_world_2d(camera_transform, cursor_pos)
                .unwrap_or(Vec2::ZERO);

            projection.scale = (projection.scale - ev.y * settings.zoom_speed)
                .clamp(settings.min_zoom, settings.max_zoom);

            let after = camera
                .viewport_to_world_2d(camera_transform, cursor_pos)
                .unwrap_or(before);

            let delta = before - after;
            transform.translation += delta.extend(0.0);
        }
    }
}
