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
        let _target = api.target().clone();

        api.fighter_mut().modifiers.add(Modifier::new(ModKind::DefenseMultiplier(0.0), Some(15)).with_tag("debug_immunity"));

        api.emit_message(format!("**{}** ganhou 15 rounds de imunidade a tudo.", fighter.name));

        Ok(())
    }   
}
