use std::ops::DerefMut;
use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::windows::UiStates;
use crate::GameState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::{Camera2d, RenderTarget};
use bevy_egui::{egui, EguiContext};
use egui::{Rgba, RichText};

pub struct AgentPlugin;

const ZOOM_SPEED: f32 = 0.5;
const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 100.0;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_agent)
                .with_system(spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_agent)
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

fn spawn_agent(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Agent {
            name: "john".to_string(),
            lifespan: 0,
        });
}

fn update_agent(
    mut agent_query: Query<(&Agent, &Transform, &Sprite, Entity)>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut ui_states: ResMut<UiStates>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = win.cursor_position() {
            println!("click at {:?}", cursor_pos);

            // convert the cursor position to a world position
            // get the camera info and transform
            // assuming there is exactly one main camera entity, so query::single() is OK
            let (camera, camera_transform) = camera_query.single();

            // get the window that the camera is displaying to (or the primary window)
            let wnd = if let RenderTarget::Window(id) = camera.target {
                windows.get(id).unwrap()
            } else {
                windows.get_primary().unwrap()
            };

            // check if the cursor is inside the window and get its position
            if let Some(screen_pos) = wnd.cursor_position() {
                // get the size of the window
                let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

                // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

                // matrix for undoing the projection and camera transform
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix.inverse();

                // use it to convert ndc to world-space coordinates
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

                // reduce it to a 2D value
                let world_pos: Vec2 = world_pos.truncate();

                eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);

                for (agent, transform, sprite, entity) in agent_query.iter() {
                    // println!("{:?}", agent)
                    let agent: &Agent = agent;
                    let transform: &Transform = transform;
                    let sprite: &Sprite = sprite;

                    println!("{:?}", transform);

                    let scale_x: f32 = 300.0;
                    let scale_y: f32 = 300.0;
                    let pos_x: f32 = transform.translation.x;
                    let pos_y: f32 = transform.translation.y;

                    // if the cursor is within the bounds of the agent then print the agent name
                    if world_pos.x >= pos_x - scale_x / 2.0
                        && world_pos.x <= pos_x + scale_x / 2.0
                        && world_pos.y >= pos_y - scale_y / 2.0
                        && world_pos.y <= pos_y + scale_y / 2.0
                    {
                        println!("{}", agent.name);
                        ui_states.deref_mut().agents.push(entity);
                    }
                }
            }
        }
    }
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
    mut whl: EventReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    windows: Res<Windows>,
) {
    let delta_zoom: f32 = whl.iter().map(|e| e.y).sum();
    if delta_zoom == 0. {
        return;
    }

    let (mut pos, mut cam) = cam.single_mut();

    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    let mouse_normalized_screen_pos =
        (window.cursor_position().unwrap() / window_size) * 2. - Vec2::ONE;
    let mouse_world_pos = pos.translation.truncate()
        + mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale;

    cam.scale -= ZOOM_SPEED * delta_zoom * cam.scale;
    cam.scale = cam.scale.clamp(MIN_ZOOM, MAX_ZOOM);

    pos.translation = (mouse_world_pos
        - mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale)
        .extend(pos.translation.z);
}
