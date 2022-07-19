use crate::agent::Agent;

use crate::GameState;
use bevy::prelude::*;

use bevy_prototype_lyon::prelude::*;

use itertools::Itertools;
use std::collections::HashMap;

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AgentZoneMapping::default())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_test_zone))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_zones));
    }
}

#[derive(Debug, Component, Clone, PartialEq)]
/// `Zone` is a struct that contains a `name` field of type `String`, a `height` field of type `f32`,
/// and a `width` field of type `f32`. this stores a rectangle shaped zone.
///
/// Properties:
///
/// * `name`: The name of the zone.
/// * `height`: The height of the zone.
/// * `width`: The width of the zone.
pub struct Zone {
    pub name: String,
    pub height: f32,
    pub width: f32,
}

#[derive(Default, Debug, Clone)]
/// `AgentZoneMapping` is a `HashMap` that maps an `Entity` to a `Vec` of `Entity`s.
///
/// Properties:
///
/// * `map`: This is a HashMap that maps an zones to a list of agents.
pub struct AgentZoneMapping {
    map: HashMap<Entity, Vec<Entity>>,
}

/// For each zone, check if any agents are in it. If they are, add them to the zone's list of agents
///
/// Arguments:
///
/// * `zones`: Query<(Entity, &Zone)> - get all zone entities.
/// * `agents`: Query<Entity, With<Agent>> - get all agent entities.
/// * `transform_q`: Query<&Transform>  - transform to query with entities
/// * `zone_mapping`: This is the resource that we created earlier.
fn update_zones(
    zones: Query<(Entity, &Zone)>,
    agents: Query<Entity, With<Agent>>,
    transform_q: Query<&Transform>,
    mut zone_mapping: ResMut<AgentZoneMapping>,
) {
    let zone_mappings = &mut zone_mapping.map;
    let zone_list: Vec<Entity> = zones.iter().map(|(e, _)| e).collect_vec();
    let agent_with_transform: Vec<(Entity, &Transform)> = agents
        .iter()
        .map(|e| (e, transform_q.get(e).unwrap()))
        .collect_vec();

    for entity in &zone_list {
        zone_mappings.entry(*entity).or_insert(vec![]);
    }

    for (zone_entity, _agents) in zone_mappings.clone().iter() {
        if !zone_list.contains(zone_entity) {
            zone_mappings.remove(zone_entity);
            continue;
        }
    }

    for (zone_entity, zone) in zones.iter() {
        for (agent_entity, agent_transform) in &agent_with_transform {
            let zone_transform = transform_q.get(zone_entity).unwrap();

            let scale_x: f32 = zone.height;
            let scale_y: f32 = zone.width;
            let pos_x: f32 = agent_transform.translation.x;
            let pos_y: f32 = agent_transform.translation.y;

            if zone_transform.translation.x >= pos_x - scale_x / 2.0
                && zone_transform.translation.x <= pos_x + scale_x / 2.0
                && zone_transform.translation.y >= pos_y - scale_y / 2.0
                && zone_transform.translation.y <= pos_y + scale_y / 2.0
            {
                println!("{:?} is in {:?}", agent_entity, zone.name);
                zone_mappings
                    .get_mut(&zone_entity)
                    .unwrap()
                    .push(*agent_entity)
            }
        }
    }
}

/// It creates a zone, spawns it, and adds it to the `AgentZoneMapping` resource
///
/// Arguments:
///
/// * `commands`: Commands - This is the command buffer that we use to spawn entities.
/// * `zones`: ResMut<AgentZoneMapping>
fn spawn_test_zone(mut commands: Commands, mut zones: ResMut<AgentZoneMapping>) {
    let zone: Zone = Zone {
        name: "TEST".parse().unwrap(),
        height: 1000.0,
        width: 1000.0,
    };
    let shape = shapes::Rectangle {
        extents: Vec2::new(zone.width, zone.height),
        origin: RectangleOrigin::Center,
    };

    let zone_entity = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::hsla(
                    rand::random::<f32>() * 360.0,
                    1.0,
                    0.5,
                    0.3,
                )),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::default(),
        ))
        .insert(zone)
        .id();

    (*zones).map.insert(zone_entity, vec![]);
}
