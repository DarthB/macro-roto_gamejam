use macroquad::prelude::*;

use super::GameState;
use crate::weapon::{WeaponStats, WeaponType};

pub fn process(gs: &mut GameState) {
    // Check for weapon selection input
    if is_key_pressed(KeyCode::Key1) {
        gs.player.add_weapon(WeaponType::EnergyBall);
        gs.set_next_state(super::GameStateEnum::Playing);
    } else if is_key_pressed(KeyCode::Key2) {
        gs.player.add_weapon(WeaponType::Pulse);
        gs.set_next_state(super::GameStateEnum::Playing);
    } else if is_key_pressed(KeyCode::Key3) {
        gs.player.add_weapon(WeaponType::HomingMissile);
        gs.set_next_state(super::GameStateEnum::Playing);
    }
}

pub fn draw(gs: &GameState) {
    // Draw the playing state underneath (frozen)
    clear_background(BLACK);
    crate::gamestate::playing::draw(gs);

    // Draw semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.7),
    );

    // Draw title
    let title = "SELECT YOUR WEAPON";
    let title_size = 40.0;
    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    draw_text(
        title,
        screen_width() / 2.0 - title_width / 2.0,
        80.0,
        title_size,
        YELLOW,
    );

    // Draw weapon cards
    let card_width = 200.0;
    let card_height = 280.0;
    let card_spacing = 40.0;
    let total_width = card_width * 3.0 + card_spacing * 2.0;
    let start_x = (screen_width() - total_width) / 2.0;
    let card_y = 140.0;

    // Energy Ball Card (1)
    let energy_ball_stats = WeaponStats::from(WeaponType::EnergyBall);
    let energy_ball_desc = generate_weapon_description(
        WeaponType::EnergyBall,
        &energy_ball_stats,
        "Fast projectile that\ntravels straight. You AIM!",
    );
    draw_weapon_card(
        start_x,
        card_y,
        card_width,
        card_height,
        "1",
        "Energy Ball",
        &energy_ball_desc,
        BLUE,
    );

    // Pulse Card (2)
    let pulse_stats = WeaponStats::from(WeaponType::Pulse);
    let pulse_desc = generate_weapon_description(
        WeaponType::Pulse,
        &pulse_stats,
        "Area attack that\nexpands from player.",
    );
    draw_weapon_card(
        start_x + card_width + card_spacing,
        card_y,
        card_width,
        card_height,
        "2",
        "Pulse",
        &pulse_desc,
        GREEN,
    );

    // Homing Missile Card (3)
    let homing_stats = WeaponStats::from(WeaponType::HomingMissile);
    let homing_desc = generate_weapon_description(
        WeaponType::HomingMissile,
        &homing_stats,
        "Seeks nearest enemy\nand follows them.",
    );
    draw_weapon_card(
        start_x + (card_width + card_spacing) * 2.0,
        card_y,
        card_width,
        card_height,
        "3",
        "Homing Missile",
        &homing_desc,
        RED,
    );

    // Draw instruction
    let instruction = "Press 1, 2, or 3 to select";
    let instruction_size = 24.0;
    let instruction_width = measure_text(instruction, None, instruction_size as u16, 1.0).width;
    draw_text(
        instruction,
        screen_width() / 2.0 - instruction_width / 2.0,
        card_y + card_height + 60.0,
        instruction_size,
        LIGHTGRAY,
    );
}

fn generate_weapon_description(
    weapon_type: WeaponType,
    stats: &WeaponStats,
    flavor_text: &str,
) -> String {
    let projectile_stats = &stats.projectile_stats;

    // Calculate range based on projectile type
    let range = match weapon_type {
        WeaponType::EnergyBall | WeaponType::HomingMissile => {
            let distance = projectile_stats.speed * projectile_stats.time_to_live;
            if distance > 500.0 {
                "Long"
            } else if distance > 250.0 {
                "Medium"
            } else {
                "Short"
            }
        }
        WeaponType::Pulse => {
            let size = projectile_stats.width.max(projectile_stats.height);
            if size > 150.0 {
                "Large"
            } else if size > 75.0 {
                "Medium"
            } else {
                "Small"
            }
        }
    };

    // Categorize damage
    let damage_level = if projectile_stats.damage >= 20.0 {
        "High"
    } else if projectile_stats.damage >= 12.0 {
        "Medium"
    } else {
        "Low"
    };

    format!(
        "{}\n\nCooldown: {:.1}s\nDamage: {} ({})\nRange: {}",
        flavor_text, stats.cooldown, damage_level, projectile_stats.damage as i32, range
    )
}

