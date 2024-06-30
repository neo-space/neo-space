use macroquad::prelude::*;

mod camera;
use camera::Camera;

mod grid;
use grid::draw_grid;
use scrollbar::draw_scrollbar;

mod scrollbar;

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 8.0;
const ZOOM_SPEED: f32 = 0.05;
const SCROLL_SPEED: f32 = 5.0; // Reduced scroll speed
const BACKGROUND_COLOR: Color = Color::new(0.98, 0.98, 0.98, 1.0);

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera::new();
    let mut is_dragging = false;
    let mut last_mouse_pos = Vec2::ZERO;

    loop {
        clear_background(BACKGROUND_COLOR);

        //TODO: refactor make part of a larger list of MODES based on what action user is intending
        // Handle panning with left mouse button
        if is_mouse_button_down(MouseButton::Left) {
            if !is_dragging {
                is_dragging = true;
                last_mouse_pos = mouse_position().into();
            }
            let current_mouse_pos: Vec2 = mouse_position().into();
            let delta = (current_mouse_pos - last_mouse_pos) / camera.zoom;
            camera.position -= delta;
            last_mouse_pos = current_mouse_pos;
        } else {
            is_dragging = false;
        }

        // TODO: refactor make zooming and scrolling part of camera
        // Handle scrolling and zooming
        let (wheel_x, wheel_y) = mouse_wheel();
        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            // Zoom with Ctrl + scroll
            if wheel_y != 0.0 {
                let zoom_factor = 1.0 + (wheel_y * ZOOM_SPEED);
                let new_zoom = (camera.zoom * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);
                let zoom_change = new_zoom / camera.zoom;
                camera.zoom = new_zoom;

                let mouse_pos: Vec2 = mouse_position().into();
                let before = camera.screen_to_world(mouse_pos);
                let after = camera.screen_to_world(mouse_pos);
                camera.position += before - after;
            }
        } else {
            // Scroll vertically and horizontally without Ctrl
            camera.position.y += wheel_y * SCROLL_SPEED / camera.zoom;
            camera.position.x += wheel_x * SCROLL_SPEED / camera.zoom;
        }

        draw_grid(&camera);
        draw_scrollbar(&camera);

        // TODO: refactor to display stat bar
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

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Infinite Canvas".to_owned(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}
