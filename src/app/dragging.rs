use macroquad::{input::{is_mouse_button_pressed, MouseButton}, math::Vec2};

use super::{drawing::segment_distance, ApplicationState};



pub fn perform_dragging(state: &mut ApplicationState, mouse_pos: &Vec2) {
    if is_mouse_button_pressed(MouseButton::Left) {
        state.highlighted = state.lines.iter().position(|&(start, end)| {
            let dist = segment_distance(start, end, mouse_pos.clone());
            dist < 5.0
        });
    }
}