use macroquad::prelude::*;

use super::GameState;

pub fn process(gs: &mut GameState) {
    clear_background(BLACK);
    draw_text(
        "GAME OVER",
        screen_width() / 2.0 - 80.0,
        screen_height() / 2.0,
        40.0,
        RED,
    );
    draw_text(
        "Press Return to Restart",
        screen_width() / 2.0 - 120.0,
        screen_height() / 2.0 + 50.0,
        20.0,
        DARKGRAY,
    );
    if is_key_pressed(KeyCode::Enter) {
        *gs = GameState::new();
    }
}
