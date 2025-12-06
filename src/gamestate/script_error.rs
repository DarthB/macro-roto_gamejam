use macroquad::prelude::*;

use super::GameState;

pub fn process(gs: &mut GameState) {
    clear_background(BLACK);
    draw_text(
        "SCRIPT ERROR",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 - 40.0,
        40.0,
        RED,
    );
    if let Some(ref msg) = gs.error_message {
        let lines: Vec<&str> = msg.lines().collect();
        for (i, line) in lines.iter().take(5).enumerate() {
            draw_text(
                line,
                20.0,
                screen_height() / 2.0 + 20.0 + (i as f32 * 20.0),
                16.0,
                DARKGRAY,
            );
        }
    }
    draw_text(
        "Fix waves.roto and press 'R' to reload",
        screen_width() / 2.0 - 150.0,
        screen_height() / 2.0 + 120.0,
        20.0,
        DARKGRAY,
    );
    draw_text(
        "Or press Return to Restart",
        screen_width() / 2.0 - 120.0,
        screen_height() / 2.0 + 150.0,
        20.0,
        DARKGRAY,
    );
    if is_key_pressed(KeyCode::Enter) {
        *gs = GameState::new();
    }
}
