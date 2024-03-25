use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Hakikotenchou;

#[async_trait::async_trait]
impl Skill for Hakikotenchou {
    fn kind(&self) -> SkillKind {
        SkillKind::Hakikotenchou
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "hakikotenchou",
            name: "Hakikotenchou",
            description: "Se congela e protege por 3 turnos para tirar a sobrecarga de ether do seu corpo. Durante os 3 turnos você perde a capacidade de agir.",
            explanation: "<HABILIDADE LENDÁRIA. EXPLICAÇÃO FALTANDO>",
            complexity: SkillComplexity::BeginnerMaster,
            use_cost: SkillCost { ether: 80 },
        }
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_>) -> Probability {
        if api.fighter().overload > 80.0 {
            Probability::new(60)
        } else {
            Probability::NEVER
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter_mut();
        fighter.overload = (fighter.overload - 80.0).max(0.0);
        
        let fighter = fighter.clone();
        api.emit_message(format!("**{}** usou Hakikotenchou e se congelou para reduzir a sobrecarga de ether!", fighter.name));

        api.apply_effect(fighter.index, Effect::new(EffectKind::Frozen, 3, fighter.index)).await;
        api.apply_effect(fighter.index, Effect::new(EffectKind::LowProtection, 6, fighter.index)).await;

        Ok(())
    }
}
