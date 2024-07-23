use macroquad::prelude::*;

use super::state::{ApplicationState, Stroke};

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
    todo!() // no longer drawing lines as clusters of lines
    // for (i, &(start, end)) in state.lines.iter().enumerate() {
    //     let color = if Some(i) == state.highlighted { RED } else { BLUE };
    //     draw_line(start.x, start.y, end.x, end.y, 2.0, color);
    // }
}

pub fn perform_drawing(state: &mut ApplicationState, mouse_pos: &Vec2) {
    if is_mouse_button_down(MouseButton::Left) {
        if let Some(stroke) = &mut state.current_stroke {
            stroke.points.push(mouse_pos.clone());
        } else {
            state.current_stroke = Some(Stroke {
                points: vec![mouse_pos.clone()],
                fickness: state.cursor_size,
            });
        }
    } else if is_mouse_button_released(MouseButton::Left) {
        if let Some(stroke) = state.current_stroke.take() {
            if stroke.points.len() > 1 {
                state.strokes.push(stroke);
            } else {
                // For a single point, create a small circle
                state.strokes.push(Stroke {
                    points: vec![mouse_pos.clone(), mouse_pos.clone()],
                    fickness: state.cursor_size,
                });
            }
        }
    }
}

pub fn render_strokes(state: &ApplicationState) {
    for (i, stroke) in state.strokes.iter().enumerate() {
        let color = if Some(i) == state.highlighted { RED } else { BLUE };
        render_stroke(stroke, color);
    }
    
    if let Some(current_stroke) = &state.current_stroke {
        render_stroke(current_stroke, BLUE);
    }
}

fn render_stroke(stroke: &Stroke, color: Color) {
    if stroke.points.len() == 1 {
        // Draw a circle for a single point
        draw_circle(stroke.points[0].x, stroke.points[0].y, stroke.fickness / 2.0, color);
    } else {
        for window in stroke.points.windows(2) {
            let start = window[0];
            let end = window[1];
            draw_line(start.x, start.y, end.x, end.y, stroke.fickness, color);
            
            // Draw circles at the joints to smooth out the connections
            draw_circle(start.x, start.y, stroke.fickness / 2.0, color);
            draw_circle(end.x, end.y, stroke.fickness / 2.0, color);
        }
    }
}