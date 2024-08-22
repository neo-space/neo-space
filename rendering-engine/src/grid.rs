use macroquad::{color::Color, math::{vec2, Vec2}, shapes::draw_circle, window::{screen_height, screen_width}};

use crate::camera::Camera;


const GRID_SIZE: f32 = 20.0;
const NORMAL_DOT_COLOR: Color = Color::new(0.7, 0.9, 1.0, 1.0);
const EMPHASIZED_DOT_COLOR: Color = Color::new(0.4, 0.7, 0.9, 1.0);
pub const BACKGROUND_COLOR: Color = Color::new(0.95, 0.96, 0.98, 1.0);

pub fn draw_grid(camera: &Camera) {
    let top_left = camera.screen_to_world(Vec2::ZERO);
    let bottom_right = camera.screen_to_world(vec2(screen_width(), screen_height()));

    let base_step = GRID_SIZE;
    let zoom_factor = 1.0 / camera.zoom;
    let step = (base_step * zoom_factor.max(1.0)).round() as i32;

    let start_x = (top_left.x / step as f32).floor() as i32 * step;
    let start_y = (top_left.y / step as f32).floor() as i32 * step;
    let end_x = (bottom_right.x / step as f32).ceil() as i32 * step;
    let end_y = (bottom_right.y / step as f32).ceil() as i32 * step;

    for x in (start_x..=end_x).step_by(step as usize) {
        for y in (start_y..=end_y).step_by(step as usize) {
            let world_pos = vec2(x as f32, y as f32);
            let screen_pos = camera.world_to_screen(world_pos);
            let size = (1.0 * camera.zoom).clamp(0.5, 2.0);
            let is_emphasized = (x / step) % 4 == 0 && (y / step) % 4 == 0;
            let color = if is_emphasized { EMPHASIZED_DOT_COLOR } else { NORMAL_DOT_COLOR };
            
            draw_circle(screen_pos.x, screen_pos.y, size, color);
        }
    }
}