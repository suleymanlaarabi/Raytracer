use ::sfml::window::Key;

use crate::camera::Camera;

const MOVE_SPEED: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.002;

pub fn apply_keyboard(cam: &mut Camera, dt: f32) -> bool {
    let basis = cam.basis();
    let mut moved = false;
    let mv = MOVE_SPEED * dt;

    if Key::Z.is_pressed() {
        cam.position += basis.forward * mv;
        moved = true;
    }
    if Key::S.is_pressed() {
        cam.position += -basis.forward * mv;
        moved = true;
    }
    if Key::Q.is_pressed() {
        cam.position += -basis.right * mv;
        moved = true;
    }
    if Key::D.is_pressed() {
        cam.position += basis.right * mv;
        moved = true;
    }
    if Key::Space.is_pressed() {
        cam.position.y += mv;
        moved = true;
    }
    if Key::LShift.is_pressed() {
        cam.position.y -= mv;
        moved = true;
    }

    moved
}

pub fn apply_mouse(cam: &mut Camera, dx: f32, dy: f32) -> bool {
    if dx == 0.0 && dy == 0.0 {
        return false;
    }
    cam.rotation.y += dx * MOUSE_SENSITIVITY;
    cam.rotation.x = (cam.rotation.x + dy * MOUSE_SENSITIVITY).clamp(
        -std::f32::consts::FRAC_PI_2 + 0.01,
        std::f32::consts::FRAC_PI_2 - 0.01,
    );
    true
}
