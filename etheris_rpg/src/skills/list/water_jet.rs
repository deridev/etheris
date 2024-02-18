use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct WaterJet;

#[async_trait::async_trait]
impl Skill for WaterJet {
    fn kind(&self) -> SkillKind {
        SkillKind::WaterJet
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "water_jet",
            name: "Jato de Água",
            description: "Lança um poderoso jato de água de extrema velocidade e pressão pela palma da mão. Útil para tirar equilíbrio, apagar fogo e dar dano em eletricidade.",
            explanation: "Requer materialização de muita água e ether suficiente pra lançar com muita potência pelas mãos, tornando-se assim uma habilidade complexa.",
            complexity: SkillComplexity::Simple,
            use_cost: SkillCost { ether: 15 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(5..=10);

        let multiplier = fighter.mixed_multiplier(0.5, 1.5);
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Water,
                amount: damage,
                balance_effectiveness: 30,
                accuracy: 80,
                effect: Some(Effect::new(EffectKind::Wet, 50, fighter.index))
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** disparou um jato de água em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
        );

        Ok(())
    }
}
