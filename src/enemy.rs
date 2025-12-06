use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Basic,
    Chaser,
}

#[derive(Debug, Clone, Copy)]
pub struct EntityStats {
    pub radius: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
}

pub struct Enemy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub enemy_type: EnemyType,
    stats: EntityStats,
}

impl Enemy {
    pub fn spawn(
        x: f32,
        y: f32,
        enemy_type: EnemyType,
        stats: EntityStats,
        spawn_target_offset: f32,
    ) -> Self {
        // random velocity to target on a circle in the center of the screen:
        let tx = screen_width() / 2.0 + rand::gen_range(-spawn_target_offset, spawn_target_offset);
        let ty = screen_height() / 2.0 + rand::gen_range(-spawn_target_offset, spawn_target_offset);

        let target = Vec2::new(tx, ty);
        let spawn_pos = Vec2::new(x, y);
        let dir = (target - spawn_pos).normalize();
        let speed = rand::gen_range(1.0, stats.max_speed);
        let vel = dir * speed;

        Self {
            pos: spawn_pos,
            vel,
            enemy_type,
            stats,
        }
    }

    pub fn draw(&self) {
        let color = match self.enemy_type {
            EnemyType::Basic => RED,
            EnemyType::Chaser => ORANGE,
        };
        draw_circle(self.pos.x, self.pos.y, self.stats.radius, color);
    }

    pub fn update(&mut self, player_pos: Option<Vec2>) {
        match self.enemy_type {
            EnemyType::Basic => self.update_basic(),
            EnemyType::Chaser => {
                if let Some(target) = player_pos {
                    self.update_chaser(target);
                } else {
                    self.update_basic();
                }
            }
        }

        self.pos += self.vel;
    }

    fn update_basic(&mut self) {
        // add acceleration in current direction
        let acc_dir = Vec2::new(
            if self.vel.x < 0.0 { -1.0 } else { 1.0 },
            if self.vel.y < 0.0 { -1.0 } else { 1.0 },
        );
        self.vel += acc_dir * self.stats.acceleration;

        // clamp velocity to max speed
        self.clamp_velocity();
    }

    fn update_chaser(&mut self, player_pos: Vec2) {
        // Calculate direction to player
        let to_player = player_pos - self.pos;
        let distance = to_player.length();

        if distance > 1.0 {
            let desired_dir = to_player / distance;
            let desired_vel = desired_dir * self.stats.max_speed;

            // Steering: gradually adjust velocity toward desired velocity
            let steering = (desired_vel - self.vel) * self.stats.acceleration;
            self.vel += steering;
        }

        // clamp velocity to max speed
        self.clamp_velocity();
    }

    fn clamp_velocity(&mut self) {
        let speed = self.vel.length();
        if speed > self.stats.max_speed {
            self.vel = self.vel.normalize() * self.stats.max_speed;
        }
    }
}

impl Collidable for Enemy {
    fn collider(&self) -> Collider {
        Collider::Circle {
            radius: self.stats.radius,
        }
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}
