use crate::{ModKind, Modifier};

use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct SolidityPact;

#[async_trait::async_trait]
impl Pact for SolidityPact {
    fn kind(&self) -> PactKind {
        PactKind::Solidity
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "solidity",
            name: "Pacto da Solidez",
            description: "Aumenta a resistência a danos em 15%.",
            explanation: "Pacto de Ether criado por uma aldeia oculta de Sunreach, o objetivo do pacto era tornar o corpo dos caçadores mais invulneráveis, sendo conhecido também como Pacto da Caça.",
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.modifiers.add(Modifier::new(ModKind::DefenseMultiplier(1.0 - 0.15), None).with_tag("pact_of_solidity_resistance_boost"));
        Ok(())
    }
}
