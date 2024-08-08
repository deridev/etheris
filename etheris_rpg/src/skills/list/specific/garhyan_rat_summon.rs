use data::{enemies::EnemyPotential, Reward};
use etheris_common::Attribute;
use etheris_data::{personality::Personality, BrainKind};

use super::super::prelude::*;

#[derive(Debug, Clone)]
pub struct GarhyanRatSummon {
    rat_ammo: u8,
}

impl Default for GarhyanRatSummon {
    fn default() -> Self {
        Self { rat_ammo: 2 }
    }
}

#[async_trait::async_trait]
impl Skill for GarhyanRatSummon {
    fn kind(&self) -> SkillKind {
        SkillKind::GarhyanRatSummon
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "garhyan_rat_summon",
            name: "Invocação de Ratos",
            description: "Cria ratos gigantes materializados a partir de ratos comuns.",
            explanation: "Habilidade repulsiva mas fascinante de um nobre caído.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 25 },
        }
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_>) -> Probability {
        if api.get_fighter_allies(api.fighter_index).len() < 2 {
            Probability::new(100)
        } else {
            Probability::new(50)
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        let team_size = api.get_fighter_allies(api.fighter_index).len();
        self.rat_ammo > 0 && self.default_can_use(api) && team_size < 3
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        self.rat_ammo = self.rat_ammo.saturating_sub(1);

        let team = api.fighter().team;

        api.battle_mut().join_fighter(FighterData {
            team,
            name: "Rato Invocado".to_string(),
            user: None,
            boss: None,
            brain: Some(BrainKind::Insane),
            actions: vec![],
            inventory: vec![],
            personalities: vec![Personality::Cowardice, Personality::Aggressiveness],
            skills: vec![SkillKind::Bite, SkillKind::Charge, SkillKind::DefensiveJump],
            strength_level: 16,
            intelligence_level: 6,
            potential: EnemyPotential::Low.to_f64(),
            immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Poison),
            weapon: None,
            resistance: Attribute::from(280),
            vitality: Attribute::from(75),
            ether: Attribute::from(25),
            drop: Reward::default(),
            pacts: vec![],
        });

        api.emit_message(format!(
            "**{}** invocou um rato para ajudar na batalha!",
            api.fighter().name
        ));

        let overload = api.rng().gen_range(3.0..=7.0);
        api.add_overload(api.fighter_index, overload).await;

        Ok(())
    }
}
