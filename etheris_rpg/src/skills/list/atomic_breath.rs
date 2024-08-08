use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct AtomicBreath;

#[async_trait::async_trait]
impl Skill for AtomicBreath {
    fn kind(&self) -> SkillKind {
        SkillKind::AtomicBreath
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "atomic_breath",
            name: "Sopro Atômico",
            description: "Sopra uma quantia alta de ether puro no seu inimigo, que ao acertar explode.",
            explanation: "Requer pulmões resistentes e prontos para liberar muito ether puro.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 60 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(10..=20); 
        let impact_damage = api.rng().gen_range(4..=8);
        let explosion_damage = api.rng().gen_range(20..=25);

        let multiplier = fighter.mixed_multiplier(0.9, 0.1);
        let impact_damage = base_damage + ((impact_damage as f32) * multiplier) as i32;
        let explosion_damage = base_damage + ((explosion_damage as f32) * multiplier * 0.8) as i32;

        let impact_damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::SpecialPhysical,
                amount: impact_damage,
                balance_effectiveness: 5,
                accuracy: 100,
                effect: None
            },
        ).await;

        let explosion_damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Special,
                amount: explosion_damage,
                balance_effectiveness: 15,
                accuracy: 70,
                effect: Some(Effect::new(EffectKind::Exhausted, 1, fighter.index))
            },
        ).await;

        if explosion_damage.missed {
            api.emit_message(
                format!(
                    "**{}** assoprou uma grande massa de ether puro em **{}** que causou **{impact_damage}** mas a explosão não causou dano!",
                    fighter.name, target.name
                ),
            );
        } else {
            api.emit_message(
                format!(
                    "**{}** assoprou uma grande massa de ether puro em **{}** que causou **{impact_damage}** e explodiu causando **{explosion_damage}**!",
                    fighter.name, target.name
                ),
            );
        }

        api.add_overload(api.fighter_index, 5.0).await;

        Ok(())
    }
}
