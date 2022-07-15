use std::ops::DerefMut;

use crate::loading::TextureAssets;
use crate::windows::UiStates;
use crate::GameState;

use bevy::prelude::*;
use bevy::render::camera::{Camera2d, RenderTarget};
use bevy_egui::EguiContext;
use bevy_prototype_lyon::draw::{DrawMode, FillMode, StrokeMode};
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_agent))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(click_agent))
            .insert_resource(Msaa { samples: 4 })
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_agent));
    }
}

#[derive(Debug, Component)]
pub struct Agent {
    pub name: String,
    pub destination: Option<Vec2>,
}

fn spawn_agent(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(1., 1., 1.)),
            ..Default::default()
        })
        .insert(Agent {
            name: "john".to_string(),
            destination: None,
        });
}

#[derive(Debug, Component)]
pub struct DestinationMarker;

fn update_agent(
    mut agent_query: Query<(&mut Agent, Entity)>,
    destination_visual_query: Query<Entity, With<DestinationMarker>>,
    mut transform_q: Query<&mut Transform>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let mut valid_dests: Vec<Entity> = Vec::new();

    for (agent, entity) in agent_query.iter_mut() {
        let mut transform: Mut<Transform> = transform_q.get_mut(entity).unwrap();
        let mut agent: Mut<Agent> = agent;

        if let Some(destination) = agent.destination {
            // mu life is broken
            let diff = destination - transform.translation.truncate();
            let angle = diff.y.atan2(diff.x);
            transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);

            let move_dir = transform.local_x() * 1000.0 * time.delta_seconds();
            transform.translation += move_dir;

            let scale_x: f32 = 20.0;
            let scale_y: f32 = 20.0;
            let pos_x: f32 = transform.translation.x;
            let pos_y: f32 = transform.translation.y;

            // if the destination is within the bounds of the agent then print the agent name
            if destination.x >= pos_x - scale_x / 2.0
                && destination.x <= pos_x + scale_x / 2.0
                && destination.y >= pos_y - scale_y / 2.0
                && destination.y <= pos_y + scale_y / 2.0
            {
                agent.destination = None;
            }

            let destination_visual: Option<(&Transform, Entity)> = destination_visual_query
                .iter()
                .map(|entity| (transform_q.get(entity).unwrap(), entity))
                .find(|(t, _)| {
                    t.translation.y == destination.y && t.translation.x == destination.x
                });

            if destination_visual.is_none() {
                let shape = shapes::RegularPolygon {
                    sides: 6,
                    feature: shapes::RegularPolygonFeature::Radius(200.0),
                    ..shapes::RegularPolygon::default()
                };

                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(Color::CYAN),
                            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
                        },
                        Transform::from_translation(destination.extend(0.0)),
                    ))
                    .insert(DestinationMarker);
            } else if let Some((_, entity)) = destination_visual {
                valid_dests.push(entity);
            }
        }
    }
    for dest in destination_visual_query.iter() {
        if !valid_dests.contains(&dest) {
            commands.entity(dest).despawn();
        }
    }
}

fn click_agent(
    mut agent_query: Query<(&mut Agent, &Transform, &Sprite, Entity)>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut ui_states: ResMut<UiStates>,
    egui_context: Res<EguiContext>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(_cursor_pos) = win.cursor_position() {
            // println!("click at {:?}", cursor_pos);

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

                // println!("World coords: {}/{}", world_pos.x, world_pos.y);

                for (agent, transform, sprite, entity) in agent_query.iter_mut() {
                    // println!("{:?}", agent)
                    let mut agent: Mut<Agent> = agent;
                    let transform: &Transform = transform;
                    let _sprite: &Sprite = sprite;

                    // println!("{:?}", transform);
                    if !egui_context.ctx().wants_pointer_input() {
                        agent.destination = Some(world_pos);
                    }

                    let scale_x: f32 = 300.0;
                    let scale_y: f32 = 300.0;
                    let pos_x: f32 = transform.translation.x;
                    let pos_y: f32 = transform.translation.y;
                    if world_pos.x >= pos_x - scale_x / 2.0
                        && world_pos.x <= pos_x + scale_x / 2.0
                        && world_pos.y >= pos_y - scale_y / 2.0
                        && world_pos.y <= pos_y + scale_y / 2.0
                        && !ui_states.agents.contains(&entity)
                    {
                        ui_states.deref_mut().agents.push(entity);
                    }
                }
            }
        }
    }
}
