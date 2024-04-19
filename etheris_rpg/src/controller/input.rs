use etheris_data::items::Item;
use etheris_database::character_model::BattleAction;
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
    GetUp,
    Upkick,
    UseItem(Item),
    UseAction(BattleAction),
    Finish(Finisher),
}

#[derive(List, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BattleInputKind {
    ChangeTarget,
    ChangeTeam,
    Attack,
    Defend,
    UseSkill,
    GetUp,
    Upkick,
    UseItem,
    Actions,
    Finish,
}

impl BattleInputKind {
    pub fn can_use(&self, api: BattleApi<'_>) -> bool {
        let is_not_on_ground = api.fighter().composure != Composure::OnGround;
        match self {
            Self::ChangeTarget | Self::ChangeTeam => api.battle().alive_fighters.len() > 2,
            Self::Attack | Self::Defend => is_not_on_ground,
            Self::UseSkill => is_not_on_ground && !api.fighter().skills.is_empty(),
            Self::Finish => api.can_finish_target(),
            Self::GetUp | Self::Upkick => api.fighter().composure == Composure::OnGround,
            Self::UseItem => !api.fighter().inventory.is_empty(),
            Self::Actions => !api.fighter().actions.is_empty(),
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
            Self::UseItem => "use_item",
            Self::Actions => "actions",
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
            Self::UseItem => "Usar Item",
            Self::Actions => "Ações",
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
            Self::UseItem => Emoji::Unicode("🎒"),
            Self::Actions => Emoji::Unicode("🏃"),
        }
    }

    pub fn button_style(&self) -> ButtonStyle {
        match self {
            Self::Finish => ButtonStyle::Danger,
            _ => ButtonStyle::Secondary,
        }
    }

    pub fn button(&self, api: BattleApi<'_>) -> ButtonBuilder {
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
            .set_style(self.button_style())
    }
}

impl From<BattleInputKind> for ButtonBuilder {
    fn from(value: BattleInputKind) -> Self {
        ButtonBuilder::new()
            .set_custom_id(value.id())
            .set_label(value.name())
            .set_emoji(value.emoji())
            .set_style(value.button_style())
    }
}
