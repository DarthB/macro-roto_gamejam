use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    
    pub vx: f32,
    pub vy: f32,
    pub v_max: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            v_max: 5.0,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, 20.0, YELLOW);
    }

    pub fn input(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.vx -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            self.vx += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            self.vy -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            self.vy += 1.0;
        }

        // Clamp velocity
        if self.vx > self.v_max {
            self.vx = self.v_max;
        }
        if self.vx < -self.v_max {
            self.vx = -self.v_max;
        }
        if self.vy > self.v_max {
            self.vy = self.v_max;
        }
        if self.vy < -self.v_max {
            self.vy = -self.v_max;
        }
    }

    pub fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        // Friction
        self.vx *= 0.9;
        self.vy *= 0.9;
    }
}