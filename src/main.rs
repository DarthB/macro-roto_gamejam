use std::rc::Rc;

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
        window_title: "Macro Roto - The Auto Battler".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(target_os = "macos")]
    {
        use std::env;
        if let Ok(exe_path) = env::current_exe()
            && let Some(exe_dir) = exe_path.parent()
        {
            let resources_dir = exe_dir.join("../Resources");
            let _ = env::set_current_dir(&resources_dir);
        }
    }

    let mut gs = GameState::new();

    gs.visual_config.char_tex = Some(Rc::new(load_texture("assets/elf_char.png").await.unwrap()));

    loop {
        match gs.state {
            GameStateEnum::WeaponSelection => {
                gamestate::weapon_selection::process(&mut gs);
                gamestate::weapon_selection::draw(&gs);
            }
            GameStateEnum::GameOver => {
                gamestate::gameover::process(&mut gs);
            }
            GameStateEnum::ScriptError => {
                gamestate::script_error::process(&mut gs);
            }
            GameStateEnum::Won => {
                gamestate::won::process(&mut gs);
            }
            GameStateEnum::Playing => {
                gs.process_global_input();
                gamestate::playing::process(&mut gs);
                clear_background(BLACK);
                gamestate::playing::draw(&gs);
            }
        }

        // Apply any pending state transitions
        gs.apply_next_state();

        next_frame().await
    }
}
