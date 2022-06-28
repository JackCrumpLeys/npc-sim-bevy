use crate::agent::Agent;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use egui::{Rgba, RichText};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiStates::default())
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(render_ui));
    }
}

#[derive(Default)]
pub struct UiStates {
    pub(crate) agents: Vec<Entity>,
}

//render windows using egui for every agent in the Ui states
fn render_ui(
    ui_states: Res<UiStates>,
    mut agents: Query<(&Agent, &mut Transform)>,
    mut egui_context: ResMut<EguiContext>,
) {
    ui_states.agents.drain(..).collect::<Entity>();
    for entity in ui_states.agents.iter() {
        println!("{:?}", entity);
        egui::Window::new("Player Diagnostics").show(egui_context.ctx_mut(), |ui| {
            let (agent, mut agent_transform) = agents.get_mut(*entity).unwrap();

            ui.strong(format!("agent {}", agent.name));
            ui.label(format!(
                "Location {:.2},{:.2}",
                agent_transform.translation.x, agent_transform.translation.y
            ));
            ui.add(
                egui::Slider::new::<f32>(&mut agent_transform.translation.x, -50.0..=50.0)
                    .text("Translation X"),
            );
            ui.add(
                egui::Slider::new::<f32>(&mut agent_transform.translation.y, -50.0..=50.0)
                    .text("Translation Y"),
            );

            if ui.button("Reset position").clicked() {
                agent_transform.translation = Vec3::default();
            }
        });
    }
}
