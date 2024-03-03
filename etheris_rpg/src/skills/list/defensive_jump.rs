use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct DefensiveJump;

#[async_trait::async_trait]
impl Skill for DefensiveJump {
    fn kind(&self) -> SkillKind {
        SkillKind::DefensiveJump
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "defensive_jump",
            name: "Pulo Defensivo",
            description: "Efetua um alto pulo de 2-3m enquanto defende seu corpo. Ficar no ar aumenta levemente seu dano e precisão!",
            explanation: "É um simples pulo defensivo. Não é uma habilidade especial, mas é um movimento estratégico inteligente.",
            complexity: SkillComplexity::VerySimple,
            use_cost: SkillCost { ether: 5 },
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        api.fighter().composure == Composure::Standing && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let meters = api.rng().gen_range(2..=3);
        api.fighter_mut().composure = Composure::OnAir(meters);
        api.fighter_mut().balance = api.fighter_mut().balance.saturating_add(5).min(100);
        api.fighter_mut().defense += 2;

        api.emit_message(format!("**{}** deu um salto defensivo de **{meters} metros**!", api.fighter().name));

        Ok(())
    }
}
