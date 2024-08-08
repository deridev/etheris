pub mod list;

use etheris_data::PactKind;

use crate::{common::DamageSpecifier, BattleApi, Fighter};

pub type PactResult<T> = anyhow::Result<T>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PactData {
    pub identifier: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub explanation: &'static str,
}

#[async_trait::async_trait]
pub trait Pact {
    fn kind(&self) -> PactKind;
    fn save_kind(&self) -> PactKind {
        self.kind()
    }

    fn data(&self, fighter: &Fighter) -> PactData;

    fn setup_fighter(&mut self, _fighter: &mut Fighter) -> PactResult<()> {
        Ok(())
    }

    fn modify_damage(&mut self, _damage: &mut DamageSpecifier) -> PactResult<()> {
        Ok(())
    }

    async fn fighter_tick(&mut self, _api: BattleApi<'_>) -> PactResult<()> {
        Ok(())
    }

    async fn on_damage(&mut self, _api: BattleApi<'_>, _damage: DamageSpecifier) -> PactResult<()> {
        Ok(())
    }
}
