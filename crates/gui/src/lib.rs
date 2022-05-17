use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, TextureDimension, TextureFormat,
        },
        renderer::RenderQueue,
        texture::CompressedImageFormats,
        RenderApp, RenderStage,
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
        app.add_enter_system(GameState::Playing, setup_map_view);
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_system_to_stage(RenderStage::Prepare, game_ui.run_if_resource_exists::<X>());
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
    //time: Res<Time>,
    // mut events_a: EventWriter<AssetEvent<Image>>,
    // mut events_b: EventWriter<AssetEvent<ColorMaterial>>,
    mut queue: ResMut<RenderQueue>,
    rassets: Res<RenderAssets<Image>>,
) {
    println!("Runningslkhflksjdflkjsdfe");

    let green = 50;

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

    let data: Vec<u8> = draw.data().iter().cloned().collect();

    // events_b.send(AssetEvent::Modified {
    //     handle: x.0.clone(),
    // });

    let gpu_image = rassets.get(&image.image_handle()).unwrap();

    let img2 = images.get(image.image_handle()).unwrap();
    queue.write_texture(
        ImageCopyTexture {
            texture: &gpu_image.texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &data,
        ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(
                std::num::NonZeroU32::new(img2.texture_descriptor.size.width * 4).unwrap(),
            ),
            rows_per_image: if img2.texture_descriptor.size.depth_or_array_layers > 1 {
                std::num::NonZeroU32::new(img2.texture_descriptor.size.height)
            } else {
                None
            },
        },
        img2.texture_descriptor.size,
    );
}
