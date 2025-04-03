pub mod eft_multi {
    pub struct Config {
        pub aimbot_enabled: bool,
        pub esp_enabled: bool,
        pub speed_hack_enabled: bool,
    }

    pub struct GameState {
        pub player_position: (f32, f32, f32),
        pub enemy_positions: Vec<(f32, f32, f32)>,
    }

    pub struct EFTMulti {
        pub config: Config,
        pub game_state: GameState,
    }

    impl EFTMulti {
        pub fn new() -> Self {
            EFTMulti {
                config: Config {
                    aimbot_enabled: true,
                    esp_enabled: true,
                    speed_hack_enabled: false,
                },
                game_state: GameState {
                    player_position: (0.0, 0.0, 0.0),
                    enemy_positions: vec![],
                },
            }
        }

        pub fn update_game_state(&mut self, new_state: GameState) {
            self.game_state = new_state;
        }
    }
}