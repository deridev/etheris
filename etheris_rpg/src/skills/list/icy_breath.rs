use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct IcyBreath;

#[async_trait::async_trait]
impl Skill for IcyBreath {
    fn kind(&self) -> SkillKind {
        SkillKind::IcyBreath
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "icy_breath",
            name: "Sopro Gelado",
            description: "Sopra uma quantia alta de vento congelante no seu inimigo, tirando o equilíbrio e congelando.",
            explanation: "Essa habilidade se baseia em armazenar muito ar no pulmão e liberar tudo enquanto usa ether para aumentar a densidade do ar e reduzir a temperatura em níveis insanos.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 12 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(3..=5);

        let multiplier = fighter.intelligence_multiplier();
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Ice,
                amount: damage,
                balance_effectiveness: 30,
                accuracy: 90,
                effect: Some(Effect::new(EffectKind::Ice, 45, fighter.index))
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** assoprou uma grande massa de ar congelante em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
        );

        Ok(())
    }
}
