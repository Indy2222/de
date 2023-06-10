mod collider;
mod marker;
mod plugin;
mod shader;
mod terrain;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use collider::TerrainCollider;
use marker::MarkerPlugin;
pub use marker::{CircleMarker, MarkerVisibility, RectangleMarker};
use plugin::TerrainPlugin;
pub use terrain::TerrainBundle;

pub const MAX_ELEVATION: f32 = 1024.;

pub struct TerrainPluginGroup;

impl PluginGroup for TerrainPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TerrainPlugin)
            .add(MarkerPlugin)
    }
}
