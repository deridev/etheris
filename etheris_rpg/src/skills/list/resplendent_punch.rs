use super::prelude::*;

#[derive(Debug, Clone)]
pub struct ResplendentPunch {
    accuracy: u8,
    extra_cost: i32,
}

impl Default for ResplendentPunch {
    fn default() -> Self {
        Self { accuracy: 40, extra_cost: 0 }
    }
}

#[async_trait::async_trait]
impl Skill for ResplendentPunch {
    fn kind(&self) -> SkillKind {
        SkillKind::ResplendentPunch
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "resplendent_punch",
            name: "Soco Resplandecente",
            description: "Ataca com um soco imbuído de muito ether, mas com baixa chance de acertar. Se resplandecer, o inimigo não poderá regenerar ether por 5 turnos e você aumenta o seu próprio dano.",
            explanation: "Utiliza o mesmo princípio do Soco Imbuído, mas é intensificiado pela metaconsciência do ether por ter uma condição de uso (baixa chance de acerto), tornando o ether mais poderoso.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 40 + self.extra_cost },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let critical = Probability::new(10).generate_random_bool();

        let base_damage = api.rng().gen_range(5..=15); 
        let damage = base_damage + api.rng().gen_range(if critical { 30..=40 } else { 25..=30 });

        let multiplier = (fighter.strength_multiplier() + fighter.intelligence_multiplier()) / 2.0;
        let damage = ((damage as f32) * multiplier) as i32;

        let accuracy = self.accuracy + if critical { 15 } else { 0 };

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::SpecialPhysical,
                amount: damage,
                balance_effectiveness: if critical { 24 } else { 13 },
                accuracy,
                effect: Some(Effect::new(EffectKind::Exhausted, 5, fighter.index)),
                ..Default::default()
            },
        ).await;

        if damage.missed {
            api.add_overload(api.fighter_index, 0.85).await;
            self.accuracy = self.accuracy.saturating_add(6);
            self.extra_cost = self.extra_cost.saturating_add(2);
        } else {
            self.extra_cost = 0;
            self.accuracy = 40;
        }

        api.fighter_mut().modifiers.add(Modifier::new(ModKind::DmgMultiplier(1.10), Some(20)).with_tag("resplendent_punch_dmg_buff"));

        if damage.missed {
            api.emit_random_message(&[
                format!(
                    "**{}** errou o soco resplandecente em **{}**. Os Punhos Resplandecentes estão prontos para outra tentativa!",
                    fighter.name, target.name
                ),
                format!(
                    "**{}** tentou dar um soco resplandecente em **{}** mas não acertou! O resplandecer está mais comum agora.",
                    fighter.name, target.name
                ),
            ]);
        } else if critical {
            api.emit_random_message(&[
                format!(
                    "**{}** deu um soco resplandecente na cara de **{}** que causou **{damage}**!",
                    fighter.name, target.name
                ),
                format!(
                    "**{}** socou o estômago de **{}** com punhos resplandecentes e causou **{damage}**!",
                    fighter.name, target.name
                ),
            ]);
        } else {
            api.emit_random_message(&[
                format!(
                    "**{}** imbuiu ether nos punhos e deu um soco resplandecente em **{}** que causou **{damage}**!",
                    fighter.name, target.name
                ),
                format!(
                    "**{}** deu um soco resplandecente em **{}** que causou **{damage}**!",
                    fighter.name, target.name
                ),
            ]);
        }

        let target = api.target_mut();

        if target.resistance.value <= 0 && !target.flags.contains(FighterFlags::ASKED_TO_RISK_LIFE)
        {
            target.ether.value = (target.ether.value as f32 * 0.6) as i32;

            let target_name = target.name.clone();
            api.emit_random_message(&[
                format!("A força do soco resplandecente foi tanta que **{}** está semi-inconsciente e um muito do seu ether vazou.", target_name),
                format!("**{}** recebeu um soco tão forte que seu ether vazou.", target_name),
            ]);
        }

        Ok(())
    }
}
