use roto::{Runtime, Val, library};

use crate::enemy::{self, EnemyType};

#[derive(Clone, Copy, Debug)]
pub struct WaveConfig {
    pub basic_enemy_count: u32,
    pub chaser_enemy_count: u32,
}

pub struct RotoScriptManager {
    runtime: Runtime,
}

impl RotoScriptManager {
    pub fn new() -> Self {
        let lib = library! {
            #[copy] type EnemyStats = Val<enemy::EnemyStats>;
            #[copy] type WaveComposition = Val<WaveConfig>;

            impl Val<enemy::EnemyStats> {
                fn new(radius: f32, max_speed: f32, acceleration: f32) -> Val<enemy::EnemyStats> {
                    Val(enemy::EnemyStats { radius, max_speed, acceleration })
                }
            }

            impl Val<WaveConfig> {
                fn new(basic_count: u32, chaser_count: u32) -> Val<WaveConfig> {
                    Val(WaveConfig { basic_enemy_count: basic_count, chaser_enemy_count: chaser_count })
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
            #[copy] type EnemyStats = Val<enemy::EnemyStats>;
            #[copy] type WaveComposition = Val<WaveConfig>;

            impl Val<enemy::EnemyStats> {
                fn new(radius: f32, max_speed: f32, acceleration: f32) -> Val<enemy::EnemyStats> {
                    Val(enemy::EnemyStats { radius, max_speed, acceleration })
                }
            }

            impl Val<WaveConfig> {
                fn new(basic_count: u32, chaser_count: u32) -> Val<WaveConfig> {
                    Val(WaveConfig { basic_enemy_count: basic_count, chaser_enemy_count: chaser_count })
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

    pub fn get_enemy_stats(&mut self, enemy_type: EnemyType) -> Result<enemy::EnemyStats, String> {
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

        let func = match pkg.get_function::<(), fn() -> Val<enemy::EnemyStats>>(func_name) {
            Ok(f) => f,
            Err(_) => {
                return Err(format!("ERROR: {} function not found", func_name));
            }
        };

        let result = func.call(&mut ());
        Ok(result.0)
    }
}
