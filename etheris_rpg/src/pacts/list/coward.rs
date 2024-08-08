use super::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct CowardPact;

#[async_trait::async_trait]
impl Pact for CowardPact {
    fn kind(&self) -> PactKind {
        PactKind::Coward
    }

    fn data(&self, _fighter: &Fighter) -> PactData {
        PactData {
            identifier: "coward",
            name: "Pacto do Covarde",
            description: "Aumenta a quebra de equilíbrio de cada ataque.",
            explanation: "Pacto banido em ringues de sumô. Criado por covardes, destinado aos mais fracos.",
        }
    }

    fn modify_damage(&mut self, damage: &mut DamageSpecifier) -> PactResult<()> {
        let extra_balance_break = if damage.balance_effectiveness > 20 { 5 } else { 3 };
        damage.balance_effectiveness = damage.balance_effectiveness.saturating_add(extra_balance_break);
        Ok(())
    }
}
