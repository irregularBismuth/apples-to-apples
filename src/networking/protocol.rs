use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum PlayerInput {
    ChooseCard(usize),
    JudgeWinner(PlayerId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct PlayerId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientToServer {
    Join,
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerToClient {
    Welcome { player_id: PlayerId },
    Pong,
    Error { message: String },
    GameStart { players: Vec<PlayerId> },
}
