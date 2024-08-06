use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct DeepCut;

#[async_trait::async_trait]
impl Skill for DeepCut {
    fn kind(&self) -> SkillKind {
        SkillKind::DeepCut
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "deep_cut",
            name: "Corte Profundo",
            description: "Emite um corte profundo no alvo materializado com ether. Requer concentração para materializar, portanto precisa de um bom equilíbrio.", 
            explanation: "Habilidade de alto nível composta por duas fases: primeira, imaginar o corte no objeto-alvo, e em seguida, usar ether para romper o objeto em forma de corte. Era uma habilidade muito usada por organizações mercenárias na era de ouro da espada.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 45 },
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        api.fighter().balance > 82 && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(15..=30);
        let damage = base_damage + api.rng().gen_range(14..=18);

        let multiplier = fighter.intelligence_multiplier();
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Cut,
                amount: damage,
                balance_effectiveness: 15,
                accuracy: 100,
                effect: Some(Effect::new(EffectKind::Bleeding, 40, fighter.index))
            },
        ).await;

        api.emit_random_message(&[
            format!(
                "**{}** disparou um corte profundo em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** rasgou **{}** com um corte profundo que causou **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** se concentrou e materializou um corte profundo em **{}**, causando **{damage}**!",
                fighter.name, target.name
            ),
        ]);

        Ok(())
    }
}
