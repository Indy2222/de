use bevy::{prelude::*, window::WindowMode};
use de_game::GamePluginGroup;
use de_gui::GuiPluginGroup;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Digital Extinction".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePluginGroup)
        .add_plugins(GuiPluginGroup)
        .run();
}
