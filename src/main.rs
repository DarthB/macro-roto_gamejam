use macroquad::prelude::*;

mod collision;
mod enemy;
mod player;
mod roto_script;

use collision::{Collidable, check_collision};
use enemy::{Enemy, EnemyType};
use player::Player;
use roto_script::{RotoScriptManager, WaveConfig};

const DT: f64 = 1.0 / 30.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameStateEnum {
    Playing,
    GameOver,
    ScriptError,
}

struct GameState {
    player: Player,
    t_frame: f64,
    t_prev: f64,
    t_passed: f64,
    n_logic_updates: u32,
    enemies: Vec<Enemy>,
    state: GameStateEnum,
    wave: u32,
    roto_manager: RotoScriptManager,
    error_message: Option<String>,
    paused: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut roto_manager = RotoScriptManager::new();

        // Try to fetch player stats from Roto, fallback to defaults if it fails
        let player_stats = roto_manager
            .get_player_stats()
            .unwrap_or(enemy::EntityStats {
                radius: 20.0,
                max_speed: 5.0,
                acceleration: 1.0,
                friction: 0.9,
            });

        Self {
            player: Player::new(screen_width() / 2.0, screen_height() / 2.0, player_stats),
            t_frame: get_time(),
            t_prev: get_time(),
            t_passed: 0.0,
            n_logic_updates: 0,
            enemies: vec![],
            state: GameStateEnum::Playing,
            wave: 0,
            roto_manager,
            error_message: None,
            paused: false,
        }
    }

    fn check_collisions(&mut self) {
        // Check player-enemy collisions
        let enemies_collided_with_player: Vec<usize> = self
            .enemies
            .iter()
            .enumerate()
            .filter_map(|(i, enemy)| {
                let collision_data = check_collision(
                    &self.player.collider(),
                    self.player.position(),
                    &enemy.collider(),
                    enemy.position(),
                );
                if collision_data.collided {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        // no health system, just game over on first collision
        if !enemies_collided_with_player.is_empty() {
            self.state = GameStateEnum::GameOver;
        }

        // later remove collided enemies
        for &i in enemies_collided_with_player.iter().rev() {
            self.despawn_enemy(i);
        }

        // Check enemy-enemy collisions with elastic bounce
        self.check_enemy_collisions();
    }

    fn check_enemy_collisions(&mut self) {
        let num_enemies = self.enemies.len();

        for i in 0..num_enemies {
            for j in (i + 1)..num_enemies {
                let (pos1, vel1, pos2, vel2) = {
                    let enemy1 = &self.enemies[i];
                    let enemy2 = &self.enemies[j];
                    (enemy1.pos, enemy1.vel, enemy2.pos, enemy2.vel)
                };

                let collision_data = check_collision(
                    &self.enemies[i].collider(),
                    pos1,
                    &self.enemies[j].collider(),
                    pos2,
                );

                if collision_data.collided {
                    // Elastic collision response (equal mass)
                    // Normal points from enemy2 to enemy1
                    let normal = collision_data.normal;

                    // Calculate relative velocity
                    let rel_vel = vel1 - vel2;

                    // Calculate relative velocity along collision normal
                    let vel_along_normal = rel_vel.dot(normal);

                    // Do not resolve if velocities are separating
                    if vel_along_normal < 0.0 {
                        // For elastic collision with equal mass, exchange normal components
                        let impulse = normal * vel_along_normal;

                        // Apply impulse to both enemies
                        self.enemies[i].vel -= impulse;
                        self.enemies[j].vel += impulse;
                    }
                }
            }
        }
    }

    fn despawn_enemy(&mut self, index: usize) {
        self.enemies.remove(index);
    }

    fn despawn_enemies_out_of_bounds(&mut self) {
        let margin = self
            .roto_manager
            .get_game_constants()
            .map(|c| c.out_of_bounds_margin)
            .unwrap_or(50.0);

        let w = screen_width();
        let h = screen_height();
        self.enemies.retain(|enemy| {
            enemy.pos.x >= -margin
                && enemy.pos.x <= w + margin
                && enemy.pos.y >= -margin
                && enemy.pos.y <= h + margin
        });
    }

    fn update_time_for_logic(&mut self) -> u32 {
        // update time counters
        self.t_frame = get_time();
        self.t_passed += self.t_frame - self.t_prev;

        // update logic at fixed time steps
        while self.t_passed >= DT {
            self.t_passed -= DT;
            self.n_logic_updates += 1;
        }

        let reval = self.n_logic_updates;
        if self.n_logic_updates > 0 {
            if self.n_logic_updates > 1 {
                println!("logic updates: {} - LOW FRAME RATE", self.n_logic_updates);
            }
            self.n_logic_updates = 0;
        }

        self.t_prev = self.t_frame;
        reval
    }

    fn reload_roto_scripts(&mut self) {
        match self.reload_roto_script_internal() {
            Ok(_) => {
                self.state = GameStateEnum::Playing;
                self.error_message = None;
            }
            Err(err) => {
                self.state = GameStateEnum::ScriptError;
                self.error_message = Some(err);
            }
        }
    }

    fn reload_roto_script_internal(&mut self) -> Result<(), String> {
        self.roto_manager.reload();

        self.player.override_stats(
            self.roto_manager
                .get_player_stats()?,
        );

        for enemy in self.enemies.iter_mut() {
            let stats = match enemy.enemy_type {
                EnemyType::Basic => self
                    .roto_manager
                    .get_enemy_stats(EnemyType::Basic)?,
                EnemyType::Chaser => self
                    .roto_manager
                    .get_enemy_stats(EnemyType::Chaser)?,
            };
            enemy.override_stats(stats);
        }
        Ok(())
    }
}

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
    gs.player.update();
    let player_pos = gs.player.pos;
    for enemy in gs.enemies.iter_mut() {
        enemy.update(Some(player_pos));
    }

    // this may trigger game over
    gs.check_collisions();

    gs.despawn_enemies_out_of_bounds();
}

fn draw(gs: &GameState) {
    gs.player.draw();
    for enemy in gs.enemies.iter() {
        enemy.draw();
    }
    draw_text(
        "Move the Player with arrow keys",
        20.0,
        20.0,
        20.0,
        DARKGRAY,
    );
    draw_text("Avoid the Red Enemies!", 20.0, 40.0, 20.0, DARKGRAY);
    draw_text("Press 'R' to reload scripts", 20.0, 60.0, 20.0, DARKGRAY);
    draw_text("Press 'P' to pause", 20.0, 80.0, 20.0, DARKGRAY);
    let wave_text = format!("Wave: {}", gs.wave);
    draw_text(&wave_text, screen_width() - 120.0, 20.0, 20.0, DARKGRAY);

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

#[macroquad::main("Auto Scriptable by Roto")]
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

    let constants = gs.roto_manager.get_game_constants()?;
    let basic_stats = gs.roto_manager.get_enemy_stats(EnemyType::Basic)?;
    let chaser_stats = gs.roto_manager.get_enemy_stats(EnemyType::Chaser)?;

    // Spawn basic enemies
    for _ in 0..config.basic_enemy_count {
        let (x, y) = get_spawn_position(w, h);
        let enemy = Enemy::spawn(
            x,
            y,
            EnemyType::Basic,
            basic_stats,
            constants.spawn_target_offset,
        );
        gs.enemies.push(enemy);
    }

    // Spawn chaser enemies
    for _ in 0..config.chaser_enemy_count {
        let (x, y) = get_spawn_position(w, h);
        let enemy = Enemy::spawn(
            x,
            y,
            EnemyType::Chaser,
            chaser_stats,
            constants.spawn_target_offset,
        );
        gs.enemies.push(enemy);
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
