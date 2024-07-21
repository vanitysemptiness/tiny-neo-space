use macroquad::{input::{is_mouse_button_down, is_mouse_button_pressed, MouseButton}, math::Vec2, prelude};

use crate::state::ApplicationState;
use crate::drawing::segment_distance;

#[derive(PartialEq)]
pub(crate) enum Mode {
    Draw,
    Grab,
}

pub fn perform_action_based_on_application_state(state: &mut ApplicationState, mouse_pos: &Vec2) {
    match state.mode {
        Mode::Draw => {
            if is_mouse_button_down(MouseButton::Left) {
                if let Some(last_pos) = state.last_mouse_position {
                    state.lines.push((last_pos, mouse_pos.clone()));
                }
                state.last_mouse_position = Some(mouse_pos.clone());
            } else {
                state.last_mouse_position = None;
            }
        }
        Mode::Grab => {
            if is_mouse_button_pressed(MouseButton::Left) {
                state.highlighted = state.lines.iter().position(|&(start, end)| {
                    let dist = segment_distance(start, end, mouse_pos.clone());
                    dist < 5.0
                });
            }
        }
    }
}