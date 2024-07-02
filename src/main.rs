use macroquad::prelude::*;

mod camera;
use camera::Camera;

mod grid;
use grid::draw_grid;
use scrollbar::{draw_scrollbar, ScrollBarConfig};
use user_action_mode::UserActionMode;

mod scrollbar;
mod user_action_mode;

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 8.0;
const ZOOM_SPEED: f32 = 0.05;
const SCROLL_SPEED: f32 = 2.0;
const BACKGROUND_COLOR: Color = Color::new(0.95, 0.96, 0.98, 1.0);


struct CanvasState {
    is_dragging: bool,
    last_mouse_position: Vec2,
}

struct Cursors {
    hand: Texture2D,
    grab: Texture2D,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera::new();
    let scroll_bar_config: ScrollBarConfig = ScrollBarConfig::new();
    let mut canvas_state = CanvasState {
        is_dragging: false,
        last_mouse_position: Vec2::ZERO,
    };
    let current_user_action_mode = UserActionMode::DRAG;

    // Load cursor images
    let cursors = Cursors {
        hand: load_texture("assets/hand_cursor.png").await.unwrap(),
        grab: load_texture("assets/grab_cursor.png").await.unwrap(),
    };

    //TODO: hide cursor icon and drawing

    loop {
        clear_background(BACKGROUND_COLOR);
        draw_grid(&camera);
        handle_scroll(&mouse_wheel(), &mut camera);
        draw_scrollbar(&scroll_bar_config, &camera);

        canvas_state = observe_user_action(&mut camera, &current_user_action_mode, canvas_state);

        // Set the appropriate cursor
        set_cursor(&current_user_action_mode, &canvas_state, &cursors);

        display_hud(&camera);

        next_frame().await
    }
}

fn observe_user_action(camera: &mut Camera, mode: &UserActionMode, mut state: CanvasState) -> CanvasState {
    match mode {
        UserActionMode::DRAG => {
            let (is_dragging, last_mouse_position) = handle_dragging(camera, state.is_dragging, state.last_mouse_position);
            state.is_dragging = is_dragging;
            state.last_mouse_position = last_mouse_position;
        }
    }
    state
}

fn handle_dragging(camera: &mut Camera, mut is_dragging: bool, mut last_mouse_position: Vec2) -> (bool, Vec2) {
    if is_mouse_button_down(MouseButton::Left) {
        if !is_dragging {
            is_dragging = true;
            last_mouse_position = mouse_position().into();
        }
        let current_mouse_position: Vec2 = mouse_position().into();
        let delta = (current_mouse_position - last_mouse_position) / camera.zoom;
        camera.position -= delta;
        last_mouse_position = current_mouse_position;
    } else {
        is_dragging = false;
    }
    (is_dragging, last_mouse_position)
}

fn set_cursor(mode: &UserActionMode, state: &CanvasState, cursors: &Cursors) {
    match mode {
        UserActionMode::DRAG => {
            if state.is_dragging {
                set_cur(cursors.grab);
            } else {
                set_cursor_custom(cursors.hand);
            }
        }
    }
}

fn display_hud(camera: &Camera) {
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

pub fn handle_scroll(scroll: &(f32, f32), camera: &mut Camera) {
    let &(wheel_x, wheel_y) = scroll;
    if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
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
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Infinite Canvas".to_owned(),
        window_width: 1200,
        window_height: 800,
        // can add the frame rate here
        ..Default::default()
    }
}
