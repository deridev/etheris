use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SimpleCut;

#[async_trait::async_trait]
impl Skill for SimpleCut {
    fn kind(&self) -> SkillKind {
        SkillKind::SimpleCut
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "simple_cut",
            name: "Corte Simples",
            description: "Dispara uma rajada de ether afiada e pequena para efetuar cortes superficiais",
            explanation: "Existem duas formas de fazer cortes utilizando ether: Um corte real, onde o ether é usado para romper o objeto-alvo e causar um corte profundo, e um corte simples, que apenas lança ether extremamente fino para cortar. Essa habilidade faz um corte simples afiando e lançando uma rajada de ar imbuída com ether.",
            complexity: SkillComplexity::Simple,
            use_cost: SkillCost { ether: 8 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(6..=10);
        let damage = base_damage + api.rng().gen_range(10..=14);

        let multiplier = fighter.intelligence_multiplier() * 0.85;
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Cut,
                amount: damage,
                balance_effectiveness: 5,
                accuracy: 95,
                effect: Some(Effect::new(EffectKind::Bleeding, 25, fighter.index))
            },
        ).await;

        api.emit_random_message(&[
            format!(
                "**{}** disparou um corte simples em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** afiou e atirou ether em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** afiou um pouco de ether e lançou em **{}**, causando **{damage}**!",
                fighter.name, target.name
            ),
        ]);

        Ok(())
    }
}
