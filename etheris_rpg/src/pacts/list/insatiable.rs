use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct InsatiablePact;

#[async_trait::async_trait]
impl Pact for InsatiablePact {
    fn kind(&self) -> PactKind {
        PactKind::Insatiable
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "solidity",
            name: "Pacto do Insaciável",
            description: "Regenera vida a cada dano causado.",
            explanation: "O pacto daqueles que vivem em combate, criado por uma guilda de duelistas que foi extinta por um único homem.",
        }
    }

    async fn on_damage(&mut self, mut api: BattleApi<'_>, damage: DamageSpecifier) -> PactResult<()> {
        let multiplier = api.rng().gen_range(0.3..=0.6);
        let fighter = api.fighter_mut();

        let heal = (damage.amount / 30).max(1);
        let heal = (heal as f64 * multiplier) as i32;

        fighter.heal(fighter.index, heal);
        Ok(())
    }
}
