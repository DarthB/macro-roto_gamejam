use macroquad::prelude::*;

const ENEMY_RADIUS: f32 = 15.0;
const ENEMY_MAX_SPEED: f32 = 3.0;
const ENEMY_ACCELERATION: f32 = 0.15;
const SPAWN_TARGET_OFFSET: f32 = 50.0;

pub struct Enemy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub v_max: f32,
    pub acc: f32,
}

impl Enemy {
    pub fn spawn(x: f32, y: f32, v_max: f32) -> Self {
        // random velocity to target on a circle in the center of the screen:
        let tx = screen_width() / 2.0 + rand::gen_range(-SPAWN_TARGET_OFFSET, SPAWN_TARGET_OFFSET);
        let ty = screen_height() / 2.0 + rand::gen_range(-SPAWN_TARGET_OFFSET, SPAWN_TARGET_OFFSET);

        let target = Vec2::new(tx, ty);
        let spawn_pos = Vec2::new(x, y);
        let dir = (target - spawn_pos).normalize();
        let speed = rand::gen_range(1.0, v_max);
        let vel = dir * speed;

        Self {
            pos: spawn_pos,
            vel,
            v_max,
            acc: ENEMY_ACCELERATION,
        }
    }

    pub fn radius(&self) -> f32 {
        ENEMY_RADIUS
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, ENEMY_RADIUS, RED);
    }

    pub fn update(&mut self) {
        // add acceleration in current direction
        let acc_dir = Vec2::new(
            if self.vel.x < 0.0 { -1.0 } else { 1.0 },
            if self.vel.y < 0.0 { -1.0 } else { 1.0 },
        );
        self.vel += acc_dir * self.acc;

        // clamp velocity to max speed
        self.clamp_velocity();

        self.pos += self.vel;
    }

    fn clamp_velocity(&mut self) {
        let speed = self.vel.length();
        if speed > self.v_max {
            self.vel = self.vel.normalize() * self.v_max;
        }
    }
}

pub const DEFAULT_ENEMY_MAX_SPEED: f32 = ENEMY_MAX_SPEED;
