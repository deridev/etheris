use super::prelude::*;

#[derive(Debug, Clone)]
pub struct Overcoming {
    multiplier: f32
}

impl Default for Overcoming {
    fn default() -> Self {
        Self { multiplier: 0.4 }
    }
}

const TAG: &str = "overcoming_multiplier";

#[async_trait::async_trait]
impl Skill for Overcoming {
    fn kind(&self) -> SkillKind {
        SkillKind::Overcoming
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "overcoming",
            name: "Superação",
            description: "Começa a batalha com seu poder pela metade, mas com o passar do tempo vai ficando mais e mais forte, até ficar mais forte que o normal.",
            explanation: "Usa a metaconsciência do ether: se impôr uma limitação te concede mais ether e poder com o tempo.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 0 },
        }
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);

        display.sub_header.push_str(&format!("\n**Poder**: {}%", (self.multiplier * 100.0) as i32));

        display
    }


    fn can_use(&self, _api: BattleApi<'_>) -> bool {
        false
    }

    async fn on_start(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        api.fighter_mut().modifiers.add(Modifier::new(ModKind::DmgMultiplier(self.multiplier), None).with_tag(TAG));
        Ok(())
    }

    async fn passive_fighter_tick(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        self.multiplier = (self.multiplier + 0.125).clamp(0.0, 2.25);
        
        let fighter = api.fighter_mut();
        if let Some(modifier) = fighter.modifiers.get_mut_with_tag(TAG) {
            *modifier = Modifier::new(ModKind::DmgMultiplier(self.multiplier), None).with_tag(TAG);
        } else {
            fighter.modifiers.add(Modifier::new(ModKind::DmgMultiplier(self.multiplier), None).with_tag(TAG));
        }

        Ok(())
    }


    async fn on_use(&mut self, _api: BattleApi<'_>) -> SkillResult<()> {
        Ok(())
    }
}
