use crate::{ModKind, Modifier};

use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct HunterPact;

#[async_trait::async_trait]
impl Pact for HunterPact {
    fn kind(&self) -> PactKind {
        PactKind::Hunter
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "hunter",
            name: "Pacto do Caçador",
            description: "Aumenta seu dano em 25% em troca de receber mais dano.",
            explanation: "Pacto misterioso.",
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.modifiers.add(Modifier::new(ModKind::DefenseMultiplier(1.2), None).with_tag("pact_of_hunter_dmg_augmentation"));
        fighter.modifiers.add(Modifier::new(ModKind::DmgMultiplier(1.25), None).with_tag("pact_of_hunter_dmg_augmentation"));
        Ok(())
    }
}
