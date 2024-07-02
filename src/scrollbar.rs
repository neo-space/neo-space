use macroquad::{
    color::Color, shapes::draw_rectangle, window::{screen_height, screen_width}
};

use crate::camera::Camera;

pub struct ScrollBarConfig {
    pub visible_area_factor: f32,
    pub scrollbar_width: f32,
    pub background_color: Color,
    pub foreground_color: Color,
}

impl ScrollBarConfig {
    pub fn new() -> Self {
        Self {
            visible_area_factor: 10.0,
            scrollbar_width: 10.0,
            background_color: Color::new(0.9, 0.9, 0.9, 1.0),
            foreground_color: Color::new(0.7, 0.7, 0.7, 1.0),
        }
    }
}

/// The scrollbar has to display relative to how much in an infinite plane the user has scrolled
/// 
pub fn draw_scrollbar(scroll_bar_config: &ScrollBarConfig, camera: &Camera) {
    let scrollbar_width: f32 = scroll_bar_config.scrollbar_width;
    let background_color: Color = scroll_bar_config.background_color;
    let scrollbar_foreground_color: Color = scroll_bar_config.foreground_color;
    let visible_area_height: f32 = screen_height() * scroll_bar_config.visible_area_factor;
    let scrollbar_height: f32 =
        (screen_height() / visible_area_height * screen_height()).min(screen_height());
    let scroll_range: f32 = visible_area_height - screen_height();
    let normalized_scroll_pos: f32 =
        (camera.position.y % visible_area_height + visible_area_height) % visible_area_height;
    let scrollbar_position: f32 =
        (normalized_scroll_pos / scroll_range) * (screen_height() - scrollbar_height);
    draw_rectangle(
        screen_width() - scrollbar_width,
        0.0,
        scrollbar_width,
        screen_height(),
        background_color,
    );
    draw_rectangle(
        screen_width() - scrollbar_width,
        scrollbar_position,
        scrollbar_width,
        scrollbar_height,
        scrollbar_foreground_color,
    );
}
