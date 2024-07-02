use macroquad::{color::WHITE, input::{mouse_position, show_mouse}, texture::{draw_texture, Texture2D}, window::{screen_height, screen_width}};

use crate::{canvas_state::CanvasState, user_action_mode::UserActionMode};



pub fn handle_cursor(mode: &UserActionMode, state: &CanvasState, cursors: &Cursors) {
    let (mouse_x, mouse_y) = mouse_position();
    let window_width = screen_width();
    let window_height = screen_height();

    if mouse_x >= 0.0 && mouse_x < window_width && mouse_y >= 0.0 && mouse_y < window_height {
        // Mouse is inside the window
        show_mouse(false);
        draw_cursor(mode, state, cursors);
    } else {
        // Mouse is outside the window
        show_mouse(true);
    }
}

pub fn draw_cursor(mode: &UserActionMode, state: &CanvasState, cursors: &Cursors) {
    let (mouse_x, mouse_y) = mouse_position();
    let cursor_texture = match mode {
        UserActionMode::DRAG => {
            if state.is_dragging {
                &cursors.grab
            } else {
                &cursors.hand
            }
        }
    };
    
    draw_texture(
        *cursor_texture,
        mouse_x - cursor_texture.width() / 2.0,
        mouse_y - cursor_texture.height() / 2.0,
        WHITE,
    );
}

pub struct Cursors {
    pub hand: Texture2D,
    pub grab: Texture2D,
}