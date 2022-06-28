use bevy::prelude::*;
use crate::agent::Agent;
use bevy_egui::{egui, EguiContext};
use egui::{Rgba, RichText};

struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiStates>();
    }
}

#[derive(Default)]
struct UiStates {
    agents: Vec<Entity>,
}

//render windows using egui for every agent in the Ui states
fn render_ui(mut ui_states: ResMut<UiStates>,
             mut agents: Query<&Agent>,
             mut egui_context: ResMut<EguiContext>,
             mut commands: Commands) {
    let mut agents_to_remove = Vec::new();
    for (agent, transform, sprite) in (&mut agents, &mut agents.transform(), &mut agents.sprite()).iter() {
        let mut agent_ui = egui_context.agent_ui_mut(agent);
        agent_ui.set_position(transform.translation.x, transform.translation.y);
        agent_ui.set_size(sprite.width(), sprite.height());
        agent_ui.set_color(Rgba::new(1.0, 1.0, 1.0, 1.0));
        agent_ui.set_text(RichText::new(format!("{}", agent.name)));
        agent_ui.set_visible(true);
        if agent_ui.is_clicked() {
            agents_to_remove.push(agent);
        }
    }
    for agent in agents_to_remove {
        agents.remove_entity(agent);
    }
}