fn draw_weapon_card(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    key: &str,
    name: &str,
    description: &str,
    color: Color,
) {
    // Draw card background
    draw_rectangle(x, y, width, height, Color::new(0.2, 0.2, 0.2, 0.95));

    // Draw card border
    draw_rectangle_lines(x, y, width, height, 3.0, color);

    // Draw key indicator at top
    let key_text = format!("[{}]", key);
    let key_size = 32.0;
    let key_width = measure_text(&key_text, None, key_size as u16, 1.0).width;
    draw_text(
        &key_text,
        x + width / 2.0 - key_width / 2.0,
        y + 40.0,
        key_size,
        color,
    );

    // Draw weapon icon (simple geometric representation)
    let icon_y = y + 70.0;
    draw_weapon_icon(x + width / 2.0, icon_y, name, color);

    // Draw weapon name
    let name_size = 22.0;
    let name_width = measure_text(name, None, name_size as u16, 1.0).width;
    draw_text(
        name,
        x + width / 2.0 - name_width / 2.0,
        y + 140.0,
        name_size,
        WHITE,
    );

    // Draw description (multi-line)
    let desc_size = 14.0;
    let desc_y_start = y + 170.0;
    let lines: Vec<&str> = description.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let line_width = measure_text(line, None, desc_size as u16, 1.0).width;
        draw_text(
            line,
            x + width / 2.0 - line_width / 2.0,
            desc_y_start + (i as f32 * 18.0),
            desc_size,
            LIGHTGRAY,
        );
    }
}

fn draw_weapon_icon(center_x: f32, center_y: f32, weapon_name: &str, color: Color) {
    match weapon_name {
        "Energy Ball" => {
            // Draw a glowing circle with rays
            draw_circle(center_x, center_y, 25.0, color);
            draw_circle(center_x, center_y, 20.0, WHITE);
            // Draw rays
            for i in 0..8 {
                let angle = (i as f32) * std::f32::consts::PI / 4.0;
                let ray_length = 15.0;
                let x1 = center_x + angle.cos() * 25.0;
                let y1 = center_y + angle.sin() * 25.0;
                let x2 = center_x + angle.cos() * (25.0 + ray_length);
                let y2 = center_y + angle.sin() * (25.0 + ray_length);
                draw_line(x1, y1, x2, y2, 3.0, color);
            }
        }
        "Pulse" => {
            // Draw concentric circles representing expanding wave
            draw_circle_lines(center_x, center_y, 35.0, 3.0, color);
            draw_circle_lines(
                center_x,
                center_y,
                25.0,
                3.0,
                Color::new(color.r, color.g, color.b, 0.7),
            );
            draw_circle_lines(
                center_x,
                center_y,
                15.0,
                3.0,
                Color::new(color.r, color.g, color.b, 0.4),
            );
            draw_circle(center_x, center_y, 8.0, WHITE);
        }
        "Homing Missile" => {
            // Draw a missile shape with trail
            let missile_length = 40.0;
            let missile_width = 12.0;

            // Missile body (pointing right)
            draw_triangle(
                Vec2::new(center_x + missile_length / 2.0, center_y),
                Vec2::new(
                    center_x - missile_length / 2.0,
                    center_y - missile_width / 2.0,
                ),
                Vec2::new(
                    center_x - missile_length / 2.0,
                    center_y + missile_width / 2.0,
                ),
                color,
            );

            // Fins
            draw_triangle(
                Vec2::new(
                    center_x - missile_length / 2.0,
                    center_y - missile_width / 2.0,
                ),
                Vec2::new(
                    center_x - missile_length / 2.0 - 10.0,
                    center_y - missile_width / 2.0 - 8.0,
                ),
                Vec2::new(center_x - missile_length / 2.0, center_y),
                RED,
            );
            draw_triangle(
                Vec2::new(
                    center_x - missile_length / 2.0,
                    center_y + missile_width / 2.0,
                ),
                Vec2::new(
                    center_x - missile_length / 2.0 - 10.0,
                    center_y + missile_width / 2.0 + 8.0,
                ),
                Vec2::new(center_x - missile_length / 2.0, center_y),
                RED,
            );

            // Highlight
            draw_circle(center_x + 5.0, center_y, 4.0, WHITE);
        }
        _ => {
            // Fallback icon
            draw_circle(center_x, center_y, 20.0, color);
        }
    }
}
