use macroquad::prelude::*;

use super::GameState;

pub fn process(gs: &mut GameState) {
    clear_background(BLACK);

    super::draw_elf_message(gs);

    // Draw victory message
    draw_text(
        "VICTORY!",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 + 80.0,
        60.0,
        GOLD,
    );

    // Draw congratulations message
    let congrats_text = format!("You survived all {} waves!", gs.game_constants.max_waves);
    let congrats_width = measure_text(&congrats_text, None, 24, 1.0).width;
    draw_text(
        &congrats_text,
        screen_width() / 2.0 - congrats_width / 2.0,
        screen_height() / 2.0 + 140.0,
        24.0,
        YELLOW,
    );

    // Draw final stats
    let level_text = format!("Final Level: {}", gs.player.get_level());
    draw_text(
        &level_text,
        screen_width() / 2.0 - 80.0,
        screen_height() / 2.0 + 170.0,
        20.0,
        LIGHTGRAY,
    );

    let xp_text = format!("Total XP: {}", gs.player.get_xp());
    draw_text(
        &xp_text,
        screen_width() / 2.0 - 70.0,
        screen_height() / 2.0 + 200.0,
        20.0,
        LIGHTGRAY,
    );

    // Draw weapon summary
    let weapons = gs.player.get_weapons();
    if !weapons.is_empty() {
        draw_text(
            "Weapons:",
            screen_width() / 2.0 - 50.0,
            screen_height() / 2.0 + 240.0,
            18.0,
            LIGHTGRAY,
        );

        for (i, weapon) in weapons.iter().enumerate() {
            let weapon_text = format!("{:?} Lvl{}", weapon.weapon_type, weapon.get_level());
            draw_text(
                &weapon_text,
                screen_width() / 2.0 - 60.0,
                screen_height() / 2.0 + 265.0 + (i as f32 * 22.0),
                16.0,
                GRAY,
            );
        }
    }

    // Draw restart instructions
    draw_text(
        "Press Return to Play Again",
        screen_width() / 2.0 - 140.0,
        screen_height() / 2.0 + 340.0,
        22.0,
        WHITE,
    );

    // Handle restart
    if is_key_pressed(KeyCode::Enter) {
        *gs = GameState::new();
    }
}
