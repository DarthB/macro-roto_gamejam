use macroquad::prelude::*;

use super::GameState;
use crate::DT;
use crate::enemy::EnemyType;
use crate::roto_script::WaveConfig;

pub fn process(gs: &mut GameState) {
    // Check if we need to spawn a new wave
    if gs.enemies.is_empty() {
        let wave = gs.wave;
        match gs.roto_manager.get_wave_config(wave) {
            Ok(config) => {
                if let Err(err) = spawn_wave(gs, config) {
                    gs.state = super::GameStateEnum::ScriptError;
                    gs.error_message = Some(err);
                } else {
                    gs.wave += 1;
                }
            }
            Err(err) => {
                gs.state = super::GameStateEnum::ScriptError;
                gs.error_message = Some(err);
            }
        }
    }

    // Perform the logic updates if any
    let num_updates = gs.update_time_for_logic();
    for _ in 0..num_updates {
        if !gs.paused {
            gs.player.input();
            update_logic(gs);
        }
    }
}

pub fn update_logic(gs: &mut GameState) {
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

    // This may trigger game over
    gs.check_collisions();
    gs.check_player_bounds();

    // Process all despawns at the end
    gs.process_despawns();
}

pub fn draw(gs: &GameState) {
    gs.player.draw();
    for enemy in gs.enemies.iter() {
        enemy.draw();
    }
    for projectile in gs.projectiles.iter() {
        projectile.draw();
    }
    draw_text(
        "Auto-battler: Move with Arrow Keys, aim with mouse, weapon fires automatically",
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
