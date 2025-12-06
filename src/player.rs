use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};
use crate::enemy::EntityStats;

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    stats: EntityStats,
}

impl Player {
    pub fn new(x: f32, y: f32, stats: EntityStats) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            stats,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.stats.radius, YELLOW);
    }

    pub fn input(&mut self) {
        let mut acceleration = Vec2::ZERO;

        if is_key_down(KeyCode::Left) {
            acceleration.x -= self.stats.acceleration;
        }
        if is_key_down(KeyCode::Right) {
            acceleration.x += self.stats.acceleration;
        }
        if is_key_down(KeyCode::Up) {
            acceleration.y -= self.stats.acceleration;
        }
        if is_key_down(KeyCode::Down) {
            acceleration.y += self.stats.acceleration;
        }

        self.vel += acceleration;

        // Clamp velocity to max speed with proper normalization
        self.clamp_velocity();
    }

    pub fn update(&mut self) {
        self.pos += self.vel;

        // Apply friction
        self.vel *= self.stats.friction;
    }

    fn clamp_velocity(&mut self) {
        let speed = self.vel.length();
        if speed > self.stats.max_speed {
            self.vel = self.vel.normalize() * self.stats.max_speed;
        }
    }
}

impl Collidable for Player {
    fn collider(&self) -> Collider {
        Collider::Circle {
            radius: self.stats.radius,
        }
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}
