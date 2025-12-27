use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum PlayerInput {
    ChooseCard(usize),
    JudgeWinner(PlayerId),
}
