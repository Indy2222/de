use anyhow::{Context, Result};
use log::info;
use sqlx::{query, Pool, Sqlite};
use thiserror::Error;

use super::model::Game;
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
    pub(super) async fn create(&self, game: Game) -> Result<(), CreationError> {
        let game_config = game.config();

        let result = query("INSERT INTO games (name, max_players) VALUES(?, ?);")
            .bind(game_config.name())
            .bind(game_config.max_players())
            .execute(self.pool)
            .await;
        collision!(result, CreationError::NameTaken);
        result.map_err(CreationError::Database)?;

        for username in game.players() {
            self.add_player(username, game_config.name()).await?;
        }

        Ok(())
    }

    pub(super) async fn add_player(&self, username: &str, game: &str) -> Result<()> {
        // TODO transaction
        // TODO return error if the game is full

        query("INSERT INTO players (username, game) VALUES(?, ?);")
            .bind(username)
            .bind(game)
            .execute(self.pool)
            .await
            .context("TODO")?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub(super) enum CreationError {
    #[error("Game name is already taken")]
    NameTaken,
    #[error("A database error encountered")]
    Database(#[source] sqlx::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
