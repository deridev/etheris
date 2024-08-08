use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct MetisPact;

#[async_trait::async_trait]
impl Pact for MetisPact {
    fn kind(&self) -> PactKind {
        PactKind::Metis
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "metis",
            name: "Pacto de Métis",
            description: "Aumenta a vida levemente.",
            explanation: "Pacto misterioso. Não se sabe onde surgiu, por que surgiu, mas virou um pacto importante e popular para vários trabalhadores que arriscam a vida."
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        let extra_resistance = (fighter.resistance.max as f32 * 0.08) as i32; 
        let extra_vitality = (fighter.vitality.max as f32 * 0.05) as i32;

        fighter.resistance.max += extra_resistance;
        fighter.resistance.value += extra_resistance;

        fighter.vitality.max += extra_vitality;
        fighter.vitality.value += extra_vitality;
        Ok(())
    }
}
