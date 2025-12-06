use macroquad::prelude::*;

use crate::projectile::{Projectile, ProjectileStats};
use crate::visual_config::ProjectileVisualConfig;

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

impl WeaponStats {
    pub fn energy_ball_default() -> Self {
        Self {
            cooldown: 1.5, // Fire every 1.5 seconds
            projectile_count: 1,
            spread_angle: 0.0,
            projectile_stats: ProjectileStats::energy_ball_default(),
        }
    }

    pub fn pulse_default() -> Self {
        Self {
            cooldown: 3.0, // Fire every 3 seconds
            projectile_count: 1,
            spread_angle: 0.0, // Not used for pulse
            projectile_stats: ProjectileStats::pulse_default(),
        }
    }

    pub fn homing_missile_default() -> Self {
        Self {
            cooldown: 2.0, // Fire every 2 seconds
            projectile_count: 1,
            spread_angle: 0.0, // Not used for single homing missile
            projectile_stats: ProjectileStats::homing_missile_default(),
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
        let stats = match weapon_type {
            WeaponType::EnergyBall => WeaponStats::energy_ball_default(),
            WeaponType::Pulse => WeaponStats::pulse_default(),
            WeaponType::HomingMissile => WeaponStats::homing_missile_default(),
        };

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

    pub fn fire(
        &mut self,
        player_pos: Vec2,
        player_facing: Vec2,
        visual_config: ProjectileVisualConfig,
    ) -> Vec<Projectile> {
        if !self.can_fire() {
            return Vec::new();
        }

        // Reset cooldown
        self.cooldown_remaining = self.stats.cooldown;

        match self.weapon_type {
            WeaponType::EnergyBall => {
                self.fire_energy_ball(player_pos, player_facing, visual_config)
            }
            WeaponType::Pulse => self.fire_pulse(player_pos, visual_config),
            WeaponType::HomingMissile => {
                self.fire_homing_missile(player_pos, player_facing, visual_config)
            }
        }
    }

    fn fire_energy_ball(
        &self,
        player_pos: Vec2,
        player_facing: Vec2,
        visual_config: ProjectileVisualConfig,
    ) -> Vec<Projectile> {
        let mut projectiles = Vec::new();

        if self.stats.projectile_count == 1 {
            // Single projectile in facing direction
            let projectile = Projectile::spawn_energy_ball(
                player_pos,
                player_facing,
                self.stats.projectile_stats,
                visual_config,
            );
            projectiles.push(projectile);
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

                let projectile = Projectile::spawn_energy_ball(
                    player_pos,
                    direction,
                    self.stats.projectile_stats,
                    visual_config,
                );
                projectiles.push(projectile);
            }
        }

        projectiles
    }

    fn fire_pulse(
        &self,
        player_pos: Vec2,
        visual_config: ProjectileVisualConfig,
    ) -> Vec<Projectile> {
        let projectile =
            Projectile::spawn_pulse(player_pos, self.stats.projectile_stats, visual_config);
        vec![projectile]
    }

    fn fire_homing_missile(
        &self,
        player_pos: Vec2,
        player_facing: Vec2,
        visual_config: ProjectileVisualConfig,
    ) -> Vec<Projectile> {
        // For now, fire in facing direction. The homing behavior will take over during update
        let projectile = Projectile::spawn_homing_missile(
            player_pos,
            player_facing,
            self.stats.projectile_stats,
            visual_config,
        );
        vec![projectile]
    }

    fn rotate_vector(&self, vec: Vec2, angle_rad: f32) -> Vec2 {
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        Vec2::new(vec.x * cos_a - vec.y * sin_a, vec.x * sin_a + vec.y * cos_a)
    }

    // Future methods for level progression
    pub fn level_up(&mut self) {
        self.level += 1;
        // TODO: Update stats based on new level
        // This will be implemented later with Roto integration
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }
}
