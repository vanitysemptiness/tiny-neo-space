use macroquad::prelude::*;

use crate::mode_selector::Mode;

pub fn draw_cursor_based_on_mode(mode: &Mode, mouse_pos: Vec2) {
    match mode {
        Mode::Draw => draw_circle(mouse_pos.x, mouse_pos.y, 5.0, BLACK),
        Mode::Grab => draw_rectangle(mouse_pos.x - 5.0, mouse_pos.y - 5.0, 10.0, 10.0, BLACK),
    }
}