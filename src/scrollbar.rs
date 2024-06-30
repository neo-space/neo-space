use macroquad::{
    color::Color, shapes::draw_rectangle, window::{screen_height, screen_width}
};

use crate::camera::Camera;

// Draw scrollbar
pub fn draw_scrollbar(camera: &Camera) {
    let VISIBLE_AREA_FACTOR: f32 = 10.0;
    let VISIBLE_AREA_HEIGHT: f32 = screen_height() * VISIBLE_AREA_FACTOR;
    let SCROLLBAR_HEIGHT: f32 =
        (screen_height() / VISIBLE_AREA_HEIGHT * screen_height()).min(screen_height());
    let SCROLL_RANGE: f32 = VISIBLE_AREA_HEIGHT - screen_height();
    let NORMALIZED_SCROLL_POS: f32 =
        (camera.position.y % VISIBLE_AREA_HEIGHT + VISIBLE_AREA_HEIGHT) % VISIBLE_AREA_HEIGHT;
    let SCROLLBAR_WIDTH: f32 = 10.0;
    let SCROLLBAR_POSITION: f32 =
        (NORMALIZED_SCROLL_POS / SCROLL_RANGE) * (screen_height() - SCROLLBAR_HEIGHT);
    let SCROLLBAR_BG_COLOR: Color = Color::new(0.9, 0.9, 0.9, 1.0);
    let SCROLLBAR_FG_COLOR: Color = Color::new(0.7, 0.7, 0.7, 1.0);
    draw_rectangle(
        screen_width() - SCROLLBAR_WIDTH,
        0.0,
        SCROLLBAR_WIDTH,
        screen_height(),
        SCROLLBAR_BG_COLOR,
    );
    draw_rectangle(
        screen_width() - SCROLLBAR_WIDTH,
        SCROLLBAR_POSITION,
        SCROLLBAR_WIDTH,
        SCROLLBAR_HEIGHT,
        SCROLLBAR_FG_COLOR,
    );
}
