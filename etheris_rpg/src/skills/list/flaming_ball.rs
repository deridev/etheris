use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FlamingBall;

#[async_trait::async_trait]
impl Skill for FlamingBall {
    fn kind(&self) -> SkillKind {
        SkillKind::FlamingBall
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "flaming_ball",
            name: "Bola Flamejante",
            description: "Lança uma bola flamejante com alta velocidade no seu inimigo.",
            explanation: "Emitir ether em uma esfera semi-perfeita ao redor de sua mão e aplicar um feitiço de alteração para queimar o ether cria uma bola de fogo em suas mãos. Em seguida basta lançar uma rápida rajada de vento pela mão pra empurrar a bola flamejante até o alvo.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 50 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(10..=20);
        let damage = api.rng().gen_range(20..=35);

        let multiplier = fighter.intelligence_multiplier();
        let damage = base_damage + ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Fire,
                amount: damage,
                balance_effectiveness: 15,
                accuracy: 70,
                effect: Some(Effect::new(EffectKind::Flaming, 45, fighter.index))
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** lançou uma bola flamejante em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
        );

        Ok(())
    }
}
