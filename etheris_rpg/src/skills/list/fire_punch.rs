use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FirePunch;

#[async_trait::async_trait]
impl Skill for FirePunch {
    fn kind(&self) -> SkillKind {
        SkillKind::FirePunch
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "fire_punch",
            name: "Soco em Chamas",
            description: "Imbui sua mão em fogo e desfere um poderoso soco em seu alvo.",
            explanation: "Requer materializar chamas na sua mão, o que fere a si mesmo mas aumenta grandemente o poder de ataque. ",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 25 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(10..=25);
        let damage = api.rng().gen_range(20..=30);
        let self_damage = api.rng().gen_range(3..=5);

        let multiplier = fighter.mixed_multiplier(0.7, 0.4);
        let damage = base_damage + ((damage as f32) * multiplier) as i32;
        let self_damage = ((self_damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Fire,
                amount: damage,
                balance_effectiveness: 20,
                accuracy: 90,
                effect: Some(Effect::new(EffectKind::Flaming, 30, fighter.index))
            },
        ).await;

        let self_damage = api.apply_damage(
            fighter.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Fire,
                amount: self_damage,
                balance_effectiveness: 5,
                accuracy: 100,
                effect: Some(Effect::new(EffectKind::Flaming, 15, fighter.index))
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** deu um soco flamejante em **{}** que causou **{damage}**! **{}** recebeu **{self_damage}** do fogo em suas mãos.",
                fighter.name, target.name, fighter.name
            ),
        );

        Ok(())
    }
}
