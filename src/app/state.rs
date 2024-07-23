use macroquad::prelude::*;

use super::mode_selector::Mode;

pub struct Stroke {
    pub points: Vec<Vec2>,
    pub fickness: f32
}

pub struct ApplicationState {
   pub mode: Mode,
    pub last_mouse_position: Option<Vec2>,
    pub strokes: Vec<Stroke>,
    pub current_stroke: Option<Stroke>,
    pub highlighted: Option<usize>,
    pub cursor_size: f32
}

impl ApplicationState {
    pub fn new() -> Self {
        ApplicationState {
            mode: Mode::Draw,
            last_mouse_position: None,
            strokes: Vec::new(),
            current_stroke: None,
            highlighted: None,
            cursor_size: 10.0
        }
    }
}