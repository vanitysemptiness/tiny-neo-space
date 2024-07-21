use macroquad::prelude::*;

use crate::{mode_selector::Mode, state::ApplicationState};

pub fn draw_button(text: &str, x: f32, y: f32, width: f32, height: f32, is_active: bool) -> bool {
    let color = if is_active { DARKGRAY } else { GRAY };
    let rect = Rect::new(x, y, width, height);
    draw_rectangle(x, y, width, height, color);
    draw_rectangle_lines(x, y, width, height, 2.0, BLACK);
    let text_size = measure_text(text, None, 20, 1.0);
    let text_x = x + (width - text_size.width) / 2.0;
    let text_y = y + (height + text_size.height) / 2.0;
    draw_text(text, text_x, text_y, 20.0, BLACK);
    is_mouse_button_pressed(MouseButton::Left) && rect.contains(Vec2::from(mouse_position()))
}

pub fn render_ui_buttons(state: &mut ApplicationState) {
    // Draw buttons
    if draw_button("Draw", 10.0, 10.0, 80.0, 30.0, state.mode == Mode::Draw) {
        state.mode = Mode::Draw;
    }
    if draw_button("Grab", 100.0, 10.0, 80.0, 30.0, state.mode == Mode::Grab) {
        state.mode = Mode::Grab;
    }
}