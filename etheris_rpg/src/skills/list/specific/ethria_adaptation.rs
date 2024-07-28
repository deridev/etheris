use super::super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct EthriaAdaptation;

#[async_trait::async_trait]
impl Skill for EthriaAdaptation {
    fn kind(&self) -> SkillKind {
        SkillKind::EthriaAdaptation
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "ethria_adaptation",
            name: "Adaptação de Ethria",
            description: "Se adapta a qualquer forma de dano conforme o recebe.",
            explanation: "Habilidade complexa de uma arrogante evolucionária.",
            complexity: SkillComplexity::Master,
            use_cost: SkillCost { ether: 0 },
        }
    }

    fn ai_chance_to_pick(&self, _api: BattleApi<'_>) -> Probability {
        Probability::NEVER
    }

    fn can_use(&self, _api: BattleApi<'_>) -> bool {
        false
    }

    async fn on_use(&mut self, _api: BattleApi<'_>) -> SkillResult<()> {
        Ok(())
    }

    async fn passive_on_damage(
        &mut self,
        mut api: BattleApi<'_>,
        damage: DamageSpecifier,
    ) -> SkillResult<()> {
        let mut immunity_kinds = match damage.kind {
            DamageKind::Cut => vec![ImmunityKind::Cut],
            DamageKind::Ice => vec![ImmunityKind::Ice],
            DamageKind::Electric => vec![ImmunityKind::Electric],
            DamageKind::Fire => vec![ImmunityKind::Fire],
            DamageKind::Poisonous => vec![ImmunityKind::Poison],
            DamageKind::Physical => vec![ImmunityKind::Physical],
            DamageKind::PhysicalCut => vec![ImmunityKind::Cut, ImmunityKind::Physical],
            DamageKind::Special => vec![ImmunityKind::Special],
            DamageKind::SpecialPhysical => vec![ImmunityKind::Physical, ImmunityKind::Special],
            DamageKind::Water => vec![ImmunityKind::Water],
            DamageKind::Wind => vec![],
        };

        if let Some(effect) = damage.effect {
            immunity_kinds.push(match effect.kind {
                EffectKind::Bleeding => ImmunityKind::Bleeding,
                EffectKind::Poisoned => ImmunityKind::Poison,
                EffectKind::Wet => ImmunityKind::Water,
                EffectKind::Frozen => ImmunityKind::Ice,
                EffectKind::Shocked => ImmunityKind::Electric,
                EffectKind::Burning | EffectKind::Flaming => ImmunityKind::Fire,
                _ => ImmunityKind::Special,
            });
        }

        for immunity in immunity_kinds {
            api.fighter_mut()
                .body_immunities
                .increase_resistance(immunity, 0.09);
            api.fighter_mut().overload += 0.25;
        }

        Ok(())
    }
}
