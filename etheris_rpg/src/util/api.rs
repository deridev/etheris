use std::fmt::Display;

use etheris_common::Probability;
use etheris_data::emojis;
use etheris_framework::CommandContext;
use rand::{seq::SliceRandom, Rng};

use crate::*;

use self::common::{DamageKind, DamageSpecifier};

pub struct BattleApi<'a, 'b> {
    pub fighter_index: FighterIndex,
    pub target_index: FighterIndex,
    pub controller: &'a mut BattleController<'b>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EffectiveDamage {
    pub damage_specifier: DamageSpecifier,
    pub amount: i32,
    pub defended: bool,
    pub missed: bool,
    pub dodged: bool,
}

impl Display for EffectiveDamage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} dano{}",
            self.amount,
            if self.dodged {
                " (DESVIOU)".to_string()
            } else if self.missed {
                " (ERROU)".to_string()
            } else if self.defended {
                format!(" ({})", emojis::SHIELD)
            } else {
                String::new()
            }
        )
    }
}

impl<'a, 'b> BattleApi<'a, 'b> {
    pub fn new(controller: &'a mut BattleController<'b>) -> Self {
        Self {
            fighter_index: controller.battle.get_current_fighter().index,
            target_index: controller.battle.get_target_fighter().index,
            controller,
        }
    }

    #[inline(always)]
    pub fn ctx(&mut self) -> &mut CommandContext {
        self.controller.ctx
    }

    #[inline(always)]
    pub fn battle(&self) -> &Battle {
        &self.controller.battle
    }

    #[inline(always)]
    pub fn battle_mut(&mut self) -> &mut Battle {
        &mut self.controller.battle
    }

    pub fn fighter(&self) -> &Fighter {
        self.battle().get_fighter(self.fighter_index)
    }

    pub fn fighter_mut(&mut self) -> &mut Fighter {
        let index = self.fighter_index;
        self.battle_mut().get_fighter_mut(index)
    }

    pub fn target(&self) -> &Fighter {
        self.battle().get_fighter(self.target_index)
    }

    pub fn target_mut(&mut self) -> &mut Fighter {
        let index = self.target_index;
        self.battle_mut().get_fighter_mut(index)
    }

    pub fn rng(&mut self) -> &mut impl Rng {
        &mut self.controller.battle.rng
    }

    pub fn emit_message(&mut self, message: impl ToString) {
        self.controller.emit_turn_message(message.to_string());
    }

    pub fn defer_message(&mut self, message: impl ToString) {
        self.controller
            .battle
            .deferred_turn_messages
            .push(message.to_string());
    }

    pub fn emit_random_message(&mut self, messages: &[impl ToString]) {
        self.controller.emit_random_turn_message(messages)
    }

    pub fn report_error(&mut self, message: impl ToString) {
        self.emit_message(format!(
            "⚠️ **Erro Etheris**: `{}`. Erro no turno de **{}**.",
            message.to_string(),
            self.fighter().name
        ));
    }

    pub fn get_fighter_allies(&self, fighter: FighterIndex) -> Vec<Fighter> {
        let teams = self.battle().teams();
        let fighter = self.battle().get_fighter(fighter);

        let Some(ally_team) = teams.get(&fighter.team) else {
            return vec![fighter.clone()];
        };

        let ally_team = ally_team
            .iter()
            .map(|index| self.battle().get_fighter(*index).clone())
            .collect::<Vec<_>>();
        ally_team
    }

