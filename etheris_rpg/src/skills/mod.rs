pub mod list;

use std::fmt::Display;

use etheris_common::Probability;
use etheris_data::{emojis, SkillKind};

use crate::{common::DamageSpecifier, BattleApi, Fighter, FighterIndex};

pub type SkillResult<T> = anyhow::Result<T>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillCost {
    pub ether: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SkillComplexity {
    VerySimple,
    Simple,
    Normal,
    Hard,
    VeryHard,
    UltraHard,
    BeginnerMaster,
    Master,
    SuperMaster,
}

impl Display for SkillComplexity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VerySimple => f.write_str("Muito Simples"),
            Self::Simple => f.write_str("Simples"),
            Self::Normal => f.write_str("Normal"),
            Self::Hard => f.write_str("Difícil"),
            Self::VeryHard => f.write_str("Muito Difícil"),
            Self::UltraHard => f.write_str("Ultra Difícil"),
            Self::BeginnerMaster => f.write_str("Mestre Iniciante"),
            Self::Master => f.write_str("Mestre"),
            Self::SuperMaster => f.write_str("Super Mestre"),
        }
    }
}

impl SkillComplexity {
    pub fn prob_of_aknowleding(&self) -> Probability {
        match self {
            Self::VerySimple => Probability::new(50),
            Self::Simple => Probability::new(40),
            Self::Normal => Probability::new(30),
            Self::Hard => Probability::new(10),
            Self::VeryHard => Probability::new(5),
            Self::UltraHard => Probability::new(3),
            Self::BeginnerMaster => Probability::new(2),
            Self::Master => Probability::new(1),
            Self::SuperMaster => Probability::new(1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillData {
    pub identifier: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub explanation: &'static str,
    pub complexity: SkillComplexity,
    pub use_cost: SkillCost,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillDisplay {
    pub header: String,
    pub sub_header: String,
    pub body: String,
}

#[async_trait::async_trait]
pub trait Skill {
    fn kind(&self) -> SkillKind;
    fn save_kind(&self) -> SkillKind {
        self.kind()
    }

    fn data(&self, fighter: &Fighter) -> SkillData;

    fn default_display(&self, fighter: &Fighter) -> SkillDisplay {
        SkillDisplay {
            header: self.data(fighter).name.to_owned(),
            sub_header: format!(
                "**{} {}**",
                emojis::ETHER,
                self.data(fighter).use_cost.ether
            ),
            body: self.data(fighter).description.to_owned(),
        }
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        self.default_display(fighter)
    }

    fn default_can_use(&self, api: BattleApi<'_>) -> bool {
        api.fighter().ether.value >= self.data(api.fighter()).use_cost.ether
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        self.default_can_use(api)
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_>) -> Probability {
        if self.can_use(api) {
            Probability::new(50)
        } else {
            Probability::NEVER
        }
    }

    async fn on_start(&mut self, _api: BattleApi<'_>) -> SkillResult<()> {
        Ok(())
    }

    async fn passive_on_cycle(&mut self, _api: BattleApi<'_>) -> SkillResult<()> {
        Ok(())
    }

    async fn passive_fighter_tick(&mut self, _api: BattleApi<'_>) -> SkillResult<()> {
        Ok(())
    }

    async fn passive_on_kill(
        &mut self,
        _api: BattleApi<'_>,
        _killed: FighterIndex,
    ) -> SkillResult<()> {
        Ok(())
    }

    async fn passive_on_damage(
        &mut self,
        _api: BattleApi<'_>,
        _damage: DamageSpecifier,
    ) -> SkillResult<()> {
        Ok(())
    }

    async fn passive_on_damage_miss(
        &mut self,
        _api: BattleApi<'_>,
        _damage: DamageSpecifier,
    ) -> SkillResult<()> {
        Ok(())
    }

    async fn on_use(&mut self, _: BattleApi<'_>) -> SkillResult<()> {
        Ok(())
    }
}
