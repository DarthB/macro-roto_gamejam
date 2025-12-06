pub mod gameover;
pub mod playing;
pub mod script_error;
pub mod weapon_selection;

use macroquad::prelude::*;
use std::collections::HashSet;

use crate::collision::{Collidable, check_collision};
use crate::enemy::{Enemy, EnemyType};
use crate::entity::{EntityId, EntityStats, SpawnCommand};
use crate::player::Player;
use crate::projectile::{Projectile, ProjectileStats, ProjectileType};
use crate::roto_script::{GameConstants, RotoScriptManager};
use crate::visual_config::GameVisualConfig;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStateEnum {
    WeaponSelection,
    Playing,
    GameOver,
    ScriptError,
}

pub struct GameState {
    pub player: Player,
    pub t_frame: f64,
    pub t_prev: f64,
    pub t_passed: f64,
    pub n_logic_updates: u32,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub state: GameStateEnum,
    pub next_state: Option<GameStateEnum>,
    pub wave: u32,
    pub roto_manager: RotoScriptManager,
    pub error_message: Option<String>,
    pub paused: bool,
    pub visual_config: GameVisualConfig,
    pub game_constants: GameConstants,
    pub basic_enemy_stats: EntityStats,
    pub chaser_enemy_stats: EntityStats,
    pub next_entity_id: EntityId,
    pub enemies_to_despawn: HashSet<EntityId>,
    pub projectiles_to_despawn: HashSet<EntityId>,
}

impl GameState {
    pub fn new() -> Self {
        let mut roto_manager = RotoScriptManager::new();

        // Try to fetch player stats from Roto, fallback to defaults if it fails
        let player_stats = roto_manager.get_player_stats().unwrap_or(EntityStats {
            radius: 20.0,
            max_speed: 5.0,
            acceleration: 1.0,
            friction: 0.9,
        });

        let visual_config = roto_manager
            .get_visual_config()
            .unwrap_or(GameVisualConfig::default());

        let game_constants = roto_manager.get_game_constants().unwrap_or(GameConstants {
            out_of_bounds_margin: 50.0,
            spawn_target_offset: 100.0,
        });

        let basic_enemy_stats =
            roto_manager
                .get_enemy_stats(EnemyType::Basic)
                .unwrap_or(EntityStats {
                    radius: 15.0,
                    max_speed: 3.0,
                    acceleration: 0.5,
                    friction: 0.95,
                });

        let chaser_enemy_stats =
            roto_manager
                .get_enemy_stats(EnemyType::Chaser)
                .unwrap_or(EntityStats {
                    radius: 12.0,
                    max_speed: 4.0,
                    acceleration: 0.8,
                    friction: 0.95,
                });

        let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0, player_stats);
        player.override_visual_config(visual_config.player);

