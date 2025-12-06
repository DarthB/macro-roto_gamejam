use macroquad::prelude::*;

mod collision;
mod enemy;
mod entity;
mod gamestate;
mod player;
mod projectile;
mod roto_script;
mod visual_config;
mod weapon;

use gamestate::{GameState, GameStateEnum};

pub const DT: f64 = 1.0 / 30.0;

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        window_title: "Auto Scriptable by Roto".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gs = GameState::new();

    loop {
        match gs.state {
            GameStateEnum::GameOver => {
                gamestate::gameover::process(&mut gs);
            }
            GameStateEnum::ScriptError => {
                gamestate::script_error::process(&mut gs);
            }
            GameStateEnum::Playing => {
                gs.process_global_input();
                gamestate::playing::process(&mut gs);
                clear_background(BLACK);
                gamestate::playing::draw(&gs);
            }
        }

        next_frame().await
    }
}
