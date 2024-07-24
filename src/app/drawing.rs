use macroquad::prelude::*;
use super::state::ApplicationState;
use super::screen_object::Stroke;

const MIN_DISTANCE_BETWEEN_POINTS: f32 = 2.0;
const MAX_POINTS_PER_STROKE: usize = 1000;

pub fn perform_drawing(state: &mut ApplicationState, mouse_pos: &Vec2) {
    if is_mouse_button_down(MouseButton::Left) {
        let new_point = simplify_point(state, mouse_pos);
        
        if let Some(point) = new_point {
            if let Some(stroke) = &mut state.current_stroke {
                stroke.points.push(point);
                if stroke.points.len() >= MAX_POINTS_PER_STROKE {
                    finalize_current_stroke(state);
                    state.current_stroke = Some(Stroke {
                        points: vec![point],
                        fickness: state.cursor_size,
                        color: BLUE,
                    });
                }
            } else {
                state.current_stroke = Some(Stroke {
                    points: vec![point],
                    fickness: state.cursor_size,
                    color: BLUE,
                });
            }
        }
    } else if is_mouse_button_released(MouseButton::Left) {
        finalize_current_stroke(state);
    }
}

fn simplify_point(state: &ApplicationState, mouse_pos: &Vec2) -> Option<Vec2> {
    if let Some(stroke) = &state.current_stroke {
        if let Some(last_point) = stroke.points.last() {
            if last_point.distance(*mouse_pos) >= MIN_DISTANCE_BETWEEN_POINTS {
                Some(*mouse_pos)
            } else {
                None
            }
        } else {
            Some(*mouse_pos)
        }
    } else {
        Some(*mouse_pos)
    }
}

fn finalize_current_stroke(state: &mut ApplicationState) {
    if let Some(stroke) = state.current_stroke.take() {
        if stroke.points.len() > 1 {
            state.strokes.push(stroke);
        }
    }
}

pub fn render_strokes(state: &ApplicationState) {
    for stroke in &state.strokes {
        render_stroke(stroke);
    }

    if let Some(current_stroke) = &state.current_stroke {
        render_stroke(current_stroke);
    }
}

fn render_stroke(stroke: &Stroke) {
    if stroke.points.len() < 2 {
        return;
    }

    let mut previous_point: Option<Vec2> = None;
    for (i, point) in stroke.points.iter().enumerate() {
        if let Some(prev) = previous_point {
            // Draw thick line
            draw_line(prev.x, prev.y, point.x, point.y, stroke.fickness, stroke.color);
            
            // Draw circle at the start point (only for the first segment)
            if i == 1 {
                draw_circle(prev.x, prev.y, stroke.fickness / 2.0, stroke.color);
            }
            
            // Draw circle at the end point
            draw_circle(point.x, point.y, stroke.fickness / 2.0, stroke.color);
        }
        previous_point = Some(*point);
    }
}