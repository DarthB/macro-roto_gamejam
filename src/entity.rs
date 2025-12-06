use macroquad::prelude::*;

use crate::enemy::EnemyType;
use crate::projectile::{ProjectileStats, ProjectileType};

pub type EntityId = u64;

#[derive(Debug, Clone, Copy)]
pub struct EntityStats {
    pub radius: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
}

#[derive(Debug)]
pub enum SpawnCommand {
    Projectile {
        projectile_type: ProjectileType,
        pos: Vec2,
        vel: Vec2,
        stats: ProjectileStats,
    },
    Enemy {
        enemy_type: EnemyType,
        pos: Vec2,
    },
}
