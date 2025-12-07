use macroquad::prelude::*;

use super::GameState;
use crate::weapon::{WeaponStats, WeaponType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponSelectionContext {
    InitialSelection, // First weapon at game start
    LevelUp,          // Level up - can pick new weapon or upgrade existing
}

pub fn process(gs: &mut GameState) {
    // Keys 1-3 always correspond to the three weapon types in order
    // Key 1: EnergyBall - add if don't have, upgrade if have
    // Key 2: Pulse - add if don't have, upgrade if have
    // Key 3: HomingMissile - add if don't have, upgrade if have

    if is_key_pressed(KeyCode::Key1) {
        handle_weapon_selection(gs, WeaponType::EnergyBall);
    } else if is_key_pressed(KeyCode::Key2) {
        handle_weapon_selection(gs, WeaponType::Pulse);
    } else if is_key_pressed(KeyCode::Key3) {
        handle_weapon_selection(gs, WeaponType::HomingMissile);
    }
}

fn handle_weapon_selection(gs: &mut GameState, weapon_type: WeaponType) {
    let weapons = gs.player.get_weapons();

    // Find if player already has this weapon type
    if let Some(index) = weapons.iter().position(|w| w.weapon_type == weapon_type) {
        // Player has this weapon - upgrade it
        gs.player.level_up_weapon(index);
        gs.set_next_state(super::GameStateEnum::Playing);
    } else {
        // Player doesn't have this weapon - add it (if room available)
        if weapons.len() < 3 {
            gs.player.add_weapon(weapon_type);
            gs.set_next_state(super::GameStateEnum::Playing);
        }
    }
}

pub fn draw(gs: &GameState) {
    // Draw the playing state underneath (frozen)
    clear_background(BLACK);

    if let Some(msg) = &gs.message_from_elf {
        let texture = &gs.visual_config.char_tex.as_ref().unwrap();

        let mut params = DrawTextureParams::default();
        let (w, h, s) = (texture.width(), texture.height(), 0.33);
        let x = 0.;
        let y = 0.;
        params.dest_size = Some(Vec2::new(w * s, h * s));

        draw_texture_ex(texture, x, y, WHITE, params);

        let x = 300.;
        let y = 60.;
        draw_text("The Guardian:", x, y, 32., YELLOW);

        let y = 100.;
        msg.split('.')
            .filter(|sentence| !sentence.is_empty())
            .enumerate()
            .for_each(|(i, sentence)| {
                let line = sentence.trim();
                draw_text(line, x, y + i as f32 * 22., 20., WHITE);
            });
    } else {
        crate::gamestate::playing::draw(gs);
        // Draw semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.7),
        );
    }

    let context = if gs.player.get_weapons().is_empty() {
        WeaponSelectionContext::InitialSelection
    } else {
        WeaponSelectionContext::LevelUp
    };

    draw_weapon_selection(gs, context);
}

