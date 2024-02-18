use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct ElectricSlap;

#[async_trait::async_trait]
impl Skill for ElectricSlap {
    fn kind(&self) -> SkillKind {
        SkillKind::ElectricSlap
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "tapa_elétrico",
            name: "Tapa Elétrico",
            description: "Acumula eletricidade na mão e dá um tapa com eletricidade concentrada.",
            explanation: "Eletricidade estática é a forma mais simples de conseguir eletricidade, depois basta aumentar com ether e você tem eletricidade de sobra para dar um poderoso tapa no seu inimigo.",
            complexity: SkillComplexity::Simple,
            use_cost: SkillCost { ether: 15 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(13..=17);

        let multiplier = (fighter.strength_multiplier() + fighter.intelligence_multiplier()) / 2.0;
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Electric,
                amount: damage,
                balance_effectiveness: 15,
                accuracy: 80,
                effect: Some(Effect::new(EffectKind::Shocked, 60, fighter.index))
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** deu um poderoso tapa elétrico em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
        );

        Ok(())
    }
}
