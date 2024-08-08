use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct PhoenixPact;

#[async_trait::async_trait]
impl Pact for PhoenixPact {
    fn kind(&self) -> PactKind {
        PactKind::Phoenix
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "phoenix",
            name: "Pacto da Fênix",
            description: "Cura a vida lentamente ao longo do tempo.",
            explanation: "Pacto criado por alquimistas que buscaram a imortalidade. Aqueles que o usam sentem a chama da vida se renovar dentro deles, curando suas feridas com o passar do tempo.",
        }
    }


    async fn fighter_tick(&mut self, mut api: BattleApi<'_>) -> PactResult<()> {
        let regen_amount = (api.fighter().health().max / 2) as f32 * 0.015;
        let mut regen_amount = api.rng().gen_range(0..=5) + (regen_amount as i32);

        if api.fighter().resistance.value <= 0 {
            regen_amount = (regen_amount as f32 * 0.8) as i32;
        }

        let fighter_index = api.fighter_index;
        api.fighter_mut().heal(fighter_index, regen_amount);

        Ok(())
    }
}
