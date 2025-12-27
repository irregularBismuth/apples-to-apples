use dsl_ractor::*;

pub enum GameMsg {
    RoundStart,
    CardPlayed,
}

#[actor(msg=GameMsg, state =())]
struct GameHandler;

impl GameHandler {
    actor_pre_start!({ Ok(()) });

    actor_handle!({
        match msg {
            GameMsg::RoundStart => {}
            GameMsg::CardPlayed => {}
        }
        Ok(())
    });
}
