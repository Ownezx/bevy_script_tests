use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, mouse::MouseWheel};

pub struct RtsCameraManager;

#[derive(Component)]
pub struct RtsCamera {
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub edge_percent_x: f32,
    pub edge_percent_y: f32,
}

impl Default for RtsCamera {
    fn default() -> Self {
        Self {
            move_speed: 1000.0,
            zoom_speed: 0.05,
            min_zoom: 0.5,
            max_zoom: 3.0,
            edge_percent_x: 0.1,
            edge_percent_y: 0.1,
        }
    }
}

impl Plugin for RtsCameraManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_movement, camera_zoom, camera_edge_scrolling));
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

fn camera_edge_scrolling(
    windows: Query<&Window>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &RtsCamera)>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };

    let width = window.width();
    let height = window.height();

    for (mut transform, camera) in &mut query {
        let mut dir = Vec2::ZERO;

        if cursor_pos.x < width * camera.edge_percent_x {
            dir.x -= 1.0;
        }
        if cursor_pos.x > width * (1.0 - camera.edge_percent_x) {
            dir.x += 1.0;
        }
        if cursor_pos.y < height * camera.edge_percent_y {
            dir.y += 1.0;
        }
        if cursor_pos.y > height * (1.0 - camera.edge_percent_y) {
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
