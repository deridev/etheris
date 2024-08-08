use etheris_common::Probability;
use etheris_data::weapon::WeaponKind;

use crate::{BattleApi, Effect, EffectKind, FighterFlags};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Finisher {
    Knockout,
    BreakNeck,
    StabNeck,
    SmashSkull,
    PierceHeart,
    Decapitate,
}

impl Finisher {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Knockout => "Nocaute",
            Self::BreakNeck => "Quebrar Pescoço",
            Self::StabNeck => "Esfaquear Pescoço",
            Self::SmashSkull => "Esmagar Crânio",
            Self::PierceHeart => "Perfurar Coração",
            Self::Decapitate => "Decapitar",
        }
    }

    pub fn fail_probability(&self) -> Probability {
        match self {
            Self::Knockout => Probability::new(10),
            Self::BreakNeck => Probability::new(30),
            Self::StabNeck => Probability::new(30),
            Self::SmashSkull => Probability::new(40),
            Self::PierceHeart => Probability::new(40),
            Self::Decapitate => Probability::new(40),
        }
    }

    pub const fn is_fatal(&self) -> bool {
        !matches!(self, Self::Knockout)
    }

    pub const fn get_weapon_finishers(weapon: WeaponKind) -> &'static [Self] {
        match weapon {
            WeaponKind::Stick => &[],
            WeaponKind::Knife => &[Self::StabNeck],
            WeaponKind::Bat => &[Self::SmashSkull],
            WeaponKind::Spear => &[Self::StabNeck, Self::PierceHeart],
            WeaponKind::Katana => &[Self::Decapitate, Self::StabNeck, Self::PierceHeart],
            WeaponKind::EthriaKatana => &[Self::Decapitate, Self::StabNeck, Self::PierceHeart],
            WeaponKind::Umbrella => &[Self::SmashSkull, Self::PierceHeart],
            WeaponKind::ScorpionFang => &[Self::PierceHeart, Self::StabNeck],
            WeaponKind::IceBat => &[Self::SmashSkull],
        }
    }

    pub async fn execute_finisher(&self, mut api: BattleApi<'_>) -> anyhow::Result<()> {
        api.target_mut().is_defeated = true;
        api.target_mut().defeated_by = Some(api.fighter_index);
        api.target_mut().resistance.value = 0;
        api.target_mut()
            .flags
            .insert(FighterFlags::HAD_A_NEAR_DEATH_EXPERIENCE);

        if self.is_fatal() {
            api.target_mut().vitality.value = 0;
            api.target_mut().killed_by = Some(api.fighter_index);
        }

        match self {
            Self::Knockout => {
                api.emit_message(format!(
                    "**{}** aplicou golpes precisos em **{}**, nocauteando.",
                    api.fighter().name,
                    api.target().name
                ));
            }
            Self::BreakNeck => {
                api.emit_message(format!(
                    "**{}** quebrou o pescoço de **{}**.",
                    api.fighter().name,
                    api.target().name
                ));
            }
            Self::StabNeck => {
                api.emit_message(format!(
                    "**{}** esfaqueou o pescoço de **{}** repetidas vezes.",
                    api.fighter().name,
                    api.target().name
                ));
            }
            Self::SmashSkull => {
                api.apply_effect(
                    api.target_index,
                    Effect::new(EffectKind::Bleeding, 300, api.fighter_index),
                )
                .await;
                api.emit_random_message(&[
                    format!(
                        "**{}** esmagou o crânio de **{}** com muita força.",
                        api.fighter().name,
                        api.target().name
                    ),
                    format!(
                        "**{}** bateu na cabeça de **{}** várias vezes até o crânio rachar.",
                        api.fighter().name,
                        api.target().name
                    ),
                    format!(
                        "**{}** arrebentou o crânio de **{}**.",
                        api.fighter().name,
                        api.target().name
                    ),
                ])
            }
            Self::PierceHeart => {
                api.emit_random_message(&[
                    format!(
                        "**{}** perfurou o peito de **{}** e atravessou seu coração.",
                        api.fighter().name,
                        api.target().name
                    ),
                    format!(
                        "**{}** atravessou o coração de **{}** com uma perfuração rápida.",
                        api.fighter().name,
                        api.target().name
                    ),
                ]);
            }
            Self::Decapitate => {
                api.emit_random_message(&[
                    format!("**{}** fez a cabeça de **{}** voar com um corte rápido no pescoço!", api.fighter().name, api.target().name),
                    format!("**{}** decapitou **{}** com um corte preciso! A cabeça cai no chão e encerra a vida de {}.", api.fighter().name, api.target().name, api.target().name),
                ]);
            }
        }

        Ok(())
    }
}
