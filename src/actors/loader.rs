use dsl_ractor::*;

#[actor(msg=(),state=())]
pub struct CardLoader {}
impl CardLoader {
    actor_pre_start!(Ok(()));
    actor_handle!(Ok(()));
}
