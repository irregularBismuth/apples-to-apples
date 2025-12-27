use super::super::core::player::PlayerId;
use ahash::AHashMap as HashMap;
use dsl_ractor::{actor, actor_handle, actor_pre_start};
enum BotManagerMsg {
    AddBot,
}

struct BotManagerState {
    ///TODO: swap usize with some player actor ref
    bots: HashMap<PlayerId, usize>,
}

#[actor(msg=BotManagerMsg,state=BotManagerState)]
struct BotManager;

impl BotManager {
    actor_pre_start!(Ok(BotManagerState {
        bots: HashMap::new(),
    }));

    actor_handle!({
        match msg {
            BotManagerMsg::AddBot => {}
        }
        Ok(())
    });
}
