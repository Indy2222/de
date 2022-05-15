use bevy::{app::PluginGroupBuilder, prelude::*, transform::TransformSystem};
use bevy_egui::{
    egui::{epaint::RectShape, Color32, Frame, Painter, Pos2, Rect, Rounding, Sense, Vec2, Window},
    EguiContext, EguiPlugin,
};
use de_core::{objects::SolidObject, state::GameState};
use de_map::size::MapBounds;
use iyes_loopless::prelude::*;

// TODO: disallow clicks through UI

const MAP_VIEW_SIZE: f32 = 0.2;

pub struct GuiPluginGroup;

impl PluginGroup for GuiPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(EguiPlugin).add(GuiPlugin);
    }
}

struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, setup_game_ui)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                game_ui
                    .run_in_state(GameState::Playing)
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

struct ViewTransform {
    px_rect: Rect,
    m_origin: glam::Vec2,
    m_to_px: f32,
}

impl ViewTransform {
    fn new(bounds: &MapBounds, screen: Rect, max_screen_fraction: f32) -> Self {
        let map_size_m = bounds.max() - bounds.min();
        let map_size_m = Vec2::new(map_size_m.x, map_size_m.y);
        let max_size_px = max_screen_fraction * screen.size();
        let m_to_px = (max_size_px / map_size_m).min_elem();
        let px_origin = screen.max - m_to_px * map_size_m;
        let px_rect = Rect {
            min: Pos2::new(px_origin.x, px_origin.y),
            max: screen.max,
        };

        Self {
            px_rect,
            m_origin: bounds.min(),
            m_to_px,
        }
    }

    fn px_rect(&self) -> Rect {
        self.px_rect
    }

    fn px_to_meters(&self, pos: Pos2) -> glam::Vec2 {
        let px_offset = pos - self.px_rect.min;
        self.m_origin + glam::Vec2::new(px_offset.x, px_offset.y) / self.m_to_px
    }

    fn meters_to_px(&self, pos: glam::Vec2) -> Pos2 {
        let px_offset = self.m_to_px * (pos - self.m_origin);
        self.px_rect.min + Vec2::new(px_offset.x, px_offset.y)
    }
}

fn setup_game_ui(mut commands: Commands, bounds: Res<MapBounds>, mut ctx: ResMut<EguiContext>) {
    commands.insert_resource(ViewTransform::new(
        bounds.as_ref(),
        ctx.ctx_mut().available_rect(),
        MAP_VIEW_SIZE,
    ));
}

fn game_ui(
    mut ctx: ResMut<EguiContext>,
    view_transform: Res<ViewTransform>,
    objects: Query<&GlobalTransform, With<SolidObject>>,
) {
    Window::new("map-view")
        .fixed_rect(view_transform.px_rect())
        .title_bar(false)
        .frame(Frame::none())
        .show(ctx.ctx_mut(), |ui| {
            Frame::none().fill(Color32::BLACK).show(ui, |ui| {
                let view_size = view_transform.px_rect().size();
                let (_response, painter) = ui.allocate_painter(view_size, Sense::click());
                draw_objects(painter, view_transform.as_ref(), &objects);
            });
        });
}

fn draw_objects(
    painter: Painter,
    view_transform: &ViewTransform,
    objects: &Query<&GlobalTransform, With<SolidObject>>,
) {
    for transform in objects.iter() {
        let pos_m = glam::Vec2::new(transform.translation.x, transform.translation.z);
        let px_pox = view_transform.meters_to_px(pos_m);
        let rect = Rect {
            min: px_pox - Vec2::new(1., 1.),
            max: px_pox + Vec2::new(1., 1.),
        };
        let shape = RectShape::filled(rect, Rounding::none(), Color32::BLUE);
        painter.add(shape);
    }
}
