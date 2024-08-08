use crate::{ModKind, Modifier};

use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct AlchemistPact;

#[async_trait::async_trait]
impl Pact for AlchemistPact {
    fn kind(&self) -> PactKind {
        PactKind::Alchemist
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "alchemist",
            name: "Pacto do Alquimista",
            description: "Aumenta a regeneração de ether em 10%.",
            explanation: "Apesar do nome proibido e história conturbada, o pacto do alquimista é amplamente usado por aqueles com afeição com o ether, com diversos usos no dia a dia.",
        }   
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.modifiers.add(Modifier::new(ModKind::EtherRegenMultiplier(1.1), None).with_tag("pact_of_alchemist_ether_regen_boost"));
        Ok(())
    }
}
