use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Debug;

#[async_trait::async_trait]
impl Skill for Debug {
    fn kind(&self) -> SkillKind {
        SkillKind::Debug
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "debug",
            name: "Debug",
            description: "Debug.",
            explanation: "Debug skill",
            complexity: SkillComplexity::SuperMaster,
            use_cost: SkillCost { ether: 1 },
        }
    }

    fn ai_chance_to_pick(&self, _api: BattleApi<'_>) -> Probability {
        Probability::NEVER
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(15..=30);

        let multiplier = fighter.strength_multiplier();
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::Physical,
                amount: damage,
                balance_effectiveness: 20,
                accuracy: 100,
                effect: Some(Effect::new(EffectKind::Poisoned, 40, fighter.index)), 
                ..Default::default()
            },
        ).await;

        api.emit_message(format!("**{}** lançou o ataque debug que causou **{damage}**! +veneno", fighter.name));

        Ok(())
    }   
}
