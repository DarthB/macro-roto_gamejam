use roto::{Runtime, Val, library};

use crate::enemy::{self, EnemyType};

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
    pub fn new() -> Self {
        let lib = library! {
            #[copy] type EntityStats = Val<enemy::EntityStats>;
            #[copy] type WaveComposition = Val<WaveConfig>;
            #[copy] type GameConstants = Val<GameConstants>;

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
        };

        let runtime = Runtime::from_lib(lib).unwrap();

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
        let lib = library! {
            #[copy] type EntityStats = Val<enemy::EntityStats>;
            #[copy] type WaveComposition = Val<WaveConfig>;
            #[copy] type GameConstants = Val<GameConstants>;

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
        };
        self.runtime = Runtime::from_lib(lib).unwrap();
        self.load_scripts();
    }

    pub fn get_wave_config(&mut self, wave_num: u32) -> Result<WaveConfig, String> {
        let result = self.runtime.compile("waves.roto");
        let mut pkg = match result {
            Ok(pkg) => pkg,
            Err(err) => {
                return Err(format!("ERROR compiling waves.roto: {}", err));
            }
        };

        let func = match pkg.get_function::<(), fn(u32) -> Val<WaveConfig>>("get_wave_composition")
        {
            Ok(f) => f,
            Err(_) => {
                return Err("ERROR: get_wave_composition function not found".to_string());
            }
        };

        let result = func.call(&mut (), wave_num);
        Ok(result.0)
    }

    pub fn get_enemy_stats(&mut self, enemy_type: EnemyType) -> Result<enemy::EntityStats, String> {
        let result = self.runtime.compile("waves.roto");
        let mut pkg = match result {
            Ok(pkg) => pkg,
            Err(err) => {
                return Err(format!("ERROR compiling waves.roto: {}", err));
            }
        };

        let func_name = match enemy_type {
            EnemyType::Basic => "get_basic_enemy_stats",
            EnemyType::Chaser => "get_chaser_enemy_stats",
        };

        let func = match pkg.get_function::<(), fn() -> Val<enemy::EntityStats>>(func_name) {
            Ok(f) => f,
            Err(_) => {
                return Err(format!("ERROR: {} function not found", func_name));
            }
        };

        let result = func.call(&mut ());
        Ok(result.0)
    }

    pub fn get_player_stats(&mut self) -> Result<enemy::EntityStats, String> {
        let result = self.runtime.compile("waves.roto");
        let mut pkg = match result {
            Ok(pkg) => pkg,
            Err(err) => {
                return Err(format!("ERROR compiling waves.roto: {}", err));
            }
        };

        let func = match pkg.get_function::<(), fn() -> Val<enemy::EntityStats>>("get_player_stats")
        {
            Ok(f) => f,
            Err(_) => {
                return Err("ERROR: get_player_stats function not found".to_string());
            }
        };

        let result = func.call(&mut ());
        Ok(result.0)
    }

    pub fn get_game_constants(&mut self) -> Result<GameConstants, String> {
        let result = self.runtime.compile("waves.roto");
        let mut pkg = match result {
            Ok(pkg) => pkg,
            Err(err) => {
                return Err(format!("ERROR compiling waves.roto: {}", err));
            }
        };

        let func = match pkg.get_function::<(), fn() -> Val<GameConstants>>("get_game_constants") {
            Ok(f) => f,
            Err(_) => {
                return Err("ERROR: get_game_constants function not found".to_string());
            }
        };

        let result = func.call(&mut ());
        Ok(result.0)
    }
}
