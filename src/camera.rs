use crate::actions::Actions;

use crate::GameState;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::{Camera2d, RenderTarget};
use itertools::Itertools;

pub struct CameraPlugin;

const ZOOM_SPEED: f32 = 0.5;
const PAN_SPEED: f32 = 0.54;
const PAN_SHIFT_MODIFIER: f32 = 0.8; // speeds it up
const PAN_ALT_MODIFIER: f32 = -0.3; // slows it Down
const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 100.0;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_camera))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(zoom_system)
                    .with_system(move_camera),
            );
    }
}

#[derive(Debug, Component)]
pub struct Agent {
    pub name: String,
    lifespan: i64,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn move_camera(
    time: Res<Time>,
    actions: Res<Actions>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    if actions.camera_movement.is_none() {
        return;
    }
    let speed = 1500.0;
    let movement = Vec3::new(
        actions.camera_movement.unwrap().x * speed * time.delta_seconds(),
        actions.camera_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut camera_transform in camera_query.iter_mut() {
        camera_transform.translation += movement;
    }
}

fn zoom_system(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_button: ResMut<Input<MouseButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    windows: Res<Windows>,
) {
    let mut pan = Vec2::ZERO;

    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    if mouse_button.pressed(MouseButton::Middle) {
        for ev in mouse_motion.iter() {
            pan += ev.delta;
        }
    }
    let delta_zoom: f32 = mouse_wheel.iter().map(|scroll| scroll.y).sum();

    if delta_zoom == 0.0 && pan == Vec2::ZERO {
        return;
    }
    let (mut pos, mut cam): (Mut<Transform>, Mut<OrthographicProjection>) = cam.single_mut();

    let mouse_normalized_screen_pos =
        (window.cursor_position().unwrap() / window_size) * 2. - Vec2::ONE;
    let mouse_world_pos = pos.translation.truncate()
        + mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale;

    cam.scale -= ZOOM_SPEED * delta_zoom * cam.scale;
    cam.scale = cam.scale.clamp(MIN_ZOOM, MAX_ZOOM);

    pos.translation = (mouse_world_pos
        - mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale)
        .extend(pos.translation.z);

    let mut pan_modifier = 0.0;

    if keyboard_input.pressed(KeyCode::LShift) {
        pan_modifier = PAN_SHIFT_MODIFIER;
    } else if keyboard_input.pressed(KeyCode::LAlt) {
        pan_modifier = PAN_ALT_MODIFIER;
    }

    pan *= cam.scale * (PAN_SPEED + pan_modifier);


    pos.translation.x -= pan.x;
    pos.translation.y += pan.y;
}
