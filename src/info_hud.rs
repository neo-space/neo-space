use macroquad::{color::BLACK, text::draw_text};

use crate::camera::Camera;


pub fn display_hud(camera: &Camera) {
    draw_text(
        &format!(
            "Camera: ({:.2}, {:.2}), Zoom: {:.2}",
            camera.position.x, camera.position.y, camera.zoom
        ),
        10.0,
        20.0,
        20.0,
        BLACK,
    );
}