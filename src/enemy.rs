use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};
use crate::entity::{EntityId, EntityStats};
use crate::visual_config::{EnemyVisualConfig, draw_direction_indicator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Basic,
    Chaser,
}

pub struct Enemy {
    pub id: EntityId,
    pub pos: Vec2,
    pub vel: Vec2,
    pub enemy_type: EnemyType,
    pub stats: EntityStats,
    pub visual_config: EnemyVisualConfig,
}

impl Enemy {
    pub fn override_stats(&mut self, stats: EntityStats) {
        self.stats = stats;
    }

    pub fn override_visual_config(&mut self, visual_config: EnemyVisualConfig) {
        self.visual_config = visual_config;
    }

    pub fn draw(&self) {
        draw_circle(
            self.pos.x,
            self.pos.y,
            self.stats.radius,
            self.visual_config.circle_color.to_color(),
        );

        // Draw direction indicator triangle
        draw_direction_indicator(
            self.pos,
            self.vel,
            self.stats.radius,
            self.visual_config.indicator_color,
            self.visual_config.indicator_size,
        );
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
