use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct HerculesPact;

#[async_trait::async_trait]
impl Pact for HerculesPact {
    fn kind(&self) -> PactKind {
        PactKind::Hercules
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "hercules",
            name: "Pacto de Hércules",
            description: "Aumenta a força.",
            explanation: "Pacto misterioso. Não se sabe onde surgiu, por que surgiu, mas virou um pacto importante e popular para vários trabalhadores braçais.",
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.strength_level += 7;
        Ok(())
    }
}
