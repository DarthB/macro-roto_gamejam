use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};
use crate::visual_config::{ProjectileVisualConfig, draw_direction_indicator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectileType {
    EnergyBall,
    Pulse,
    HomingMissile,
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectileStats {
    pub damage: f32,
    pub speed: f32,
    pub radius: f32, // For EnergyBall and HomingMissile (circle)
    pub width: f32,  // For Pulse (AABB)
    pub height: f32, // For Pulse (AABB)
    pub time_to_live: f32,
    pub turning_rate: f32, // For HomingMissile steering speed (radians per second)
}

impl ProjectileStats {
    pub fn energy_ball_default() -> Self {
        Self {
            damage: 10.0,
            speed: 300.0,
            radius: 8.0,
            width: 0.0,  // Not used for energy ball
            height: 0.0, // Not used for energy ball
            time_to_live: 2.0,
            turning_rate: 0.0, // Not used for energy ball
        }
    }

    pub fn pulse_default() -> Self {
        Self {
            damage: 15.0,
            speed: 0.0,  // Not used for pulse
            radius: 0.0, // Not used for pulse
            width: 100.0,
            height: 100.0,
            time_to_live: 0.3,
            turning_rate: 0.0, // Not used for pulse
        }
    }

    pub fn homing_missile_default() -> Self {
        Self {
            damage: 20.0,
            speed: 250.0,
            radius: 6.0,
            width: 0.0,  // Not used for homing missile
            height: 0.0, // Not used for homing missile
            time_to_live: 3.0,
            turning_rate: 3.0, // 3 radians per second turning rate
        }
    }
}

pub struct Projectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub projectile_type: ProjectileType,
    pub stats: ProjectileStats,
    pub time_remaining: f32,
    pub source_pos: Vec2, // Origin position (useful for pulse)
    pub visual_config: ProjectileVisualConfig,
}

impl Projectile {
    pub fn spawn_energy_ball(
        pos: Vec2,
        direction: Vec2,
        stats: ProjectileStats,
        visual_config: ProjectileVisualConfig,
    ) -> Self {
        let vel = direction.normalize() * stats.speed;
        Self {
            pos,
            vel,
            projectile_type: ProjectileType::EnergyBall,
            stats,
            time_remaining: stats.time_to_live,
            source_pos: pos,
            visual_config,
        }
    }

    pub fn spawn_pulse(
        pos: Vec2,
        stats: ProjectileStats,
        visual_config: ProjectileVisualConfig,
    ) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            projectile_type: ProjectileType::Pulse,
            stats,
            time_remaining: stats.time_to_live,
            source_pos: pos,
            visual_config,
        }
    }

    pub fn spawn_homing_missile(
        pos: Vec2,
        direction: Vec2,
        stats: ProjectileStats,
        visual_config: ProjectileVisualConfig,
    ) -> Self {
        let vel = direction.normalize() * stats.speed;
        Self {
            pos,
            vel,
            projectile_type: ProjectileType::HomingMissile,
            stats,
            time_remaining: stats.time_to_live,
            source_pos: pos,
            visual_config,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time_remaining -= dt;

        match self.projectile_type {
            ProjectileType::EnergyBall => {
                self.pos += self.vel * dt;
            }
            ProjectileType::Pulse => {
                // Pulse stays at source position, doesn't move
                self.pos = self.source_pos;
            }
            ProjectileType::HomingMissile => {
                self.pos += self.vel * dt;
                // Homing behavior is handled separately via update_homing
            }
        }
    }

    pub fn update_homing(&mut self, dt: f32, enemies: &[crate::enemy::Enemy]) {
        if self.projectile_type != ProjectileType::HomingMissile {
            return;
        }

        // Find nearest enemy
        let nearest_enemy = enemies.iter().min_by(|a, b| {
            let dist_a = (a.pos - self.pos).length_squared();
            let dist_b = (b.pos - self.pos).length_squared();
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(target) = nearest_enemy {
            let to_target = (target.pos - self.pos).normalize();
            let current_dir = self.vel.normalize();

            // Calculate desired turn angle
            let cross = current_dir.x * to_target.y - current_dir.y * to_target.x;
            let dot = current_dir.dot(to_target);
            let angle_diff = cross.atan2(dot);

            // Limit turning rate
            let max_turn = self.stats.turning_rate * dt;
            let turn_angle = angle_diff.clamp(-max_turn, max_turn);

            // Apply rotation to velocity
            let cos_turn = turn_angle.cos();
            let sin_turn = turn_angle.sin();
            let rotated_vel = Vec2::new(
                self.vel.x * cos_turn - self.vel.y * sin_turn,
                self.vel.x * sin_turn + self.vel.y * cos_turn,
            );

            self.vel = rotated_vel.normalize() * self.stats.speed;
        }
    }

    pub fn is_expired(&self) -> bool {
        self.time_remaining <= 0.0
    }

    pub fn draw(&self) {
        match self.projectile_type {
            ProjectileType::EnergyBall => {
                draw_circle(
                    self.pos.x,
                    self.pos.y,
                    self.stats.radius,
                    self.visual_config.primary_color.to_color(),
                );
            }
            ProjectileType::Pulse => {
                // Draw semi-transparent rectangle for pulse with fade
                let alpha = (self.time_remaining / self.stats.time_to_live).clamp(0.0, 1.0);
                let mut fill_color = self.visual_config.primary_color;
                fill_color.a *= alpha;

                draw_rectangle(
                    self.pos.x - self.stats.width / 2.0,
                    self.pos.y - self.stats.height / 2.0,
                    self.stats.width,
                    self.stats.height,
                    fill_color.to_color(),
                );

                // Draw outline
                draw_rectangle_lines(
                    self.pos.x - self.stats.width / 2.0,
                    self.pos.y - self.stats.height / 2.0,
                    self.stats.width,
                    self.stats.height,
                    2.0,
                    self.visual_config.secondary_color.to_color(),
                );
            }
            ProjectileType::HomingMissile => {
                // Draw circle for homing missile
                draw_circle(
                    self.pos.x,
                    self.pos.y,
                    self.stats.radius,
                    self.visual_config.primary_color.to_color(),
                );

                // Draw direction indicator (small triangle pointing in velocity direction)
                draw_direction_indicator(
                    self.pos,
                    self.vel,
                    self.stats.radius,
                    self.visual_config.indicator_color,
                    2.0,
                );
            }
        }
    }

    pub fn damage(&self) -> f32 {
        self.stats.damage
    }
}

impl Collidable for Projectile {
    fn collider(&self) -> Collider {
        match self.projectile_type {
            ProjectileType::EnergyBall | ProjectileType::HomingMissile => Collider::Circle {
                radius: self.stats.radius,
            },
            ProjectileType::Pulse => Collider::Rect {
                width: self.stats.width,
                height: self.stats.height,
            },
        }
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}