        Self {
            player,
            t_frame: get_time(),
            t_prev: get_time(),
            t_passed: 0.0,
            n_logic_updates: 0,
            enemies: vec![],
            projectiles: vec![],
            state: GameStateEnum::WeaponSelection,
            next_state: None,
            wave: 0,
            roto_manager,
            error_message: None,
            paused: false,
            visual_config,
            game_constants,
            basic_enemy_stats,
            chaser_enemy_stats,
            next_entity_id: 0,
            enemies_to_despawn: HashSet::new(),
            projectiles_to_despawn: HashSet::new(),
        }
    }

    pub fn check_collisions(&mut self) {
        // Check player-enemy collisions
        let mut game_over = false;
        for enemy in &self.enemies {
            let collision_data = check_collision(
                &self.player.collider(),
                self.player.position(),
                &enemy.collider(),
                enemy.position(),
            );
            if collision_data.collided {
                game_over = true;
                self.enemies_to_despawn.insert(enemy.id);
            }
        }

        if game_over {
            self.set_next_state(GameStateEnum::GameOver);
        }

        // Check projectile-enemy collisions
        self.check_projectile_enemy_collisions();

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

    fn check_projectile_enemy_collisions(&mut self) {
        for projectile in &self.projectiles {
            for enemy in &self.enemies {
                let collision_data = check_collision(
                    &projectile.collider(),
                    projectile.position(),
                    &enemy.collider(),
                    enemy.position(),
                );

                if collision_data.collided {
                    self.enemies_to_despawn.insert(enemy.id);
                    // Energy balls get removed on hit, pulses stay
                    match projectile.projectile_type {
                        ProjectileType::EnergyBall | ProjectileType::HomingMissile => {
                            self.projectiles_to_despawn.insert(projectile.id);
                        }
                        ProjectileType::Pulse => {
                            // Pulse continues to exist and can hit multiple enemies
                        }
                    }
                }
            }
        }
    }

    pub fn check_player_bounds(&mut self) {
        let w = screen_width();
        let h = screen_height();

        if self.player.pos.x < 0.0
            || self.player.pos.x > w
            || self.player.pos.y < 0.0
            || self.player.pos.y > h
        {
            self.set_next_state(GameStateEnum::GameOver);
        }
    }

    fn is_in_bounds(pos: Vec2, margin: f32) -> bool {
        let w = screen_width();
        let h = screen_height();
        pos.x >= -margin && pos.x <= w + margin && pos.y >= -margin && pos.y <= h + margin
    }

    pub fn despawn_enemies_out_of_bounds(&mut self) {
        let margin = self.game_constants.out_of_bounds_margin;

        for enemy in &self.enemies {
            if !Self::is_in_bounds(enemy.pos, margin) {
                self.enemies_to_despawn.insert(enemy.id);
            }
        }
    }

    pub fn update_time_for_logic(&mut self) -> u32 {
        // update time counters
        self.t_frame = get_time();
        self.t_passed += self.t_frame - self.t_prev;

        // update logic at fixed time steps
        while self.t_passed >= crate::DT {
            self.t_passed -= crate::DT;
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

    pub fn process_global_input(&mut self) {
        // Hot reload Roto scripts on 'R' key
        if is_key_pressed(KeyCode::R) {
            self.reload_roto_scripts();
        }

        // Toggle pause on 'P' key
        if is_key_pressed(KeyCode::P) {
            self.paused = !self.paused;
        }
    }

    pub fn reload_roto_scripts(&mut self) {
        match self.reload_roto_script_internal() {
            Ok(_) => {
                self.set_next_state(GameStateEnum::Playing);
                self.error_message = None;
            }
            Err(err) => {
                self.set_next_state(GameStateEnum::ScriptError);
                self.error_message = Some(err);
            }
        }
    }

    fn reload_roto_script_internal(&mut self) -> Result<(), String> {
        self.roto_manager.reload();

        self.player
            .override_stats(self.roto_manager.get_player_stats()?);

        // Reload game constants and enemy stats
        self.game_constants = self.roto_manager.get_game_constants()?;
        self.basic_enemy_stats = self.roto_manager.get_enemy_stats(EnemyType::Basic)?;
        self.chaser_enemy_stats = self.roto_manager.get_enemy_stats(EnemyType::Chaser)?;

        for enemy in self.enemies.iter_mut() {
            let stats = match enemy.enemy_type {
                EnemyType::Basic => self.basic_enemy_stats,
                EnemyType::Chaser => self.chaser_enemy_stats,
            };
            enemy.override_stats(stats);
        }

        // Reload visual configuration
        self.visual_config = self.roto_manager.get_visual_config()?;

        // Override visual configs for existing entities
        self.player
            .override_visual_config(self.visual_config.player);

        // Enemies will get updated visual config on next spawn
        // Existing enemies keep their current visual config (intentional)

        // Note: Projectiles get visual config when created, so existing ones keep their colors
        // This is actually desired behavior - projectiles in flight maintain their appearance

        Ok(())
    }

    fn spawn_projectile(&mut self, projectile_type: ProjectileType, pos: Vec2, vel: Vec2) {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        let stats = ProjectileStats::from(projectile_type);
        let visual_config = match projectile_type {
            ProjectileType::EnergyBall => self.visual_config.energy_ball,
            ProjectileType::Pulse => self.visual_config.pulse,
            ProjectileType::HomingMissile => self.visual_config.homing_missile,
        };

        let projectile = match projectile_type {
            ProjectileType::EnergyBall => {
                let normalized_vel = vel.normalize() * stats.speed;
                Projectile {
                    id,
                    pos,
                    vel: normalized_vel,
                    projectile_type: ProjectileType::EnergyBall,
                    stats,
                    time_remaining: stats.time_to_live,
                    source_pos: pos,
                    visual_config,
                }
            }
            ProjectileType::Pulse => Projectile {
                id,
                pos,
                vel: Vec2::ZERO,
                projectile_type: ProjectileType::Pulse,
                stats,
                time_remaining: stats.time_to_live,
                source_pos: pos,
                visual_config,
            },
            ProjectileType::HomingMissile => {
                let normalized_vel = vel.normalize() * stats.speed;
                Projectile {
                    id,
                    pos,
                    vel: normalized_vel,
                    projectile_type: ProjectileType::HomingMissile,
                    stats,
                    time_remaining: stats.time_to_live,
                    source_pos: pos,
                    visual_config,
                }
            }
        };

        self.projectiles.push(projectile);
    }

    pub fn spawn_enemy(&mut self, enemy_type: EnemyType, pos: Vec2) -> Result<(), String> {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        let stats = match enemy_type {
            EnemyType::Basic => self.basic_enemy_stats,
            EnemyType::Chaser => self.chaser_enemy_stats,
        };
        let visual_config = match enemy_type {
            EnemyType::Basic => self.visual_config.basic_enemy,
            EnemyType::Chaser => self.visual_config.chaser_enemy,
        };

        // Calculate random velocity toward center of screen with offset
        let tx = screen_width() / 2.0
            + rand::gen_range(
                -self.game_constants.spawn_target_offset,
                self.game_constants.spawn_target_offset,
            );
        let ty = screen_height() / 2.0
            + rand::gen_range(
                -self.game_constants.spawn_target_offset,
                self.game_constants.spawn_target_offset,
            );

        let target = Vec2::new(tx, ty);
        let dir = (target - pos).normalize();
        let speed = rand::gen_range(1.0, stats.max_speed);
        let vel = dir * speed;

        let enemy = Enemy {
            id,
            pos,
            vel,
            enemy_type,
            stats,
            visual_config,
        };

        self.enemies.push(enemy);
        Ok(())
    }

    pub fn execute_spawn_commands(&mut self, commands: Vec<SpawnCommand>) {
        for command in commands {
            match command {
                SpawnCommand::Projectile {
                    projectile_type,
                    pos,
                    vel,
                } => {
                    self.spawn_projectile(projectile_type, pos, vel);
                }
                SpawnCommand::Enemy { enemy_type, pos } => {
                    if let Err(err) = self.spawn_enemy(enemy_type, pos) {
                        eprintln!("Failed to spawn enemy: {}", err);
                    }
                }
            }
        }
    }

    pub fn process_despawns(&mut self) {
        // Award XP for each enemy killed
        let enemies_killed = self.enemies_to_despawn.len() as u32;
        if enemies_killed > 0 {
            // Award 1 XP per enemy killed
            let leveled_up = self.player.add_xp(enemies_killed);

            // If player leveled up, transition to weapon selection
            if leveled_up {
                self.set_next_state(GameStateEnum::WeaponSelection);
            }
        }

        self.enemies
            .retain(|e| !self.enemies_to_despawn.contains(&e.id));
        self.projectiles
            .retain(|p| !self.projectiles_to_despawn.contains(&p.id));
        self.enemies_to_despawn.clear();
        self.projectiles_to_despawn.clear();
    }

    pub fn set_next_state(&mut self, next_state: GameStateEnum) {
        self.next_state = Some(next_state);
    }

    pub fn apply_next_state(&mut self) {
        if let Some(next_state) = self.next_state.take() {
            // Handle state exit logic
            match self.state {
                GameStateEnum::WeaponSelection => {
                    // Exiting weapon selection - nothing to clean up
                }
                GameStateEnum::Playing => {
                    // Exiting playing state - nothing to clean up
                }
                GameStateEnum::GameOver => {
                    // Exiting game over - nothing to clean up
                }
                GameStateEnum::ScriptError => {
                    // Exiting script error - nothing to clean up
                }
            }

            // Handle state entry logic
            match next_state {
                GameStateEnum::WeaponSelection => {
                    // Entering weapon selection - nothing to initialize
                }
                GameStateEnum::Playing => {
                    // Entering playing state - ensure player has a weapon
                    self.t_prev = get_time();
                }
                GameStateEnum::GameOver => {
                    // Entering game over - reset player for next game
                    let w = screen_width();
                    let h = screen_height();
                    self.player.reset(w / 2.0, h / 2.0);
                }
                GameStateEnum::ScriptError => {
                    // Entering script error - nothing to initialize
                }
            }

            self.state = next_state;
        }
    }

    pub fn despawn_projectiles_out_of_bounds(&mut self) {
        let margin = self.game_constants.out_of_bounds_margin;

        for projectile in &self.projectiles {
            // Only remove energy balls and homing missiles that go out of bounds, keep pulses
            match projectile.projectile_type {
                ProjectileType::EnergyBall | ProjectileType::HomingMissile => {
                    if !Self::is_in_bounds(projectile.pos, margin) {
                        self.projectiles_to_despawn.insert(projectile.id);
                    }
                }
                ProjectileType::Pulse => {
                    // Pulses stay centered on player
                }
            }
        }
    }
}