    pub async fn add_overload(
        &mut self,
        target_index: FighterIndex,
        amount: f64,
    ) {
        let fighter = self.battle_mut().get_fighter_mut(target_index);
        let base_overload = fighter.overload;
        fighter.overload += amount;

        let fighter = fighter.clone();
        if base_overload <= 5.0 && fighter.overload >= 5.0 {
            self.emit_message(format!("O corpo de **{}** está sobrecarregando pelo uso de ether!", fighter.name));
        } else if base_overload <= 15.0 && fighter.overload >= 15.0 {
            self.emit_message(format!("**{}** sentiu os orgãos doerem de tanta sobrecarga!", fighter.name));
        } else if base_overload <= 50.0 && fighter.overload >= 50.0 {
            self.emit_message(format!("O cérebro de **{}** está sobreaquecendo!", fighter.name));
        } else if base_overload <= 150.0 && fighter.overload >= 150.0 {
            self.emit_message(format!("O ether de **{}** está totalmente fora de controle! A regeneração de ether está desativada até a sobrecarga baixar de 150%.", fighter.name));
        } else if base_overload <= 400.0 && fighter.overload >= 400.0 {
            self.emit_message(format!("**{}** está quase cedendo de tanta sobrecarga. 500% de sobrecarga é morte na hora!", fighter.name));
        }

        if fighter.overload >= 150.0 {
            self.battle_mut().get_fighter_mut(target_index).flags.insert(FighterFlags::CANNOT_REGEN_ETHER_OVERLOAD);
        } else {
            self.battle_mut().get_fighter_mut(target_index).flags.remove(FighterFlags::CANNOT_REGEN_ETHER_OVERLOAD);
        }

        if base_overload <= 100.0 && fighter.overload >= 100.0 {
            let overload_damage = self.rng().gen_range(30..=50);
            let overload_damage = (overload_damage as f32 * fighter.intelligence_multiplier() * 0.9) as i32;

            let dmg = self.apply_damage(fighter.index, DamageSpecifier { 
                kind: DamageKind::Special, 
                amount: overload_damage, 
                balance_effectiveness: 20, 
                accuracy: 100, 
                effect: Some(Effect::new(EffectKind::Bleeding, 30, fighter.index)), 
                culprit: fighter.index
            }).await;

            self.emit_message(format!("**{}** recebeu **{dmg}** de sobrecarga no ether.", fighter.name))
        }

        if fighter.overload >= 500.0 {
            let fighter = self.battle_mut().get_fighter_mut(target_index);
            fighter.vitality.value = 0;
            fighter.resistance.value = 0;
            fighter.ether.value = 0;

            let fighter = fighter.clone();
            self.emit_message(format!("**{}** morreu de tanta sobrecarga. Seu ether interno implodiu e destruiu todos seus orgãos.", fighter.name))
        }
    }

    pub async fn apply_damage(
        &mut self,
        target_index: FighterIndex,
        mut damage: DamageSpecifier,
    ) -> EffectiveDamage {
        let culprit = self.battle().get_fighter(damage.culprit).clone();
        let culprit_index = culprit.index;

        let mut missed = false;
        let mut dodged = false;
        let mut defended = false;

        let target = self.battle_mut().get_fighter_mut(target_index);

        if target.defense > 0 {
            defended = true;
            damage.amount = ((damage.amount as f32) * 0.6) as i32;
        }

        if target.has_effect(EffectKind::LowProtection) {
            defended = true;
            damage.amount = ((damage.amount as f32) * 0.6) as i32;
        }

        if culprit.balance < 85 {
            let accuracy_loss = if culprit.balance < 20 {
                30
            } else if culprit.balance < 50 {
                15
            } else if culprit.balance < 70 {
                5
            } else {
                3
            };

            damage.accuracy = damage.accuracy.saturating_sub(accuracy_loss);
        }

        if damage.accuracy < 100 {
            let mut dodge_prob = Probability::new(if target.balance > 90 { 5 } else { 0 });
            if target.has_effect(EffectKind::Paralyzed) || target.has_effect(EffectKind::Frozen) {
                dodge_prob = Probability::NEVER;
            }

            let unlucky_miss_prob = Probability::new(2);
            let bad_accuracy_prob = Probability::new(
                100u8.saturating_sub(damage.accuracy + if target.balance > 80 { 15 } else { 0 }),
            );

            let good_accuracy =
                Probability::new(if target.balance > 90 { 5 } else { 15 }).generate_random_bool();

            if !good_accuracy
                && (bad_accuracy_prob.generate_random_bool()
                    || unlucky_miss_prob.generate_random_bool())
            {
                missed = true;
            } else if dodge_prob.generate_random_bool() {
                missed = true;
                dodged = true;
            }
        }

        if missed {
            damage.amount = 0;
            damage.balance_effectiveness = 0;
            damage.effect = None;
        }

        if target.has_effect(EffectKind::Wet) {
            if damage.kind.is_physical() {
                damage.amount = (damage.amount as f32 * 1.25) as i32;
            }

            damage.balance_effectiveness = damage.balance_effectiveness.saturating_add(8);
        }

        if damage.amount > (target.resistance.value + target.vitality.value) / 2 {
            damage.balance_effectiveness = damage.balance_effectiveness.saturating_mul(3);
        }

        let target_name = target.name.clone();

        target.take_damage(culprit_index, damage);
        target.balance = target.balance.saturating_sub(damage.balance_effectiveness);

        let falling_prob = match target.balance {
            0..=10 => 0.7,
            11..=30 => 0.5,
            31..=50 => 0.3,
            51..=80 => 0.1,
            81..=90 => 0.02,
            _ => 0.0,
        };

        self.battle_mut()
            .turn_end_queues
            .damages
            .push((damage, target_index));

        if missed {
            self.battle_mut()
                .turn_end_queues
                .damage_misses
                .push((damage, target_index));
        }

        let has_fallen = self.battle_mut().rng.gen_bool(falling_prob);

        let target = self.battle_mut().get_fighter(target_index).clone();

        if !missed
            && target.composure == Composure::Standing
            && has_fallen
            && !matches!(damage.kind, DamageKind::Special | DamageKind::Cut)
        {
            self.battle_mut().get_fighter_mut(target_index).composure = Composure::OnGround;

            let message = [
                format!("**{}** caiu no chão!", target_name),
                format!("**{}** perdeu o equilíbrio e caiu no chão!", target_name),
                format!("**{}** foi lançado ao chão!", target_name),
                if target.has_effect(EffectKind::Wet) {
                    format!("**{}** deslizou e caiu no chão!", target_name)
                } else {
                    format!("**{}** caiu de costas no chão!", target_name)
                },
            ]
            .choose(self.rng())
            .cloned()
            .unwrap_or_default();

            self.battle_mut().deferred_turn_messages.push(message);
        }

        if let Some(effect) = damage.effect {
            self.apply_effect(target_index, effect).await;
        }

        if matches!(
            damage.kind,
            DamageKind::Physical | DamageKind::PhysicalCut | DamageKind::SpecialPhysical
        ) && target.has_effect(EffectKind::Shocked)
        {
            self.battle_mut()
                .get_fighter_mut(target.index)
                .remove_effect(Effect::new(EffectKind::Shocked, 10, culprit_index));
            self.apply_effect(
                culprit_index,
                Effect::new(EffectKind::Shocked, 20, target_index),
            )
            .await;
        }

        EffectiveDamage {
            damage_specifier: damage,
            amount: damage.amount,
            defended,
            missed,
            dodged,
        }
    }

