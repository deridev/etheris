use crate::{ModKind, Modifier};

use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct CouragePact;

#[async_trait::async_trait]
impl Pact for CouragePact {
    fn kind(&self) -> PactKind {
        PactKind::Courage
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "courage",
            name: "Pacto da Coragem",
            description: "Aumenta o dano em 50% quando a vida total está abaixo de 25%.",
            explanation: "Pacto criado pelo lendário general Hakiko, usado para derrotar seu rival mais poderoso durante a guerra.",
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.modifiers.add(Modifier::new(ModKind::DmgMultiplier(1.0), None).with_tag("pact_of_courage_dmg_augmentation"));
        Ok(())
    }

    async fn fighter_tick(&mut self, mut api: BattleApi<'_>) -> PactResult<()> {
        let fighter = api.fighter_mut();    
        let health_ratio = fighter.health().value as f64 / fighter.health().max as f64;
        let is_within_ratio = health_ratio < 0.25;

        if let Some(modifier) = fighter.modifiers.get_mut_with_tag("pact_of_courage_dmg_augmentation") {
            modifier.kind = ModKind::DmgMultiplier(if is_within_ratio { 1.5 } else { 1.0 });
        }

        Ok(())
    }
}
