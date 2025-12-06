use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};

const PLAYER_RADIUS: f32 = 20.0;
const PLAYER_MAX_SPEED: f32 = 5.0;
const PLAYER_ACCELERATION: f32 = 1.0;
const PLAYER_FRICTION: f32 = 0.9;

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub v_max: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            v_max: PLAYER_MAX_SPEED,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, PLAYER_RADIUS, YELLOW);
    }

    pub fn input(&mut self) {
        let mut acceleration = Vec2::ZERO;

        if is_key_down(KeyCode::Left) {
            acceleration.x -= PLAYER_ACCELERATION;
        }
        if is_key_down(KeyCode::Right) {
            acceleration.x += PLAYER_ACCELERATION;
        }
        if is_key_down(KeyCode::Up) {
            acceleration.y -= PLAYER_ACCELERATION;
        }
        if is_key_down(KeyCode::Down) {
            acceleration.y += PLAYER_ACCELERATION;
        }

        self.vel += acceleration;

        // Clamp velocity to max speed with proper normalization
        self.clamp_velocity();
    }

    pub fn update(&mut self) {
        self.pos += self.vel;

        // Apply friction
        self.vel *= PLAYER_FRICTION;
    }

    fn clamp_velocity(&mut self) {
        let speed = self.vel.length();
        if speed > self.v_max {
            self.vel = self.vel.normalize() * self.v_max;
        }
    }
}

impl Collidable for Player {
    fn collider(&self) -> Collider {
        Collider::Circle {
            radius: PLAYER_RADIUS,
        }
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}
