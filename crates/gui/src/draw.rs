use bevy::{
    prelude::{Assets, Handle, Image},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use glam::UVec2;
use rgb::RGBA8;

pub struct ImageDraw<'a> {
    image: &'a mut Image,
    size: UVec2,
}

impl<'a> ImageDraw<'a> {
    pub fn new(image: &'a mut Image) -> Self {
        let size = image.texture_descriptor.size;
        Self {
            image,
            size: UVec2::new(size.width, size.height),
        }
    }

    pub fn set_pixel(&mut self, coords: UVec2, color: RGBA8) {
        let index = 4 * (self.size.x as usize * coords.y as usize + coords.x as usize);
        self.image.data[index] = color.r;
        self.image.data[index + 1] = color.g;
        self.image.data[index + 2] = color.b;
        self.image.data[index + 3] = color.a;
    }
}
