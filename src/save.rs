use std::{fs, path::Path};

use crate::game::Game;

const SAVE_PATH: &str = "save/last_save.json";

pub fn save_exists() -> bool {
    Path::new(SAVE_PATH).exists()
}

pub fn save_game(game: &Game) -> Result<(), String> {
    let path = Path::new(SAVE_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let data = serde_json::to_string_pretty(game).map_err(|error| error.to_string())?;
    fs::write(path, data).map_err(|error| error.to_string())
}

pub fn load_game() -> Result<Game, String> {
    let data = fs::read_to_string(SAVE_PATH).map_err(|error| error.to_string())?;
    serde_json::from_str(&data).map_err(|error| error.to_string())
}
