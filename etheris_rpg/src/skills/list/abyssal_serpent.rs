use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct AbyssalSerpent;

#[async_trait::async_trait]
impl Skill for AbyssalSerpent {
    fn kind(&self) -> SkillKind {
        SkillKind::AbyssalSerpent
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "abyssal_serpent",
            name: "Serpente Abissal",
            description: "Invoca uma serpente branca que morde seu inimigo, envenena e desaparece.",
            explanation: "Requer ligação com a serpente abissal, desprovida de luz.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 65 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(10..=17);
        let damage = api.rng().gen_range(20..=24);

        let multiplier = fighter.intelligence_multiplier() * 1.1;
        let damage = base_damage + ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Cut,
                amount: damage,
                balance_effectiveness: 17,
                accuracy: 95,
                effect: Some(Effect::new(EffectKind::Poisoned, 32, fighter.index))
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** invocou uma serpente abissal que mordeu **{}** e causou **{damage}**!", 
                fighter.name, target.name
            ),
        );

        Ok(())
    }
}
