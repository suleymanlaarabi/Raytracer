use ::sfml::cpp::FBox;
use ::sfml::graphics::Texture;

use crate::rendering::color::Color;

pub struct TextureBuffer {
    texture: FBox<Texture>,
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

impl TextureBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut texture = Texture::new().expect("Unable to create texture");
        texture
            .create(width, height)
            .expect("Unable to create texture with given dimensions");
        texture.set_smooth(true);
        Self {
            texture,
            pixels: vec![0u8; (width * height * 4) as usize],
            width,
            height,
        }
    }

    pub fn update(&mut self, buffer: &[Color]) {
        for (i, color) in buffer.iter().enumerate() {
            let base = i * 4;
            self.pixels[base] = color.r;
            self.pixels[base + 1] = color.g;
            self.pixels[base + 2] = color.b;
            self.pixels[base + 3] = 255;
        }
        self.texture
            .update_from_pixels(&self.pixels, self.width, self.height, 0, 0);
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}
