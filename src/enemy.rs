use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};
use crate::visual_config::EnemyVisualConfig;

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
    visual_config: EnemyVisualConfig,
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

        let visual_config = match enemy_type {
            EnemyType::Basic => EnemyVisualConfig::basic_default(),
            EnemyType::Chaser => EnemyVisualConfig::chaser_default(),
        };

        Self {
            pos: spawn_pos,
            vel,
            enemy_type,
            stats,
            visual_config,
        }
    }

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
        if self.vel.length() > 0.1 {
            let dir = self.vel.normalize();
            let tip = self.pos + dir * (self.stats.radius + 5.0);
            let base_offset = dir * self.stats.radius;
            let perpendicular = Vec2::new(-dir.y, dir.x) * self.visual_config.indicator_size;

            let p1 = tip;
            let p2 = self.pos + base_offset + perpendicular;
            let p3 = self.pos + base_offset - perpendicular;

            draw_triangle(p1, p2, p3, self.visual_config.indicator_color.to_color());
        }
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
