use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct ElectricalDischarge;

#[async_trait::async_trait]
impl Skill for ElectricalDischarge {
    fn kind(&self) -> SkillKind {
        SkillKind::ElectricalDischarge
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "electrical_discharge",
            name: "Descarga Elétrica",
            description: "Junta ambas as mãos e dispara um trovão entre os dedos. A eletricidade é tão forte que acerta você também!",
            explanation: "Habilidade criada por aqueles que buscam controlar a natureza. Ao invés de se aproximar da natureza, uma habilidade letal de assassinato usado por clãs de Yiuricat surgiu. A Descarga Elétrica consiste em materializar uma quantia massiva de ether e lançar em alta velocidade, moldando com eletricidade natural da atmosfera.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 60 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(30..=40);
        let damage = api.rng().gen_range(20..=30);

        let self_base_damage = api.rng().gen_range(5..=8);
        let self_damage = api.rng().gen_range(4..=15);

        let multiplier = fighter.intelligence_multiplier();
        let damage = base_damage + ((damage as f32) * multiplier) as i32;

        let self_damage = self_base_damage + ((self_damage as f32) * multiplier * 0.9) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Electric,
                amount: damage,
                balance_effectiveness: 30,
                accuracy: 100,
                effect: Some(Effect::new(EffectKind::Shocked, 40, fighter.index))
            },
        ).await;

        let self_damage = api.apply_damage(
            fighter.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Electric,
                amount: self_damage,
                balance_effectiveness: 15,
                accuracy: 255,
                effect: Some(Effect::new(EffectKind::Shocked, 10, fighter.index))
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** descarregou eletricidade em **{}** que causou **{damage}** e **{self_damage}** em si mesmo!",
                fighter.name, target.name
            ),
        );

        Ok(())
    }
}
