use macroquad::math::Vec2;

pub struct CanvasState {
    pub is_dragging: bool,
    pub last_mouse_position: Vec2,
}