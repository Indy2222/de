use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::CompressedImageFormats,
    },
    sprite::MaterialMesh2dBundle,
    transform::TransformSystem,
};
use bevy_egui::{
    egui::{epaint::RectShape, Color32, Frame, Painter, Pos2, Rect, Rounding, Sense, Vec2, Window},
    EguiContext, EguiPlugin,
};
use de_core::{objects::SolidObject, state::GameState};
use de_map::size::MapBounds;
use draw::ImageDraw;
use iyes_loopless::prelude::*;
use mapview::MapView;
use rgb::{RGBA, RGBA8};

mod draw;
mod mapview;

// TODO: disallow clicks through UI

const MAP_VIEW_SIZE: f32 = 0.2;

pub struct GuiPluginGroup;

impl PluginGroup for GuiPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(GuiPlugin);
    }
}

struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, setup_map_view)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                game_ui
                    .run_in_state(GameState::Playing)
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

struct X(Handle<ColorMaterial>);

fn setup_map_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let map_view = MapView::init(UVec2::new(200, 100), images.as_mut());

    let material = materials.add(ColorMaterial {
        color: Color::WHITE,
        texture: Some(map_view.image_handle()),
    });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::new(200., 100., 0.)),
        material: material.clone(),
        ..default()
    });

    commands.insert_resource(map_view);
    commands.insert_resource(X(material));
}

fn game_ui(
    image: Res<MapView>,
    x: Res<X>,
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
    mut events_a: EventWriter<AssetEvent<Image>>,
    mut events_b: EventWriter<AssetEvent<ColorMaterial>>,
) {
    let green = (100. * time.seconds_since_startup()).rem_euclid(256.) as u8;

    let mut draw = image.get_draw(images.as_mut()).unwrap();

    for y in 0..100 {
        for x in 0..200 {
            if y == 10 {
                draw.set_pixel(UVec2::new(x, y), RGBA8::new(255, 0, 128, 255));
            } else {
                draw.set_pixel(UVec2::new(x, y), RGBA8::new(0, green, 0, 255));
            }
        }
    }

    events_b.send(AssetEvent::Modified {
        handle: x.0.clone(),
    });
}
