use macroquad::prelude::*;

/// RGB color configuration that can be used with Roto
#[derive(Debug, Clone, Copy)]
pub struct ColorConfig {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorConfig {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_color(&self) -> Color {
        Color::new(self.r, self.g, self.b, self.a)
    }

    // Predefined colors for defaults
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    pub fn yellow() -> Self {
        Self::new(1.0, 1.0, 0.0, 1.0)
    }

    pub fn orange() -> Self {
        Self::new(1.0, 0.65, 0.0, 1.0)
    }

    pub fn purple() -> Self {
        Self::new(0.5, 0.0, 0.5, 1.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

/// Visual configuration for player
#[derive(Debug, Clone, Copy)]
pub struct PlayerVisualConfig {
    pub circle_color: ColorConfig,
    pub indicator_color: ColorConfig,
    pub indicator_size: f32, // Size multiplier for direction triangle
}

impl PlayerVisualConfig {
    pub fn default() -> Self {
        Self {
            circle_color: ColorConfig::yellow(),
            indicator_color: ColorConfig::green(),
            indicator_size: 3.0,
        }
    }
}

/// Visual configuration for enemies
#[derive(Debug, Clone, Copy)]
pub struct EnemyVisualConfig {
    pub circle_color: ColorConfig,
    pub indicator_color: ColorConfig,
    pub indicator_size: f32, // Size multiplier for direction triangle
}

impl EnemyVisualConfig {
    pub fn basic_default() -> Self {
        Self {
            circle_color: ColorConfig::red(),
            indicator_color: ColorConfig::white(),
            indicator_size: 3.0,
        }
    }

    pub fn chaser_default() -> Self {
        Self {
            circle_color: ColorConfig::orange(),
            indicator_color: ColorConfig::white(),
            indicator_size: 3.0,
        }
    }
}

/// Visual configuration for projectiles
#[derive(Debug, Clone, Copy)]
pub struct ProjectileVisualConfig {
    pub primary_color: ColorConfig,
    pub secondary_color: ColorConfig, // For blending/effects
    pub indicator_color: ColorConfig, // For direction indicators
}

impl ProjectileVisualConfig {
    pub fn energy_ball_default() -> Self {
        Self {
            primary_color: ColorConfig::purple(),
            secondary_color: ColorConfig::purple(), // Same as primary for now
            indicator_color: ColorConfig::white(),
        }
    }

    pub fn pulse_default() -> Self {
        Self {
            primary_color: ColorConfig::new(0.5, 0.0, 0.5, 0.3), // Semi-transparent purple
            secondary_color: ColorConfig::purple(),              // Outline color
            indicator_color: ColorConfig::white(),
        }
    }

    pub fn homing_missile_default() -> Self {
        Self {
            primary_color: ColorConfig::orange(),
            secondary_color: ColorConfig::yellow(), // For direction triangle
            indicator_color: ColorConfig::yellow(),
        }
    }
}

/// Blend configuration for effects like pulse
#[derive(Debug, Clone, Copy)]
pub struct BlendConfig {
    pub inner_color: ColorConfig,
    pub outer_color: ColorConfig,
}

impl BlendConfig {
    pub fn new(inner: ColorConfig, outer: ColorConfig) -> Self {
        Self {
            inner_color: inner,
            outer_color: outer,
        }
    }

    /// Blend between inner and outer colors based on t (0.0 = inner, 1.0 = outer)
    pub fn blend(&self, t: f32) -> ColorConfig {
        let t = t.clamp(0.0, 1.0);
        ColorConfig::new(
            self.inner_color.r + (self.outer_color.r - self.inner_color.r) * t,
            self.inner_color.g + (self.outer_color.g - self.inner_color.g) * t,
            self.inner_color.b + (self.outer_color.b - self.inner_color.b) * t,
            self.inner_color.a + (self.outer_color.a - self.inner_color.a) * t,
        )
    }

    pub fn pulse_default() -> Self {
        Self::new(
            ColorConfig::new(0.8, 0.2, 0.8, 0.8), // Bright purple center
            ColorConfig::new(0.3, 0.0, 0.3, 0.1), // Dark purple edge
        )
    }
}

/// Complete visual configuration for the game
#[derive(Debug, Clone, Copy)]
pub struct GameVisualConfig {
    pub player: PlayerVisualConfig,
    pub basic_enemy: EnemyVisualConfig,
    pub chaser_enemy: EnemyVisualConfig,
    pub energy_ball: ProjectileVisualConfig,
    pub pulse: ProjectileVisualConfig,
    pub homing_missile: ProjectileVisualConfig,
    pub pulse_blend: BlendConfig,
}

impl GameVisualConfig {
    pub fn default() -> Self {
        Self {
            player: PlayerVisualConfig::default(),
            basic_enemy: EnemyVisualConfig::basic_default(),
            chaser_enemy: EnemyVisualConfig::chaser_default(),
            energy_ball: ProjectileVisualConfig::energy_ball_default(),
            pulse: ProjectileVisualConfig::pulse_default(),
            homing_missile: ProjectileVisualConfig::homing_missile_default(),
            pulse_blend: BlendConfig::pulse_default(),
        }
    }
}

/// Helper function to draw a direction indicator triangle
pub fn draw_direction_indicator(
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    color: ColorConfig,
    size_multiplier: f32,
) {
    if vel.length() > 0.1 {
        let dir = vel.normalize();
        let tip = pos + dir * (radius + 5.0);
        let base_offset = dir * radius;
        let perpendicular = Vec2::new(-dir.y, dir.x) * size_multiplier;

        let p1 = tip;
        let p2 = pos + base_offset + perpendicular;
        let p3 = pos + base_offset - perpendicular;

        draw_triangle(p1, p2, p3, color.to_color());
    }
}
