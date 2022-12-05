use anyhow::{Context, Result};
use log::info;
use sqlx::{query, Pool, Sqlite};
use thiserror::Error;

use super::model::GameConfig;
use crate::{auth::model::MAX_USERNAME_LEN, collision, games::model::MAX_GAME_NAME_LEN};

#[derive(Clone)]
pub(super) struct Games {
    pool: &'static Pool<Sqlite>,
}

impl Games {
    // TODO document that this has to be called after Auth
    pub(super) async fn init(pool: &'static Pool<Sqlite>) -> Result<Self> {
        let init_query = format!(
            r#"
CREATE TABLE IF NOT EXISTS games (
    name CHARACTER({game_name_len}) NOT NULL PRIMARY KEY,
    max_players TINYINT
);

CREATE TABLE IF NOT EXISTS players (
    username CHARACTER({username_len}) NOT NULL,
    game CHARACTER({game_name_len}) NOT NULL,

    FOREIGN KEY(username) REFERENCES users(username)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY(game) REFERENCES games(name)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
"#,
            username_len = MAX_USERNAME_LEN,
            game_name_len = MAX_GAME_NAME_LEN,
        );

        info!("Initializing games...");
        query(&init_query)
            .execute(pool)
            .await
            .context("DB initialization failed")?;
        Ok(Self { pool })
    }

    // TODO docs
    pub(super) async fn create(&self, game: GameConfig) -> Result<(), CreationError> {
        let result = query("INSERT INTO games (name, max_players) VALUES(?, ?);")
            .bind(game.name())
            .bind(game.max_players())
            .execute(self.pool)
            .await;

        collision!(result, CreationError::NameTaken);
        result.map_err(CreationError::Database)?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub(super) enum CreationError {
    #[error("Game name is already taken")]
    NameTaken,
    #[error("A database error encountered")]
    Database(#[source] sqlx::Error),
}
