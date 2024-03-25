use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct TornadoKick;

#[async_trait::async_trait]
impl Skill for TornadoKick {
    fn kind(&self) -> SkillKind {
        SkillKind::TornadoKick
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "tornado_kick",
            name: "Chute Tornado",
            description: "Pula no ar e se lança ao inimigo para acertar com um poderoso chute de uma perna.",
            explanation: "Necessita um pulo alto seguido de ether para se lançar em direção ao inimigo com impulso do vento, para acertar um chute com extrema potência e velocidade. Requer muita precisão.",
            complexity: SkillComplexity::Simple,
            use_cost: SkillCost { ether: 20 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(16..=22);

        let multiplier = fighter.mixed_multiplier(1.2, 0.6);
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::Physical,
                amount: damage,
                balance_effectiveness: 20,
                accuracy: 70,
                ..Default::default()
            },
        ).await;

        api.emit_random_message(&[
            format!(
                "**{}** se lançou ao ar e chutou **{}** com força, causando **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** deu um chute tornado em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
        ]);

        let target = api.target_mut();
        if target.resistance.value <= 0 && !target.flags.contains(FighterFlags::ASKED_TO_RISK_LIFE) && target.composure == Composure::Standing
        {
            target.composure = Composure::OnGround;

            let target_name = target.name.clone();
            api.emit_random_message(&[
                format!("A força do chute tornado foi tanta que **{}** caiu no chão.", target_name),
                format!("**{}** recebeu um chute tão forte que seu equilíbrio cedeu e caiu no chão.", target_name),
            ]);
        }

        Ok(())
    }
}
