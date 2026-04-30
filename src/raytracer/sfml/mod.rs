mod controls;
mod display;

use ::sfml::cpp::FBox;
use ::sfml::graphics::{
    Color as SfColor, Font, RenderTarget, RenderWindow, Sprite, Text, Transformable,
};
use ::sfml::system::{Vector2f, Vector2i};
use ::sfml::window::{ContextSettings, Event, Key, Style, VideoMode};
use std::time::Instant;

use crate::rendering::Renderer;

static FONT_BYTES: &[u8] = include_bytes!("../../../assets/Roboto-Bold.ttf");

pub struct SfmlPreview {
    renderer: Renderer,
}

impl SfmlPreview {
    pub fn new(renderer: Renderer) -> Self {
        Self { renderer }
    }

    pub fn run(mut self) {
        let display_w = self.renderer.scene.camera.resolution.width;
        let display_h = self.renderer.scene.camera.resolution.height;

        let render_w = (display_w / 2).max(1);
        let render_h = (display_h / 2).max(1);
        self.renderer.scene.camera.resolution.width = render_w;
        self.renderer.scene.camera.resolution.height = render_h;
        self.renderer.render();

        let mut tex_buf = display::TextureBuffer::new(render_w, render_h);
        tex_buf.update(&self.renderer.buffer);

        let scale = Vector2f::new(
            display_w as f32 / render_w as f32,
            display_h as f32 / render_h as f32,
        );
        let center = Vector2i::new(display_w as i32 / 2, display_h as i32 / 2);

        let mut window = create_window(display_w, display_h);
        let mut mouse_captured = true;
        set_capture(&mut window, center, true);

        let font = Font::from_memory_static(FONT_BYTES).expect("Police introuvable");
        let mut fps_text = Text::new("FPS: 0", &font, 18);
        fps_text.set_fill_color(SfColor::WHITE);
        fps_text.set_outline_color(SfColor::BLACK);
        fps_text.set_outline_thickness(1.0);
        fps_text.set_position(Vector2f::new(8.0, 8.0));

        let mut fps_timer = 0.0f32;
        let mut fps_frames = 0u32;
        let mut last_frame = Instant::now();

        while window.is_open() {
            let now = Instant::now();
            let dt = now.duration_since(last_frame).as_secs_f32().min(0.1);
            last_frame = now;

            fps_timer += dt;
            fps_frames += 1;
            if fps_timer >= 0.5 {
                let fps = fps_frames as f32 / fps_timer;
                fps_text.set_string(&format!("{:.0} FPS", fps));
                fps_timer = 0.0;
                fps_frames = 0;
            }

            while let Some(evt) = window.poll_event() {
                match evt {
                    Event::Closed => window.close(),
                    Event::KeyPressed {
                        code: Key::Escape, ..
                    } => {
                        mouse_captured = !mouse_captured;
                        set_capture(&mut window, center, mouse_captured);
                    }
                    Event::MouseButtonPressed { .. } if !mouse_captured => {
                        mouse_captured = true;
                        set_capture(&mut window, center, true);
                    }
                    Event::LostFocus => {
                        mouse_captured = false;
                        set_capture(&mut window, center, false);
                    }
                    _ => {}
                }
            }

            let (dx, dy) = mouse_delta(&mut window, center, mouse_captured);
            let cam = &mut self.renderer.scene.camera;

            controls::apply_keyboard(cam, dt);
            controls::apply_mouse(cam, dx, dy);
            self.renderer.render();
            tex_buf.update(&self.renderer.buffer);
            let mut sprite = Sprite::with_texture(tex_buf.texture());
            sprite.set_scale(scale);
            window.clear(SfColor::BLACK);
            window.draw(&sprite);
            window.draw(&fps_text);
            window.display();
        }
    }
}

fn create_window(w: u32, h: u32) -> FBox<RenderWindow> {
    let mut window = RenderWindow::new(
        VideoMode::new(w, h, 32),
        "Raytracer Preview",
        Style::DEFAULT,
        &ContextSettings::default(),
    )
    .expect("Unable to create window");
    window.set_framerate_limit(60);

    window
}

fn set_capture(window: &mut RenderWindow, center: Vector2i, captured: bool) {
    window.set_mouse_cursor_visible(!captured);
    window.set_mouse_cursor_grabbed(captured);
    if captured {
        window.set_mouse_position(center);
    }
}

fn mouse_delta(window: &mut RenderWindow, center: Vector2i, captured: bool) -> (f32, f32) {
    if !captured {
        return (0.0, 0.0);
    }
    let pos = window.mouse_position();
    window.set_mouse_position(center);
    ((pos.x - center.x) as f32, (pos.y - center.y) as f32)
}
