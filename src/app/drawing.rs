use macroquad::prelude::*;

use super::state::ApplicationState;

pub fn segment_distance(start: Vec2, end: Vec2, point: Vec2) -> f32 {
    let line_vec = end - start;
    let point_vec = point - start;
    let line_len = line_vec.length();
    let project = point_vec.dot(line_vec) / line_len;
    if project < 0.0 {
        point_vec.length()
    } else if project > line_len {
        (point - end).length()
    } else {
        let projection = start + line_vec * (project / line_len);
        (point - projection).length()
    }
}

pub fn render_drawings(state: &ApplicationState) {
    for (i, &(start, end)) in state.lines.iter().enumerate() {
        let color = if Some(i) == state.highlighted { RED } else { BLUE };
        draw_line(start.x, start.y, end.x, end.y, 2.0, color);
    }
}

pub fn perform_drawing(state: &mut ApplicationState, mouse_pos: &Vec2) {
    if is_mouse_button_pressed(MouseButton::Left) {
        // Start a new potential line or dot
        state.last_mouse_position = Some(mouse_pos.clone());
    } else if is_mouse_button_down(MouseButton::Left) {
        if let Some(last_pos) = state.last_mouse_position {
            if last_pos != *mouse_pos {
                // If the mouse has moved, add a line
                state.lines.push((last_pos, mouse_pos.clone()));
                state.last_mouse_position = Some(mouse_pos.clone());
            }
        }
    } else if !is_mouse_button_down(MouseButton::Left) {
        // Mouse button released
        if let Some(last_pos) = state.last_mouse_position {
            if last_pos == *mouse_pos {
                // If the mouse hasn't moved, add a dot
                state.lines.push((last_pos, last_pos));
            }
        }
        state.last_mouse_position = None;
    }
}