use macroquad::prelude::*;

use super::mode_selector::Mode;

pub struct ApplicationState {
   pub mode: Mode,
    pub last_mouse_position: Option<Vec2>,
    pub lines: Vec<(Vec2, Vec2)>,
    pub highlighted: Option<usize>,
}

impl ApplicationState {
    pub fn new() -> Self {
        ApplicationState {
            mode: Mode::Draw,
            last_mouse_position: None,
            lines: Vec::new(),
            highlighted: None,
        }
    }
}