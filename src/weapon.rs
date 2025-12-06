use macroquad::prelude::*;

use crate::entity::SpawnCommand;
use crate::projectile::{ProjectileStats, ProjectileType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponType {
    EnergyBall,
    Pulse,
    HomingMissile,
}

#[derive(Debug, Clone, Copy)]
pub struct WeaponStats {
    pub cooldown: f32,
    pub projectile_count: u32,
    pub spread_angle: f32, // In degrees, for multiple projectiles
    pub projectile_stats: ProjectileStats,
}

impl From<WeaponType> for WeaponStats {
    fn from(weapon_type: WeaponType) -> Self {
        match weapon_type {
            WeaponType::EnergyBall => Self {
                cooldown: 1.5, // Fire every 1.5 seconds
                projectile_count: 1,
                spread_angle: 0.0,
                projectile_stats: ProjectileStats::from(ProjectileType::EnergyBall),
            },
            WeaponType::Pulse => Self {
                cooldown: 3.0, // Fire every 3 seconds
                projectile_count: 1,
                spread_angle: 0.0, // Not used for pulse
                projectile_stats: ProjectileStats::from(ProjectileType::Pulse),
            },
            WeaponType::HomingMissile => Self {
                cooldown: 2.0, // Fire every 2 seconds
                projectile_count: 1,
                spread_angle: 0.0, // Not used for single homing missile
                projectile_stats: ProjectileStats::from(ProjectileType::HomingMissile),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub level: u32, // For future use with Roto integration
    pub cooldown_remaining: f32,
    pub stats: WeaponStats,
}

impl Weapon {
    pub fn new(weapon_type: WeaponType) -> Self {
        let stats = WeaponStats::from(weapon_type);

        Self {
            weapon_type,
            level: 1,                // Start at level 1
            cooldown_remaining: 0.0, // Start ready to fire
            stats,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.cooldown_remaining > 0.0 {
            self.cooldown_remaining -= dt;
        }
    }

    pub fn can_fire(&self) -> bool {
        self.cooldown_remaining <= 0.0
    }

    pub fn fire(&mut self, player_pos: Vec2, player_facing: Vec2) -> Vec<SpawnCommand> {
        if !self.can_fire() {
            return Vec::new();
        }

        // Reset cooldown
        self.cooldown_remaining = self.stats.cooldown;

        match self.weapon_type {
            WeaponType::EnergyBall => self.fire_energy_ball(player_pos, player_facing),
            WeaponType::Pulse => self.fire_pulse(player_pos),
            WeaponType::HomingMissile => self.fire_homing_missile(player_pos, player_facing),
        }
    }

    fn fire_energy_ball(&self, player_pos: Vec2, player_facing: Vec2) -> Vec<SpawnCommand> {
        let mut commands = Vec::new();

        if self.stats.projectile_count == 1 {
            // Single projectile in facing direction
            let vel = player_facing.normalize() * self.stats.projectile_stats.speed;
            commands.push(SpawnCommand::Projectile {
                projectile_type: ProjectileType::EnergyBall,
                pos: player_pos,
                vel,
            });
        } else {
            // Multiple projectiles with spread
            let spread_rad = self.stats.spread_angle.to_radians();
            let angle_step = if self.stats.projectile_count > 1 {
                spread_rad * 2.0 / (self.stats.projectile_count - 1) as f32
            } else {
                0.0
            };

            for i in 0..self.stats.projectile_count {
                let angle_offset = -spread_rad + (i as f32) * angle_step;
                let direction = self.rotate_vector(player_facing, angle_offset);
                let vel = direction.normalize() * self.stats.projectile_stats.speed;

                commands.push(SpawnCommand::Projectile {
                    projectile_type: ProjectileType::EnergyBall,
                    pos: player_pos,
                    vel,
                });
            }
        }

        commands
    }

    fn fire_pulse(&self, player_pos: Vec2) -> Vec<SpawnCommand> {
        vec![SpawnCommand::Projectile {
            projectile_type: ProjectileType::Pulse,
            pos: player_pos,
            vel: Vec2::ZERO,
        }]
    }

    fn fire_homing_missile(&self, player_pos: Vec2, player_facing: Vec2) -> Vec<SpawnCommand> {
        // For now, fire in facing direction. The homing behavior will take over during update
        let vel = player_facing.normalize() * self.stats.projectile_stats.speed;
        vec![SpawnCommand::Projectile {
            projectile_type: ProjectileType::HomingMissile,
            pos: player_pos,
            vel,
        }]
    }

    fn rotate_vector(&self, vec: Vec2, angle_rad: f32) -> Vec2 {
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        Vec2::new(vec.x * cos_a - vec.y * sin_a, vec.x * sin_a + vec.y * cos_a)
    }

    // Level up the weapon, improving its stats
    pub fn level_up(&mut self) {
        self.level += 1;

        // Improve weapon stats based on weapon type and level
        match self.weapon_type {
            WeaponType::EnergyBall => {
                // Every 2 levels, add an additional projectile
                if self.level.is_multiple_of(2) {
                    self.stats.projectile_count += 1;
                    self.stats.spread_angle = 30.0; // 30 degree spread for multiple projectiles
                }
                // Reduce cooldown by 5% per level (min 0.5s)
                self.stats.cooldown = (self.stats.cooldown * 0.95).max(0.5);
                // Increase projectile speed by 5%
                self.stats.projectile_stats.speed *= 1.05;
                // Increase damage by 2
                self.stats.projectile_stats.damage += 2.0;
            }
            WeaponType::Pulse => {
                // Increase pulse size by 15 per level
                self.stats.projectile_stats.width += 15.0;
                self.stats.projectile_stats.height += 15.0;
                // Reduce cooldown by 5% per level (min 1.0s)
                self.stats.cooldown = (self.stats.cooldown * 0.95).max(1.0);
                // Increase damage by 3
                self.stats.projectile_stats.damage += 3.0;
                // Increase pulse duration slightly
                self.stats.projectile_stats.time_to_live += 0.05;
            }
            WeaponType::HomingMissile => {
                // Reduce cooldown by 8% per level (min 0.5s)
                self.stats.cooldown = (self.stats.cooldown * 0.92).max(0.5);
                // Increase damage by 4
                self.stats.projectile_stats.damage += 4.0;
                // Increase homing accuracy (turning rate) by 10%
                self.stats.projectile_stats.turning_rate *= 1.1;
                // Increase speed by 5%
                self.stats.projectile_stats.speed *= 1.05;
            }
        }
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }
}
