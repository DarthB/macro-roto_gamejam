use macroquad::prelude::*;

mod collision;
mod enemy;
mod entity;
mod gamestate;
mod player;
mod projectile;
mod roto_script;
mod visual_config;
mod weapon;

use enemy::EnemyType;
use gamestate::{GameState, GameStateEnum};
use roto_script::WaveConfig;

pub const DT: f64 = 1.0 / 30.0;

fn process_state_gameover(gs: &mut GameState) {
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

fn process_state_script_error(gs: &mut GameState) {
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

fn update_state_playing(gs: &mut GameState) {
    // check if we need to spawn a new wave:
    if gs.enemies.is_empty() {
        // spawn new wave
        let wave = gs.wave;
        match gs.roto_manager.get_wave_config(wave) {
            Ok(config) => {
                if let Err(err) = spawn_wave(gs, config) {
                    gs.state = GameStateEnum::ScriptError;
                    gs.error_message = Some(err);
                } else {
                    gs.wave += 1;
                }
            }
            Err(err) => {
                gs.state = GameStateEnum::ScriptError;
                gs.error_message = Some(err);
            }
        }
    }

    // perform the logic updates if any
    let num_updates = gs.update_time_for_logic();
    for _ in 0..num_updates {
        if !gs.paused {
            input(gs);
            update(gs);
        }
    }
}

fn input(gs: &mut GameState) {
    gs.player.input();
}

fn global_input(gs: &mut GameState) {
    // Hot reload Roto scripts on 'R' key
    if is_key_pressed(KeyCode::R) {
        gs.reload_roto_scripts();
    }

    // Toggle pause on 'P' key
    if is_key_pressed(KeyCode::P) {
        gs.paused = !gs.paused;
    }
}

fn update(gs: &mut GameState) {
    let dt = DT as f32;

    // Update player and get spawn commands from weapon firing
    let spawn_commands = gs.player.update(dt);
    gs.execute_spawn_commands(spawn_commands);

    let player_pos = gs.player.pos;
    for enemy in gs.enemies.iter_mut() {
        enemy.update(Some(player_pos));
    }

    // Update projectiles
    for projectile in gs.projectiles.iter_mut() {
        projectile.update(dt);
        // Update homing behavior for homing missiles
        projectile.update_homing(dt, &gs.enemies);
    }

    // Mark expired projectiles for despawn
    for projectile in &gs.projectiles {
        if projectile.is_expired() {
            gs.projectiles_to_despawn.insert(projectile.id);
        }
    }

    // Mark out-of-bounds entities for despawn
    gs.despawn_projectiles_out_of_bounds();
    gs.despawn_enemies_out_of_bounds();

    // this may trigger game over
    gs.check_collisions();
    gs.check_player_bounds();

    // Process all despawns at the end
    gs.process_despawns();
}

fn draw(gs: &GameState) {
    gs.player.draw();
    for enemy in gs.enemies.iter() {
        enemy.draw();
    }
    for projectile in gs.projectiles.iter() {
        projectile.draw();
    }
    draw_text(
        "Auto-battler: Move with arrow keys, weapon fire automatically",
        20.0,
        20.0,
        20.0,
        DARKGRAY,
    );
    draw_text(
        "Avoid the enemies. Don't leave the Screen! OR DIE!",
        20.0,
        40.0,
        20.0,
        DARKGRAY,
    );
    draw_text("Press 'R' to reload scripts", 20.0, 60.0, 20.0, DARKGRAY);
    draw_text("Press 'P' to pause", 20.0, 80.0, 20.0, DARKGRAY);
    let wave_text = format!("Wave: {}", gs.wave);
    draw_text(&wave_text, screen_width() - 120.0, 20.0, 20.0, DARKGRAY);

    // Show current weapon info
    if let Some(weapon) = gs.player.get_weapons().first() {
        let weapon_text = format!("Weapon: {:?} Lvl{}", weapon.weapon_type, weapon.get_level());
        draw_text(&weapon_text, screen_width() - 200.0, 40.0, 16.0, DARKGRAY);
    }

    if gs.paused {
        draw_text(
            "PAUSED",
            screen_width() / 2.0 - 50.0,
            screen_height() / 2.0,
            40.0,
            YELLOW,
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        window_title: "Auto Scriptable by Roto".to_owned(),
        ..Default::default() // Use other defaults
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gs = GameState::new();

    loop {
        match gs.state {
            GameStateEnum::GameOver => {
                process_state_gameover(&mut gs);
            }
            GameStateEnum::ScriptError => {
                process_state_script_error(&mut gs);
            }
            GameStateEnum::Playing => {
                global_input(&mut gs);
                update_state_playing(&mut gs);
                // render every frame for Playing state:
                clear_background(BLACK);
                draw(&gs);
            }
        }

        next_frame().await
    }
}

fn spawn_wave(gs: &mut GameState, config: WaveConfig) -> Result<(), String> {
    let w = screen_width();
    let h = screen_height();

    // Spawn basic enemies
    for _ in 0..config.basic_enemy_count {
        let (x, y) = get_spawn_position(w, h);
        gs.spawn_enemy(EnemyType::Basic, Vec2::new(x, y))?;
    }

    // Spawn chaser enemies
    for _ in 0..config.chaser_enemy_count {
        let (x, y) = get_spawn_position(w, h);
        gs.spawn_enemy(EnemyType::Chaser, Vec2::new(x, y))?;
    }

    Ok(())
}

fn get_spawn_position(w: f32, h: f32) -> (f32, f32) {
    let x = if rand::gen_range(0, 2) == 0 {
        // left or right edge
        if rand::gen_range(0, 2) == 0 { 0.0 } else { w }
    } else {
        rand::gen_range(0.0, w)
    };
    let y = if x == 0.0 || x == w {
        rand::gen_range(0.0, h)
    } else if rand::gen_range(0, 2) == 0 {
        0.0
    } else {
        h
    };
    (x, y)
}
