use etheris_discord::{
    twilight_model::channel::message::component::ButtonStyle, ButtonBuilder, Emoji,
};
use etheris_macros::List;

use crate::{data::finishers::Finisher, BattleApi, Composure, FighterIndex, FighterSkill};

#[derive(Debug, Clone, PartialEq)]
pub enum BattleInput {
    Nothing,
    Reinput,
    ChangeTarget(FighterIndex),
    ChangeTeam(u8),
    Attack,
    Defend,
    UseSkill(FighterSkill),
    Finish(Finisher),
    GetUp,
    Upkick,
}

#[derive(List, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BattleInputKind {
    ChangeTarget,
    ChangeTeam,
    Attack,
    Defend,
    UseSkill,
    Finish,
    GetUp,
    Upkick,
}

impl BattleInputKind {
    pub fn can_use(&self, api: BattleApi<'_, '_>) -> bool {
        let is_standing = api.fighter().composure == Composure::Standing;

        let finish_threshold = if api.target().balance < 50 { 0.3 } else { 0.15 };
        let can_finish = (api.target().vitality.value as f32)
            <= ((api.target().vitality.max as f32) * finish_threshold);

        match self {
            Self::ChangeTarget | Self::ChangeTeam => api.battle().alive_fighters.len() > 2,
            Self::Attack | Self::Defend => is_standing,
            Self::UseSkill => is_standing && !api.fighter().skills.is_empty(),
            Self::Finish => {
                !api.fighter().finishers.is_empty()
                    && can_finish
                    && api.target().defense < 1
                    && api.fighter().composure == Composure::Standing
            }
            Self::GetUp | Self::Upkick => api.fighter().composure == Composure::OnGround,
        }
    }

    pub fn second_row(&self) -> bool {
        matches!(self, Self::ChangeTarget | Self::ChangeTeam)
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::ChangeTarget => "change_target",
            Self::ChangeTeam => "change_team",
            Self::Attack => "attack",
            Self::Defend => "defend",
            Self::UseSkill => "use_skill",
            Self::Finish => "finish",
            Self::GetUp => "get_up",
            Self::Upkick => "upkick",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ChangeTarget => "Trocar Alvo",
            Self::ChangeTeam => "Trocar Time",
            Self::Attack => "Atacar",
            Self::Defend => "Defender",
            Self::UseSkill => "Usar Habilidade",
            Self::Finish => "Finalizar",
            Self::GetUp => "Levantar",
            Self::Upkick => "Upkick (chutar)",
        }
    }

    pub fn emoji(&self) -> Emoji<'static> {
        match self {
            Self::ChangeTarget => Emoji::Unicode("🎯"),
            Self::ChangeTeam => Emoji::Unicode("🚩"),
            Self::Attack => Emoji::Unicode("👊"),
            Self::Defend => Emoji::Unicode("🛡️"),
            Self::UseSkill => Emoji::Unicode("🌀"),
            Self::Finish => Emoji::Unicode("⚔️"),
            Self::GetUp => Emoji::Unicode("🏋️"),
            Self::Upkick => Emoji::Unicode("🦵"),
        }
    }

    pub fn button(&self, api: BattleApi<'_, '_>) -> ButtonBuilder {
        if *self == Self::Attack {
            if let Some(weapon) = api.fighter().weapon {
                return weapon
                    .input_button()
                    .set_custom_id(self.id())
                    .set_style(ButtonStyle::Secondary);
            }
        }

        ButtonBuilder::new()
            .set_custom_id(self.id())
            .set_label(self.name())
            .set_emoji(self.emoji())
            .set_style(ButtonStyle::Secondary)
    }
}

impl From<BattleInputKind> for ButtonBuilder {
    fn from(value: BattleInputKind) -> Self {
        ButtonBuilder::new()
            .set_custom_id(value.id())
            .set_label(value.name())
            .set_emoji(value.emoji())
            .set_style(ButtonStyle::Secondary)
    }
}
