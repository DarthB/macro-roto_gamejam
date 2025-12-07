use macroquad::prelude::*;

use super::GameState;

pub fn process(gs: &mut GameState) {
    clear_background(BLACK);

    super::draw_elf_message(gs);

    draw_text(
        "GAME OVER",
        screen_width() / 2.0 - 80.0,
        screen_height() / 2.0 + 160.,
        40.0,
        RED,
    );
    draw_text(
        "Press Return to Restart",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 + 250.0,
        20.0,
        DARKGRAY,
    );
    if is_key_pressed(KeyCode::Enter) {
        *gs = GameState::new(gs.assets.clone());
    }
}
