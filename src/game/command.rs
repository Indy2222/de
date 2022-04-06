use super::{
    movement::SendFlockEvent, objects::Movable, pointer::Pointer, selection::Selected, Labels,
};
use bevy::{
    input::mouse::MouseButtonInput,
    prelude::{
        App, Entity, EventReader, EventWriter, MouseButton, ParallelSystemDescriptorCoercion,
        Plugin, Query, Res, SystemSet, With,
    },
};
use glam::Vec2;

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new().with_system(
                mouse_click_handler
                    .label(Labels::InputUpdate)
                    .after(Labels::PreInputUpdate),
            ),
        );
    }
}

fn mouse_click_handler(
    mut click_events: EventReader<MouseButtonInput>,
    mut send_flock_events: EventWriter<SendFlockEvent>,
    selected: Query<Entity, (With<Selected>, With<Movable>)>,
    pointer: Res<Pointer>,
) {
    if !click_events.iter().any(|e| e.button == MouseButton::Right) {
        return;
    }

    let target = match pointer.terrain_point() {
        Some(point) => Vec2::new(point.x, point.z),
        None => return,
    };

    let selected_entities: Vec<Entity> = selected.iter().collect();
    send_flock_events.send(SendFlockEvent::new(selected_entities, target));
}
