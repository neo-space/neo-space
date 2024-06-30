use egui_macroquad::egui;
use macroquad::prelude::*;

const GRID_SIZE: f32 = 20.0;
const MIN_SQUARE_SIZE: f32 = 60.0;
const RESIZE_HANDLE_SIZE: f32 = 10.0;

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Select,
    CreateSquare,
    CreateCircle,
}

struct Shape {
    pos: Vec2,
    size: Vec2,
    color: Color,
    text: String,
    shape_type: ShapeType,
}

#[derive(Clone, Copy, PartialEq)]
enum ShapeType {
    Square,
    Circle,
}

struct AppState {
    shapes: Vec<Shape>,
    selected_shape: Option<usize>,
    current_tool: Tool,
    drag_state: DragState,
    editing_text: bool,
}

enum DragState {
    None,
    Moving(usize),
    Resizing(usize),
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Wireframe Tool".to_owned(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState {
        shapes: Vec::new(),
        selected_shape: None,
        current_tool: Tool::Select,
        drag_state: DragState::None,
        editing_text: false,
    };

    loop {
        clear_background(WHITE);

        // Draw grid
        for x in (0..screen_width() as i32).step_by(GRID_SIZE as usize) {
            for y in (0..screen_height() as i32).step_by(GRID_SIZE as usize) {
                draw_circle(x as f32, y as f32, 1.0, Color::new(0.7, 0.9, 1.0, 1.0));
            }
        }

        // Draw shapes
        for (i, shape) in state.shapes.iter().enumerate() {
            match shape.shape_type {
                ShapeType::Square => draw_rectangle(
                    shape.pos.x,
                    shape.pos.y,
                    shape.size.x,
                    shape.size.y,
                    shape.color,
                ),
                ShapeType::Circle => draw_circle(
                    shape.pos.x + shape.size.x / 2.0,
                    shape.pos.y + shape.size.y / 2.0,
                    shape.size.x / 2.0,
                    shape.color,
                ),
            }

            let wrapped_text = wrap_text(&shape.text, shape.size.x - 20.0, 20.0);
            for (j, line) in wrapped_text.iter().enumerate() {
                draw_text(
                    line,
                    shape.pos.x + 10.0,
                    shape.pos.y + 30.0 + j as f32 * 25.0,
                    20.0,
                    BLACK,
                );
            }

            if Some(i) == state.selected_shape {
                draw_rectangle_lines(
                    shape.pos.x,
                    shape.pos.y,
                    shape.size.x,
                    shape.size.y,
                    2.0,
                    RED,
                );
                draw_rectangle(
                    shape.pos.x + shape.size.x - RESIZE_HANDLE_SIZE,
                    shape.pos.y + shape.size.y - RESIZE_HANDLE_SIZE,
                    RESIZE_HANDLE_SIZE,
                    RESIZE_HANDLE_SIZE,
                    RED,
                );
            }
        }

        // Handle drag states
        let mouse_pos = mouse_position();
        match state.drag_state {
            DragState::Moving(index) => {
                state.shapes[index].pos = Vec2::new(
                    (mouse_pos.0 / GRID_SIZE).floor() * GRID_SIZE,
                    (mouse_pos.1 / GRID_SIZE).floor() * GRID_SIZE,
                );
            }
            DragState::Resizing(index) => {
                state.shapes[index].size = Vec2::new(
                    ((mouse_pos.0 - state.shapes[index].pos.x) / GRID_SIZE).ceil() * GRID_SIZE,
                    ((mouse_pos.1 - state.shapes[index].pos.y) / GRID_SIZE).ceil() * GRID_SIZE,
                );
                state.shapes[index].size.x = state.shapes[index].size.x.max(MIN_SQUARE_SIZE);
                state.shapes[index].size.y = state.shapes[index].size.y.max(MIN_SQUARE_SIZE);
            }
            DragState::None => {}
        }

        // Handle mouse input
        if is_mouse_button_pressed(MouseButton::Left) {
            match state.current_tool {
                Tool::Select => {
                    let clicked_shape = state.shapes.iter().position(|s| {
                        mouse_pos.0 >= s.pos.x
                            && mouse_pos.0 < s.pos.x + s.size.x
                            && mouse_pos.1 >= s.pos.y
                            && mouse_pos.1 < s.pos.y + s.size.y
                    });

                    if let Some(index) = clicked_shape {
                        state.selected_shape = Some(index);
                        if mouse_pos.0
                            >= state.shapes[index].pos.x + state.shapes[index].size.x
                                - RESIZE_HANDLE_SIZE
                            && mouse_pos.1
                                >= state.shapes[index].pos.y + state.shapes[index].size.y
                                    - RESIZE_HANDLE_SIZE
                        {
                            state.drag_state = DragState::Resizing(index);
                        } else {
                            state.drag_state = DragState::Moving(index);
                        }
                    } else {
                        state.selected_shape = None;
                    }
                }
                Tool::CreateSquare => {
                    state.shapes.push(Shape {
                        pos: Vec2::new(
                            (mouse_pos.0 / GRID_SIZE).floor() * GRID_SIZE,
                            (mouse_pos.1 / GRID_SIZE).floor() * GRID_SIZE,
                        ),
                        size: Vec2::new(MIN_SQUARE_SIZE, MIN_SQUARE_SIZE),
                        color: Color::new(
                            rand::gen_range(0.0, 1.0),
                            rand::gen_range(0.0, 1.0),
                            rand::gen_range(0.0, 1.0),
                            0.5,
                        ),
                        text: String::new(),
                        shape_type: ShapeType::Square,
                    });
                    state.selected_shape = Some(state.shapes.len() - 1);
                }
                Tool::CreateCircle => {
                    state.shapes.push(Shape {
                        pos: Vec2::new(
                            (mouse_pos.0 / GRID_SIZE).floor() * GRID_SIZE,
                            (mouse_pos.1 / GRID_SIZE).floor() * GRID_SIZE,
                        ),
                        size: Vec2::new(MIN_SQUARE_SIZE, MIN_SQUARE_SIZE),
                        color: Color::new(
                            rand::gen_range(0.0, 1.0),
                            rand::gen_range(0.0, 1.0),
                            rand::gen_range(0.0, 1.0),
                            0.5,
                        ),
                        text: String::new(),
                        shape_type: ShapeType::Circle,
                    });
                    state.selected_shape = Some(state.shapes.len() - 1);
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            state.drag_state = DragState::None;
        }

        // Handle keyboard input for text editing
        if state.editing_text {
            if let Some(index) = state.selected_shape {
                let mut text_changed = false;
                if is_key_pressed(KeyCode::Backspace) {
                    state.shapes[index].text.pop();
                    text_changed = true;
                } else if let Some(char) = get_char_pressed() {
                    state.shapes[index].text.push(char);
                    text_changed = true;
                }
                if text_changed {
                    // Ensure the Egui text edit widget updates
                    egui_macroquad::ui(|_| {});
                }
            }
        }

        // Egui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Tools").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut state.current_tool, Tool::Select, "Select");
                    ui.selectable_value(&mut state.current_tool, Tool::CreateSquare, "Square");
                    ui.selectable_value(&mut state.current_tool, Tool::CreateCircle, "Circle");
                });

                if let Some(index) = state.selected_shape {
                    ui.separator();
                    ui.label("Edit Shape");
                    state.editing_text = ui
                        .text_edit_singleline(&mut state.shapes[index].text)
                        .gained_focus();

                    // Color editing
                    let mut color = [
                        state.shapes[index].color.r,
                        state.shapes[index].color.g,
                        state.shapes[index].color.b,
                    ];
                    if ui.color_edit_button_rgb(&mut color).changed() {
                        state.shapes[index].color.r = color[0];
                        state.shapes[index].color.g = color[1];
                        state.shapes[index].color.b = color[2];
                    }
                    // Alpha transparency slider
                    let mut alpha = state.shapes[index].color.a;
                    if ui
                        .add(egui::Slider::new(&mut alpha, 0.0..=1.0).text("Opacity"))
                        .changed()
                    {
                        state.shapes[index].color.a = alpha;
                    }
                }
            });
        });

        egui_macroquad::draw();

        next_frame().await
    }
}

fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let _space_width = measure_text(" ", None, font_size as u16, 1.0).width;

    for word in text.split_whitespace() {
        let word_width = measure_text(word, None, font_size as u16, 1.0).width;
        if measure_text(&current_line, None, font_size as u16, 1.0).width + word_width > max_width {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }
            if word_width > max_width {
                // If the word is too long, split it
                let chars_per_line = (max_width / (word_width / word.len() as f32)) as usize;
                for (i, c) in word.chars().enumerate() {
                    if i > 0 && i % chars_per_line == 0 {
                        lines.push(current_line);
                        current_line = String::new();
                    }
                    current_line.push(c);
                }
            } else {
                current_line = word.to_string();
            }
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}
