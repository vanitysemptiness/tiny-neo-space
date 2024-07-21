use macroquad::prelude::*;

use crate::state::ApplicationState;

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