use area::AreaPlugin;
pub(crate) use area::{AreaSelectLabels, SelectInRectEvent};
use bevy::prelude::*;
use bookkeeping::BookkeepingPlugin;
pub(crate) use bookkeeping::{SelectEvent, Selected, SelectionLabels, SelectionMode};

mod area;
mod bookkeeping;

pub(crate) struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BookkeepingPlugin).add_plugin(AreaPlugin);
    }
}
