use crate::ImmunityKind;

use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct FallenAngelPact;

#[async_trait::async_trait]
impl Pact for FallenAngelPact {
    fn kind(&self) -> PactKind {
        PactKind::FallenAngel
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "fallen_angel",
            name: "Pacto do Anjo-Caído",
            description: "Aumenta a resistência a cortes.",
            explanation: "Não se sabe ao certo quem criou esse pacto, mas boatos dizem que foi criado para enfrentar um terrível anjo caído.",
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.body_immunities.add_resistance(ImmunityKind::Cut, 0.3); 
        Ok(())
    }
}
