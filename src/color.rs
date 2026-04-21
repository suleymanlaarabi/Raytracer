pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        return Color { r, g, b };
    }
}
