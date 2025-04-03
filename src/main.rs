use std::sync::{Arc, Mutex};
use tokio::task;
use reqwest::Client;
use rand::Rng;

struct Config {
    aimbot_enabled: bool,
    esp_enabled: bool,
    speed_hack_enabled: bool,
}

struct GameState {
    player_position: (f32, f32, f32),
    enemy_positions: Vec<(f32, f32, f32)>,
}

struct EFTMulti {
    config: Arc<Mutex<Config>>,
    game_state: Arc<Mutex<GameState>>,
    client: Client,
}

impl EFTMulti {
    fn new() -> Self {
        let config = Arc::new(Mutex::new(Config {
            aimbot_enabled: true,
            esp_enabled: true,
            speed_hack_enabled: false,
        }));
        let game_state = Arc::new(Mutex::new(GameState {
            player_position: (0.0, 0.0, 0.0),
            enemy_positions: vec![],
        }));
        let client = Client::new();
        EFTMulti { config, game_state, client }
    }

    async fn fetch_game_data(&self) {
        let response = self.client.get("http://game.api/data").send().await.unwrap();
        let data: GameState = response.json().await.unwrap();
        let mut game_state = self.game_state.lock().unwrap();
        *game_state = data;
    }

    fn aimbot(&self) {
        let config = self.config.lock().unwrap();
        if config.aimbot_enabled {
            let game_state = self.game_state.lock().unwrap();
            if let Some(target) = game_state.enemy_positions.first() {
                self.move_to_target(target);
            }
        }
    }

    fn move_to_target(&self, target: &(f32, f32, f32)) {
        let mut rng = rand::thread_rng();
        let offset_x: f32 = rng.gen_range(-1.0..1.0);
        let offset_y: f32 = rng.gen_range(-1.0..1.0);
        let new_position = (target.0 + offset_x, target.1 + offset_y, target.2);
        self.update_player_position(new_position);
    }

    fn update_player_position(&self, position: (f32, f32, f32)) {
        let mut game_state = self.game_state.lock().unwrap();
        game_state.player_position = position;
    }

    fn run(&self) {
        let this = self.clone();
        task::spawn(async move {
            loop {
                this.fetch_game_data().await;
                this.aimbot();
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });
    }
}

impl Clone for EFTMulti {
    fn clone(&self) -> Self {
        EFTMulti {
            config: Arc::clone(&self.config),
            game_state: Arc::clone(&self.game_state),
            client: self.client.clone(),
        }
    }
}

#[tokio::main]
async fn main() {
    let eft_multi = EFTMulti::new();
    eft_multi.run();
}