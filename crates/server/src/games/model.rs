use serde::{Deserialize, Serialize};

pub(super) const MAX_GAME_NAME_LEN: usize = 32;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Game {
    config: GameConfig,
    players: Vec<String>,
}

impl Game {
    // TODO doc
    pub(super) fn new(config: GameConfig, author: String) -> Self {
        Self {
            config,
            players: vec![author],
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GameConfig {
    name: String,
    max_players: u8,
}

impl GameConfig {
    pub(super) fn name(&self) -> &str {
        self.name.as_str()
    }

    pub(super) fn max_players(&self) -> u8 {
        self.max_players
    }
}

// TODO validate
// * max_players >= 2
// * name is valid (generalize)
