use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Refresh;

#[async_trait::async_trait]
impl Skill for Refresh {
    fn kind(&self) -> SkillKind {
        SkillKind::Refresh
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "refresh",
            name: "Refrescar",
            description: "Reduz muito todos os efeitos negativos de um aliado.",
            explanation: "Habilidade de renegeração: a categoria mais difícil do controle de ether. Alguns efeitos como fogo só precisam de um ether que apague, mas outros efeitos como sangramento precisam de um ether que regenera o ferimento. Requer muito ether.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 15 },
        }
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_>) -> Probability {
        let allies = api.get_fighter_allies(api.fighter_index);
        if allies.iter().any(|a| !a.effects.is_empty()){
            Probability::new(70)
        } else {
            Probability::new(10)
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();

        let ally = api_input::select_ally(&mut api).await?;
        let Some(ally) = ally else {
            api.report_error("erro tentando selecionar o aliado.");
            return Ok(());
        };

        let ally = api.battle_mut().get_fighter_mut(ally.index);
        ally.balance = ally.balance.saturating_add(30).clamp(0, 100);
        ally.remove_effect(Effect::new(EffectKind::Bleeding, 50, fighter.index));
        ally.remove_effect(Effect::new(EffectKind::Flaming, 80, fighter.index));
        ally.remove_effect(Effect::new(EffectKind::Burning, 50, fighter.index));
        ally.remove_effect(Effect::new(EffectKind::Ice, 70, fighter.index));
        ally.remove_effect(Effect::new(EffectKind::Shocked, 80, fighter.index));
        ally.remove_effect(Effect::new(EffectKind::Frozen, 2, fighter.index));
        ally.remove_effect(Effect::new(EffectKind::Paralyzed, 2, fighter.index));
        ally.remove_effect(Effect::new(EffectKind::Wet, 50, fighter.index));

        let ally = ally.clone();
        api.emit_message(format!("**{}** refrescou **{}** e melhorou vários efeitos negativos!", fighter.name, ally.name));

        Ok(())
    }
}
