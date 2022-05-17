use bevy::{
    prelude::{Assets, Handle, Image},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use glam::UVec2;

use super::draw::ImageDraw;

pub struct MapView {
    handle: Handle<Image>,
}

impl MapView {
    pub fn init(size: UVec2, assets: &mut Assets<Image>) -> Self {
        let mut image = Image::default();
        image.texture_descriptor.format = TextureFormat::Rgba8Unorm;
        image.texture_descriptor.dimension = TextureDimension::D2;
        image.texture_descriptor.size = Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        };

        let num_bytes = 4 * (size.x as usize) * (size.y as usize);
        image.data = Vec::with_capacity(num_bytes);
        for _ in 0..num_bytes {
            image.data.push(255);
        }

        Self {
            handle: assets.add(image),
        }
    }

    pub fn get_draw<'a>(&self, assets: &'a mut Assets<Image>) -> Option<ImageDraw<'a>> {
        assets.get_mut(&self.handle).map(ImageDraw::new)
    }

    pub fn image_handle(&self) -> Handle<Image> {
        self.handle.clone()
    }
}
