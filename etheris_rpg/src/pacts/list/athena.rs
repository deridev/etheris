use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct AthenaPact;

#[async_trait::async_trait]
impl Pact for AthenaPact {
    fn kind(&self) -> PactKind {
        PactKind::Athena
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "athena",
            name: "Pacto de Atena",
            description: "Aumenta a inteligência.",
            explanation: "Pacto misterioso. Não se sabe onde surgiu, por que surgiu, mas virou um pacto importante e popular para vários trabalhadores intelectuais.",
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        fighter.intelligence_level += 7;
        Ok(())
    }
}
