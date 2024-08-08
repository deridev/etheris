use crate::ImmunityKind;

use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct RockPact;

#[async_trait::async_trait]
impl Pact for RockPact {
    fn kind(&self) -> PactKind {
        PactKind::Rock
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "rock",
            name: "Pacto da Rocha",
            description: "Aumenta a resistência a dano físico.",
            explanation: "Pacto simples criado por lutadores.",
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.body_immunities.add_resistance(ImmunityKind::Physical, 0.2); 
        Ok(())
    }
}
