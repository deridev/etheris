use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Suplex;

#[async_trait::async_trait]
impl Skill for Suplex {
    fn kind(&self) -> SkillKind {
        SkillKind::Suplex
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "suplex",
            name: "Suplex",
            description: "Agarra seu inimigo quando ele ter equilíbrio baixo e com um movimento ágil o joga no chão de cabeça.",
            explanation: "Requer pouco ether físico, mas gasta uma quantia extrema de energia para impulsionar seu corpo com velocidade para efetuar o movimento.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 25 },
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        api.target().balance < 80 && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(9..=15);
        let damage = api.rng().gen_range(12..=23);
        let damage = base_damage + (damage as f32 * fighter.strength_multiplier()) as i32;

        let paralyze = Probability::new(20).generate_random_bool();

        let damage = api.apply_damage(
            target.index, 
            DamageSpecifier { 
                kind: DamageKind::Physical, 
                amount: damage, 
                balance_effectiveness: 30, 
                accuracy: 90, 
                effect: if paralyze { Some(Effect::new(EffectKind::Paralyzed, 1, fighter.index)) } else { None }, 
                culprit: fighter.index 
            }
        ).await;

        if paralyze {
            api.emit_message(format!("**{}** deu suplex tão forte em **{}** que causou **{damage}** e paralisou seus músculos!", fighter.name, target.name));
        } else {
            api.emit_message(format!("**{}** deu um rápido suplex em **{}** que causou **{damage}**!", fighter.name, target.name));
        }

        Ok(())
    }
}
