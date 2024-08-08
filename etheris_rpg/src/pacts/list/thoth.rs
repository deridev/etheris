use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct ThothPact;

#[async_trait::async_trait]
impl Pact for ThothPact {
    fn kind(&self) -> PactKind {
        PactKind::Thoth
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "thoth",
            name: "Pacto de Thoth",
            description: "Diminui a vida e aumenta o ether.", 
            explanation: "Pacto totalmente misterioso."
        }
    }

    fn setup_fighter(&mut self, fighter: &mut Fighter) -> PactResult<()> {
        let lost_resistance = (fighter.resistance.max as f32 * 0.08) as i32; 
        let lost_vitality = (fighter.vitality.max as f32 * 0.05) as i32;

        let extra_ether = 3 + (fighter.ether.max as f32 * 0.1) as i32;

        fighter.resistance.max -= lost_resistance;
        fighter.resistance.value -= lost_resistance;

        fighter.vitality.max -= lost_vitality;
        fighter.vitality.value -= lost_vitality;

        fighter.ether.max += extra_ether;
        fighter.ether.value += extra_ether;

        Ok(())
    }
}