    pub async fn apply_effect(&mut self, target_index: FighterIndex, effect: Effect) {
        let target = self.battle_mut().get_fighter_mut(target_index);
        let target_name = target.name.clone();
        let should_emit_message = target.apply_effect(effect);

        let message = match effect.kind {
            EffectKind::Flaming => format!("**{}** está queimando!", target_name),
            EffectKind::Burning => format!("**{}** entrou em combustão!", target_name),
            EffectKind::Shocked => format!("**{}** está em choque!", target_name),
            EffectKind::Paralyzed => format!("**{}** paralisou!", target_name),
            EffectKind::Ice => format!("**{}** está congelando!", target_name),
            EffectKind::Wet => format!("**{}** está molhado!", target_name),
            EffectKind::Frozen => format!("**{}** congelou!", target_name),
            EffectKind::Bleeding => format!("**{}** começou a sangrar!", target_name),
            EffectKind::LowProtection => format!(
                "**{}** está com uma proteção extra a danos leves!",
                target_name
            ),
        };

        if should_emit_message {
            self.battle_mut().deferred_turn_messages.push(message)
        };

        let target = self.battle_mut().get_fighter_mut(target_index);

        if let Some(flaming) = target.get_effect(EffectKind::Flaming) {
            if flaming.amount >= 100 {
                target.remove_effect(flaming);
                target.apply_effect(Effect {
                    amount: 20,
                    kind: EffectKind::Burning,
                    culprit: flaming.culprit,
                });

                self.defer_message(format!(
                    "**{}** queimou tanto que entrou em combustão!",
                    target_name
                ));
            }
        }

        let target = self.battle_mut().get_fighter_mut(target_index);

        if let Some(ice) = target.get_effect(EffectKind::Ice) {
            if ice.amount >= 100 {
                target.remove_effect(ice);
                target.apply_effect(Effect {
                    amount: 1,
                    kind: EffectKind::Frozen,
                    culprit: ice.culprit,
                });

                self.battle_mut()
                    .deferred_turn_messages
                    .push(format!("**{}** congelou completamente!", target_name));
            }
        }

        let target = self.battle_mut().get_fighter_mut(target_index);

        if let Some(shock) = target.get_effect(EffectKind::Shocked) {
            if shock.amount >= 100 {
                target.remove_effect(shock);
                target.apply_effect(Effect {
                    amount: 1,
                    kind: EffectKind::Paralyzed,
                    culprit: shock.culprit,
                });

                self.battle_mut()
                    .deferred_turn_messages
                    .push(format!("**{}** paralisou devido ao choque!", target_name));
            }
        }
    }
}
