use crate::agent::Agent;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiStates::default())
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(render_ui));
    }
}

#[derive(Default)]
/// `UiStates` is a struct that contains all the entity's that
/// have menus that need to be rendered
///
/// Properties:
///
/// * `agents`: A vector of entities that are agents. these are rendered by egui.
pub struct UiStates {
    pub(crate) agents: Vec<Entity>,
}

/// `render_ui` renders the user interface for the selected agents
///
/// Arguments:
///
/// * `ui_states`: ResMut<UiStates> - resource containing a list of entities that are being rendered in the user interface.
/// * `agents`: Query<(&mut Agent, &mut Transform)> - query containing agents and their transforms.
/// * `egui_context`: ResMut<EguiContext> - resource containing the context for the Egui user interface.
fn render_ui(
    ui_states: ResMut<UiStates>,
    mut agents: Query<(&mut Agent, &mut Transform)>,
    mut egui_context: ResMut<EguiContext>,
) {
    for entity in ui_states.agents.iter() {
        egui::Window::new("Agent Info").show(egui_context.ctx_mut(), |ui| {
            let (mut agent, mut agent_transform): (Mut<Agent>, Mut<Transform>) =
                agents.get_mut(*entity).unwrap();

            ui.strong(format!("agent {}", agent.name));

            ui.collapsing("position", |ui| {
                ui.label(format!(
                    "Location {:.2},{:.2}",
                    agent_transform.translation.x, agent_transform.translation.y
                ));
                ui.add(
                    egui::DragValue::new::<f32>(&mut agent_transform.translation.x).prefix("X:"),
                );
                ui.add(
                    egui::DragValue::new::<f32>(&mut agent_transform.translation.y).prefix("Y:"),
                );

                if ui.button("Reset position").clicked() {
                    agent_transform.translation = Vec3::new(1.0, 1.0, 1.0);
                }
            });

            ui.collapsing("agent", |ui| {
                let mut destination_toggled = agent.destination.is_some();
                let _destination_toggle =
                    ui.checkbox(&mut destination_toggled, "have a destination?");

                if agent.destination.is_none() && destination_toggled {
                    agent.destination = Some(Vec2::new(10.0, 10.0));
                }
                if let Some(mut destination) = agent.destination {
                    if !destination_toggled {
                        agent.destination = None;
                    } else {
                        ui.collapsing("destination", |ui| {
                            ui.label(format!(
                                "Location {:.2},{:.2}",
                                destination.x, destination.y
                            ));
                            ui.add(egui::DragValue::new::<f32>(&mut destination.x).prefix("Y:"));
                            ui.add(egui::DragValue::new::<f32>(&mut destination.y).prefix("Y:"));

                            agent.destination = Some(destination);

                            if ui.button("Reset destination").clicked() {
                                agent.destination = Some(Vec2::new(10.0, 10.0));
                            }
                        });
                    }
                }
            });
        });
    }
}
