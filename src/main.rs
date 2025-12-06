use macroquad::prelude::*;

mod collision;
mod enemy;
mod player;

use collision::{Collidable, check_collision};
use enemy::{DEFAULT_ENEMY_MAX_SPEED, Enemy};
use player::Player;

const DT: f64 = 1.0 / 30.0;

const OUT_OF_BOUNDS_MARGIN: f32 = 50.0;

struct GameState {
    player: Player,
    t_frame: f64,
    enemies: Vec<Enemy>,
    b_game_over: bool,
    wave: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(screen_width() / 2.0, screen_height() / 2.0),
            t_frame: 0.0,
            enemies: vec![],
            b_game_over: false,
            wave: 0,
        }
    }

    fn check_collisions(&mut self) {
        let enemies_collided_with_player: Vec<usize> = self
            .enemies
            .iter()
            .enumerate()
            .filter_map(|(i, enemy)| {
                let collision_data = check_collision(
                    &self.player.collider(),
                    self.player.position(),
                    &enemy.collider(),
                    enemy.position(),
                );
                if collision_data.collided {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        // no health system, just game over on first collision
        if !enemies_collided_with_player.is_empty() {
            self.b_game_over = true;
        }

        // later remove collided enemies
        for &i in enemies_collided_with_player.iter().rev() {
            self.despawn_enemy(i);
        }
    }

    fn despawn_enemy(&mut self, index: usize) {
        self.enemies.remove(index);
    }

    fn despawn_enemies_out_of_bounds(&mut self) {
        let w = screen_width();
        let h = screen_height();
        self.enemies.retain(|enemy| {
            enemy.pos.x >= -OUT_OF_BOUNDS_MARGIN
                && enemy.pos.x <= w + OUT_OF_BOUNDS_MARGIN
                && enemy.pos.y >= -OUT_OF_BOUNDS_MARGIN
                && enemy.pos.y <= h + OUT_OF_BOUNDS_MARGIN
        });
    }
}

fn input(gs: &mut GameState) {
    gs.player.input();
}

fn update(gs: &mut GameState) {
    gs.player.update();
    for enemy in gs.enemies.iter_mut() {
        enemy.update();
    }

    // this may trigger game over
    gs.check_collisions();

    gs.despawn_enemies_out_of_bounds();
}

fn draw(gs: &GameState) {
    gs.player.draw();
    for enemy in gs.enemies.iter() {
        enemy.draw();
    }
    draw_text(
        "Move the Player with arrow keys",
        20.0,
        20.0,
        20.0,
        DARKGRAY,
    );
    draw_text("Avoid the Red Enemies!", 20.0, 40.0, 20.0, DARKGRAY);
    let wave_text = format!("Wave: {}", gs.wave);
    draw_text(&wave_text, screen_width() - 120.0, 20.0, 20.0, DARKGRAY);
}

#[macroquad::main("Auto Scriptable by Roto")]
async fn main() {
    init_roto();

    let mut gs = GameState::new();
    let mut t_prev = get_time();
    let mut t_passed: f64 = 0.0;
    let mut n_logic_updates: u32 = 0;

    loop {
        if gs.b_game_over {
            clear_background(BLACK);
            draw_text(
                "GAME OVER",
                screen_width() / 2.0 - 80.0,
                screen_height() / 2.0,
                40.0,
                RED,
            );
            draw_text(
                "Press Return to Restart",
                screen_width() / 2.0 - 120.0,
                screen_height() / 2.0 + 50.0,
                20.0,
                DARKGRAY,
            );
            if is_key_pressed(KeyCode::Enter) {
                gs = GameState::new();
            }

            next_frame().await;
            continue;
        } else if gs.enemies.is_empty() {
            // spawn new wave
            let wave = gs.wave;
            spawn_wave(&mut gs, 10 + wave * 5);
            gs.wave += 1;
        }

        // update time counters
        gs.t_frame = get_time();
        t_passed += gs.t_frame - t_prev;

        // update logic at fixed time steps
        while t_passed >= DT {
            input(&mut gs);
            update(&mut gs);
            t_passed -= DT;
            n_logic_updates += 1;
        }

        if n_logic_updates > 0 {
            if n_logic_updates > 1 {
                println!("logic updates: {} - LOW FRAME RATE", n_logic_updates);
            }
            n_logic_updates = 0;
        }

        // render every frame:
        clear_background(BLACK);
        draw(&gs);

        t_prev = gs.t_frame;
        next_frame().await
    }
}

fn spawn_wave(gs: &mut GameState, n_enemies: u32) {
    let w = screen_width();
    let h = screen_height();
    for _ in 0..n_enemies {
        let x = if rand::gen_range(0, 2) == 0 {
            // left or right edge
            if rand::gen_range(0, 2) == 0 { 0.0 } else { w }
        } else {
            rand::gen_range(0.0, w)
        };
        let y = if x == 0.0 || x == w {
            rand::gen_range(0.0, h)
        } else if rand::gen_range(0, 2) == 0 {
            0.0
        } else {
            h
        };
        let enemy = Enemy::spawn(x, y, DEFAULT_ENEMY_MAX_SPEED);
        gs.enemies.push(enemy);
    }
}

use roto::Runtime;

fn init_roto() {
    // Step 1: Create a runtime
    let rt = Runtime::new();

    // Step 2: Compile the script and check for type errors
    let result = rt.compile("script.roto");
    let mut pkg = match result {
        Ok(pkg) => pkg,
        Err(err) => {
            panic!("{err}");
        }
    };

    // Step 3: Extract the function
    let func = pkg.get_function::<(), fn(i32) -> i32>("times_two").unwrap();

    // Step 4: Call the function
    let result = func.call(&mut (), 4);
    println!("times_two(4) = {result}");
}
