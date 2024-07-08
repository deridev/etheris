use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct ImbuedPunch;

#[async_trait::async_trait]
impl Skill for ImbuedPunch {
    fn kind(&self) -> SkillKind {
        SkillKind::ImbuedPunch
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "imbued_punch",
            name: "Soco Imbuído",
            description: "Ataca com um soco imbuído de ether, com impacto e força maior.",
            explanation: "Concentrar ether nos punhos para fortalecer o golpe é um dos ataques mais simples, porém eficaz.",
            complexity: SkillComplexity::VerySimple,
            use_cost: SkillCost { ether: 10 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let critical = Probability::new(5).generate_random_bool();

        let base_damage = api.rng().gen_range(3..=7); 
        let damage = api.rng().gen_range(if critical { 25..=33 } else { 15..=22 });

        let multiplier = fighter.mixed_multiplier(0.7, 0.4);
        let damage = base_damage + ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::SpecialPhysical,
                amount: damage,
                balance_effectiveness: if critical { 20 } else { 10 },
                accuracy: if critical { 99 } else { 80 },
                ..Default::default()
            },
        ).await;

        if critical {
            api.emit_random_message(&[
                format!(
                    "**{}** deu um soco imbuído na cara de **{}** que causou **{damage}**!",
                    fighter.name, target.name
                ),
                format!(
                    "**{}** socou o estômago de **{}** com os punhos imbuídos em ether e causou **{damage}**!",
                    fighter.name, target.name
                ),
            ]);
        } else {
            api.emit_random_message(&[
                format!(
                    "**{}** imbuiu ether nos punhos e deu um soco em **{}** que causou **{damage}**!",
                    fighter.name, target.name
                ),
                format!(
                    "**{}** deu um soco imbuído em ether em **{}** que causou **{damage}**!",
                    fighter.name, target.name
                ),
            ]);
        }

        let target = api.target_mut();

        if target.resistance.value <= 0 && !target.flags.contains(FighterFlags::ASKED_TO_RISK_LIFE)
        {
            target.ether.value = (target.ether.value as f32 * 0.8) as i32;

            let target_name = target.name.clone();
            api.emit_random_message(&[
                format!("A força do soco imbuído foi tanta que **{}** está semi-inconsciente e um pouco do seu ether vazou.", target_name),
                format!("**{}** recebeu um soco tão forte que seu ether vazou.", target_name),
            ]);
        }

        Ok(())
    }
}
