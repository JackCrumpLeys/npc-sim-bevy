use crate::actions::Actions;

use crate::GameState;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::{Camera2d, RenderTarget};
use itertools::Itertools;

pub struct CameraPlugin;

const ZOOM_SPEED: f32 = 0.5;
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
    mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    windows: Res<Windows>,
) {
    let mut pan = Vec2::ZERO;

    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    if mouse_button.pressed(MouseButton::Middle) {
        for ev in mouse_motion.iter() {
            // // get the camera info and transform
            // // assuming there is exactly one main camera entity, so query::single() is OK
            // let (camera, camera_transform) = camera_query.single();
            //
            // // get the window that the camera is displaying to (or the primary window)
            // let wnd = if let RenderTarget::Window(id) = camera.target {
            //     windows.get(id).unwrap()
            // } else {
            //     windows.get_primary().unwrap()
            // };
            //
            // // check if the cursor is inside the window and get its position
            // let screen_pos = ev.delta;
            // // get the size of the window
            // let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            //
            // // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            // let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            //
            // // matrix for undoing the projection and camera transform
            // let ndc_to_world =
            //     camera_transform.compute_matrix() * camera.projection_matrix.inverse();
            //
            // // use it to convert ndc to world-space coordinates
            // let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            //
            // // reduce it to a 2D value
            // let world_pos: Vec2 = world_pos.truncate();
            //
            // pan += world_pos;
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

    pan *= cam.scale;

    pos.translation.x -= pan.x;
    pos.translation.y += pan.y;
}
