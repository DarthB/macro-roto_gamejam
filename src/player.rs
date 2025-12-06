use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};
use crate::entity::{EntityStats, SpawnCommand};
use crate::visual_config::{PlayerVisualConfig, draw_direction_indicator};
use crate::weapon::{Weapon, WeaponType};

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub facing: Vec2, // Direction player is facing for weapon firing
    stats: EntityStats,
    weapons: Vec<Weapon>,
    visual_config: PlayerVisualConfig,
}

impl Player {
    pub fn new(x: f32, y: f32, stats: EntityStats) -> Self {
        // Randomly select one of the three weapon types at game start
        let weapon_type = match rand::gen_range(0, 3) {
            0 => WeaponType::EnergyBall,
            1 => WeaponType::Pulse,
            _ => WeaponType::HomingMissile,
        };

        let weapon = Weapon::new(weapon_type);

        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            facing: Vec2::new(1.0, 0.0), // Start facing right
            stats,
            weapons: vec![weapon],
            visual_config: PlayerVisualConfig::default(),
        }
    }

    pub fn override_stats(&mut self, stats: EntityStats) {
        self.stats = stats;
    }

    pub fn override_visual_config(&mut self, visual_config: PlayerVisualConfig) {
        self.visual_config = visual_config;
    }

    pub fn get_weapons(&self) -> &Vec<Weapon> {
        &self.weapons
    }

    pub fn draw(&self) {
        draw_circle(
            self.pos.x,
            self.pos.y,
            self.stats.radius,
            self.visual_config.circle_color.to_color(),
        );

        // Draw direction indicator triangle
        let mouse_pos = mouse_position();
        let to_mouse = Vec2::new(mouse_pos.0, mouse_pos.1) - self.pos;
        draw_direction_indicator(
            self.pos,
            to_mouse,
            self.stats.radius,
            self.visual_config.indicator_color,
            self.visual_config.indicator_size,
        );
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

        // Update facing direction based on mouse cursor position
        let mouse_pos = mouse_position();
        let to_mouse = Vec2::new(mouse_pos.0, mouse_pos.1) - self.pos;
        if to_mouse.length() > 1.0 {
            self.facing = to_mouse.normalize();
        }

        // Clamp velocity to max speed with proper normalization
        self.clamp_velocity();
    }

    pub fn update(&mut self, dt: f32) -> Vec<SpawnCommand> {
        self.pos += self.vel;

        // Apply friction
        self.vel *= self.stats.friction;

        // Update weapons and collect spawn commands
        let mut spawn_commands = Vec::new();

        for weapon in &mut self.weapons {
            weapon.update(dt);
            let commands = weapon.fire(self.pos, self.facing);
            spawn_commands.extend(commands);
        }

        spawn_commands
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
