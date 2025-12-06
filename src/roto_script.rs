use roto::{Runtime, Val, library};

use crate::enemy::{self, EnemyType};
use crate::visual_config::{
    BlendConfig, ColorConfig, EnemyVisualConfig, GameVisualConfig, PlayerVisualConfig,
    ProjectileVisualConfig,
};

#[derive(Clone, Copy, Debug)]
pub struct WaveConfig {
    pub basic_enemy_count: u32,
    pub chaser_enemy_count: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct GameConstants {
    pub out_of_bounds_margin: f32,
    pub spawn_target_offset: f32,
}

pub struct RotoScriptManager {
    runtime: Runtime,
}

impl RotoScriptManager {
    fn create_runtime() -> Runtime {
        let lib = library! {
            #[copy] type EntityStats = Val<enemy::EntityStats>;
            #[copy] type WaveComposition = Val<WaveConfig>;
            #[copy] type GameConstants = Val<GameConstants>;
            #[copy] type ColorConfig = Val<ColorConfig>;
            #[copy] type PlayerVisualConfig = Val<PlayerVisualConfig>;
            #[copy] type EnemyVisualConfig = Val<EnemyVisualConfig>;
            #[copy] type ProjectileVisualConfig = Val<ProjectileVisualConfig>;
            #[copy] type BlendConfig = Val<BlendConfig>;
            #[copy] type GameVisualConfig = Val<GameVisualConfig>;

            impl Val<enemy::EntityStats> {
                fn new(radius: f32, max_speed: f32, acceleration: f32, friction: f32) -> Val<enemy::EntityStats> {
                    Val(enemy::EntityStats { radius, max_speed, acceleration, friction })
                }
            }

            impl Val<WaveConfig> {
                fn new(basic_count: u32, chaser_count: u32) -> Val<WaveConfig> {
                    Val(WaveConfig { basic_enemy_count: basic_count, chaser_enemy_count: chaser_count })
                }
            }

            impl Val<GameConstants> {
                fn new(out_of_bounds_margin: f32, spawn_target_offset: f32) -> Val<GameConstants> {
                    Val(GameConstants { out_of_bounds_margin, spawn_target_offset })
                }
            }

            impl Val<ColorConfig> {
                fn new(r: f32, g: f32, b: f32, a: f32) -> Val<ColorConfig> {
                    Val(ColorConfig::new(r, g, b, a))
                }
                fn red() -> Val<ColorConfig> { Val(ColorConfig::red()) }
                fn green() -> Val<ColorConfig> { Val(ColorConfig::green()) }
                fn blue() -> Val<ColorConfig> { Val(ColorConfig::blue()) }
                fn yellow() -> Val<ColorConfig> { Val(ColorConfig::yellow()) }
                fn orange() -> Val<ColorConfig> { Val(ColorConfig::orange()) }
                fn purple() -> Val<ColorConfig> { Val(ColorConfig::purple()) }
                fn white() -> Val<ColorConfig> { Val(ColorConfig::white()) }
                fn black() -> Val<ColorConfig> { Val(ColorConfig::black()) }
            }

            impl Val<PlayerVisualConfig> {
                fn new(circle_color: Val<ColorConfig>, indicator_color: Val<ColorConfig>, indicator_size: f32) -> Val<PlayerVisualConfig> {
                    Val(PlayerVisualConfig { circle_color: circle_color.0, indicator_color: indicator_color.0, indicator_size })
                }
            }

            impl Val<EnemyVisualConfig> {
                fn new(circle_color: Val<ColorConfig>, indicator_color: Val<ColorConfig>, indicator_size: f32) -> Val<EnemyVisualConfig> {
                    Val(EnemyVisualConfig { circle_color: circle_color.0, indicator_color: indicator_color.0, indicator_size })
                }
            }

            impl Val<ProjectileVisualConfig> {
                fn new(primary_color: Val<ColorConfig>, secondary_color: Val<ColorConfig>, indicator_color: Val<ColorConfig>) -> Val<ProjectileVisualConfig> {
                    Val(ProjectileVisualConfig { primary_color: primary_color.0, secondary_color: secondary_color.0, indicator_color: indicator_color.0 })
                }
            }

            impl Val<BlendConfig> {
                fn new(inner_color: Val<ColorConfig>, outer_color: Val<ColorConfig>) -> Val<BlendConfig> {
                    Val(BlendConfig::new(inner_color.0, outer_color.0))
                }
            }

            impl Val<GameVisualConfig> {
                fn new(
                    player: Val<PlayerVisualConfig>,
                    basic_enemy: Val<EnemyVisualConfig>,
                    chaser_enemy: Val<EnemyVisualConfig>,
                    energy_ball: Val<ProjectileVisualConfig>,
                    pulse: Val<ProjectileVisualConfig>,
                    homing_missile: Val<ProjectileVisualConfig>,
                    pulse_blend: Val<BlendConfig>
                ) -> Val<GameVisualConfig> {
                    Val(GameVisualConfig {
                        player: player.0,
                        basic_enemy: basic_enemy.0,
                        chaser_enemy: chaser_enemy.0,
                        energy_ball: energy_ball.0,
                        pulse: pulse.0,
                        homing_missile: homing_missile.0,
                        pulse_blend: pulse_blend.0,
                    })
                }
            }
        };

        Runtime::from_lib(lib).unwrap()
    }

