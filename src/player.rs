use macroquad::prelude::*;

use crate::collision::{Collidable, Collider};
use crate::enemy::EntityStats;
use crate::projectile::Projectile;
use crate::weapon::{Weapon, WeaponType};

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub facing: Vec2, // Direction player is facing for weapon firing
    stats: EntityStats,
    weapons: Vec<Weapon>,
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
        }
    }

    pub fn override_stats(&mut self, stats: EntityStats) {
        self.stats = stats;
    }

    pub fn get_weapons(&self) -> &Vec<Weapon> {
        &self.weapons
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

        // Update facing direction based on movement
        if acceleration.length() > 0.0 {
            self.facing = acceleration.normalize();
        }

        // Clamp velocity to max speed with proper normalization
        self.clamp_velocity();
    }

    pub fn update(&mut self, dt: f32) -> Vec<Projectile> {
        self.pos += self.vel;

        // Apply friction
        self.vel *= self.stats.friction;

        // Update weapons and collect projectiles to spawn
        let mut new_projectiles = Vec::new();

        for weapon in &mut self.weapons {
            weapon.update(dt);
            let projectiles = weapon.fire(self.pos, self.facing);
            new_projectiles.extend(projectiles);
        }

        new_projectiles
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
