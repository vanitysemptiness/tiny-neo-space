use macroquad::prelude::*;

mod hud;
use hud::Hud;

#[derive(PartialEq)]
enum Mode {
    Draw,
    Grab,
}

struct State {
    mode: Mode,
    last_mouse_position: Option<Vec2>,
    lines: Vec<(Vec2, Vec2)>,
    highlighted: Option<usize>,
}

impl State {
    fn new() -> Self {
        State {
            mode: Mode::Draw,
            last_mouse_position: None,
            lines: Vec::new(),
            highlighted: None,
        }
    }
}

fn draw_button(text: &str, x: f32, y: f32, width: f32, height: f32, is_active: bool) -> bool {
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

#[macroquad::main("neo-space")]
async fn main() {
    let mut state = State::new();
    let mut hud = Hud::new();

    loop {
        let dt = get_frame_time();
        hud.update(dt);

        clear_background(LIGHTGRAY);

        // Draw buttons
        if draw_button("Draw", 10.0, 10.0, 80.0, 30.0, state.mode == Mode::Draw) {
            state.mode = Mode::Draw;
        }
        if draw_button("Grab", 100.0, 10.0, 80.0, 30.0, state.mode == Mode::Grab) {
            state.mode = Mode::Grab;
        }

        // Handle mouse input
        let mouse_pos = Vec2::from(mouse_position());
        
        match state.mode {
            Mode::Draw => {
                if is_mouse_button_down(MouseButton::Left) {
                    if let Some(last_pos) = state.last_mouse_position {
                        state.lines.push((last_pos, mouse_pos));
                    }
                    state.last_mouse_position = Some(mouse_pos);
                } else {
                    state.last_mouse_position = None;
                }
            }
            Mode::Grab => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    state.highlighted = state.lines.iter().position(|&(start, end)| {
                        let dist = segment_distance(start, end, mouse_pos);
                        dist < 5.0
                    });
                }
            }
        }

        // Draw lines
        for (i, &(start, end)) in state.lines.iter().enumerate() {
            let color = if Some(i) == state.highlighted { RED } else { BLUE };
            draw_line(start.x, start.y, end.x, end.y, 2.0, color);
        }

        // Draw cursor
        match state.mode {
            Mode::Draw => draw_circle(mouse_pos.x, mouse_pos.y, 5.0, BLACK),
            Mode::Grab => draw_rectangle(mouse_pos.x - 5.0, mouse_pos.y - 5.0, 10.0, 10.0, BLACK),
        }

        // Draw HUD
        hud.draw();

        next_frame().await
    }
}

fn segment_distance(start: Vec2, end: Vec2, point: Vec2) -> f32 {
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