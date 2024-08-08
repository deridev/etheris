use crate::{ModKind, Modifier};

use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct AresPact;

#[async_trait::async_trait]
impl Pact for AresPact {
    fn kind(&self) -> PactKind {
        PactKind::Ares
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "ares",
            name: "Pacto de Ares",
            description: "Dá 15% mais dano ao sobrecarregar seus músculos com ether.",
            explanation: "O pacto forjado em guerra que só trás a ruína e a solidão.",
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.modifiers.add(Modifier::new(ModKind::DmgMultiplier(1.15), None).with_tag("pact_of_ares_dmg_augmentation"));
        Ok(())
    }

    async fn fighter_tick(&mut self, mut api: BattleApi<'_>) -> PactResult<()> {
        api.add_overload(api.fighter_index, 0.15).await;
        Ok(())
    }
}
