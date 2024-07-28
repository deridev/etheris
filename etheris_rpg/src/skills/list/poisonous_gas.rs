use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct PoisonousGas;

#[async_trait::async_trait]
impl Skill for PoisonousGas {
    fn kind(&self) -> SkillKind {
        SkillKind::PoisonousGas
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "poisonous_gas",
            name: "Gás Venenoso",
            description: "Emite um gás venenoso pelos poros do seu corpo que envenena seu alvo.",
            explanation: "Técnica de uma tribo isolada de assassinos da savana, o gás venenoso causa dano ao longo do tempo e pode acumular.",
            complexity: SkillComplexity::Simple,
            use_cost: SkillCost { ether: 25 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(3..=10);

        let multiplier = fighter.intelligence_multiplier();
        let damage = 3 + ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::Poisonous,
                amount: damage,
                balance_effectiveness: 7,
                accuracy: 100,
                effect: Some(Effect::new(EffectKind::Poisoned, 40, fighter.index)),
                ..Default::default()
            },
        ).await;

        api.emit_random_message(&[
            format!(
                "**{}** emitiu um gás tóxico em **{}**, causando **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** lançou um gás venenoso em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
        ]);

        Ok(())
    }
}
