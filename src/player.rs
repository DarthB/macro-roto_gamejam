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
    pub xp: u32,
    pub level: u32,
}

impl Player {
    pub fn new(x: f32, y: f32, stats: EntityStats) -> Self {
        // Player starts without a weapon - it will be set by weapon selection popup
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            facing: Vec2::new(1.0, 0.0), // Start facing right
            stats,
            weapons: vec![],
            visual_config: PlayerVisualConfig::default(),
            xp: 0,
            level: 0,
        }
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.pos = Vec2::new(x, y);
        self.vel = Vec2::ZERO;
        self.facing = Vec2::new(1.0, 0.0);
        self.weapons.clear();
        self.xp = 0;
        self.level = 0;
    }

    pub fn xp_for_level(level: u32) -> u32 {
        // XP thresholds: 5, 15, 30, 50, 75, 105, 140, 180, 225, 275
        // Each level requires 5 more XP than the previous increment
        if level == 0 {
            0
        } else {
            let mut total = 0;
            for i in 1..=level {
                total += 5 * i;
            }
            total
        }
    }

    pub fn xp_for_next_level(&self) -> u32 {
        Self::xp_for_level(self.level + 1)
    }

    pub fn add_xp(&mut self, xp: u32) -> bool {
        self.xp += xp;

        // Check if we leveled up
        if self.xp >= self.xp_for_next_level() {
            self.level += 1;
            return true; // Level up occurred
        }
        false
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_xp(&self) -> u32 {
        self.xp
    }

    pub fn add_weapon(&mut self, weapon_type: WeaponType) {
        let weapon = Weapon::new(weapon_type);
        self.weapons.push(weapon);
    }

    pub fn level_up_weapon(&mut self, index: usize) {
        if index < self.weapons.len() {
            self.weapons[index].level_up();
        }
    }

    pub fn get_weapons_mut(&mut self) -> &mut Vec<Weapon> {
        &mut self.weapons
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