    pub fn new() -> Self {
        let runtime = Self::create_runtime();
        let mut manager = Self { runtime };
        manager.load_scripts();
        manager
    }

    fn load_scripts(&mut self) {
        match self.runtime.compile("waves.roto") {
            Ok(_) => {
                println!("✓ Loaded waves.roto successfully");
            }
            Err(err) => {
                eprintln!("ERROR loading waves.roto: {}", err);
            }
        }
    }

    pub fn reload(&mut self) {
        println!("Reloading waves.roto...");
        self.runtime = Self::create_runtime();
        self.load_scripts();
    }

    fn call_roto_function<F, R>(&mut self, _func_name: &str, call: F) -> Result<R, String>
    where
        F: FnOnce(&mut roto::Package) -> Result<R, String>,
    {
        let mut pkg = self
            .runtime
            .compile("waves.roto")
            .map_err(|err| format!("ERROR compiling waves.roto: {}", err))?;

        call(&mut pkg)
    }

    pub fn get_wave_config(&mut self, wave_num: u32) -> Result<WaveConfig, String> {
        self.call_roto_function("get_wave_composition", |pkg| {
            let func = pkg
                .get_function::<(), fn(u32) -> Val<WaveConfig>>("get_wave_composition")
                .map_err(|_| "ERROR: get_wave_composition function not found".to_string())?;
            Ok(func.call(&mut (), wave_num).0)
        })
    }

    pub fn get_enemy_stats(&mut self, enemy_type: EnemyType) -> Result<enemy::EntityStats, String> {
        let func_name = match enemy_type {
            EnemyType::Basic => "get_basic_enemy_stats",
            EnemyType::Chaser => "get_chaser_enemy_stats",
        };

        self.call_roto_function(func_name, |pkg| {
            let func = pkg
                .get_function::<(), fn() -> Val<enemy::EntityStats>>(func_name)
                .map_err(|_| format!("ERROR: {} function not found", func_name))?;
            Ok(func.call(&mut ()).0)
        })
    }

    pub fn get_player_stats(&mut self) -> Result<enemy::EntityStats, String> {
        self.call_roto_function("get_player_stats", |pkg| {
            let func = pkg
                .get_function::<(), fn() -> Val<enemy::EntityStats>>("get_player_stats")
                .map_err(|_| "ERROR: get_player_stats function not found".to_string())?;
            Ok(func.call(&mut ()).0)
        })
    }

    pub fn get_game_constants(&mut self) -> Result<GameConstants, String> {
        self.call_roto_function("get_game_constants", |pkg| {
            let func = pkg
                .get_function::<(), fn() -> Val<GameConstants>>("get_game_constants")
                .map_err(|_| "ERROR: get_game_constants function not found".to_string())?;
            Ok(func.call(&mut ()).0)
        })
    }

    pub fn get_visual_config(&mut self) -> Result<GameVisualConfig, String> {
        self.call_roto_function("get_visual_config", |pkg| {
            match pkg.get_function::<(), fn() -> Val<GameVisualConfig>>("get_visual_config") {
                Ok(func) => Ok(func.call(&mut ()).0),
                Err(_) => {
                    // If no visual config function found, return default
                    Ok(GameVisualConfig::default())
                }
            }
        })
    }
}
