use super::{defaults, Brain, BrainKind};
use crate::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct InsaneBrain;

#[async_trait::async_trait]
impl Brain for InsaneBrain {
    fn kind(&self) -> BrainKind {
        BrainKind::Insane
    }

    async fn should_risk_life(&mut self, _: BattleApi<'_>) -> bool {
        true
    }

    async fn select_input(&mut self, api: BattleApi<'_>) -> BattleInput {
        defaults::select_a_input(api).await
    }

    async fn allow_fighter_to_enter_his_team(
        &mut self,
        mut api: BattleApi<'_>,
        fighter: FighterIndex,
    ) -> bool {
        defaults::select_target(&mut api).await;
        defaults::allow_fighter_to_enter_his_team(api, fighter).await
    }
}
