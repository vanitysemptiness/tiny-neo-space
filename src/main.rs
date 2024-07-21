use drawing::render_drawings;
use macroquad::prelude::*;

mod hud;
use hud::Hud;
mod mode_selector;
use mode_selector::perform_action_based_on_application_state;
mod buttons;
use buttons::render_ui_buttons;
mod state;
use state::ApplicationState;
mod cursor;
use cursor::draw_cursor_based_on_mode;
mod drawing;


#[macroquad::main("neo-space")]
async fn main() {
    let mut state = ApplicationState::new();
    let mut hud = Hud::new();
    // infinite rendering loop
    loop {
        // count the FPS
        let dt = get_frame_time();
        // show the hud
        hud.update(dt);
        // the background
        clear_background(LIGHTGRAY);
        // the ui buttons
        render_ui_buttons(&mut state);
        // Handle mouse input
        let mouse_pos = Vec2::from(mouse_position());
        // observe changes in state and perform necessary actions
        perform_action_based_on_application_state(&mut state, &mouse_pos);
        // Draw lines
        render_drawings(&state);
        // Draw cursor based on whatever mode user has selected
        draw_cursor_based_on_mode(&state.mode, mouse_pos);
        // Draw HUD
        hud.draw();
        next_frame().await
    }
}