#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn from_colorf(c: ColorF) -> Color {
        Color {
            r: (c.r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (c.g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (c.b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorF {
    pub const BLACK: ColorF = ColorF::from_rgb(0.0, 0.0, 0.0);
    pub const WHITE: ColorF = ColorF::from_rgb(1.0, 1.0, 1.0);

    pub const fn from_rgb(r: f32, g: f32, b: f32) -> ColorF {
        ColorF { r, g, b }
    }

    pub fn from_u8(r: u8, g: u8, b: u8) -> ColorF {
        ColorF {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }
}
