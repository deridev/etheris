use super::{defaults, Brain, BrainKind};
use crate::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct SimpleBrain;

#[async_trait::async_trait]
impl Brain for SimpleBrain {
    fn kind(&self) -> BrainKind {
        BrainKind::Simple
    }

    async fn should_risk_life(&mut self, api: BattleApi<'_>) -> bool {
        defaults::should_risk_life(api).await
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
