pub mod defaults;
mod simple;

pub use simple::SimpleBrain;

pub use etheris_data::BrainKind;

use crate::*;

#[async_trait::async_trait]
pub trait Brain {
    fn kind(&self) -> BrainKind;
    async fn should_risk_life(&mut self, mut api: BattleApi<'_>) -> bool;
    async fn select_input(&mut self, mut api: BattleApi<'_>) -> BattleInput;
    async fn allow_fighter_to_enter_his_team(
        &mut self,
        mut api: BattleApi<'_>,
        fighter: FighterIndex,
    ) -> bool;
}

pub fn make_brain(kind: BrainKind) -> Box<dyn Brain + Send + 'static> {
    match kind {
        BrainKind::Simple => Box::new(SimpleBrain),
    }
}
