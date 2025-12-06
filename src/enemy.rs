use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};

const SPAWN_TARGET_OFFSET: f32 = 50.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Basic,
    Chaser,
}

pub struct EnemyStats {
    pub radius: f32,
    pub max_speed: f32,
    pub acceleration: f32,
}

pub fn get_enemy_stats(enemy_type: EnemyType) -> EnemyStats {
    match enemy_type {
        EnemyType::Basic => EnemyStats {
            radius: 15.0,
            max_speed: 3.0,
            acceleration: 0.15,
        },
        EnemyType::Chaser => EnemyStats {
            radius: 12.0,
            max_speed: 4.5,
            acceleration: 0.25,
        },
    }
}

pub struct Enemy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn spawn(x: f32, y: f32, enemy_type: EnemyType) -> Self {
        let stats = get_enemy_stats(enemy_type);

        // random velocity to target on a circle in the center of the screen:
        let tx = screen_width() / 2.0 + rand::gen_range(-SPAWN_TARGET_OFFSET, SPAWN_TARGET_OFFSET);
        let ty = screen_height() / 2.0 + rand::gen_range(-SPAWN_TARGET_OFFSET, SPAWN_TARGET_OFFSET);

        let target = Vec2::new(tx, ty);
        let spawn_pos = Vec2::new(x, y);
        let dir = (target - spawn_pos).normalize();
        let speed = rand::gen_range(1.0, stats.max_speed);
        let vel = dir * speed;

        Self {
            pos: spawn_pos,
            vel,
            enemy_type,
        }
    }

    pub fn draw(&self) {
        let stats = get_enemy_stats(self.enemy_type);
        let color = match self.enemy_type {
            EnemyType::Basic => RED,
            EnemyType::Chaser => ORANGE,
        };
        draw_circle(self.pos.x, self.pos.y, stats.radius, color);
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
        let stats = get_enemy_stats(self.enemy_type);

        // add acceleration in current direction
        let acc_dir = Vec2::new(
            if self.vel.x < 0.0 { -1.0 } else { 1.0 },
            if self.vel.y < 0.0 { -1.0 } else { 1.0 },
        );
        self.vel += acc_dir * stats.acceleration;

        // clamp velocity to max speed
        self.clamp_velocity();
    }

    fn update_chaser(&mut self, player_pos: Vec2) {
        let stats = get_enemy_stats(self.enemy_type);

        // Calculate direction to player
        let to_player = player_pos - self.pos;
        let distance = to_player.length();

        if distance > 1.0 {
            let desired_dir = to_player / distance;
            let desired_vel = desired_dir * stats.max_speed;

            // Steering: gradually adjust velocity toward desired velocity
            let steering = (desired_vel - self.vel) * stats.acceleration;
            self.vel += steering;
        }

        // clamp velocity to max speed
        self.clamp_velocity();
    }

    fn clamp_velocity(&mut self) {
        let stats = get_enemy_stats(self.enemy_type);
        let speed = self.vel.length();
        if speed > stats.max_speed {
            self.vel = self.vel.normalize() * stats.max_speed;
        }
    }
}

impl Collidable for Enemy {
    fn collider(&self) -> Collider {
        let stats = get_enemy_stats(self.enemy_type);
        Collider::Circle {
            radius: stats.radius,
        }
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}
