use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct IcyShot;

#[async_trait::async_trait]
impl Skill for IcyShot {
    fn kind(&self) -> SkillKind {
        SkillKind::IcyShot
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "icy_shot",
            name: "Disparo Gélido",
            description: "Dispara uma bala de gelo afiada no seu alvo.",
            explanation: "Precisão com ether é algo complexo, por isso materializar uma bala afiada de ether e moldar o ether em gelo requer grande concentração e energia, principalmente na hora de lançar a bala com alta velocidade e precisão.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 30 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(15..=20);

        let multiplier = fighter.intelligence_multiplier();
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Ice,
                amount: damage,
                balance_effectiveness: 3,
                accuracy: 80,
                effect: Some(Effect::new(EffectKind::Ice, 25, fighter.index))
            },
        ).await;

        api.apply_effect(target.index, Effect::new(EffectKind::Bleeding, 40, fighter.index)).await;

        api.emit_message(
            format!(
                "**{}** lançou uma bala de gelo em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
        );

        Ok(())
    }
}
