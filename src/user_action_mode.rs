use macroquad::{input::{is_mouse_button_down, mouse_position, MouseButton}, math::Vec2};

use crate::{camera::Camera, canvas_state::CanvasState};


pub enum UserActionMode {
    /// user is dragging canvas background
    DRAG
}

pub fn observe_user_action(camera: &mut Camera, mode: &UserActionMode, mut state: CanvasState) -> CanvasState {
    match mode {
        UserActionMode::DRAG => {
            let (is_dragging, last_mouse_position) = handle_dragging(camera, state.is_dragging, state.last_mouse_position);
            state.is_dragging = is_dragging;
            state.last_mouse_position = last_mouse_position;
        }
    }
    state
}

pub fn handle_dragging(camera: &mut Camera, mut is_dragging: bool, mut last_mouse_position: Vec2) -> (bool, Vec2) {
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