use crate::Composure;

use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct UnshakablePact {
    turn_count: u8,
}

#[async_trait::async_trait]
impl Pact for UnshakablePact {
    fn kind(&self) -> PactKind {
        PactKind::Unshakable
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "unshakable",
            name: "Pacto do Inabalável",
            description: "Duas vezes por batalha, se levanta imediatamente ao cair.",
            explanation: "Dizem que foi criado por duelistas, mas seu passado é um mistério. O Pacto do Inabalável é popular em combate.",
        }   
    }

    async fn fighter_tick(&mut self, mut api: BattleApi<'_>) -> PactResult<()> {
        if self.turn_count == 2 {
            return Ok(());
        }

        if api.fighter().composure == Composure::OnGround {
            self.turn_count += 1;
            
            let fighter = api.fighter_mut();
            fighter.composure = Composure::Standing;
            fighter.balance = fighter.balance.saturating_add(30).min(100);
            api.defer_message(format!("**{}** teve o corpo levantado pelo Pacto do Inabalável!", api.fighter().name));
        }

        Ok(())
    }
}
