use canvas_state::CanvasState;
use cursor::{draw_cursor, handle_cursor, Cursors};
use macroquad::prelude::*;

mod camera;
use camera::Camera;

mod grid;
use grid::draw_grid;
use scrollbar::{draw_scrollbar, handle_scroll, ScrollBarConfig};
use user_action_mode::{observe_user_action, UserActionMode};

mod scrollbar;
mod user_action_mode;
mod canvas_state;
mod cursor;
mod info_hud;
use info_hud::display_hud;


#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera::new();
    let scroll_bar_config: ScrollBarConfig = ScrollBarConfig::new();
    let mut canvas_state = CanvasState {
        is_dragging: false,
        last_mouse_position: Vec2::ZERO,
    };
    let current_user_action_mode = UserActionMode::DRAG;

    // Load cursor images, path is given from root Cargo.toml not package level
    let cursors = Cursors {
        hand: load_texture("rendering-engine/src/assets/hand_cursor.png").await.unwrap(),
        grab: load_texture("rendering-engine/src/assets/grab_cursor.png").await.unwrap(),
    };

        // Hide the default system cursor
        show_mouse(false);

    loop {
        clear_background(grid::BACKGROUND_COLOR);
        draw_grid(&camera);
        handle_scroll(&mouse_wheel(), &mut camera);
        draw_scrollbar(&scroll_bar_config, &camera);

        canvas_state = observe_user_action(&mut camera, &current_user_action_mode, canvas_state);

        // Draw the appropriate cursor
        draw_cursor(&current_user_action_mode, &canvas_state, &cursors);
        // Handle cursor visibility and drawing
        handle_cursor(&current_user_action_mode, &canvas_state, &cursors);
        display_hud(&camera);

        next_frame().await
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
