use std::collections::HashMap;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::GameState;
use crate::windows::UiStates;

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AgentZoneMapping::default()).add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_test_zone));
    }
}

#[derive(Debug, Component, Clone, PartialEq)]
pub struct Zone {
    pub name: String,
    pub height: f32,
    pub width: f32,
}

#[derive(Default, Debug)]
pub struct AgentZoneMapping {
    map: HashMap<Entity, Vec<Entity>>
}

fn spawn_test_zone(mut commands: Commands, mut zones: ResMut<AgentZoneMapping>) {
    let zone: Zone = Zone {name: "TEST".parse().unwrap(), height:1000.0, width:1000.0};
    let shape = shapes::Rectangle {
        extents: Vec2::new(zone.width,zone.height),
        origin: RectangleOrigin::Center
    };

    let zone_entity = commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsla(rand::random::<f32>() * 360.0, 1.0, 0.5, 0.3)),
            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
        },
        Transform::default(),
    ))
        .insert(zone).id();

    (*zones).map.insert(zone_entity, vec![]);
}