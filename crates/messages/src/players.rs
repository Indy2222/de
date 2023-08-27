use bincode::{Decode, Encode};

/// Messages to be sent by a player/client or occasionally the game server to
/// other players.
#[derive(Debug, Encode, Decode)]
pub struct FromPlayers {
    /// ID of the sending player.
    source: u8,
    /// Original message.
    message: ToPlayers,
}

impl FromPlayers {
    pub fn new(source: u8, message: ToPlayers) -> Self {
        Self { source, message }
    }

    /// ID of the sending player
    pub fn source(&self) -> u8 {
        self.source
    }

    pub fn message(&self) -> &ToPlayers {
        &self.message
    }
}

/// Messages to be sent by a player/client or occasionally the game server to
/// other players.
#[derive(Debug, Encode, Decode)]
pub enum ToPlayers {}
