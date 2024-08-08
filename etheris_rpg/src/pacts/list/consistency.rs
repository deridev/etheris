use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct ConsistencyPact;

#[async_trait::async_trait]
impl Pact for ConsistencyPact {
    fn kind(&self) -> PactKind {
        PactKind::Consistency
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "consistency",
            name: "Pacto da Consistência",
            description: "Recupera equilíbrio mais rápido.",
            explanation: "Pacto criado por monges de Wornpeaks que dedicaram suas vidas ao equílibrio."
        }
    }

    async fn fighter_tick(&mut self, mut api: BattleApi<'_>) -> PactResult<()> {
        let fighter = api.fighter_mut();
        fighter.add_balance(2);
        Ok(())
    }
}
