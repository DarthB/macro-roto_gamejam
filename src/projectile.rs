use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectileType {
    EnergyBall,
    Pulse,
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectileStats {
    pub damage: f32,
    pub speed: f32,
    pub radius: f32, // For EnergyBall (circle)
    pub width: f32,  // For Pulse (AABB)
    pub height: f32, // For Pulse (AABB)
    pub time_to_live: f32,
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
}

impl Projectile {
    pub fn spawn_energy_ball(pos: Vec2, direction: Vec2, stats: ProjectileStats) -> Self {
        let vel = direction.normalize() * stats.speed;
        Self {
            pos,
            vel,
            projectile_type: ProjectileType::EnergyBall,
            stats,
            time_remaining: stats.time_to_live,
            source_pos: pos,
        }
    }

    pub fn spawn_pulse(pos: Vec2, stats: ProjectileStats) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            projectile_type: ProjectileType::Pulse,
            stats,
            time_remaining: stats.time_to_live,
            source_pos: pos,
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
        }
    }

    pub fn is_expired(&self) -> bool {
        self.time_remaining <= 0.0
    }

    pub fn draw(&self) {
        match self.projectile_type {
            ProjectileType::EnergyBall => {
                draw_circle(self.pos.x, self.pos.y, self.stats.radius, PURPLE);
            }
            ProjectileType::Pulse => {
                // Draw semi-transparent purple rectangle for pulse
                let alpha = (self.time_remaining / self.stats.time_to_live).clamp(0.0, 1.0);
                let color = Color::new(0.5, 0.0, 0.5, alpha * 0.3);

                draw_rectangle(
                    self.pos.x - self.stats.width / 2.0,
                    self.pos.y - self.stats.height / 2.0,
                    self.stats.width,
                    self.stats.height,
                    color,
                );

                // Draw outline
                draw_rectangle_lines(
                    self.pos.x - self.stats.width / 2.0,
                    self.pos.y - self.stats.height / 2.0,
                    self.stats.width,
                    self.stats.height,
                    2.0,
                    PURPLE,
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
            ProjectileType::EnergyBall => Collider::Circle {
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