fn draw_weapon_selection(gs: &GameState, context: WeaponSelectionContext) {
    // Draw title
    let title = match context {
        WeaponSelectionContext::InitialSelection => "SELECT OUR MAGIC!",
        WeaponSelectionContext::LevelUp => "LEVEL UP - SELECT OUR MAGIC!",
    };
    let title_size = 40.0;
    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    draw_text(
        title,
        screen_width() / 2.0 - title_width / 2.0,
        450.0,
        title_size,
        YELLOW,
    );

    // Draw weapon cards
    let card_width = 200.0;
    let card_height = 280.0;
    let card_spacing = 40.0;
    let card_y = 480.0;
    let total_width = card_width * 3.0 + card_spacing * 2.0;
    let start_x = (screen_width() - total_width) / 2.0;

    let all_weapon_types = [
        WeaponType::EnergyBall,
        WeaponType::Pulse,
        WeaponType::HomingMissile,
    ];

    let weapons = gs.player.get_weapons();

    // Draw all three weapon types
    for (i, weapon_type) in all_weapon_types.iter().enumerate() {
        let x = start_x + (card_width + card_spacing) * i as f32;
        let key = format!("{}", i + 1);
        let name = format!("{:?}", weapon_type);
        let color = get_weapon_color(*weapon_type);

        // Check if player has this weapon
        if let Some(weapon) = weapons.iter().find(|w| w.weapon_type == *weapon_type) {
            // Player has this weapon - show upgrade card
            draw_level_up_card(
                x,
                card_y,
                card_width,
                card_height,
                &key,
                &name,
                weapon,
                color,
            );
        } else {
            // Player doesn't have this weapon - show new weapon card
            let stats = WeaponStats::from(*weapon_type);

            // Always show flavor text
            let flavor_text = match weapon_type {
                WeaponType::EnergyBall => "Fast projectile that\ntravels straight. You AIM!",
                WeaponType::Pulse => "Area attack that\nexpands from player.",
                WeaponType::HomingMissile => "Seeks nearest enemy\nand follows them.",
            };

            let desc = generate_weapon_description(*weapon_type, &stats, flavor_text);
            draw_weapon_card(
                x,
                card_y,
                card_width,
                card_height,
                &key,
                &name,
                &desc,
                color,
            );
        }
    }

    // Draw level up subtitle below cards
    if context == WeaponSelectionContext::LevelUp {
        let subtitle = format!("Level {} - Choose an Upgrade", gs.player.get_level());
        let subtitle_size = 24.0;
        let subtitle_width = measure_text(&subtitle, None, subtitle_size as u16, 1.0).width;
        draw_text(
            &subtitle,
            screen_width() / 2.0 - subtitle_width / 2.0,
            card_y + card_height + 30.0,
            subtitle_size,
            YELLOW,
        );
    }

    // Draw instruction
    let (instruction, instruction_size) = match context {
        WeaponSelectionContext::InitialSelection => ("Press 1, 2, or 3 to select", 24.0),
        WeaponSelectionContext::LevelUp => ("Press 1-3 to upgrade or acquire weapon", 20.0),
    };
    let instruction_width = measure_text(instruction, None, instruction_size as u16, 1.0).width;
    draw_text(
        instruction,
        screen_width() / 2.0 - instruction_width / 2.0,
        card_y + card_height + 60.0,
        instruction_size,
        LIGHTGRAY,
    );
}

fn get_weapon_color(weapon_type: WeaponType) -> Color {
    match weapon_type {
        WeaponType::EnergyBall => BLUE,
        WeaponType::Pulse => GREEN,
        WeaponType::HomingMissile => RED,
    }
}

fn draw_level_up_card(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    key: &str,
    name: &str,
    weapon: &crate::weapon::Weapon,
    color: Color,
) {
    // Draw card background
    draw_rectangle(x, y, width, height, Color::new(0.2, 0.3, 0.2, 0.95));

    // Draw card border (thicker for level up)
    draw_rectangle_lines(x, y, width, height, 4.0, GOLD);

    // Draw key indicator
    let key_text = format!("[{}]", key);
    let key_size = 28.0;
    let key_width = measure_text(&key_text, None, key_size as u16, 1.0).width;
    draw_text(
        &key_text,
        x + width / 2.0 - key_width / 2.0,
        y + 35.0,
        key_size,
        GOLD,
    );

    // Draw weapon icon
    let icon_y = y + 60.0;
    draw_weapon_icon(x + width / 2.0, icon_y, name, color);

    // Draw weapon name and current level
    let name_text = format!("{} Lvl{}", name, weapon.get_level());
    let name_size = 18.0;
    let name_width = measure_text(&name_text, None, name_size as u16, 1.0).width;
    draw_text(
        &name_text,
        x + width / 2.0 - name_width / 2.0,
        y + 120.0,
        name_size,
        WHITE,
    );

    // Draw "UPGRADE" text
    let upgrade_text = "UPGRADE";
    let upgrade_size = 20.0;
    let upgrade_width = measure_text(upgrade_text, None, upgrade_size as u16, 1.0).width;
    draw_text(
        upgrade_text,
        x + width / 2.0 - upgrade_width / 2.0,
        y + 150.0,
        upgrade_size,
        GOLD,
    );

    // Draw current stats preview
    let stats_text = format!(
        "Cooldown: {:.1}s\nDamage: {}\nLevel: {} → {}",
        weapon.stats.cooldown,
        weapon.stats.projectile_stats.damage as i32,
        weapon.get_level(),
        weapon.get_level() + 1
    );
    let stats_size = 13.0;
    let stats_y_start = y + 175.0;
    for (i, line) in stats_text.lines().enumerate() {
        let line_width = measure_text(line, None, stats_size as u16, 1.0).width;
        draw_text(
            line,
            x + width / 2.0 - line_width / 2.0,
            stats_y_start + (i as f32 * 16.0),
            stats_size,
            LIGHTGRAY,
        );
    }
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
