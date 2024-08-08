use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct ApolloPact;

#[async_trait::async_trait]
impl Pact for ApolloPact {
    fn kind(&self) -> PactKind {
        PactKind::Apollo
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "apollo",
            name: "Pacto de Apollo",
            description: "Aumenta a precisão.",
            explanation: "Pacto criado por atiradores de elite e caçadores, melhora o tempo de reação e a visão.",
        }
    }

    fn modify_damage(&mut self, damage: &mut DamageSpecifier) -> PactResult<()> {
        damage.accuracy = damage.accuracy.saturating_add(8);
        Ok(())
    }
}
