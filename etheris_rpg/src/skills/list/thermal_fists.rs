use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct ThermalFists;

#[async_trait::async_trait]
impl Skill for ThermalFists {
    fn kind(&self) -> SkillKind {
        SkillKind::ThermalFists
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "thermal_fists",
            name: "Punhos Térmicos",
            description: "Dois socos imbuídos, um em fogo e outro em gelo.",
            explanation: "Concentra ether nas mãos, criando um punho de fogo e outro de gelo. Requer controle preciso para manter as temperaturas extremas sem se ferir.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 35 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(4..=8); 
        let fire_damage = api.rng().gen_range(8..=14);
        let ice_damage = api.rng().gen_range(8..=14);

        let multiplier = fighter.mixed_multiplier(0.7, 0.4);
        let fire_damage = base_damage + ((fire_damage as f32) * multiplier) as i32;
        let ice_damage = base_damage + ((ice_damage as f32) * multiplier) as i32;

        let fire_damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Fire,
                amount: fire_damage,
                balance_effectiveness: 7,
                accuracy: 85,
                effect: Some(Effect::new(EffectKind::Flaming, 30, fighter.index))
            },
        ).await;

        let ice_damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Ice,
                amount: ice_damage,
                balance_effectiveness: 7,
                accuracy: 85,
                effect: Some(Effect::new(EffectKind::Ice, 25, fighter.index))
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** atacou **{}** com Punhos Térmicos! O soco de fogo causou **{fire_damage}** e o soco de gelo causou **{ice_damage}**!",
                fighter.name, target.name
            ),
        );

        Ok(())
    }
}